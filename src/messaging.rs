mod consumer;
mod producer;

use bindings::messaging_types::{self, Error, HostClient, HostError};
use bytes::Bytes;
use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;

pub mod bindings {
    pub use wasi::messaging::*;

    // pub use crate::nats::Client;
    pub use super::Client;

    wasmtime::component::bindgen!({
        world: "messaging",
        path: "./src/messaging/wit",
        tracing: true,
        async: true,
        with: {
            "wasi:messaging/messaging-types/client": Client,
        },
    });
}

#[allow(clippy::module_name_repetitions)]
#[async_trait::async_trait]
pub trait MessagingView: WasiView + Send {
    async fn connect(&mut self, name: String) -> anyhow::Result<Resource<Client>>;
}

impl<T: MessagingView> messaging_types::Host for T {}

#[async_trait::async_trait]
impl<T: MessagingView> HostClient for T {
    /// Connect to the NATS server specified by `name` and return a client resource.
    async fn connect(
        &mut self, name: String,
    ) -> wasmtime::Result<anyhow::Result<Resource<Client>, Resource<Error>>> {
        let resource = self.connect(name).await?;
        Ok(Ok(resource))
    }

    /// Drop the specified NATS client resource.
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

#[allow(clippy::module_name_repetitions)]
#[async_trait::async_trait]
pub trait MessagingClient: Sync + Send {
    // type Subscriber: Send;

    async fn subscribe(&self, ch: String) -> anyhow::Result<async_nats::Subscriber>;

    async fn publish(&self, ch: String, data: Bytes) -> anyhow::Result<()>;
}

// #[derive(Clone)]
pub struct Client {
    inner: Box<dyn MessagingClient>,
}

impl Client {
    pub fn new(inner: Box<dyn MessagingClient>) -> Self {
        Self { inner }
    }

    pub async fn subscribe(&self, ch: String) -> anyhow::Result<async_nats::Subscriber> {
        self.inner.subscribe(ch).await
    }

    pub async fn publish(&self, ch: String, data: Bytes) -> anyhow::Result<()> {
        self.inner.publish(ch, data).await
    }
}
