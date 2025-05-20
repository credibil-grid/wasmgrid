use anyhow::{Error, Result};
use async_nats::Client;
use futures::StreamExt;
use tokio::time::{Duration, sleep};
use wasmtime::component::{InstancePre, Linker, Resource};
use wasmtime_wasi::ResourceTable;

use crate::messaging::generated::wasi::messaging::messaging_types::{GuestConfiguration, Message};
use crate::messaging::generated::wasi::messaging::{consumer, messaging_types, producer};
use crate::messaging::server;
use crate::{Ctx, Resources};

pub struct MsgHost<'a> {
    table: &'a mut ResourceTable,
    instance_pre: &'a InstancePre<Ctx>,
    resources: &'a Resources,
}

impl MsgHost<'_> {
    const fn new(c: &mut Ctx) -> MsgHost<'_> {
        MsgHost {
            table: &mut c.table,
            instance_pre: &c.instance_pre,
            resources: &c.resources,
        }
    }
}

/// Add all the `wasi-keyvalue` world's interfaces to a [`Linker`].
pub fn add_to_linker(l: &mut Linker<Ctx>) -> Result<()> {
    consumer::add_to_linker_get_host(l, MsgHost::new)?;
    producer::add_to_linker_get_host(l, MsgHost::new)?;
    messaging_types::add_to_linker_get_host(l, MsgHost::new)
}

impl messaging_types::Host for MsgHost<'_> {
    fn convert_error(&mut self, err: Error) -> anyhow::Result<Error> {
        Ok(err)
    }
}

impl messaging_types::HostClient for MsgHost<'_> {
    async fn connect(&mut self, name: String) -> Result<Result<Resource<Client>, Resource<Error>>> {
        tracing::trace!("HostClient::connect {name}");
        let client = self.resources.nats()?;
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
        let messages = stream.map(|m| server::msg_conv(&m)).collect().await;
        subscriber.unsubscribe().await?;

        Ok(Ok(Some(messages)))
    }

    async fn subscribe_receive(
        &mut self, rep: Resource<Client>, ch: String,
    ) -> Result<Result<Vec<Message>, Resource<Error>>> {
        tracing::trace!("consumer::Host::subscribe_receive {ch}");

        let client = self.table.get(&rep)?;
        let mut subscriber = client.subscribe(ch).await?;
        let messages = subscriber.by_ref().take(1).map(|m| server::msg_conv(&m)).collect().await;
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
        server::subscribe(gc.channels, self.resources, self.instance_pre).await?;
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
