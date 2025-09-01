//! # Wasmgrid CLI

use std::env;
use std::path::PathBuf;

use anyhow::{Context, Result, anyhow};
use credibil_otel::Telemetry;
use dotenv::dotenv;
use runtime::{Cli, Parser, Runtime};
use tracing::instrument;
use {
    wasi_blobstore_mdb as blobstore, wasi_http as http, wasi_keyvalue_nats as keyvalue,
    wasi_messaging_nats as messaging, wasi_otel as otel, wasi_vault_az as vault,
};

const DEF_NATS_ADDR: &str = "demo.nats.io";
const DEF_KV_ADDR: &str = "https://kv-credibil-demo.vault.azure.net";

mod generate {
    // runtime_macros::runtime!({
    // resources: {
    //     "nats": nats,
    //     "mongo": mongodb,
    //     "azkeyvault": az_keyvault,
    // },
    // services: {
    //     "wasi:messaging": messaging::Service,
    //     "wasi:http": http::Service,
    //     "wasi:otel": otel::Service,
    //     "wasi:blobstore": blobstore::Service,
    //     "wasi:keyvalue": wasi_keyvalue::nats::Service,
    //     "wasi:vault": vault::Service,
    // }
    // });
}

/// Main entry point for the Wasmgrid CLI.
///
/// # Errors
///
/// Returns an error if resources cannot be initialized.
///
/// # Panics
///
/// This function will panic if the environment variables are not set.
#[tokio::main]
pub async fn main() -> Result<()> {
    if cfg!(debug_assertions) {
        dotenv().ok();
    }

    let wasm = Cli::parse().wasm;

    // telemetry
    let Some(file) = wasm.file_name() else {
        return Err(anyhow!("file name not specified"));
    };
    let name: &str = file.to_str().unwrap_or_default();
    let Some((prefix, _)) = name.split_once('.') else {
        return Err(anyhow!("file name does not have an extension"));
    };
    let mut builder = Telemetry::new(prefix);
    if let Ok(endpoint) = env::var("OTEL_GRPC_ADDR") {
        builder = builder.endpoint(endpoint);
    }
    builder.build().context("initializing telemetry")?;

    // run until shutdown
    start(&wasm)?.shutdown().await
}

// Start the runtime for the specified wasm file.
#[instrument]
fn start(wasm: &PathBuf) -> Result<Runtime> {
    tracing::info!("starting runtime");

    // mongodb
    let mongo_uri = env::var("MONGODB_URI").context("fetching MONGODB_URI env var")?;
    // let mongodb_client= mongodb::Client;

    // nats
    let nats_addr = env::var("NATS_ADDR").unwrap_or_else(|_| DEF_NATS_ADDR.into());
    let jwt = env::var("NATS_JWT").ok();
    let seed = env::var("NATS_SEED").ok();
    let kv_addr = env::var("KV_ADDR").unwrap_or_else(|_| DEF_KV_ADDR.into());
    // let nats_client= async_nats::Client;

    let http = http::Service::default();
    let otel = otel::Service::default();
    let blobstore = blobstore::Service::default();
    // blobstore.add_resource(mongodb_client).context("adding resource")?;
    let keyvalue = keyvalue::Service::default();
    // keyvalue.add_resource(nats_client).context("adding resource")?;
    let messaging = messaging::Service::default();
    // messaging.add_resource(nats_client).context("adding resource")?;
    let vault = vault::Service::default();
    // vault.add_resource(secret_client).context("adding resource")?;

    let mut rt = Runtime::from_file(wasm)?;
    rt.add_to_linker(&otel).context("linking otel")?;
    rt.add_to_linker(&blobstore).context("linking blobstore")?;
    rt.add_to_linker(&keyvalue).context("linking keyvalue")?;
    rt.add_to_linker(&http).context("linking http")?;
    rt.add_to_linker(&messaging).context("linking messaging")?;
    rt.add_to_linker(&vault).context("linking vault")?;

    // servers
    rt.run(http)?;
    rt.run(messaging)?;

    Ok(rt)
}
