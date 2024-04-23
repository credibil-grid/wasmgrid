//! # NATS Messaging Runtime
//!
//! This module implements a NATS wasi:messaging runtime.

use std::collections::HashMap;
use std::pin::Pin;
use std::task::{Context, Poll};

use anyhow::anyhow;
use bytes::Bytes;
use futures::stream::{Stream, StreamExt};
use http_body_util::BodyExt;
use hyper::body::Incoming;
use hyper::Request;
use wasi_messaging::bindings::wasi::messaging::messaging_types::{
    Error, FormatSpec, GuestConfiguration, Message,
};
use wasi_messaging::bindings::Messaging;
use wasi_messaging::{self, MessagingView, RuntimeClient, RuntimeSubscriber};
use wasmtime::component::{Component, InstancePre, Linker, Resource};
use wasmtime::StoreLimits; // StoreLimitsBuilder
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::body::HyperOutgoingBody;
use wasmtime_wasi_http::proxy::{self, Proxy};
use wasmtime_wasi_http::{hyper_response_error, WasiHttpCtx, WasiHttpView};

// HandlerProxy is a proxy for the wasm messaging Host, wrapping calls to the Guest's
// messaging API.
#[derive(Clone)]
pub struct HandlerProxy {
    engine: Engine,
    instance_pre: InstancePre<Host>,
}

impl HandlerProxy {
    // Create a new HandlerProxy for the specified wasm Guest.
    pub fn new(wasm: String) -> anyhow::Result<Self> {
        let mut config = Config::new();
        config.async_support(true);
        let engine = Engine::new(&config)?;

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_async(&mut linker)?;

        // link specific runtime modules
        Messaging::add_to_linker(&mut linker, |t| t)?;
        proxy::add_only_http_to_linker(&mut linker)?;

        let component = Component::from_file(&engine, wasm)?;
        let instance_pre = linker.instantiate_pre(&component)?;

        Ok(Self { engine, instance_pre })
    }

    // Return the list of channels the Guest wants to subscribe to.
    pub async fn channels(&self) -> anyhow::Result<Vec<String>> {
        let mut store = Store::new(&self.engine, Host::new());
        let (messaging, _) = Messaging::instantiate_pre(&mut store, &self.instance_pre).await?;

        let gc = match messaging.wasi_messaging_messaging_guest().call_configure(&mut store).await?
        {
            Ok(gc) => gc,
            Err(e) => {
                // let err = store.data_mut().table().get(&e)?;
                let err = WasiView::table(store.data_mut()).get(&e)?;
                return Err(anyhow!(err.to_string()));
            }
        };

        Ok(gc.channels)
    }

    // Forward NATS message to the wasm Guest.
    pub async fn message(&self, client: Client, message: Message) -> anyhow::Result<()> {
        // set up host state
        let mut host = Host::new();

        // add client to ResourceTable
        host.add_client(client)?;

        let mut store = Store::new(&self.engine, host);
        let (messaging, _) = Messaging::instantiate_pre(&mut store, &self.instance_pre).await?;

        // call guest with message
        if let Err(e) =
            messaging.wasi_messaging_messaging_guest().call_handler(&mut store, &[message]).await?
        {
            // let err = store.data_mut().table().get(&e)?;
            let err = WasiView::table(store.data_mut()).get(&e)?;
            return Err(anyhow!(err.to_string()));
        }

        Ok(())
    }

    // Forward NATS message to the wasm Guest.
    pub async fn request(
        self, request: Request<Incoming>,
    ) -> anyhow::Result<hyper::Response<HyperOutgoingBody>> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let engine = self.engine.clone();
        // let req_id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let instance_pre = self.instance_pre.clone();

        let task = tokio::spawn(async move {
            let mut store = Store::new(&engine, Host::new());
            store.limiter(|t| &mut t.limits);

            let (parts, body) = request.into_parts();
            let req = hyper::Request::from_parts(parts, body.map_err(hyper_response_error).boxed());

            let req = store.data_mut().new_incoming_request(req)?;
            let out = store.data_mut().new_response_outparam(sender)?;

            let (proxy, _) = Proxy::instantiate_pre(&mut store, &instance_pre).await?;

            // call guest with request
            proxy.wasi_http_incoming_handler().call_handle(&mut store, req, out).await
        });

        match receiver.await {
            Ok(Ok(resp)) => Ok(resp),
            Ok(Err(e)) => Err(e.into()),
            Err(_) => {
                // An error in the receiver (`RecvError`) only indicates that the
                // task exited before a response was sent (i.e., the sender was
                // dropped); it does not describe the underlying cause of failure.
                // Instead we retrieve and propagate the error from inside the task
                // which should more clearly tell the user what went wrong. Note
                // that we assume the task has already exited at this point so the
                // `await` should resolve immediately.
                let e = match task.await {
                    Ok(r) => {
                        r.expect_err("if the receiver has an error, the task must have failed")
                    }
                    Err(e) => e.into(),
                };

                Err(anyhow!("guest never invoked `response-outparam::set` method: {e:?}"))
            }
        }
    }
}

// Host implements messaging host interfaces. In addition, it holds the host-defined
// state used by the wasm runtime [`Store`].
struct Host {
    keys: HashMap<String, u32>,
    table: ResourceTable,
    ctx: WasiCtx,
    http_ctx: WasiHttpCtx,
    limits: StoreLimits,
}

impl Host {
    // Create a new Host instance.
    fn new() -> Self {
        Self {
            keys: HashMap::default(),
            table: ResourceTable::default(),
            ctx: WasiCtxBuilder::new().inherit_args().inherit_env().inherit_stdio().build(),
            http_ctx: WasiHttpCtx {},
            limits: StoreLimits::default(),
        }
    }

    // Add a new client to the host state.
    fn add_client(&mut self, client: Client) -> anyhow::Result<Resource<wasi_messaging::Client>> {
        let name = client.name.clone();
        let client: wasi_messaging::Client = Box::new(client);

        let resource = self.table.push(client)?;
        self.keys.insert(name, resource.rep());

        Ok(resource)
    }
}

// Implement the [`wasi_messaging::MessagingView`]` trait for Host.
#[async_trait::async_trait]
impl MessagingView for Host {
    async fn connect(&mut self, name: String) -> anyhow::Result<Resource<wasi_messaging::Client>> {
        let resource = if let Some(key) = self.keys.get(&name) {
            // reuse existing connection
            Resource::new_own(*key)
        } else {
            // create a new connection
            let client = Client::connect(name.clone()).await?;
            self.add_client(client)?
        };

        Ok(resource)
    }

    // TODO: implement update_configuration
    async fn update_configuration(
        &mut self, _gc: GuestConfiguration,
    ) -> anyhow::Result<(), Resource<Error>> {
        println!("TODO: update_configuration");
        Ok(())
    }
}

// Implement the [`wasmtime_wasi::ctx::WasiView`] trait for Host.
impl WasiView for Host {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

impl WasiHttpView for Host {
    fn table(&mut self) -> &mut wasmtime::component::ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.http_ctx
    }
}

// Client holds a reference to the the NATS client. It is used to implement the
// [`wasi_messaging::RuntimeClient`] trait used by the messaging Host.
#[derive(Clone)]
pub struct Client {
    name: String,
    inner: async_nats::Client,
}

impl Client {
    // Create a new Client for the specified NATS server.
    pub async fn connect(name: String) -> anyhow::Result<Self> {
        let inner = async_nats::connect(&name).await?;
        Ok(Self { name, inner })
    }
}

// Implement the [`wasi_messaging::RuntimeClient`] trait for Client. This trait
// implementation is used by the messaging Host to interact with the NATS client.
#[async_trait::async_trait]
impl RuntimeClient for Client {
    async fn subscribe(&self, ch: String) -> anyhow::Result<wasi_messaging::Subscriber> {
        let subscriber = Subscriber {
            inner: self.inner.subscribe(ch).await?,
        };
        Ok(Box::pin(subscriber))
    }

    async fn publish(&self, ch: String, data: Bytes) -> anyhow::Result<()> {
        Ok(self.inner.publish(ch, data).await?)
    }
}

// // Subscriber holds a reference to the the NATS client. It is used to implement the
// [`wasi_messaging::RuntimeClient`] trait used by the messaging Host.
struct Subscriber {
    inner: async_nats::Subscriber,
}

// Implement the [`wasi_messaging::RuntimeClient`] trait for Client. This trait
// implementation is used by the messaging Host to interact with the NATS client.
#[async_trait::async_trait]
impl RuntimeSubscriber for Subscriber {
    async fn unsubscribe(&mut self) -> anyhow::Result<()> {
        Ok(self.inner.unsubscribe().await?)
    }
}

impl Stream for Subscriber {
    type Item = Message;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // convert async_nats::Message to wasi_messaging::Message
        self.inner.poll_next_unpin(cx).map(|m| {
            let m = m?;
            Some(Message {
                data: m.payload.to_vec(),
                metadata: Some(vec![(String::from("channel"), m.subject.to_string())]),
                format: FormatSpec::Raw,
            })
        })
    }
}
