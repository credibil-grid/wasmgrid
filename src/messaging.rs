mod consumer;
mod producer;

use bindings::messaging_types::{self, Client, Error, HostClient, HostError};
use bytes::Bytes;
use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;

pub mod bindings {
    pub use wasi::messaging::*;

    pub use crate::nats::Client;

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
    type Client: MessagingClient;

    async fn connect(&mut self, name: String) -> anyhow::Result<Resource<Self::Client>>;
}

#[allow(clippy::module_name_repetitions)]
pub trait MessagingClient {
    // type Subscriber: Send;

    async fn subscribe(&self, ch: String) -> anyhow::Result<async_nats::Subscriber>;

    async fn publish(&self, ch: String, data: Bytes) -> anyhow::Result<()>;
}

impl<T> messaging_types::Host for T where T: MessagingView<Client = Client> {}

#[async_trait::async_trait]
impl<T> HostClient for T
where
    T: MessagingView<Client = Client>,
{
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
impl<T> HostError for T
where
    T: MessagingView,
{
    async fn trace(&mut self) -> wasmtime::Result<String> {
        Ok(String::from("trace HostError"))
    }

    fn drop(&mut self, err: Resource<Error>) -> wasmtime::Result<()> {
        println!("Implement drop for {err:?}");
        Ok(())
    }
}
