//! # Wasmgrid CLI

use std::env;

use dotenv::dotenv;
use runtime::{Cli, Parser};
// use services::{Resources, http, keyvalue, messaging, rpc};
use services::{Resources, http, jsondb, keyvalue};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

const DEF_MGO_CNN: &str = "mongodb://localhost:27017";
const DEF_NATS_ADDR: &str = "demo.nats.io";

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
            runtime::compile(&wasm, output)?;
            return Ok(());
        }
        runtime::Command::Run { wasm, compile } => {
            tracing::info!("initialising runtime");

            let compiled = if compile { runtime::compile(&wasm, None)? } else { wasm };
            let mut rt = runtime::Runtime::new(compiled)?;

            // link services
            rt.link(&http::Service)?;
            // rt.link(&rpc::Service)?;
            rt.link(&keyvalue::Service)?;
            // rt.link(&messaging::Service)?;
            rt.link(&jsondb::Service)?;

            // load external resources
            let addr = env::var("NATS_ADDR").unwrap_or_else(|_| DEF_NATS_ADDR.into());
            let jwt = env::var("NATS_JWT").ok();
            let seed = env::var("NATS_SEED").ok();
            let mgo_cnn = env::var("MGO_CNN").unwrap_or_else(|_| DEF_MGO_CNN.into());

            let resources = Resources::new();
            resources.with_nats(addr, jwt, seed);
            resources.with_mongo(mgo_cnn);

            // start `Runnable` services (servers)
            rt.run(http::Service, resources.clone())?;
            // rt.run(rpc::Service, resources.clone())?;
            // rt.run(messaging::Service, resources)?;

            rt.shutdown().await
        }
    }
}
