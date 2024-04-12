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

/// Host wasm runtime for a vault service that stores signing keys and credentials for a Verifiable
/// Credential wallet.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to the wasm file to run.
    #[arg(short, long)]
    wasm: String,
}

#[tokio::main]
pub async fn main() -> wasmtime::Result<()> {
    let args = Args::parse();

    // initialise Engine (compilation/management of wasm modules)
    let mut config = Config::new();
    config.async_support(true);
    let engine = Engine::new(&config)?;

    // messaging state store
    let mut store = Store::new(&engine, Nats::default());

    // load wasm (Guest)
    let component = Component::from_file(&engine, args.wasm)?;

    // link dependencies
    let mut linker = Linker::new(&engine);
    command::add_to_linker(&mut linker)?;
    Messaging::add_to_linker(&mut linker, |t| t)?;

    let (messaging, _) = Messaging::instantiate_async(&mut store, &component, &linker).await?;

    // start Host as non-blocking process
    tokio::spawn(
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
