use futures::stream::StreamExt;
use tokio::time::{sleep, Duration};
use wasmtime::component::Resource;

use super::bindings::consumer;
use super::bindings::messaging_types::{Client, Error, GuestConfiguration, Message};
use crate::MessagingView;

#[async_trait::async_trait]
impl<T: MessagingView> consumer::Host for T {
    async fn subscribe_try_receive(
        &mut self, client: Resource<Client>, ch: String, t_milliseconds: u32,
    ) -> wasmtime::Result<anyhow::Result<Option<Vec<Message>>, Resource<Error>>> {
        // subscribe to channel
        let client = self.table().get(&client)?;
        let mut subscriber = client.subscribe(ch).await?;

        // create stream that times out after t_milliseconds
        let stream =
            subscriber.by_ref().take_until(sleep(Duration::from_millis(u64::from(t_milliseconds))));
        let messages = stream.collect().await;
        subscriber.unsubscribe().await?;

        Ok(Ok(Some(messages)))
    }

    async fn subscribe_receive(
        &mut self, client: Resource<Client>, ch: String,
    ) -> wasmtime::Result<anyhow::Result<Vec<Message>, Resource<Error>>> {
        let client = self.table().get(&client)?;
        let mut subscriber = client.subscribe(ch).await?;

        // get first message
        let messages = subscriber.by_ref().take(1).collect().await;
        subscriber.unsubscribe().await?;

        Ok(Ok(messages))
    }

    async fn update_guest_configuration(
        &mut self, gc: GuestConfiguration,
    ) -> wasmtime::Result<anyhow::Result<(), Resource<Error>>> {
        Ok(self.update_configuration(gc).await)
    }

    // TODO: implement complete_message
    async fn complete_message(
        &mut self, msg: Message,
    ) -> wasmtime::Result<anyhow::Result<(), Resource<Error>>> {
        println!("TODO: implement complete_message: {:?}", msg.metadata);
        Ok(Ok(()))
    }

    // TODO: implement abandon_message
    async fn abandon_message(
        &mut self, msg: Message,
    ) -> wasmtime::Result<anyhow::Result<(), Resource<Error>>> {
        println!("TODO: implement abandon_message: {:?}", msg.metadata);
        Ok(Ok(()))
    }
}
