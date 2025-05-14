use anyhow::{Result, anyhow};
use futures::stream::{self, StreamExt};
use wasmtime::Store;
use wasmtime::component::InstancePre;
use wasmtime_wasi::IoView;

use crate::messaging::generated::MessagingPre;
use crate::messaging::generated::wasi::messaging::messaging_types::{FormatSpec, Message};
use crate::{Ctx, Resources};

pub async fn run(pre: InstancePre<Ctx>, resources: Resources) -> Result<()> {
    // bail if server is not required
    let component_type = pre.component().component_type();
    let mut exports = component_type.exports(pre.engine());
    if !exports.any(|e| e.0.starts_with("wasi:messaging")) {
        tracing::debug!("messaging server not required");
        return Ok(());
    }

    // get guest configuration
    let mut store = Store::new(pre.engine(), Ctx::new(resources.clone(), pre.clone()));
    let msg_pre = MessagingPre::new(pre.clone())?;
    let messaging = msg_pre.instantiate_async(&mut store).await?;
    let Ok(gc) = messaging.wasi_messaging_messaging_guest().call_configure(&mut store).await?
    else {
        return Err(anyhow!("failed to configure messaging guest"));
    };

    subscribe(gc.channels, &resources, &pre).await
}

pub async fn subscribe(
    channels: Vec<String>, resources: &Resources, pre: &InstancePre<Ctx>,
) -> Result<()> {
    let mut subscribers = vec![];
    let client = resources.nats()?;

    for ch in channels {
        tracing::debug!("subscribing to {ch}");
        let subscriber = client.subscribe(ch.clone()).await?;
        subscribers.push(subscriber);
    }

    // process messages until terminated
    let mut messages = stream::select_all(subscribers);
    while let Some(msg) = messages.next().await {
        let pre = pre.clone();
        let res = resources.clone();
        tokio::spawn(async move {
            if let Err(e) = call_guest(pre, res, msg).await {
                tracing::error!("error processing message {e}");
            }
        });
    }

    Ok(())
}

// Forward message to the wasm component.
async fn call_guest(
    pre: InstancePre<Ctx>, resources: Resources, msg: async_nats::Message,
) -> Result<()> {
    let mut store = Store::new(pre.engine(), Ctx::new(resources, pre.clone()));
    let msg_pre = MessagingPre::new(pre)?;
    let messaging = msg_pre.instantiate_async(&mut store).await?;
    let wasi_msg = msg_conv(&msg);

    if let Err(e) =
        messaging.wasi_messaging_messaging_guest().call_handler(&mut store, &[wasi_msg]).await?
    {
        let err = store.data_mut().table().get(&e)?;
        return Err(anyhow!("{err}"));
    }

    Ok(())
}

pub fn msg_conv(msg: &async_nats::Message) -> Message {
    Message {
        data: msg.payload.to_vec(),
        metadata: Some(vec![(String::from("channel"), msg.subject.to_string())]),
        format: FormatSpec::Raw,
    }
}
