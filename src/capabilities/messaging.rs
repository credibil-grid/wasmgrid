//! # WASI Messaging Capability
//!
//! This module implements a runtime capability for `wasi:messaging`
//! (<https://github.com/WebAssembly/wasi-messaging>).

pub use anyhow::{anyhow, Error};
use bindings::wasi::messaging::messaging_types::{
    self, FormatSpec, GuestConfiguration, HostClient, HostError, Message,
};
use bindings::wasi::messaging::{consumer, producer};
use bindings::Messaging;
use bytes::Bytes;
use futures::stream::{self, StreamExt};
use tokio::time::{sleep, Duration};
use wasmtime::component::{Linker, Resource};
use wasmtime_wasi::WasiView;

use crate::runtime::{self, Runtime, State};

/// Wrap generation of wit bindings to simplify exports
mod bindings {
    #![allow(clippy::future_not_send)]

    pub use super::{Client, Error};

    wasmtime::component::bindgen!({
        world: "messaging",
        path: "wit",
        tracing: true,
        async: true,
        with: {
            "wasi:messaging/messaging-types/client": Client,
            "wasi:messaging/messaging-types/error": Error,
        },
        // trappable_error_type: {
        //     "wasi:messaging/messaging-types/error" => Error,
        // },
    });
}

pub struct Capability {
    pub addr: String,
}

pub const fn new(addr: String) -> Capability {
    Capability { addr }
}

#[async_trait::async_trait]
impl runtime::Capability for Capability {
    fn namespace(&self) -> &str {
        "wasi:messaging"
    }

    fn add_to_linker(&self, linker: &mut Linker<State>) -> anyhow::Result<()> {
        Messaging::add_to_linker(linker, |t| t)
    }

    /// Provide messaging capability for the specified wasm component.
    async fn run(&self, runtime: Runtime) -> anyhow::Result<()> {
        let client = Client::connect(self.addr.clone()).await?;
        tracing::info!("connected to NATS");

        // subscribe to channels
        let mut subscribers = vec![];
        for ch in channels(&runtime).await? {
            let subscriber = client.subscribe(ch.clone()).await?;
            subscribers.push(subscriber);
        }

        // process messages until terminated
        let mut messages = stream::select_all(subscribers);
        while let Some(m) = messages.next().await {
            let runtime = runtime.clone();
            let client = client.clone();

            if let Err(e) =
                tokio::spawn(async move { handle_message(&runtime, client, to_message(m)).await })
                    .await
            {
                tracing::error!("error processing message {e:?}");
            }
        }

        Ok(())
    }
}

// Return the channels the Guest wants to subscribe to.
async fn channels(runtime: &Runtime) -> anyhow::Result<Vec<String>> {
    tracing::debug!("channels");

    let mut store = runtime.store();
    let (messaging, _) = Messaging::instantiate_pre(&mut store, runtime.instance_pre()).await?;

    let gc = match messaging.wasi_messaging_messaging_guest().call_configure(&mut store).await? {
        Ok(gc) => gc,
        Err(e) => {
            let error = store.data_mut().table().get(&e)?;
            // let Some(err) = error.as_ref().as_any().downcast_ref::<Error>() else {
            //     return Err(anyhow!("invalid JetStream store"));
            // };

            return Err(anyhow!(error.to_string()));
        }
    };

    Ok(gc.channels)
}

// Forward NATS message to the wasm Guest.
async fn handle_message(runtime: &Runtime, client: Client, message: Message) -> anyhow::Result<()> {
    tracing::debug!("handle_message: {message:?}");

    // add client to ResourceTable
    let mut store = runtime.store();
    store.data_mut().add_client(client)?;

    let (messaging, _) = Messaging::instantiate_pre(&mut store, runtime.instance_pre()).await?;

    // call guest with message
    if let Err(e) =
        messaging.wasi_messaging_messaging_guest().call_handler(&mut store, &[message]).await?
    {
        let error = store.data_mut().table().get(&e)?;
        // let Some(err) = error.as_ref().as_any().downcast_ref::<Error>() else {
        //     return Err(anyhow!("invalid JetStream store"));
        // };

        return Err(anyhow!(error.to_string()));
    }

    Ok(())
}

impl State {
    // Add a new client to the host state.
    fn add_client(&mut self, client: Client) -> anyhow::Result<Resource<bindings::Client>> {
        let name = client.name.clone();

        let resource = self.table().push(client)?;
        self.metadata.insert(name, Box::new(resource.rep()));

        Ok(resource)
    }
}

impl messaging_types::Host for State {
    // fn convert_error(&mut self, e: anyhow::Error) -> anyhow::Result<Error> {
    //     todo!()
    // }
}

#[async_trait::async_trait]
impl HostError for State {
    async fn trace(&mut self) -> wasmtime::Result<String> {
        tracing::debug!("HostError::trace");
        todo!()
    }

    fn drop(&mut self, _rep: Resource<Error>) -> wasmtime::Result<()> {
        tracing::debug!("HostError::drop");
        todo!()
    }
}

// Implement the [`wasi_messaging::MessagingView`]` trait for State.
#[async_trait::async_trait]
impl HostClient for State {
    async fn connect(
        &mut self, name: String,
    ) -> wasmtime::Result<anyhow::Result<Resource<Client>, Resource<Error>>> {
        tracing::debug!("MessagingView::connect {name}");

        let resource = if let Some(key) = self.metadata.get(&name) {
            // reuse existing connection
            let key = key.downcast_ref::<u32>().unwrap();
            Resource::new_own(*key)
        } else {
            // create a new connection
            let client = Client::connect(name.clone()).await?;
            self.add_client(client)?
        };

        Ok(Ok(resource))
    }

    fn drop(&mut self, client: Resource<Client>) -> wasmtime::Result<()> {
        tracing::debug!("HostClient::drop");
        self.table().delete(client)?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl consumer::Host for State {
    async fn subscribe_try_receive(
        &mut self, client: Resource<bindings::Client>, ch: String, t_milliseconds: u32,
    ) -> wasmtime::Result<Result<Option<Vec<Message>>, Resource<Error>>> {
        tracing::debug!("ConsumerView::subscribe_try_receive {ch}, {t_milliseconds}");

        // subscribe to channel
        let client = self.table().get(&client)?;
        let mut subscriber = client.subscribe(ch).await?;

        // create stream that times out after t_milliseconds
        let stream =
            subscriber.by_ref().take_until(sleep(Duration::from_millis(u64::from(t_milliseconds))));
        let messages = stream.map(to_message).collect().await;

        subscriber.unsubscribe().await?;

        Ok(Ok(Some(messages)))
    }

    async fn subscribe_receive(
        &mut self, client: Resource<bindings::Client>, ch: String,
    ) -> wasmtime::Result<Result<Vec<Message>, Resource<Error>>> {
        tracing::debug!("ConsumerView::subscribe_receive {ch}");

        let client = self.table().get(&client)?;
        let mut subscriber = client.subscribe(ch).await?;
        let messages = subscriber.by_ref().take(1).map(to_message).collect().await;
        // subscriber.unsubscribe().await?;

        Ok(Ok(messages))
    }

    async fn update_guest_configuration(
        &mut self, _gc: GuestConfiguration,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::warn!("TODO: ConsumerView::update_guest_configuration");
        Ok(Ok(()))
    }

    // TODO: implement complete_message
    async fn complete_message(
        &mut self, msg: Message,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::warn!("FIXME: ConsumerView::complete_message: {:?}", msg.metadata);
        Ok(Ok(()))
    }

    // TODO: implement abandon_message
    async fn abandon_message(
        &mut self, msg: Message,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::warn!("FIXME: ConsumerView::abandon_message: {:?}", msg.metadata);
        Ok(Ok(()))
    }
}

#[async_trait::async_trait]
impl producer::Host for State {
    async fn send(
        &mut self, client: Resource<bindings::Client>, ch: String, messages: Vec<Message>,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::debug!("ProducerView::send: {:?}", ch);

        let client = self.table().get(&client)?;
        for m in messages {
            let data = Bytes::from(m.data.clone());
            client.publish(ch.clone(), data).await?;
        }

        Ok(Ok(()))
    }
}

// Client holds a reference to the the NATS client. It is used to implement the
// [`wasi_messaging::RuntimeClient`] trait used by the messaging State.
#[derive(Clone)]
pub struct Client {
    name: String,
    inner: async_nats::Client,
}

impl Client {
    // Create a new Client for the specified NATS server.
    async fn connect(name: String) -> anyhow::Result<Self> {
        tracing::trace!("Client::connect {name}");

        let inner = async_nats::connect(&name).await?;
        Ok(Self { name, inner })
    }

    async fn subscribe(&self, ch: String) -> anyhow::Result<async_nats::Subscriber> {
        tracing::debug!("Client::subscribe {ch}");
        Ok(self.inner.subscribe(ch).await?)
    }

    async fn publish(&self, ch: String, data: Bytes) -> anyhow::Result<()> {
        tracing::debug!("Client::publish {ch}");
        Ok(self.inner.publish(ch, data).await?)
    }
}

#[allow(clippy::needless_pass_by_value)]
fn to_message(m: async_nats::Message) -> Message {
    Message {
        data: m.payload.to_vec(),
        metadata: Some(vec![(String::from("channel"), m.subject.to_string())]),
        format: FormatSpec::Raw,
    }
}
