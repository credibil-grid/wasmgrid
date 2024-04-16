use bytes::Bytes;
use wasmtime::component::Resource;

use super::bindings::messaging_types::{Client, Error, Message};
use super::bindings::producer;
use crate::MessagingView;

#[async_trait::async_trait]
impl<T: MessagingView> producer::Host for T {
    async fn send(
        &mut self, client: Resource<Client>, ch: String, msg: Vec<Message>,
    ) -> wasmtime::Result<anyhow::Result<(), Resource<Error>>> {
        println!("send: ch: {ch}");

        let data = Bytes::from(msg[0].data.clone());
        let client = self.table().get(&client)?;
        client.publish(ch, data).await?;

        Ok(Ok(()))
    }
}
