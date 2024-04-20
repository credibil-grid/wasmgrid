mod http;
mod messaging;

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

    // start messaging Host
    // let e = engine.clone();
    // let w = args.wasm.clone();
    // tokio::spawn(async move { messaging::serve(e, w, "demo.nats.io".to_string()).await });

    // start Http server
    let e = engine.clone();
    let w = args.wasm.clone();
    tokio::spawn(async move { http::serve(e, w, "demo.nats.io".to_string()).await });

    shutdown().await
}

// Wait for shutdown signal
async fn shutdown() -> Result<(), Error> {
    tokio::select! {
        _ = tokio::signal::ctrl_c() => Ok(()),
    }
}
