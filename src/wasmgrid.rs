//! # Wasmgrid CLI

use std::env;

use dotenv::dotenv;
use runtime::{Cli, Parser};
use services::{Resources, http, jsondb, keyvalue, rpc, vault};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

const DEF_MGO_URI: &str = "mongodb://localhost:27017";
const DEF_NATS_ADDR: &str = "demo.nats.io";
const DEF_KV_ADDR: &str = "https://kv-credibil-demo.vault.azure.net";

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    if cfg!(debug_assertions) {
        dotenv().ok();
    }
    let subscriber =
        FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
    tracing::subscriber::set_global_default(subscriber)?;

    match Cli::parse().command {
        runtime::Command::Compile { wasm, output } => {
            runtime::compile(wasm, output)?;
            return Ok(());
        }
        runtime::Command::Run { wasm, compile } => {
            tracing::info!("initialising runtime");

            let mut rt = if compile {
                runtime::Runtime::from_wasm(wasm)?
            } else {
                runtime::Runtime::from_compiled(wasm)?
            };

            // link services
            rt.link(&http::Service)?;
            rt.link(&rpc::Service)?;
            rt.link(&keyvalue::Service)?;
            rt.link(&jsondb::Service)?;
            rt.link(&vault::Service)?;

            // load external resources
            let nats_addr = env::var("NATS_ADDR").unwrap_or_else(|_| DEF_NATS_ADDR.into());
            let jwt = env::var("NATS_JWT").ok();
            let seed = env::var("NATS_SEED").ok();
            let mgo_uri = env::var("MGO_URI").unwrap_or_else(|_| DEF_MGO_URI.into());
            let kv_addr = env::var("KV_ADDR").unwrap_or_else(|_| DEF_KV_ADDR.into());

            let resources = Resources::new();
            resources.with_nats(nats_addr, jwt, seed);
            resources.with_mongo(mgo_uri);
            resources.with_azkeyvault(kv_addr);

            // start `Runnable` servers
            rt.run(http::Service, resources.clone())?;
            rt.run(rpc::Service, resources.clone())?;

            rt.shutdown().await
        }
    }
}
