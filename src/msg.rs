//! # NATS Messaging Runtime
//!
//! This module implements a NATS wasi:messaging runtime.

use std::pin::Pin;
use std::task::{Context, Poll};

use anyhow::anyhow;
use bytes::Bytes;
use futures::stream::{self, Stream, StreamExt};
use wasi_messaging::bindings::wasi::messaging::messaging_types::{
    Error, FormatSpec, GuestConfiguration, Message,
};
use wasi_messaging::bindings::Messaging;
use wasi_messaging::{self, MessagingView, RuntimeClient, RuntimeSubscriber};
use wasmtime::component::Resource;
use wasmtime::Store;
use wasmtime_wasi::WasiView;

use crate::handler::{self, HandlerProxy};

/// Start and run NATS for the specified wasm component.
pub async fn serve(handler: HandlerProxy, addr: String) -> anyhow::Result<()> {
    let msg_handler = handler.clone();

    // connect to NATS
    let client = Client::connect(addr).await?;

    // subscribe to channels
    let mut subscribers = vec![];
    for ch in &handler.channels().await? {
        let subscriber = client.subscribe(ch.clone()).await?;
        subscribers.push(subscriber);
    }

    // process messages until terminated
    let mut messages = stream::select_all(subscribers);
    while let Some(message) = messages.next().await {
        let handler = msg_handler.clone();
        let client = client.clone();
        if let Err(e) = tokio::spawn(async move { handler.message(client, message).await }).await {
            eprintln!("Error: {:?}", e);
        }
    }

    Ok(())
}

impl HandlerProxy {
    // Return the list of channels the Guest wants to subscribe to.
    pub async fn channels(&self) -> anyhow::Result<Vec<String>> {
        let mut store = Store::new(&self.engine, handler::Host::new());
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
        let mut host = handler::Host::new();

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
}

impl handler::Host {
    // Add a new client to the host state.
    pub fn add_client(
        &mut self, client: Client,
    ) -> anyhow::Result<Resource<wasi_messaging::Client>> {
        let name = client.name.clone();
        let client: wasi_messaging::Client = Box::new(client);

        let resource = self.table.push(client)?;
        self.keys.insert(name, resource.rep());

        Ok(resource)
    }
}

// Implement the [`wasi_messaging::MessagingView`]` trait for Host.
#[async_trait::async_trait]
impl MessagingView for handler::Host {
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
