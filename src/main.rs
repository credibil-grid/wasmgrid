mod messaging;
mod nats;

use anyhow::Error;
use clap::Parser;
use wasmtime::{Config, Engine};

/// Host wasm runtime for a vault service that stores signing keys and credentials for a Verifiable
/// Credential wallet.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to the wasm file to serve.
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
    tokio::spawn(async move { nats::serve(&engine, args.wasm).await });

    shutdown().await
}

// Wait for shutdown signal
async fn shutdown() -> Result<(), Error> {
    tokio::select! {
        _ = tokio::signal::ctrl_c() => Ok(()),
    }
}
