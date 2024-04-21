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

    /// The http host.
    #[arg(long)]
    #[arg(default_value = "localhost:8080")]
    http_host: String,

    /// The NATS host.
    #[arg(long)]
    #[arg(default_value = "demo.nats.io")]
    nats_host: String,
}

#[tokio::main]
pub async fn main() -> wasmtime::Result<()> {
    let args = Args::parse();

    // initialise Engine to compile and manage wasm modules
    let mut config = Config::new();
    config.async_support(true);
    let engine = Engine::new(&config)?;

    // start messaging Host
    let e = engine.clone();
    let w = args.wasm.clone();
    tokio::spawn(async move { messaging::serve(e, w, args.nats_host).await });

    // start Http server
    let e = engine.clone();
    let w = args.wasm.clone();
    tokio::spawn(async move { http::serve(e, w, args.http_host).await });

    shutdown().await
}

// Wait for shutdown signal
async fn shutdown() -> Result<(), Error> {
    tokio::select! {
        _ = tokio::signal::ctrl_c() => Ok(()),
    }
}
