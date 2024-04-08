mod nats;

// use anyhow::{anyhow, bail, Context};
use clap::Parser;
use wasmtime::component::{bindgen, Component, Linker};
use wasmtime::{AsContextMut, Config, Engine, Store};
use wasmtime_wasi::command;

bindgen!({ path: "../wit", world: "messaging", async: true });
use wasi::messaging::messaging_types::{FormatSpec, Message};

use crate::nats::NatsHost;
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
    let file = "target/wasm32-wasi/release/guest.wasm";

    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = Engine::new(&config)?;

    let component = Component::from_file(&engine, file)?;
    let mut store = Store::new(&engine, NatsHost::new().await?);
    let mut linker = Linker::new(&engine);
    add_to_linker(&mut linker)?;

    let (messaging, _) = Messaging::instantiate_async(&mut store, &component, &linker).await?;

    // get channels guest wants to subscribe to
    // N.B. As soon as configuration is retrieved, we should kill the wasm instance.
    let gc =
        messaging.wasi_messaging_messaging_guest().call_configure(store.as_context_mut()).await;
    println!("{gc:?}");

    // send message to configured channel
    let msg = Message {
        data: b"test".to_vec(),
        metadata: Some(vec![(String::from("channel"), String::from("b"))]),
        format: FormatSpec::Raw,
    };

    let result = messaging
        .wasi_messaging_messaging_guest()
        .call_handler(store.as_context_mut(), &[msg])
        .await?;
    println!("{result:?}");

    Ok(())
}

fn add_to_linker(l: &mut Linker<NatsHost>) -> anyhow::Result<()> {
    command::add_to_linker(l)?;
    messaging_types::add_to_linker(l, |t| t)?;
    producer::add_to_linker(l, |t| t)?;
    consumer::add_to_linker(l, |t| t)?;
    Ok(())
}
