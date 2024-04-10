mod nats;

use std::str::from_utf8;

use anyhow::{Error, Result};
use clap::Parser;
use futures::stream::StreamExt;
use tokio::signal::unix::{signal, SignalKind};
use wasmtime::component::{bindgen, Component, Linker};
use wasmtime::{AsContextMut, Config, Engine, Store};
use wasmtime_wasi::command;

bindgen!({ path: "wit", world: "messaging", async: true });
// use wasi::messaging::messaging_types::{FormatSpec, Message};

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
    // let file = "target/wasm32-wasi/release/guest.wasm";
    let wasm = include_bytes!("../target/wasm32-wasi/release/guest.wasm");
    let mut host_state = HostState::new().await?;

    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = Engine::new(&config)?;

    // let component = Component::from_file(&engine, file)?;
    let component = Component::from_binary(&engine, wasm)?;
    let mut store = Store::new(&engine, host_state);
    let mut linker = Linker::new(&engine);

    command::add_to_linker(&mut linker)?;
    messaging_types::add_to_linker(&mut linker, |t| t)?;
    producer::add_to_linker(&mut linker, |t| t)?;
    consumer::add_to_linker(&mut linker, |t| t)?;

    let (messaging, _) = Messaging::instantiate_async(&mut store, &component, &linker).await?;

    // host_state.store = Some(Box::new(store));

    // get channels guest wants to subscribe to
    // N.B. As soon as configuration is retrieved, we should kill the wasm instance.
    let gc =
        messaging.wasi_messaging_messaging_guest().call_configure(store.as_context_mut()).await?;

    // subscribe to channels
    let hs = store.data_mut();
    hs.init(gc.unwrap()).await?;

    //-------------------------------------------------------------------------
    // NATS
    //-------------------------------------------------------------------------

    tokio::spawn({
        let client = hs.client.clone();
        async move {
            for i in 0..100 {
                client.publish("b", format!("car number {i}").into()).await?;
            }
            Ok::<(), Error>(())
        }
    });
    tokio::spawn({
        let client = hs.client.clone();
        async move {
            for i in 0..100 {
                client.publish("c", format!("ship number {i}").into()).await?;
            }
            Ok::<(), Error>(())
        }
    });
    tokio::spawn({
        let client = hs.client.clone();
        async move {
            for i in 0..100 {
                client.publish("d", format!("plane number {i}").into()).await?;
            }
            Ok::<(), Error>(())
        }
    });

    await_shutdown().await?;

    //-------------------------------------------------------------------------
    //-------------------------------------------------------------------------

    // // send message to configured channel
    // let msg = Message {
    //     data: b"test".to_vec(),
    //     metadata: Some(vec![(String::from("channel"), String::from("d"))]),
    //     format: FormatSpec::Raw,
    // };
    // let result = messaging
    //     .wasi_messaging_messaging_guest()
    //     .call_handler(store.as_context_mut(), &[msg])
    //     .await?;
    // println!("call_handler {result:?}");

    // let msg = Message {
    //     data: b"test 2".to_vec(),
    //     metadata: Some(vec![(String::from("channel"), String::from("b"))]),
    //     format: FormatSpec::Raw,
    // };
    // let result = messaging
    //     .wasi_messaging_messaging_guest()
    //     .call_handler(store.as_context_mut(), &[msg])
    //     .await?;
    // println!("call_handler {result:?}");

    Ok(())
}

async fn await_shutdown() -> Result<(), Error> {
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;
    let mut sigquit = signal(SignalKind::quit())?;

    tokio::select! {
        _ = sigint.recv() => Ok::<(), Error>(()),
        _ = sigterm.recv() => Ok(()),
        _ = sigquit.recv() => Ok(()),
    }
}
