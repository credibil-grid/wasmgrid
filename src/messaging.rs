mod consumer;
mod producer;
pub mod types;

use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;

use crate::wasi::messaging::messaging_types::{self, Client, Error, HostClient, HostError};

#[async_trait::async_trait]
pub trait WasiMessagingView: WasiView + Send {
    async fn connect(&mut self, name: String) -> anyhow::Result<Resource<Client>>;
}

impl<T: WasiMessagingView> messaging_types::Host for T {}

#[async_trait::async_trait]
impl<T: WasiMessagingView> HostClient for T {
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
impl<T: WasiMessagingView> HostError for T {
    async fn trace(&mut self) -> wasmtime::Result<String> {
        Ok(String::from("trace HostError"))
    }

    fn drop(&mut self, err: Resource<Error>) -> wasmtime::Result<()> {
        println!("Implement drop for {err:?}");
        Ok(())
    }
}

// impl<T: WasiMessagingView> WasiView for T {
//     fn table(&mut self) -> &mut ResourceTable {
//         self.table()
//     }

//     fn ctx(&mut self) -> &mut WasiCtx {
//         self.ctx()
//     }
// }
