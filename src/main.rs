mod handler;
mod http;
mod msg;

use anyhow::Error;
use clap::Parser;

use crate::handler::{HandlerProxy, Plugin};

/// Host wasm runtime for a vault service that stores signing keys and credentials for a Verifiable
/// Credential wallet.
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

    let mut plugins = Vec::<&dyn Plugin>::new();
    plugins.push(&http::Handler {});
    plugins.push(&msg::Handler {});

    let handler = HandlerProxy::new(args.wasm, plugins)?;

    // start messaging server
    let h = handler.clone();
    if let Err(e) = tokio::spawn(async move { msg::serve(h, args.nats_addr).await }).await {
        eprintln!("Error: {e:?}");
    };

    // start http server
    let h = handler.clone();
    if let Err(e) = tokio::spawn(async move { http::serve(h, args.http_addr).await }).await {
        eprintln!("Error: {e:?}");
    };

    shutdown().await
}

// Wait for shutdown signal
async fn shutdown() -> Result<(), Error> {
    tokio::select! {
        _ = tokio::signal::ctrl_c() => Ok(()),
    }
}
