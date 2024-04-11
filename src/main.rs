mod nats;

use anyhow::Error;
pub use async_nats::Client;
use clap::Parser;
use futures::stream;
use futures::stream::StreamExt;
// use tokio::signal::unix::{signal, SignalKind};
use wasi::messaging::messaging_types::HostClient;
use wasmtime::component::{bindgen, Component, Linker};
use wasmtime::{AsContextMut, Config, Engine, Store};
use wasmtime_wasi::command;

use crate::wasi::messaging::messaging_types::{FormatSpec, Message};

bindgen!({
    world: "messaging",
    path: "wit",
    tracing: true,
    async: true,
    with: {
        "wasi:messaging/messaging-types/client": Client,
    },
});

use crate::nats::HostState;
use crate::wasi::messaging::{consumer, messaging_types, producer};

/// Host wasm runtime for a vault service that stores signing keys and credentials for a Verifiable
/// Credential wallet.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to the wasm file to run.
    wasm_file: String,
}

#[tokio::main]
pub async fn main() -> wasmtime::Result<()> {
    // let args = Args::parse();
    let wasm = include_bytes!("../target/wasm32-wasi/release/guest.wasm");

    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = Engine::new(&config)?;

    let mut linker = Linker::new(&engine);
    command::add_to_linker(&mut linker)?;
    messaging_types::add_to_linker(&mut linker, |t| t)?;
    producer::add_to_linker(&mut linker, |t| t)?;
    consumer::add_to_linker(&mut linker, |t| t)?;

    let component = Component::from_binary(&engine, wasm)?;

    let mut host_state = HostState::new();

    // TODO: move to nats package
    let client = host_state.connect("demo.nats.io".to_string()).await?.unwrap();
    let client = host_state.client(&client)?;

    let mut store = Store::new(&engine, host_state);
    let (messaging, _instance) =
        Messaging::instantiate_async(&mut store, &component, &linker).await?;

    let guest = messaging.wasi_messaging_messaging_guest();
    let gc = guest.call_configure(store.as_context_mut()).await?;
    let mut subscribers = vec![];
    for ch in &gc.unwrap().channels {
        let subscriber = client.subscribe(ch.to_owned()).await?;
        subscribers.push(subscriber);
    }

    let mut messages = stream::select_all(subscribers);
    while let Some(message) = messages.next().await {
        let msg = Message {
            data: message.payload.to_vec(),
            metadata: Some(vec![(String::from("channel"), message.subject.to_string())]),
            format: FormatSpec::Raw,
        };
        let _ = guest.call_handler(store.as_context_mut(), &[msg]).await?;
    }

    Ok::<(), Error>(())

    // shutdown().await
}

// // Wait for shutdown signal
// async fn shutdown() -> Result<(), Error> {
//     let mut sigint = signal(SignalKind::interrupt())?;
//     let mut sigterm = signal(SignalKind::terminate())?;
//     let mut sigquit = signal(SignalKind::quit())?;

//     tokio::select! {
//         _ = sigint.recv() => Ok(()),
//         _ = sigterm.recv() => Ok(()),
//         _ = sigquit.recv() => Ok(()),
//     }
// }
