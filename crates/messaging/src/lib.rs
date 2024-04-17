//! # WASI Messaging Host

mod consumer;
mod producer;

use bindings::messaging_types::{self, Error, HostClient, HostError};
use bytes::Bytes;
use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;

pub mod bindings {
    pub use wasi::messaging::*;

    pub use super::Client;

    wasmtime::component::bindgen!({
        world: "messaging",
        path: "wit",
        tracing: true,
        async: true,
        with: {
            "wasi:messaging/messaging-types/client": Client,
        },
    });
}

/// MessageView is implemented by the messaging runtime to provide the host with
/// access to runtime-specific functionality.
#[allow(clippy::module_name_repetitions)]
#[async_trait::async_trait]
pub trait MessagingView: WasiView + Send {
    async fn connect(&mut self, name: String) -> anyhow::Result<Resource<Client>>;
}

// Type T — the host — is supplied by the messaging runtime.
impl<T: MessagingView> messaging_types::Host for T {}

#[async_trait::async_trait]
impl<T: MessagingView> HostClient for T {
    // Connect to the runtime's messaging server.
    async fn connect(
        &mut self, name: String,
    ) -> wasmtime::Result<anyhow::Result<Resource<Client>, Resource<Error>>> {
        let resource = self.connect(name).await?;
        Ok(Ok(resource))
    }

    // Drop the specified client resource.
    fn drop(&mut self, client: Resource<Client>) -> wasmtime::Result<()> {
        let _ = self.table().delete(client)?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl<T: MessagingView> HostError for T {
    async fn trace(&mut self) -> wasmtime::Result<String> {
        Ok(String::from("trace HostError"))
    }

    fn drop(&mut self, err: Resource<Error>) -> wasmtime::Result<()> {
        println!("Implement drop for {err:?}");
        Ok(())
    }
}

/// MessagingClient is implemented by the messaging runtime to provide this host with
/// access to runtime functionality.
#[allow(clippy::module_name_repetitions)]
#[async_trait::async_trait]
pub trait MessagingClient: Sync + Send {
    async fn subscribe(&self, ch: String) -> anyhow::Result<async_nats::Subscriber>;

    async fn publish(&self, ch: String, data: Bytes) -> anyhow::Result<()>;
}

// #[derive(Clone)]
pub struct Client {
    inner: Box<dyn MessagingClient>,
}

impl Client {
    #[must_use]
    pub fn new(inner: Box<dyn MessagingClient>) -> Self {
        Self { inner }
    }

    /// Subscribe to the specified channel.
    ///
    /// # Errors
    pub async fn subscribe(&self, ch: String) -> anyhow::Result<async_nats::Subscriber> {
        self.inner.subscribe(ch).await
    }

    /// Publish a message to the specified channel.
    ///
    /// # Errors
    pub async fn publish(&self, ch: String, data: Bytes) -> anyhow::Result<()> {
        self.inner.publish(ch, data).await
    }
}
