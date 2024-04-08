use anyhow::anyhow;
use futures_util::stream::StreamExt;
use wasmtime::component::Resource;

use crate::wasi::messaging::consumer;
use crate::wasi::messaging::messaging_types::{Client, Error, GuestConfiguration, Message};

#[async_trait::async_trait]
impl consumer::Host for super::NatsHost {
    async fn subscribe_try_receive(
        &mut self, client: Resource<Client>, ch: String, t_milliseconds: u32,
    ) -> wasmtime::Result<anyhow::Result<Option<Vec<Message>>, Resource<Error>>> {
        println!("client: {client:?}");

        // self.client.subscribe(ch).await.map_or_else(|e| Err(anyhow!(e)), |_| Ok(Ok(None)))?;
        let Some(subscriber) = self.subscriber.as_mut() else {
            return Err(anyhow!("No subscriber found"));
        };

        while let Some(message) = subscriber.next().await {
            println!("received message: {:?}", message);
        }

        Ok(Ok(None))
    }

    async fn subscribe_receive(
        &mut self, client: Resource<Client>, ch: String,
    ) -> wasmtime::Result<anyhow::Result<Vec<Message>, Resource<Error>>> {
        todo!("Implement subscribe_receive for {client:?} on channel {ch}")
    }

    async fn update_guest_configuration(
        &mut self, gc: GuestConfiguration,
    ) -> wasmtime::Result<anyhow::Result<(), Resource<Error>>> {
        todo!("Implement update_guest_configuration {gc:?} ")
    }

    async fn complete_message(
        &mut self, msg: Message,
    ) -> wasmtime::Result<anyhow::Result<(), Resource<Error>>> {
        todo!("Implement complete_message for message {msg:?} ")
    }

    async fn abandon_message(
        &mut self, msg: Message,
    ) -> wasmtime::Result<anyhow::Result<(), Resource<Error>>> {
        todo!("Implement abandon_message for message {msg:?} ")
    }
}
