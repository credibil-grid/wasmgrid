use futures::stream::StreamExt;
use tokio::time::{sleep, Duration};
use wasmtime::component::Resource;

use super::bindings::wasi::messaging::consumer;
use super::bindings::wasi::messaging::messaging_types::{
    Client, Error, GuestConfiguration, Message,
};
use crate::MessagingView;

#[async_trait::async_trait]
impl<T: MessagingView> consumer::Host for T {
    async fn subscribe_try_receive(
        &mut self, client: Resource<Client>, ch: String, t_milliseconds: u32,
    ) -> wasmtime::Result<Result<Option<Vec<Message>>, Resource<Error>>> {
        tracing::debug!("Host::subscribe_try_receive {ch}, {t_milliseconds}");

        // subscribe to channel
        let client = self.table().get(&client)?;
        let mut subscriber = client.subscribe(ch).await?;

        // create stream that times out after t_milliseconds
        let stream =
            subscriber.by_ref().take_until(sleep(Duration::from_millis(u64::from(t_milliseconds))));
        let messages = stream.collect().await;
        // subscriber.unsubscribe().await?;

        Ok(Ok(Some(messages)))
    }

    async fn subscribe_receive(
        &mut self, client: Resource<Client>, ch: String,
    ) -> wasmtime::Result<Result<Vec<Message>, Resource<Error>>> {
        tracing::debug!("Host::subscribe_receive {ch}");

        let client = self.table().get(&client)?;
        let mut subscriber = client.subscribe(ch).await?;
        let messages = subscriber.by_ref().take(1).collect().await;
        // subscriber.unsubscribe().await?;

        Ok(Ok(messages))
    }

    async fn update_guest_configuration(
        &mut self, gc: GuestConfiguration,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::debug!("Host::update_guest_configuration");
        Ok(self.update_configuration(gc).await)
    }

    // TODO: implement complete_message
    async fn complete_message(
        &mut self, msg: Message,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::warn!("FIXME: implement Host::complete_message: {:?}", msg.metadata);
        Ok(Ok(()))
    }

    // TODO: implement abandon_message
    async fn abandon_message(
        &mut self, msg: Message,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::warn!("FIXME: implement Host::abandon_message: {:?}", msg.metadata);
        Ok(Ok(()))
    }
}
