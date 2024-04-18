use bytes::Bytes;
use wasmtime::component::Resource;

use super::bindings::messaging_types::{Client, Error, Message};
use super::bindings::producer;
use crate::MessagingView;
use crate::MessagingClient;

#[async_trait::async_trait]
impl<T: MessagingView> producer::Host for T {
    // Publish Guest messages to the specified channel.
    async fn send(
        &mut self, client: Resource<Client>, ch: String, messages: Vec<Message>,
    ) -> wasmtime::Result<anyhow::Result<(), Resource<Error>>> {
        let client = self.table().get(&client)?;

        for m in messages {
            let data = Bytes::from(m.data.clone());
            client.publish(ch.clone(), data).await?;
        }

        Ok(Ok(()))
    }
}
