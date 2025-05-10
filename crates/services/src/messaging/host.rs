use anyhow::{Error, Result};
use async_nats::Client;
use futures::StreamExt;
use tokio::time::{Duration, sleep};
use wasmtime::component::{Linker, Resource};
use wasmtime_wasi::ResourceTable;

use crate::messaging::generated::wasi::messaging::messaging_types::{GuestConfiguration, Message};
use crate::messaging::generated::wasi::messaging::{consumer, messaging_types, producer};
use crate::messaging::server;

pub struct MsgHost<'a> {
    client: &'a Client,
    table: &'a mut ResourceTable,
}

impl<'a> MsgHost<'a> {
    pub const fn new(client: &'a Client, table: &'a mut ResourceTable) -> Self {
        Self { client, table }
    }
}

/// Add all the `wasi-keyvalue` world's interfaces to a [`Linker`].
pub fn add_to_linker<T: Send>(
    l: &mut Linker<T>, f: impl Fn(&mut T) -> MsgHost<'_> + Send + Sync + Copy + 'static,
) -> Result<()> {
    consumer::add_to_linker_get_host(l, f)?;
    producer::add_to_linker_get_host(l, f)?;
    messaging_types::add_to_linker_get_host(l, f)
}

impl messaging_types::Host for MsgHost<'_> {}

impl messaging_types::HostClient for MsgHost<'_> {
    async fn connect(&mut self, name: String) -> Result<Result<Resource<Client>, Resource<Error>>> {
        tracing::trace!("HostClient::connect {name}");
        let client = self.client;
        let resource = self.table.push(client.clone())?;
        Ok(Ok(resource))
    }

    async fn drop(&mut self, rep: Resource<Client>) -> Result<()> {
        tracing::trace!("HostClient::drop");
        self.table.delete(rep)?;
        Ok(())
    }
}

// Host produces messages.
impl producer::Host for MsgHost<'_> {
    async fn send(
        &mut self, client: Resource<Client>, ch: String, messages: Vec<Message>,
    ) -> Result<Result<(), Resource<Error>>> {
        tracing::trace!("producer::Host::send: {:?}", ch);

        let client = self.table.get(&client)?;
        for m in messages {
            let data = m.data.clone().into();
            client.publish(ch.clone(), data).await?;
        }

        Ok(Ok(()))
    }
}

// Host consumes messages.
impl consumer::Host for MsgHost<'_> {
    async fn subscribe_try_receive(
        &mut self, rep: Resource<Client>, ch: String, t_milliseconds: u32,
    ) -> Result<Result<Option<Vec<Message>>, Resource<Error>>> {
        tracing::debug!("consumer::Host::subscribe_try_receive {ch}, {t_milliseconds}");

        // subscribe to channel
        let client = self.table.get(&rep)?;
        let mut subscriber = client.subscribe(ch).await?;

        // create stream that times out after `t_milliseconds`
        let stream =
            subscriber.by_ref().take_until(sleep(Duration::from_millis(u64::from(t_milliseconds))));
        let messages = stream.map(server::msg_conv).collect().await;
        subscriber.unsubscribe().await?;

        Ok(Ok(Some(messages)))
    }

    async fn subscribe_receive(
        &mut self, rep: Resource<Client>, ch: String,
    ) -> Result<Result<Vec<Message>, Resource<Error>>> {
        tracing::trace!("consumer::Host::subscribe_receive {ch}");

        let client = self.table.get(&rep)?;
        let mut subscriber = client.subscribe(ch).await?;
        let messages = subscriber.by_ref().take(1).map(server::msg_conv).collect().await;
        subscriber.unsubscribe().await?;

        Ok(Ok(messages))
    }

    // TODO: implement `complete_message` using JetStream
    async fn complete_message(&mut self, msg: Message) -> Result<Result<(), Resource<Error>>> {
        tracing::warn!("TODO: consumer::Host::complete_message: {:?}", msg.metadata);
        Ok(Ok(()))
    }

    // TODO: implement `abandon_message` using JetStream
    async fn abandon_message(&mut self, msg: Message) -> Result<Result<(), Resource<Error>>> {
        tracing::warn!("TODO: consumer::Host::abandon_message: {:?}", msg.metadata);
        Ok(Ok(()))
    }

    async fn update_guest_configuration(
        &mut self, gc: GuestConfiguration,
    ) -> Result<Result<(), Resource<Error>>> {
        tracing::trace!("consumer::Host::update_guest_configuration");

        // let processor = Processor {
        //     pre: None,
        //     client: self.client.clone(),
        // };
        // processor.subscribe(gc.channels).await?;

        Ok(Ok(()))
    }
}

impl messaging_types::HostError for MsgHost<'_> {
    async fn trace(&mut self) -> Result<String> {
        tracing::trace!("HostError::trace");
        Ok("error".to_string())
    }

    async fn drop(&mut self, rep: Resource<Error>) -> Result<()> {
        tracing::trace!("HostError::drop");
        self.table.delete(rep)?;
        Ok(())
    }
}
