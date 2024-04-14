// use std::collections::HashMap;

// use async_nats::Client;
use futures::stream::{self, StreamExt};
use wasmtime::component::{Component, Linker};
use wasmtime::{Engine, Store};
use wasmtime_wasi::command;

use crate::messaging;
use crate::wasi::messaging::messaging_types::{FormatSpec, HostClient, Message};

pub async fn serve(engine: &Engine, wasm: String) -> anyhow::Result<()> {
    let mut store = Store::new(engine, messaging::Host::new());
    let component = Component::from_file(engine, wasm)?;

    let mut linker = Linker::new(engine);
    command::add_to_linker(&mut linker)?;
    crate::Messaging::add_to_linker(&mut linker, |t| t)?;

    let (messaging, _) =
        crate::Messaging::instantiate_async(&mut store, &component, &linker).await?;
    let guest = messaging.wasi_messaging_messaging_guest();

    // connect to NATS server
    let host = store.data_mut();
    let Ok(client) = host.connect("demo.nats.io".to_string()).await? else {
        return Err(anyhow::anyhow!("Failed to connect to NATS server"));
    };
    let client = host.table.get(&client)?.clone();

    // get channels to subscribe to
    let Ok(gc) = guest.call_configure(&mut store).await? else {
        return Err(anyhow::anyhow!("Failed to configure NATS client"));
    };

    // subscribe to channels
    let mut subscribers = vec![];
    for ch in &gc.channels {
        let subscriber = client.subscribe(ch.to_owned()).await?;
        subscribers.push(subscriber);
    }

    // process messages until terminated
    let mut messages = stream::select_all(subscribers);
    while let Some(message) = messages.next().await {
        let msg = Message {
            data: message.payload.to_vec(),
            metadata: Some(vec![(String::from("channel"), message.subject.to_string())]),
            format: FormatSpec::Raw,
        };
        let _ = guest.call_handler(&mut store, &[msg]).await?;
    }

    Ok(())
}
