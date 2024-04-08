use wasmtime::component::Resource;

use crate::wasi::messaging::messaging_types::{Client, Error, Message};
use crate::wasi::messaging::producer;

#[async_trait::async_trait]
impl producer::Host for super::NatsHost {
    async fn send(
        &mut self, client: Resource<Client>, ch: String, msg: Vec<Message>,
    ) -> wasmtime::Result<anyhow::Result<(), Resource<Error>>> {
        todo!("Implement send for {client:?} on channel {ch} with message {msg:?}")
    }
}
