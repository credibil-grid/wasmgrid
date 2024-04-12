mod nats;

use anyhow::Error;
pub use async_nats::Client;
use clap::Parser;
// use tokio::signal::unix::{signal, SignalKind};
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
use crate::wasi::messaging::{self, messaging_types};

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

    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = Engine::new(&config)?;

    let wasm = include_bytes!("../target/wasm32-wasi/release/guest.wasm");
    let component = Component::from_binary(&engine, wasm)?;

    let mut linker = Linker::new(&engine);
    command::add_to_linker(&mut linker)?;
    messaging_types::add_to_linker(&mut linker, |t| t)?;
    messaging::producer::add_to_linker(&mut linker, |t| t)?;
    messaging::consumer::add_to_linker(&mut linker, |t| t)?;

    let mut store = Store::new(&engine, Nats::new());
    let (messaging, _) = Messaging::instantiate_async(&mut store, &component, &linker).await?;

    Nats::run(&mut store, &messaging).await?;

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
