#![allow(clippy::redundant_pub_crate)]

mod capabilities;
mod runtime;

use anyhow::Error;
use clap::Parser;
// use tracing_subscriber::{EnvFilter, FmtSubscriber};
use tracing_subscriber::FmtSubscriber;

use crate::capabilities::{http, keyvalue, messaging, signature, sql};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to the wasm file to serve.
    wasm: String,

    /// The http host.
    #[arg(long, default_value = "0.0.0.0:8080")]
    http_addr: String,

    /// The NATS host.
    #[arg(long, default_value = "demo.nats.io")]
    nats_addr: String,

    /// The MongoDB connection string.
    #[arg(long, default_value = "demo.nats.io")]
    mgo_cnn: String,
}

#[tokio::main]
pub async fn main() -> wasmtime::Result<()> {
    let args = Args::parse();

    // tracing
    let subscriber = FmtSubscriber::builder()
        // .with_env_filter(EnvFilter::from_default_env())
        // .with_max_level(tracing::Level::DEBUG)
        .with_env_filter("wasmgrid=debug,wasi_sql=debug")
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    runtime::Builder::new()
        .capability(http::new(args.http_addr))
        .capability(messaging::new(args.nats_addr.clone()))
        .capability(keyvalue::new(args.nats_addr))
        .capability(signature::new())
        .capability(sql::new(args.mgo_cnn))
        .run(args.wasm)?;

    shutdown().await
}

// Wait for shutdown signal
async fn shutdown() -> Result<(), Error> {
    tokio::select! {
        _ = tokio::signal::ctrl_c() => Ok(()),
    }
}
