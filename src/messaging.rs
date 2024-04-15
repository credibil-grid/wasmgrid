mod consumer;
mod producer;

use bytes::Bytes;
use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;

use crate::wasi::messaging::messaging_types::{self, Client, Error, HostClient, HostError};

#[async_trait::async_trait]
pub trait MessagingView: WasiView + Send {
    type Client: MessagingClient;

    async fn connect(&mut self, name: String) -> anyhow::Result<Resource<Self::Client>>;
}

pub trait MessagingClient {
    async fn subscribe(&self, ch: String) -> anyhow::Result<async_nats::Subscriber>;

    async fn publish(&self, ch: String, data: Bytes) -> anyhow::Result<()>;
}

impl<T: MessagingView> messaging_types::Host for T where T: MessagingView<Client = Client> {}

#[async_trait::async_trait]
impl<T: MessagingView> HostClient for T
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
impl<T: MessagingView> HostError for T {
    async fn trace(&mut self) -> wasmtime::Result<String> {
        Ok(String::from("trace HostError"))
    }

    fn drop(&mut self, err: Resource<Error>) -> wasmtime::Result<()> {
        println!("Implement drop for {err:?}");
        Ok(())
    }
}
