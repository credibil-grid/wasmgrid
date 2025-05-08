//! # Wasmgrid CLI

use anyhow::Error;
use clap::Parser;
use dotenv::dotenv;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasmgrid::Runtime;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the wasm file to host.
    guest: String,

    /// Compile the wasm file specified by `--wasm`.
    #[arg(short, long, default_value_t = false)]
    compile: bool,

    /// Path to a compiled wasm file.
    #[arg(short, long)]
    binary: Option<String>,
}

#[tokio::main]
pub async fn main() -> wasmtime::Result<()> {
    if cfg!(debug_assertions) {
        dotenv().ok();
    }
    let subscriber =
        FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
    tracing::subscriber::set_global_default(subscriber)?;
    tracing::trace!("initializing");

    let args = Args::parse();
    if args.compile {
        wasmgrid::compile(args.guest)?;
        return Ok(());
    }

    // init services
    let runtime = Runtime::new();
    runtime.start(args.guest)?;

    shutdown().await
}

// Wait for shutdown signal
async fn shutdown() -> Result<(), Error> {
    tokio::select! {
        _ = tokio::signal::ctrl_c() => Ok(()),
    }
}
