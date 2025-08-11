//! # Wasmgrid CLI

use std::env;
use std::path::PathBuf;

use anyhow::{Context, Result, anyhow};
use credibil_telemetry::Otel;
use dotenv::dotenv;
use runtime::{Cli, Command, Parser, Runtime};
use services::{Ctx, Resources};
use tracing::instrument;
use wasi_blobstore::mongodb as blobstore;
use wasi_keyvalue::nats as keyvalue;
use wasi_messaging::nats as messaging;
use wasi_vault::az_keyvault as vault;
use {wasi_http as http, wasi_otel as otel};

const DEF_NATS_ADDR: &str = "demo.nats.io";
const DEF_KV_ADDR: &str = "https://kv-credibil-demo.vault.azure.net";

#[tokio::main]
pub async fn main() -> Result<()> {
    if cfg!(debug_assertions) {
        dotenv().ok();
    }

    match Cli::parse().command {
        Command::Run { wasm } => {
            // telemetry
            let Some(file) = wasm.file_name() else {
                return Err(anyhow!("file name not specified"));
            };
            let name: &str = file.to_str().unwrap_or_default();
            let Some((prefix, _)) = name.split_once('.') else {
                return Err(anyhow!("file name does not have an extension"));
            };
            Otel::new(prefix).init().context("initializing telemetry")?;

            // run until shutdown
            start(&wasm)?.shutdown().await
        }

        #[cfg(feature = "compile")]
        runtime::Command::Compile { wasm, output } => runtime::compile(&wasm, output),
    }
}

// Start the runtime for the specified wasm file.
#[instrument]
fn start(wasm: &PathBuf) -> Result<Runtime<Ctx>> {
    tracing::info!("starting runtime");

    let mut rt = Runtime::from_file(wasm)?;

    // services
    rt.link(&otel::Service).context("linking otel")?;
    rt.link(&http::Service).context("linking http")?;
    rt.link(&blobstore::Service).context("linking blobstore")?;
    rt.link(&keyvalue::Service).context("linking keyvalue")?;
    rt.link(&messaging::Service).context("linking messaging")?;
    rt.link(&vault::Service).context("linking vault")?;

    // external resources
    let nats_addr = env::var("NATS_ADDR").unwrap_or_else(|_| DEF_NATS_ADDR.into());
    let jwt = env::var("NATS_JWT").ok();
    let seed = env::var("NATS_SEED").ok();
    let kv_addr = env::var("KV_ADDR").unwrap_or_else(|_| DEF_KV_ADDR.into());
    let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    // TODO: add az keyvault env vars

    let resources = Resources::new();
    resources.with_nats(nats_addr, jwt, seed);
    resources.with_mongo(mongo_uri);
    resources.with_azkeyvault(kv_addr);

    // servers
    rt.run(http::Service, resources.clone())?;
    rt.run(messaging::Service, resources)?;

    Ok(rt)
}
