// use std::str::from_utf8;

use anyhow::anyhow;
use futures::stream::StreamExt;
use tokio::time::{sleep, Duration};
use wasmtime::component::Resource;

use crate::wasi::messaging::consumer;
use crate::wasi::messaging::messaging_types::{
    Client, Error, FormatSpec, GuestConfiguration, Message,
};

#[async_trait::async_trait]
impl consumer::Host for super::Nats {
    async fn subscribe_try_receive(
        &mut self, client: Resource<Client>, ch: String, t_milliseconds: u32,
    ) -> wasmtime::Result<anyhow::Result<Option<Vec<Message>>, Resource<Error>>> {
        // subscribe to channel
        let client = self.table.get(&client)?;
        let mut subscriber = match client.subscribe(ch).await {
            Ok(s) => s,
            Err(e) => return Err(anyhow!(e)),
        };

        // TODO: remove spawn task
        let _result = tokio::spawn(async move {
            let stream = subscriber
                .by_ref()
                .take_until(sleep(Duration::from_millis(u64::from(t_milliseconds))));
            let messages = stream
                .map(|m| Message {
                    data: m.payload.to_vec(),
                    metadata: Some(vec![(String::from("channel"), m.subject.to_string())]),
                    format: FormatSpec::Raw,
                })
                .collect::<Vec<_>>()
                .await;

            let _ = subscriber.unsubscribe().await;

            Ok::<Vec<Message>, Error>(messages)
        });

        Ok(Ok(None))
    }

    async fn subscribe_receive(
        &mut self, client: Resource<Client>, ch: String,
    ) -> wasmtime::Result<anyhow::Result<Vec<Message>, Resource<Error>>> {
        let client = self.table.get(&client)?;
        let mut subscriber = match client.subscribe(ch).await {
            Ok(s) => s,
            Err(e) => return Err(anyhow!(e)),
        };

        let messages = subscriber
            .by_ref()
            .take(1)
            .map(|m| Message {
                data: m.payload.to_vec(),
                metadata: Some(vec![(String::from("channel"), m.subject.to_string())]),
                format: FormatSpec::Raw,
            })
            .collect::<Vec<_>>()
            .await;

        let _ = subscriber.unsubscribe().await;

        Ok(Ok(messages))
    }

    async fn update_guest_configuration(
        &mut self, _gc: GuestConfiguration,
    ) -> wasmtime::Result<anyhow::Result<(), Resource<Error>>> {
        // self.subscribers.clear();

        // let mut subs = vec![];
        // for ch in &gc.channels {
        //     let subscriber = self.client.subscribe(ch.to_owned()).await.map_err(|e| anyhow!(e))?;
        //     self.subscribers.push(subscriber);
        // }

        // tokio::spawn(async move {
        //     let mut messages = futures::stream::select_all(subs).take(300);
        //     while let Some(message) = messages.next().await {
        //         println!(
        //             "received message on subject {} with paylaod {}",
        //             message.subject,
        //             from_utf8(&message.payload).unwrap()
        //         );

        //         // send message to configured channel
        //         let msg = Message {
        //             data: b"test".to_vec(),
        //             metadata: Some(vec![(String::from("channel"), message.subject.to_string())]),
        //             format: FormatSpec::Raw,
        //         };

        //         // let result =
        //         //     self.guest.unwrap().call_handler(store.as_context_mut(), &[msg]).await?;
        //         // println!("call_handler {result:?}");
        //     }
        // });

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
        println!("Implement abandon_message for message {msg:?} ");
        Ok(Ok(()))
    }
}
