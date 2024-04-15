// use std::str::from_utf8;

use anyhow::anyhow;
use futures::stream::StreamExt;
use tokio::time::{sleep, Duration};
use wasmtime::component::Resource;

use crate::messaging::WasiMessagingView;
use crate::wasi::messaging::consumer;
use crate::wasi::messaging::messaging_types::{
    Client, Error, FormatSpec, GuestConfiguration, Message,
};

#[async_trait::async_trait]
impl<T: WasiMessagingView> consumer::Host for T {
    async fn subscribe_try_receive(
        &mut self, client: Resource<Client>, ch: String, t_milliseconds: u32,
    ) -> wasmtime::Result<anyhow::Result<Option<Vec<Message>>, Resource<Error>>> {
        // subscribe to channel
        let client = self.table().get(&client)?;
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
        let client = self.table().get(&client)?;
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
        // TODO: implement update_guest_configuration

        // let builder = super::Builder::new().engine(self.engine.clone()).wasm(self.wasm.clone());
        // tokio::spawn(async move { builder.run().await });

        Ok(Ok(()))
    }

    async fn complete_message(
        &mut self, _msg: Message,
    ) -> wasmtime::Result<anyhow::Result<(), Resource<Error>>> {
        println!("Implement complete_message");
        Ok(Ok(()))
    }

    async fn abandon_message(
        &mut self, _msg: Message,
    ) -> wasmtime::Result<anyhow::Result<(), Resource<Error>>> {
        println!("Implement abandon_message");
        Ok(Ok(()))
    }
}
