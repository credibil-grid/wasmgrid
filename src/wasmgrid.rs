//! # Wasmgrid CLI

use std::path::PathBuf;

use anyhow::{Result, anyhow};
use azkeyvault::AzKeyVault;
use dotenv::dotenv;
use mongodb::MongoDb;
use nats::Nats;
use runtime::{AddResource, Cli, Command, Parser, ResourceBuilder, Run, Runtime, ServiceBuilder};
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
    let (Ok(mongodb_client), Ok(secret_client), Ok(nats_client)) =
        tokio::join!(MongoDb::new(), AzKeyVault::new(), Nats::new())
    else {
        return Err(anyhow!("failed to create clients"));
    };

    // create runtime for wasm component
    let mut rt = Runtime::from_file(&wasm)?;

    // initialize services
    wasi_otel::Service::new().add_to_linker(&mut rt.linker)?;
    wasi_blobstore_mdb::Service::new().resource(mongodb_client)?.add_to_linker(&mut rt.linker)?;
    wasi_keyvalue_nats::Service::new()
        .resource(nats_client.clone())?
        .add_to_linker(&mut rt.linker)?;
    wasi_vault_az::Service::new().resource(secret_client)?.add_to_linker(&mut rt.linker)?;
    wasi_http::Service::new().add_to_linker(&mut rt.linker)?.register(&mut rt);
    wasi_messaging_nats::Service::new()
        .resource(nats_client)?
        .add_to_linker(&mut rt.linker)?
        .register(&mut rt);

    // run and wait for shutdown
    rt.serve().await
}

// Candidate macro or configuration
// runtime_macros::runtime!({
//     resources: {
//         mongo: MongoDb,
//     },
//     services: [
//         "wasi_http::Service": {
//             run: true
//         },
//         "wasi_blobstore_mdb::Service": {
//              resources: [mongo]
//         }
//     ]
// });
