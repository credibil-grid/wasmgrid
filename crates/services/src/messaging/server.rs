use anyhow::{Result, anyhow};
use futures::stream::{self, StreamExt};
use wasmtime::Store;
use wasmtime::component::InstancePre;
use wasmtime_wasi::IoView;

use crate::Ctx;
use crate::messaging::generated::MessagingPre;
use crate::messaging::generated::wasi::messaging::messaging_types::{FormatSpec, Message};

pub type Client = async_nats::Client;

pub async fn run(pre: InstancePre<Ctx>, client: Client) -> Result<()> {
    // bail if server is not required
    let component_type = pre.component().component_type();
    let mut exports = component_type.exports(pre.engine());
    if !exports.any(|e| e.0.starts_with("wasi:rpc")) {
        tracing::debug!("rpc server not required");
        return Ok(());
    }

    // get guest configuration
    let pre = MessagingPre::new(pre.clone())?;
    let mut store = Store::new(pre.engine(), Ctx::new(client.clone()));
    let messaging = pre.instantiate_async(&mut store).await?;
    let Ok(gc) = messaging.wasi_messaging_messaging_guest().call_configure(&mut store).await?
    else {
        return Err(anyhow!("failed to configure messaging guest"));
    };

    subscribe(gc.channels, client, pre).await
}

async fn subscribe(channels: Vec<String>, client: Client, pre: MessagingPre<Ctx>) -> Result<()> {
    tracing::debug!("subscribing to requests");

    let mut subscribers = vec![];
    for ch in channels {
        let subscriber = client.subscribe(ch.clone()).await?;
        subscribers.push(subscriber);
    }

    // process messages until terminated
    let mut messages = stream::select_all(subscribers);
    while let Some(msg) = messages.next().await {
        let pre = pre.clone();
        let cli = client.clone();
        tokio::spawn(async move {
            if let Err(e) = call_guest(pre, cli, msg).await {
                tracing::error!("error processing message {e}");
            }
        });
    }

    Ok(())
}

// Forward message to the wasm Guest.
async fn call_guest(
    pre: MessagingPre<Ctx>, client: Client, msg: async_nats::Message,
) -> Result<()> {
    let mut store = Store::new(pre.engine(), Ctx::new(client));
    let messaging = pre.instantiate_async(&mut store).await?;
    let wasi_msg = msg_conv(msg);

    if let Err(e) =
        messaging.wasi_messaging_messaging_guest().call_handler(&mut store, &[wasi_msg]).await?
    {
        let err = store.data_mut().table().get(&e)?;
        return Err(anyhow!("{err}"));
    }

    Ok(())
}

pub fn msg_conv(msg: async_nats::Message) -> Message {
    Message {
        data: msg.payload.to_vec(),
        metadata: Some(vec![(String::from("channel"), msg.subject.to_string())]),
        format: FormatSpec::Raw,
    }
}
