use wasmtime::component::Resource;

use crate::wasi::messaging::messaging_types;
use crate::wasi::messaging::messaging_types::{Client, Error};

impl messaging_types::Host for super::NatsHost {}

#[async_trait::async_trait]
impl messaging_types::HostClient for super::NatsHost {
    async fn connect(
        &mut self, ch: String,
    ) -> wasmtime::Result<anyhow::Result<Resource<Client>, Resource<Error>>> {
        todo!("Implement connect for {ch}")
    }

    fn drop(&mut self, client: Resource<Client>) -> wasmtime::Result<()> {
        todo!("Implement drop for {client:?}")
    }
}

#[async_trait::async_trait]
impl messaging_types::HostError for super::NatsHost {
    async fn trace(&mut self) -> wasmtime::Result<String> {
        todo!("Implement trace")
    }

    fn drop(&mut self, err: Resource<Error>) -> wasmtime::Result<()> {
        todo!("Implement drop for {err:?}")
    }
}
