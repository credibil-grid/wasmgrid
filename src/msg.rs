//! # NATS Messaging System
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
use wasmtime::component::{Linker, Resource};
use wasmtime_wasi::WasiView;

use crate::system::{self, System, State};

pub struct Runtime {
    pub addr: String,
}

impl Runtime {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
}

#[async_trait::async_trait]
impl system::Runtime for Runtime {
    fn add_to_linker(&self, linker: &mut Linker<State>) -> anyhow::Result<()> {
        Messaging::add_to_linker(linker, |t| t)
    }

    /// Start and run NATS for the specified wasm component.
    async fn run(&self, handler: System) -> anyhow::Result<()> {
        let client = Client::connect(self.addr.clone()).await?;
        println!("Connected to NATS: {}", self.addr);

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
            if let Err(e) =
                tokio::spawn(async move { handler.message(client, message).await }).await
            {
                eprintln!("Error: {e:?}");
            }
        }

        Ok(())
    }
}

impl System {
    // Return the list of channels the Guest wants to subscribe to.
    async fn channels(&self) -> anyhow::Result<Vec<String>> {
        let mut store = self.store();
        let (messaging, _) = Messaging::instantiate_pre(&mut store, self.instance_pre()).await?;

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
    async fn message(&self, client: Client, message: Message) -> anyhow::Result<()> {
        // set up host state
        let mut host = State::new();

        // add client to ResourceTable
        host.add_client(client)?;

        let mut store = self.store();
        let (messaging, _) = Messaging::instantiate_pre(&mut store, self.instance_pre()).await?;

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

impl State {
    // Add a new client to the host state.
    fn add_client(&mut self, client: Client) -> anyhow::Result<Resource<wasi_messaging::Client>> {
        let name = client.name.clone();
        let client: wasi_messaging::Client = Box::new(client);

        let resource = self.table().push(client)?;
        self.msg_ctx.insert(name, resource.rep());

        Ok(resource)
    }
}

// Implement the [`wasi_messaging::MessagingView`]` trait for State.
#[async_trait::async_trait]
impl MessagingView for State {
    async fn connect(&mut self, name: String) -> anyhow::Result<Resource<wasi_messaging::Client>> {
        let resource = if let Some(key) = self.msg_ctx.get(&name) {
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
// [`wasi_messaging::RuntimeClient`] trait used by the messaging State.
#[derive(Clone)]
pub struct Client {
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

// Implement the [`wasi_messaging::RuntimeClient`] trait for Client. This trait
// implementation is used by the messaging State to interact with the NATS client.
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
// [`wasi_messaging::RuntimeClient`] trait used by the messaging State.
struct Subscriber {
    inner: async_nats::Subscriber,
}

// Implement the [`wasi_messaging::RuntimeClient`] trait for Client. This trait
// implementation is used by the messaging State to interact with the NATS client.
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
