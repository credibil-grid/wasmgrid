#![allow(clippy::redundant_pub_crate)]

mod capabilities;
mod runtime;

use std::env;

use anyhow::Error;
use clap::Parser;
use dotenv::dotenv;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use crate::capabilities::{docdb, http, keyvalue, messaging, p2p, signature, wrpc};

const DEF_HTTP_ADDR: &str = "0.0.0.0:8080";
const DEF_MGO_CNN: &str = "mongodb://localhost:27017";
const DEF_NATS_CNN: &str = "demo.nats.io";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to the wasm file to serve.
    wasm: String,
}

#[tokio::main]
pub async fn main() -> wasmtime::Result<()> {
    let args = Args::parse();

    // env vars
    if cfg!(debug_assertions) {
        dotenv().ok();
        env::set_var("RUST_LOG", "none,wasmgrid=debug,http_p2p=debug");
    }

    let http_addr = env::var("HTTP_ADDR").unwrap_or_else(|_| DEF_HTTP_ADDR.to_string());
    let nats_cnn = env::var("NATS_CNN").unwrap_or_else(|_| DEF_NATS_CNN.to_string());
    let mgo_cnn = env::var("MGO_CNN").unwrap_or_else(|_| DEF_MGO_CNN.to_string());

    // tracing
    let subscriber =
        FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
    tracing::subscriber::set_global_default(subscriber)?;

    runtime::Builder::new()
        .capability(http::new(http_addr))
        .capability(messaging::new(nats_cnn.clone()))
        .capability(keyvalue::new(nats_cnn.clone()))
        .capability(signature::new())
        .capability(docdb::new(mgo_cnn))
        .capability(p2p::new())
        .capability(wrpc::new(nats_cnn))
        .run(args.wasm)?;

    shutdown().await
}

// Wait for shutdown signal
async fn shutdown() -> Result<(), Error> {
    tokio::select! {
        _ = tokio::signal::ctrl_c() => Ok(()),
    }
}
