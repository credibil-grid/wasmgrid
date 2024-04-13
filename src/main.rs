mod messaging;

use anyhow::Error;
pub use async_nats::Client;
use clap::Parser;
use tokio::signal::unix::{signal, SignalKind};
use wasmtime::component::bindgen;
use wasmtime::{Config, Engine};

bindgen!({
    world: "messaging",
    path: "wit",
    tracing: true,
    async: true,
    with: {
        "wasi:messaging/messaging-types/client": Client,
    },
});

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

    // start messaging Host as non-blocking process
    // let builder = nats::Builder::new().engine(engine.clone()).wasm(args.wasm);
    let mut nats = messaging::Host::new(engine.clone(), args.wasm);
    tokio::spawn(async move { nats.run().await });

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
