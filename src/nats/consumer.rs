use std::str::from_utf8;

use anyhow::anyhow;
use futures::stream::StreamExt;
use wasmtime::component::Resource;

use crate::wasi::messaging::consumer;
use crate::wasi::messaging::messaging_types::{
    Client, Error, FormatSpec, GuestConfiguration, Message,
};

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
        // self.subscribers.clear();

        let mut subs = vec![];

        for ch in &gc.channels {
            let subscriber = match self.client.subscribe(ch.to_owned()).await {
                Ok(sub) => sub,
                Err(e) => return Err(anyhow!(e)),
            };

            //self.subscribers.push(subscriber);
            subs.push(subscriber);
        }

        tokio::spawn(async move {
            let mut messages = futures::stream::select_all(subs).take(300);
            while let Some(message) = messages.next().await {
                println!(
                    "received message on subject {} with paylaod {}",
                    message.subject,
                    from_utf8(&message.payload).unwrap()
                );

                // send message to configured channel
                let msg = Message {
                    data: b"test".to_vec(),
                    metadata: Some(vec![(String::from("channel"), message.subject.to_string())]),
                    format: FormatSpec::Raw,
                };

                // let result =
                //     self.guest.unwrap().call_handler(self.as_context_mut(), &[msg]).await?;
                // println!("call_handler {result:?}");
            }
        });

        Ok(Ok(()))
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
