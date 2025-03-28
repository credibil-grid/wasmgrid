#![allow(clippy::redundant_pub_crate)]
#![feature(let_chains)]
#![feature(duration_constructors)]

mod capabilities;
mod runtime;

use std::env;

use anyhow::Error;
use clap::Parser;
use dotenv::dotenv;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[cfg(feature = "http")]
use crate::capabilities::http;
#[cfg(feature = "jsondb")]
use crate::capabilities::jsondb;
#[cfg(feature = "keyvalue")]
use crate::capabilities::keyvalue;
#[cfg(feature = "messaging")]
use crate::capabilities::messaging;
#[cfg(feature = "p2p")]
use crate::capabilities::p2p;
#[cfg(feature = "rpc")]
use crate::capabilities::rpc;
#[cfg(feature = "vault")]
use crate::capabilities::vault;

const DEF_HTTP_ADDR: &str = "0.0.0.0:8080";
const DEF_MGO_CNN: &str = "mongodb://localhost:27017";
const DEF_NATS_ADDR: &str = "demo.nats.io";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to the wasm file to serve.
    // #[clap(short, long)]
    wasm: String,
}

#[derive(Debug, Clone)]
struct NatsCreds {
    jwt: String,
    seed: String,
}

#[tokio::main]
pub async fn main() -> wasmtime::Result<()> {
    let args = Args::parse();

    // env vars
    if cfg!(debug_assertions) {
        dotenv().ok();
    }

    let http_addr = env::var("HTTP_ADDR").unwrap_or_else(|_| DEF_HTTP_ADDR.into());
    let mgo_cnn = env::var("MGO_CNN").unwrap_or_else(|_| DEF_MGO_CNN.into());
    let nats_cnn = env::var("NATS_ADDR").unwrap_or_else(|_| DEF_NATS_ADDR.into());
    let nats_creds = if let Ok(jwt) = env::var("NATS_JWT")
        && let Ok(seed) = env::var("NATS_SEED")
    {
        Some(NatsCreds { jwt, seed })
    } else {
        None
    };

    let subscriber =
        FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();

    tracing::subscriber::set_global_default(subscriber)?;

    // init capabilities
    let builder = runtime::Builder::new();
    #[cfg(feature = "http")]
    let builder = builder.capability(http::new(http_addr));
    #[cfg(feature = "jsondb")]
    let builder = builder.capability(jsondb::new(mgo_cnn));
    #[cfg(feature = "keyvalue")]
    let builder = builder.capability(keyvalue::new(nats_cnn.clone(), nats_creds.clone()));
    #[cfg(feature = "messaging")]
    let builder = builder.capability(messaging::new(nats_cnn.clone()));
    #[cfg(feature = "p2p")]
    let builder = builder.capability(p2p::new());
    #[cfg(feature = "rpc")]
    let builder = builder.capability(rpc::new(nats_cnn, nats_creds));
    #[cfg(feature = "vault")]
    let builder = builder.capability(vault::new());

    builder.run(args.wasm)?;

    shutdown().await
}

// Wait for shutdown signal
async fn shutdown() -> Result<(), Error> {
    tokio::select! {
        _ = tokio::signal::ctrl_c() => Ok(()),
    }
}
