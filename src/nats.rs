//! # NATS Messaging Runtime
//!
//! This module implements a NATS wasi:messaging runtime.

use std::collections::HashMap;
use std::pin::Pin;
use std::task::{Context, Poll};

use anyhow::anyhow;
use bytes::Bytes;
use futures::stream::{self, Stream, StreamExt};
use messaging::bindings::messaging_types::{Error, FormatSpec, GuestConfiguration, Message};
use messaging::bindings::Messaging;
use messaging::{self, MessagingView, RuntimeClient, RuntimeSubscriber};
use wasmtime::component::{Component, InstancePre, Linker, Resource};
use wasmtime::{Engine, Store};
use wasmtime_wasi::{command, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

/// Start and run NATS for the specified wasm component.
pub async fn serve(engine: &Engine, wasm: String) -> anyhow::Result<()> {
    let handler = HandlerProxy::new(engine.clone(), wasm)?;

    // connect to NATS
    let client = Client::connect("demo.nats.io".to_string()).await?;

    // subscribe to channels
    let mut subscribers = vec![];
    for ch in &handler.channels().await? {
        let subscriber = client.subscribe(ch.clone()).await?;
        subscribers.push(subscriber);
    }

    // process messages until terminated
    let mut messages = stream::select_all(subscribers);
    while let Some(message) = messages.next().await {
        let handler = handler.clone();
        let client = client.clone();
        tokio::spawn(async move { handler.message(client, message).await });
    }

    Ok(())
}

// HandlerProxy is a proxy for the wasm messaging Host, wrapping calls to the Guest's
// messaging API.
#[derive(Clone)]
struct HandlerProxy {
    engine: Engine,
    instance_pre: InstancePre<Host>,
}

impl HandlerProxy {
    // Create a new HandlerProxy for the specified wasm Guest.
    fn new(engine: Engine, wasm: String) -> anyhow::Result<Self> {
        let mut linker = Linker::new(&engine);
        command::add_to_linker(&mut linker)?;
        Messaging::add_to_linker(&mut linker, |t| t)?;

        let component = Component::from_file(&engine, wasm)?;
        let instance_pre = linker.instantiate_pre(&component)?;

        Ok(Self { engine, instance_pre })
    }

    // Return the list of channels the Guest wants to subscribe to.
    async fn channels(&self) -> anyhow::Result<Vec<String>> {
        let mut store = Store::new(&self.engine, Host::new());
        let (messaging, _) = Messaging::instantiate_pre(&mut store, &self.instance_pre).await?;

        let gc = match messaging.wasi_messaging_messaging_guest().call_configure(&mut store).await?
        {
            Ok(gc) => gc,
            Err(e) => {
                let err = store.data_mut().table().get(&e)?;
                return Err(anyhow!(err.to_string()));
            }
        };

        Ok(gc.channels)
    }

    // Forward NATS message to the wasm Guest.
    async fn message(&self, client: Client, message: Message) -> anyhow::Result<()> {
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
            let err = store.data_mut().table().get(&e)?;
            return Err(anyhow!(err.to_string()));
        }

        Ok(())
    }
}

// Host implements messaging host interfaces. In addition, it holds the host-defined
// state used by the wasm runtime [`Store`].
struct Host {
    keys: HashMap<String, u32>,
    table: ResourceTable,
    ctx: WasiCtx,
}

impl Host {
    // Create a new Host instance.
    fn new() -> Self {
        Self {
            keys: HashMap::default(),
            table: ResourceTable::default(),
            ctx: WasiCtxBuilder::new().inherit_args().inherit_env().inherit_stdio().build(),
        }
    }

    // Add a new client to the host state.
    fn add_client(&mut self, client: Client) -> anyhow::Result<Resource<messaging::Client>> {
        let name = client.name.clone();
        let client = messaging::Client::new(Box::new(client));

        let resource = self.table.push(client)?;
        self.keys.insert(name, resource.rep());

        Ok(resource)
    }
}

// Implement the [`messaging::MessagingView`]` trait for Host.
#[async_trait::async_trait]
impl MessagingView for Host {
    async fn connect(&mut self, name: String) -> anyhow::Result<Resource<messaging::Client>> {
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

// Client holds a reference to the the NATS client. It is used to implement the
// [`messaging::RuntimeClient`] trait used by the messaging Host.
#[derive(Clone)]
struct Client {
    name: String,
    inner: async_nats::Client,
}

impl Client {
    // Create a new Client for the specified NATS server.
    async fn connect(name: String) -> anyhow::Result<Self> {
        let inner = async_nats::connect(&name).await?;
        Ok(Self { name, inner })
    }
}

// Implement the [`messaging::RuntimeClient`] trait for Client. This trait
// implementation is used by the messaging Host to interact with the NATS client.
#[async_trait::async_trait]
impl RuntimeClient for Client {
    async fn subscribe(&self, ch: String) -> anyhow::Result<messaging::Subscriber> {
        let subscriber = messaging::Subscriber::new(Box::pin(Subscriber {
            inner: self.inner.subscribe(ch).await?,
        }));
        Ok(subscriber)
    }

    async fn publish(&self, ch: String, data: Bytes) -> anyhow::Result<()> {
        Ok(self.inner.publish(ch, data).await?)
    }
}

// // Subscriber holds a reference to the the NATS client. It is used to implement the
// [`messaging::RuntimeClient`] trait used by the messaging Host.
struct Subscriber {
    inner: async_nats::Subscriber,
}

// Implement the [`messaging::RuntimeClient`] trait for Client. This trait
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
        let opt = self.inner.poll_next_unpin(cx);

        opt.map(|m| {
            let Some(m) = m else {
                return None;
            };

            return Some(Message {
                data: m.payload.to_vec(),
                metadata: Some(vec![(String::from("channel"), m.subject.to_string())]),
                format: FormatSpec::Raw,
            });
        })
    }
}
