#![allow(clippy::redundant_pub_crate)]
#![feature(let_chains)]
#![feature(duration_constructors)]

mod runtime;
mod service;

use anyhow::Error;
use clap::Parser;
use dotenv::dotenv;
use runtime::Runtime;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

// const DEF_MGO_CNN: &str = "mongodb://localhost:27017";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the wasm file to host.
    wasm: String,

    /// Compile the wasm file only.
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
        runtime::compile(args.wasm)?;
        return Ok(());
    }

    // init services
    let runtime = Runtime::new();

    runtime.start(args.wasm)?;

    shutdown().await
}

// Wait for shutdown signal
async fn shutdown() -> Result<(), Error> {
    tokio::select! {
        _ = tokio::signal::ctrl_c() => Ok(()),
    }
}
