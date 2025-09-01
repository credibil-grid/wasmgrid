//! # Wasmgrid CLI

use anyhow::{Result, anyhow};
use azkeyvault::AzKeyVault;
use dotenv::dotenv;
use mongodb::MongoDb;
use nats::Nats;
use runtime::{AddResource, Cli, Command, Parser, ResourceBuilder, Runtime, ServiceBuilder};
use tracing::instrument;
use {
    wasi_blobstore_mdb as blobstore, wasi_http as http, wasi_keyvalue_nats as keyvalue,
    wasi_messaging_nats as messaging, wasi_otel as otel, wasi_vault_az as vault,
};

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

    let cli = Cli::parse();
    match cli.command {
        Command::Run { wasm } => {
            let rt = Runtime::from_file(&wasm)?;
            start(rt).await?.shutdown().await
        }

        #[cfg(feature = "compile")]
        Command::Compile { wasm, output } => {
            runtime::compile(&wasm, output).map_err(|e| anyhow!(e))
        }
    }
}

// Start the runtime for the specified wasm file.
#[instrument]
async fn start(rt: Runtime) -> Result<Runtime> {
    // create resources (in parallel)
    let (Ok(mongodb_client), Ok(secret_client), Ok(nats_client)) =
        tokio::join!(MongoDb::new(), AzKeyVault::new(), Nats::new())
    else {
        return Err(anyhow!("failed to create clients"));
    };

    // add resources to services
    let mut rt = rt;
    otel::Service::new().add_to_linker(&mut rt.linker)?;
    blobstore::Service::new().resource(mongodb_client)?.add_to_linker(&mut rt.linker)?;
    keyvalue::Service::new().resource(nats_client.clone())?.add_to_linker(&mut rt.linker)?;
    vault::Service::new().resource(secret_client)?.add_to_linker(&mut rt.linker)?;
    let http = http::Service::new().add_to_linker(&mut rt.linker)?;
    let messaging =
        messaging::Service::new().resource(nats_client)?.add_to_linker(&mut rt.linker)?;

    rt.run(http)?;
    rt.run(messaging)?;

    Ok(rt)
}

// runtime_macros::runtime!({
//     resources: {
//         nats: Nats,
//         mongo: MongoDb,
//         azkeyvault: AzKeyVault,
//     },
//     services: [
//         "wasi_http::Service": {
//             run: true
//         },
//         "wasi_otel::Service",
//         "wasi_blobstore_mdb::Service": {
//              resources: [mongo]
//         },
//         "wasi_keyvalue_nats::Service": {
//              resources: [nats]
//         },
//         "wasi_messaging_nats::Service": {
//              resources: [nats],
//              run: true
//         },
//         "wasi_vault::Service": {
//              resources: [azkeyvault]
//         }
//     ]
// });
