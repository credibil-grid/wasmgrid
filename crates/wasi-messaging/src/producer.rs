use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;

use super::bindings::wasi::messaging::messaging_types::{Client, Error, Message};
use super::bindings::wasi::messaging::producer;

/// ProducerView is implemented by the messaging runtime to provide the host with
/// access to runtime-specific functionality.
#[allow(clippy::module_name_repetitions)]
#[async_trait::async_trait]
pub trait ProducerView: WasiView + Send {
    /// Publish Guest messages to the specified channel.
    async fn send(
        &mut self, client: Resource<Client>, ch: String, messages: Vec<Message>,
    ) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl<T: ProducerView> producer::Host for T {
    async fn send(
        &mut self, client: Resource<Client>, ch: String, messages: Vec<Message>,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::debug!("Host::send: {:?}", ch);

        Ok(Ok(T::send(self, client, ch, messages).await?))
    }
}
