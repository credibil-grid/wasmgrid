mod nats;

use anyhow::Error;
pub use async_nats::Client;
use clap::Parser;
use tokio::signal::unix::{signal, SignalKind};
use wasmtime::component::{bindgen, Component, Linker};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::command;

bindgen!({
    world: "messaging",
    path: "wit",
    tracing: true,
    async: true,
    with: {
        "wasi:messaging/messaging-types/client": Client,
    },
});

use crate::nats::Nats;
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

    // Initialise Engine (global context for compilation/management of wasm modules)
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = Engine::new(&config)?;

    // link dependencies — the wasmtime command and messaging types
    let mut linker = Linker::new(&engine);
    command::add_to_linker(&mut linker)?;
    messaging_types::add_to_linker(&mut linker, |t| t)?;
    producer::add_to_linker(&mut linker, |t| t)?;
    consumer::add_to_linker(&mut linker, |t| t)?;

    // load wasm Guest
    let wasm = include_bytes!("../target/wasm32-wasi/release/guest.wasm");
    let component = Component::from_binary(&engine, wasm)?;

    // start NATS messaging Host
    let mut store = Store::new(&engine, Nats::default());
    let (messaging, _) = Messaging::instantiate_async(&mut store, &component, &linker).await?;
    tokio::spawn(
        // run in a non-blocking manner
        async move { Nats::run(&mut store, messaging.wasi_messaging_messaging_guest()).await },
    );

    shutdown().await
}

// Wait for shutdown signal
async fn shutdown() -> Result<(), Error> {
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;
    let mut sigquit = signal(SignalKind::quit())?;

    tokio::select! {
        _ = sigint.recv() => Ok(()),
        _ = sigterm.recv() => Ok(()),
        _ = sigquit.recv() => Ok(()),
    }
}
