use anyhow::anyhow;
use bytes::Bytes;
use wasmtime::component::Resource;

use crate::wasi::messaging::messaging_types::{Client, Error, Message};
use crate::wasi::messaging::producer;

#[async_trait::async_trait]
impl producer::Host for super::HostState {
    async fn send(
        &mut self, client: Resource<Client>, ch: String, msg: Vec<Message>,
    ) -> wasmtime::Result<anyhow::Result<(), Resource<Error>>> {
        println!("send: ch: {ch}");
        let client = self.table.get(&client).unwrap();

        let data = Bytes::from(msg[0].data.clone());
        client.publish(ch, data).await.map_or_else(|e| Err(anyhow!(e)), |_| Ok(Ok(())))
    }
}
