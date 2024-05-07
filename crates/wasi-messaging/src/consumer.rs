use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;

use super::bindings::wasi::messaging::consumer;
use super::bindings::wasi::messaging::messaging_types::{
    Client, Error, GuestConfiguration, Message,
};

/// ConsumerView is implemented by the messaging runtime to provide the host with
/// access to runtime-specific functionality.
#[allow(clippy::module_name_repetitions)]
#[async_trait::async_trait]
pub trait ConsumerView: WasiView + Send {
    /// Try to receive messages from the specified channel.
    async fn subscribe_try_receive(
        &mut self, client: Resource<Client>, ch: String, t_milliseconds: u32,
    ) -> anyhow::Result<Option<Vec<Message>>>;

    /// Receive messages from the specified channel.
    async fn subscribe_receive(
        &mut self, client: Resource<Client>, ch: String,
    ) -> anyhow::Result<Vec<Message>>;

    /// Update the guest configuration.
    async fn update_guest_configuration(&mut self, gc: GuestConfiguration) -> anyhow::Result<()>;

    /// Complete the specified message.
    async fn complete_message(&mut self, msg: Message) -> anyhow::Result<()>;

    /// Abandon the specified message.
    async fn abandon_message(&mut self, msg: Message) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl<T: ConsumerView> consumer::Host for T {
    async fn subscribe_try_receive(
        &mut self, client: Resource<Client>, ch: String, t_milliseconds: u32,
    ) -> wasmtime::Result<Result<Option<Vec<Message>>, Resource<Error>>> {
        tracing::debug!("Host::subscribe_try_receive {ch}, {t_milliseconds}");
        Ok(Ok(T::subscribe_try_receive(self, client, ch, t_milliseconds).await?))
    }

    async fn subscribe_receive(
        &mut self, client: Resource<Client>, ch: String,
    ) -> wasmtime::Result<Result<Vec<Message>, Resource<Error>>> {
        tracing::debug!("Host::subscribe_receive {ch}");
        Ok(Ok(T::subscribe_receive(self, client, ch).await?))
    }

    async fn update_guest_configuration(
        &mut self, gc: GuestConfiguration,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::debug!("Host::update_guest_configuration");
        Ok(Ok(T::update_guest_configuration(self, gc).await?))
    }

    async fn complete_message(
        &mut self, msg: Message,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::warn!("Host::complete_message: {:?}", msg.metadata);
        Ok(Ok(T::complete_message(self, msg).await?))
    }

    async fn abandon_message(
        &mut self, msg: Message,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::warn!("Host::abandon_message: {:?}", msg.metadata);
        Ok(Ok(T::abandon_message(self, msg).await?))
    }
}
