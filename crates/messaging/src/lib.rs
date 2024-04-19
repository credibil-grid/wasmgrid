//! # WASI Messaging Host

mod consumer;
mod producer;

use std::pin::Pin;
use std::task::{Context, Poll};

use bindings::messaging_types::{self, Error, GuestConfiguration, HostClient, HostError, Message};
use bytes::Bytes;
use futures::stream::Stream;
use futures::StreamExt;
use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;

pub type Client = Box<dyn RuntimeClient>;

/// Wrap generation of wit bindings to simplify exports
pub mod bindings {
    pub use anyhow::Error;
    pub use wasi::messaging::*;

    pub use super::Client;

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

/// MessageView is implemented by the messaging runtime to provide the host with
/// access to runtime-specific functionality.
#[allow(clippy::module_name_repetitions)]
#[async_trait::async_trait]
pub trait MessagingView: WasiView + Send {
    async fn connect(&mut self, name: String) -> anyhow::Result<Resource<Client>>;

    async fn update_configuration(
        &mut self, gc: GuestConfiguration,
    ) -> anyhow::Result<(), Resource<Error>>;
}

// Type T — the host — is provided by the messaging runtime.
impl<T: MessagingView> messaging_types::Host for T {
    // fn convert_error(&mut self, e: anyhow::Error) -> anyhow::Result<Error> {
    //     todo!()
    // }
}

#[async_trait::async_trait]
impl<T: MessagingView> HostClient for T {
    // Connect to the runtime's messaging server.
    async fn connect(
        &mut self, name: String,
    ) -> wasmtime::Result<anyhow::Result<Resource<Client>, Resource<Error>>> {
        Ok(Ok(T::connect(self, name).await?))
    }

    // Drop the specified client resource.
    fn drop(&mut self, client: Resource<Client>) -> wasmtime::Result<()> {
        self.table().delete(client)?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl<T: MessagingView> HostError for T {
    async fn trace(&mut self) -> wasmtime::Result<String> {
        Ok(String::from("TODO: trace HostError"))
    }

    fn drop(&mut self, err: Resource<Error>) -> wasmtime::Result<()> {
        println!("TODO: implement drop for {err:?}");
        Ok(())
    }
}

/// RuntimeClient is implemented by the runtime to provide this host with access
/// to runtime functionality.
#[async_trait::async_trait]
pub trait RuntimeClient: Sync + Send {
    // type S: RuntimeSubscriber;

    /// Subscribe to the specified channel.
    async fn subscribe(&self, ch: String) -> anyhow::Result<Subscriber>;

    /// Publish a message to the specified channel.
    async fn publish(&self, ch: String, data: Bytes) -> anyhow::Result<()>;
}

/// RuntimeSubscriber is implemented by the runtime to provide the host with access
/// to runtime subscriber functionality.
#[async_trait::async_trait]
pub trait RuntimeSubscriber: Stream<Item = Message> + Send {
    async fn unsubscribe(&mut self) -> anyhow::Result<()>;
}

pub struct Subscriber {
    runtime: Pin<Box<dyn RuntimeSubscriber>>,
}

impl Subscriber {
    #[must_use]
    pub fn new(runtime: Pin<Box<dyn RuntimeSubscriber>>) -> Self {
        Self { runtime }
    }

    async fn unsubscribe(&mut self) -> anyhow::Result<()> {
        // self.runtime.unsubscribe().await?;
        Ok(())
    }
}

impl Stream for Subscriber {
    type Item = Message;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.runtime.poll_next_unpin(cx)
    }
}
