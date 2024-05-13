//! # WASI Messaging Capability
//!
//! This module implements a runtime capability for `wasi:messaging`
//! (<https://github.com/WebAssembly/wasi-messaging>).

use std::sync::OnceLock;

use anyhow::anyhow;
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

pub type Client = async_nats::Client;
pub type Error = anyhow::Error;

static PROCESSOR: OnceLock<Processor> = OnceLock::new();

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

    async fn run(&self, runtime: Runtime) -> anyhow::Result<()> {
        let client = async_nats::connect(&self.addr).await?;

        // message processor needs to be accessible to Guest callbacks
        let processor = PROCESSOR.get_or_init(|| Processor {
            runtime: runtime.clone(),
            client: client.clone(),
        });

        // get guest configuration (channels to subscribe to)
        let mut store = runtime.new_store();
        let (messaging, _) = Messaging::instantiate_pre(&mut store, runtime.instance_pre()).await?;
        let gc = match messaging.wasi_messaging_messaging_guest().call_configure(&mut store).await?
        {
            Ok(gc) => gc,
            Err(e) => {
                let error = store.data_mut().table().get(&e)?;
                return Err(anyhow!(error.to_string()));
            }
        };

        // initiate message processing
        processor.subscribe(gc.channels).await
    }
}

#[derive(Clone)]
struct Processor {
    runtime: Runtime,
    client: Client,
}

impl Processor {
    async fn subscribe(&self, channels: Vec<String>) -> anyhow::Result<()> {
        tracing::debug!("Processor::subscribe: {:?}", channels);

        // subscribe to channels
        let mut subscribers = vec![];
        for ch in channels {
            let subscriber = self.client.subscribe(ch.clone()).await?;
            subscribers.push(subscriber);
        }

        // process messages until terminated
        let mut messages = stream::select_all(subscribers);
        while let Some(m) = messages.next().await {
            let self_ = self.clone();
            if let Err(e) = tokio::spawn(async move { self_.forward(m).await }).await {
                tracing::error!("error processing message {e:?}");
            }
        }

        Ok(())
    }

    // Forward NATS message to the wasm Guest.
    async fn forward(&self, msg: async_nats::Message) -> anyhow::Result<()> {
        tracing::debug!("handle_message: {msg:?}");

        let mut store = self.runtime.new_store();
        let (messaging, _) =
            Messaging::instantiate_pre(&mut store, self.runtime.instance_pre()).await?;
        let message = to_message(msg);

        if let Err(e) =
            messaging.wasi_messaging_messaging_guest().call_handler(&mut store, &[message]).await?
        {
            let error = store.data_mut().table().get(&e)?;
            return Err(anyhow!(error.to_string()));
        }

        Ok(())
    }
}

impl messaging_types::Host for State {
    // fn convert_error(&mut self, e: anyhow::Error) -> anyhow::Result<Error> {
    //     todo!()
    // }
}

#[async_trait::async_trait]
impl HostClient for State {
    async fn connect(
        &mut self, name: String,
    ) -> wasmtime::Result<anyhow::Result<Resource<Client>, Resource<Error>>> {
        tracing::debug!("HostClient::connect {name}");

        let processor = PROCESSOR.get().ok_or_else(|| anyhow!("PROCESSOR not initialized"))?;
        let client = processor.client.clone();

        let resource = self.table().push(client)?;
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
        &mut self, client: Resource<Client>, ch: String, t_milliseconds: u32,
    ) -> wasmtime::Result<Result<Option<Vec<Message>>, Resource<Error>>> {
        tracing::debug!("consumer::Host::subscribe_try_receive {ch}, {t_milliseconds}");

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
        &mut self, client: Resource<Client>, ch: String,
    ) -> wasmtime::Result<Result<Vec<Message>, Resource<Error>>> {
        tracing::debug!("consumer::Host::subscribe_receive {ch}");

        let client = self.table().get(&client)?;
        let mut subscriber = client.subscribe(ch).await?;
        let messages = subscriber.by_ref().take(1).map(to_message).collect().await;
        subscriber.unsubscribe().await?;

        Ok(Ok(messages))
    }

    async fn update_guest_configuration(
        &mut self, gc: GuestConfiguration,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::debug!("consumer::Host::update_guest_configuration");

        let processor = PROCESSOR.get().ok_or_else(|| anyhow!("Processor not initialized"))?;
        Ok(Ok(processor.subscribe(gc.channels).await?))
    }

    // TODO: implement `complete_message` using JetStream
    async fn complete_message(
        &mut self, msg: Message,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::warn!("TODO: consumer::Host::complete_message: {:?}", msg.metadata);
        Ok(Ok(()))
    }

    // TODO: implement `abandon_message` using JetStream
    async fn abandon_message(
        &mut self, msg: Message,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::warn!("TODO: consumer::Host::abandon_message: {:?}", msg.metadata);
        Ok(Ok(()))
    }
}

#[async_trait::async_trait]
impl producer::Host for State {
    async fn send(
        &mut self, client: Resource<Client>, ch: String, messages: Vec<Message>,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::debug!("producer::Host::send: {:?}", ch);

        let client = self.table().get(&client)?;
        for m in messages {
            let data = Bytes::from(m.data.clone());
            client.publish(ch.clone(), data).await?;
        }

        Ok(Ok(()))
    }
}

#[async_trait::async_trait]
impl HostError for State {
    async fn trace(&mut self, rep: Resource<Error>) -> wasmtime::Result<String> {
        tracing::debug!("HostError::trace");
        let error = self.table().get(&rep)?;
        Ok(error.to_string())
    }

    fn drop(&mut self, rep: Resource<Error>) -> wasmtime::Result<()> {
        tracing::debug!("HostError::drop");
        self.table().delete(rep)?;
        Ok(())
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
