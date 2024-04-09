use anyhow::anyhow;
use futures_util::stream::StreamExt;
use wasmtime::component::Resource;

use crate::wasi::messaging::consumer;
use crate::wasi::messaging::messaging_types::{Client, Error, GuestConfiguration, Message};

#[async_trait::async_trait]
impl consumer::Host for super::HostState {
    async fn subscribe_try_receive(
        &mut self, client: Resource<Client>, ch: String, _t_milliseconds: u32,
    ) -> wasmtime::Result<anyhow::Result<Option<Vec<Message>>, Resource<Error>>> {
        println!("subscribe_try_receive: ch: {ch}");

        let mut subscriber = match self.client.subscribe(ch).await {
            Ok(sub) => sub,
            Err(e) => return Err(anyhow!(e)),
        };

        let _handle = tokio::spawn(async move {
            while let Some(message) = subscriber.next().await {
                println!("received message: {:?}", message);
            }
        });

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
        for ch in &gc.channels {
            let subscriber = match self.client.subscribe(ch.to_owned()).await {
                Ok(sub) => sub,
                Err(e) => return Err(anyhow!(e)),
            };

            self.subscribers.push(subscriber);
        }

        Ok(Ok(()))
        // todo!("Implement update_guest_configuration {gc:?} ")
    }

    async fn complete_message(
        &mut self, msg: Message,
    ) -> wasmtime::Result<anyhow::Result<(), Resource<Error>>> {
        // todo!("Implement complete_message for message {msg:?} ")
        println!("complete_message: {msg:?}");
        Ok(Ok(()))
    }

    async fn abandon_message(
        &mut self, msg: Message,
    ) -> wasmtime::Result<anyhow::Result<(), Resource<Error>>> {
        todo!("Implement abandon_message for message {msg:?} ")
    }
}
