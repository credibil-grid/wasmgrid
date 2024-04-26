mod http;
mod msg;
mod runtime;

use anyhow::Error;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to the wasm file to serve.
    wasm: String,

    /// The http host.
    #[arg(long, default_value = "localhost:8080")]
    http_addr: String,

    /// The NATS host.
    #[arg(long, default_value = "demo.nats.io")]
    nats_addr: String,
}

#[tokio::main]
pub async fn main() -> wasmtime::Result<()> {
    let args = Args::parse();

    runtime::Builder::new()
        .capability(http::Capability::new(args.http_addr))
        .capability(msg::Capability::new(args.nats_addr))
        .run(args.wasm)?;

    shutdown().await
}

// Wait for shutdown signal
async fn shutdown() -> Result<(), Error> {
    tokio::select! {
        _ = tokio::signal::ctrl_c() => Ok(()),
    }
}
