//! # Wasmgrid CLI

use std::path::PathBuf;

use anyhow::{Result, anyhow};
use azkeyvault::AzKeyVault;
use dotenv::dotenv;
use mongodb::MongoDb;
use nats::Nats;
use runtime::{AddResource, Cli, Command, Parser, ResourceBuilder, Runtime};
use tracing::instrument;

/// Main entry point for the Wasmgrid CLI.
///
/// # Errors
///
/// Returns an error if resources cannot be initialized.
///
/// # Panics
///
/// Panics if the runtime cannot be initialized.
#[tokio::main]
pub async fn main() -> Result<()> {
    if cfg!(debug_assertions) {
        dotenv().ok();
    }
    match Cli::parse().command {
        Command::Run { wasm } => init_runtime(wasm).await,
        #[cfg(feature = "compile")]
        Command::Compile { wasm, output } => runtime::compile(&wasm, output),
    }
}

// Start the runtime for the specified wasm file.
#[instrument(skip(wasm))]
async fn init_runtime(wasm: PathBuf) -> Result<()> {
    // create resources (in parallel)
    let (Ok(mongodb), Ok(az_secret), Ok(nats)) =
        tokio::join!(MongoDb::new(), AzKeyVault::new(), Nats::new())
    else {
        return Err(anyhow!("failed to create clients"));
    };

    Runtime::new(wasm)
        .register(wasi_otel::Otel)
        .register(wasi_http::Http)
        .register(wasi_blobstore_mdb::Blobstore.resource(mongodb)?)
        .register(wasi_keyvalue_nats::KeyValue.resource(nats.clone())?)
        .register(wasi_vault_az::Vault.resource(az_secret)?)
        .register(wasi_messaging_nats::Messaging.resource(nats)?)
        .await
}
