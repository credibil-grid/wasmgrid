use std::path::PathBuf;

use anyhow::{Result, anyhow};
use azkeyvault::AzKeyVault;
use dotenv::dotenv;
use mongodb::MongoDb;
use nats::Nats;
use runtime::{AddResource, Cli, Command, Parser, ResourceBuilder, Runtime};
use wasi_blobstore_mdb::Blobstore;
use wasi_http::Http;
use wasi_keyvalue_nats::KeyValue;
use wasi_messaging_nats::Messaging;
use wasi_otel::Otel;
use wasi_vault_az::Vault;

#[tokio::main]
async fn main() -> Result<()> {
    if cfg!(debug_assertions) {
        dotenv().ok();
    }
    match Cli::parse().command {
        Command::Run { wasm } => init_runtime(wasm).await,
        #[cfg(feature = "compile")]
        Command::Compile { wasm, output } => runtime::compile(&wasm, output),
    }
}

async fn init_runtime(wasm: PathBuf) -> Result<()> {
    // create resources (in parallel)
    let (Ok(mongodb), Ok(az_secret), Ok(nats)) =
        tokio::join!(MongoDb::new(), AzKeyVault::new(), Nats::new())
    else {
        return Err(anyhow!("failed to create resources"));
    };

    Runtime::new(wasm)
        .register(Otel)
        .register(Http)
        .register(Blobstore.resource(mongodb)?)
        .register(KeyValue.resource(nats.clone())?)
        .register(Vault.resource(az_secret)?)
        .register(Messaging.resource(nats)?)
        .await
}
