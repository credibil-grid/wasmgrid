#![allow(clippy::redundant_pub_crate)]

mod capabilities;
mod runtime;

use std::env;

use anyhow::Error;
use clap::Parser;
use dotenv::dotenv;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use crate::capabilities::{http, keyvalue, messaging, signature, sql};

const DEF_HTTP_ADDR: &str = "0.0.0.0:8080";
const DEF_NATS_CNN: &str = "demo.nats.io";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to the wasm file to serve.
    wasm: String,
    // /// The http host.
    // #[arg(long, default_value = "0.0.0.0:8080")]
    // http_addr: String,

    // /// The NATS host.
    // #[arg(long, default_value = "demo.nats.io")]
    // nats_addr: String,

    // /// The `MongoDB` connection string.
    // #[arg(long)]
    // mgo_cnn: String,
}

#[tokio::main]
pub async fn main() -> wasmtime::Result<()> {
    let args = Args::parse();

    // env vars
    if cfg!(debug_assertions) {
        dotenv().ok();
    }
    let http_addr = env::var("HTTP_ADDR").unwrap_or(DEF_HTTP_ADDR.to_string());
    let nats_cnn = env::var("NATS_CNN").unwrap_or(DEF_NATS_CNN.to_string());
    let mgo_cnn = env::var("MGO_CNN").expect("MGO_CNN should be set");

    // tracing
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        // .with_max_level(tracing::Level::DEBUG)
        // .with_env_filter("wasmgrid=debug")
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    runtime::Builder::new()
        .capability(http::new(http_addr))
        .capability(messaging::new(nats_cnn.clone()))
        .capability(keyvalue::new(nats_cnn))
        .capability(signature::new())
        .capability(sql::new(mgo_cnn))
        .run(args.wasm)?;

    shutdown().await
}

// Wait for shutdown signal
async fn shutdown() -> Result<(), Error> {
    tokio::select! {
        _ = tokio::signal::ctrl_c() => Ok(()),
    }
}
