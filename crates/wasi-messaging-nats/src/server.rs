use async_nats::Message;
use futures::stream::{self, StreamExt};
use runtime::RunState;
use tracing::{Instrument, info_span};
use wasmtime::Store;
use wasmtime::component::InstancePre;

use super::generated::MessagingPre;
use super::generated::exports::wasi::messaging::incoming_handler::Error;

pub type Result<T, E = Error> = anyhow::Result<T, E>;

pub async fn run(pre: InstancePre<RunState>) -> anyhow::Result<()> {
    // bail if server is not required
    let component_type = pre.component().component_type();
    let mut exports = component_type.exports(pre.engine());
    if !exports.any(|e| e.0.starts_with("wasi:messaging")) {
        tracing::debug!("messaging server not required");
        return Ok(());
    }

    // get guest configuration
    let mut store = Store::new(pre.engine(), RunState::new());
    let msg_pre = MessagingPre::new(pre.clone())?;
    let msg = msg_pre.instantiate_async(&mut store).await?;
    let config = msg.wasi_messaging_incoming_handler().call_configure(&mut store).await??;

    // process requests
    subscribe(config.topics, msg_pre).await
}

pub async fn subscribe(channels: Vec<String>, pre: MessagingPre<RunState>) -> anyhow::Result<()> {
    tracing::trace!("subscribing to messaging channels: {channels:?}");

    let mut subscribers = vec![];
    let client = crate::nats()?;

    for ch in channels {
        tracing::debug!("subscribing to {ch}");
        let subscriber = client.subscribe(ch.clone()).await?;
        subscribers.push(subscriber);
    }

    // process messages until terminated
    let mut messages = stream::select_all(subscribers);
    while let Some(msg) = messages.next().await {
        let pre = pre.clone();

        tokio::spawn(
            async move {
                if let Err(e) = call_guest(pre, msg).await {
                    tracing::error!("error processing message {e}");
                }
            }
            .instrument(info_span!("message")),
        );
    }

    Ok(())
}

// Forward message to the wasm component.
async fn call_guest(pre: MessagingPre<RunState>, message: Message) -> Result<()> {
    let mut state = RunState::new();
    let res_msg = state.table.push(message)?;
    let mut store = Store::new(pre.engine(), state);
    let messaging = pre.instantiate_async(&mut store).await?;
    messaging.wasi_messaging_incoming_handler().call_handle(&mut store, res_msg).await?
}
