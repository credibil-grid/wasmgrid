//! # Wasmgrid CLI

use std::env;

use anyhow::Result;
use dotenv::dotenv;
use runtime::{Cli, Parser};
use services::{
    Resources, blobstore_mdb as blobstore, http, keyvalue_nats as keyvalue,
    messaging_nats as messaging, vault_az as vault,
};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

const DEF_NATS_ADDR: &str = "demo.nats.io";
const DEF_KV_ADDR: &str = "https://kv-credibil-demo.vault.azure.net";

#[tokio::main]
pub async fn main() -> Result<()> {
    if cfg!(debug_assertions) {
        dotenv().ok();
    }
    let subscriber =
        FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
    tracing::subscriber::set_global_default(subscriber)?;

    match Cli::parse().command {
        runtime::Command::Run { wasm } => {
            tracing::info!("initialising runtime");
            let mut rt = runtime::Runtime::from_file(&wasm)?;

            // link services
            rt.link(&http::Service)?;
            rt.link(&blobstore::Service)?;
            rt.link(&keyvalue::Service)?;
            rt.link(&messaging::Service)?;
            rt.link(&vault::Service)?;

            // load external resources
            let nats_addr = env::var("NATS_ADDR").unwrap_or_else(|_| DEF_NATS_ADDR.into());
            let jwt = env::var("NATS_JWT").ok();
            let seed = env::var("NATS_SEED").ok();
            let kv_addr = env::var("KV_ADDR").unwrap_or_else(|_| DEF_KV_ADDR.into());
            let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
            
            let resources = Resources::new();
            resources.with_nats(nats_addr, jwt, seed);
            resources.with_mongo(mongo_uri);
            resources.with_azkeyvault(kv_addr.clone());

            // start `Runnable` servers
            rt.run(http::Service, resources.clone())?;
            rt.run(messaging::Service, resources.clone())?;

            rt.shutdown().await
        }

        #[cfg(feature = "compile")]
        runtime::Command::Compile { wasm, output } => runtime::compile(&wasm, output),
    }
}
