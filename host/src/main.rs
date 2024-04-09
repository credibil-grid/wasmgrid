mod nats;

// use anyhow::{anyhow, bail, Context};
use clap::Parser;
use futures_util::stream::StreamExt;
use wasmtime::component::{bindgen, Component, Linker};
use wasmtime::{AsContextMut, Config, Engine, Store};
use wasmtime_wasi::command;

bindgen!({ path: "../wit", world: "messaging", async: true });
use wasi::messaging::messaging_types::{FormatSpec, Message};

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
    let wasm = include_bytes!("../../target/wasm32-wasi/release/guest.wasm");
    let host_state = HostState::new().await?;

    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = Engine::new(&config)?;

    // let component = Component::from_file(&engine, file)?;
    let component = Component::from_binary(&engine, wasm)?;
    let mut store = Store::new(&engine, host_state);
    let mut linker = Linker::new(&engine);
    add_to_linker(&mut linker)?;

    let (messaging, _) = Messaging::instantiate_async(&mut store, &component, &linker).await?;

    // get channels guest wants to subscribe to
    // N.B. As soon as configuration is retrieved, we should kill the wasm instance.
    let gc =
        messaging.wasi_messaging_messaging_guest().call_configure(store.as_context_mut()).await?;
    println!("{gc:?}");

    // subscribe to channels
    let hs = store.data_mut();
    hs.init(gc.unwrap()).await?;

    // for sub in hs.subscribers.iter_mut() {
    //     tokio::spawn(async move {
    //         while let Some(message) = sub.next().await {
    //             println!("received message: {:?}", message);
    //         }
    //     });
    // }

    // send message to configured channel
    let msg = Message {
        data: b"test".to_vec(),
        metadata: Some(vec![(String::from("channel"), String::from("d"))]),
        format: FormatSpec::Raw,
    };
    let result = messaging
        .wasi_messaging_messaging_guest()
        .call_handler(store.as_context_mut(), &[msg])
        .await?;
    println!("call_handler {result:?}");

    let msg = Message {
        data: b"test 2".to_vec(),
        metadata: Some(vec![(String::from("channel"), String::from("b"))]),
        format: FormatSpec::Raw,
    };
    let result = messaging
        .wasi_messaging_messaging_guest()
        .call_handler(store.as_context_mut(), &[msg])
        .await?;
    println!("call_handler {result:?}");

    Ok(())
}

fn add_to_linker(l: &mut Linker<HostState>) -> anyhow::Result<()> {
    command::add_to_linker(l)?;
    messaging_types::add_to_linker(l, |t| t)?;
    producer::add_to_linker(l, |t| t)?;
    consumer::add_to_linker(l, |t| t)?;
    Ok(())
}
