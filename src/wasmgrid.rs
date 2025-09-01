//! # Wasmgrid CLI

use anyhow::{Context, Result, anyhow};
use azkeyvault::AzKeyVault;
use dotenv::dotenv;
use mongodb::MongoDb;
use nats::Nats;
use runtime::{AddResource, AddToLinker, Cli, Parser, ResourceBuilder, Runtime};
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

    let wasm = Cli::parse().wasm;
    let rt = Runtime::from_file(&wasm)?;

    start(rt).await?.shutdown().await
}

// Start the runtime for the specified wasm file.
#[instrument]
async fn start(rt: Runtime) -> Result<Runtime> {
    tracing::info!("starting runtime");

    // create resources (in parallel)
    let (Ok(secret_client), Ok(mongodb_client), Ok(nats_client)) =
        tokio::join!(AzKeyVault::new(), MongoDb::new(), Nats::new())
    else {
        return Err(anyhow!("failed to create clients"));
    };

    // add resources to services
    let mut rt = rt;

    let http = http::Service::default();
    http.add_to_linker(&mut rt.linker).context("linking http")?;

    let otel = otel::Service::default();
    otel.add_to_linker(&mut rt.linker).context("linking otel")?;

    // blobstore::Service::new()
    //    .add_resource(mongodb_client)
    //    .add_to_linker(&mut rt.linker)?;

    let mut blobstore = blobstore::Service::default();
    blobstore.add_resource(mongodb_client).context("adding mongodb")?;
    blobstore.add_to_linker(&mut rt.linker).context("linking blobstore")?;

    let mut keyvalue = keyvalue::Service::default();
    keyvalue.add_resource(nats_client.clone()).context("adding nats")?;
    keyvalue.add_to_linker(&mut rt.linker).context("linking keyvalue")?;

    let mut messaging = messaging::Service::default();
    messaging.add_resource(nats_client).context("adding nats")?;
    messaging.add_to_linker(&mut rt.linker).context("linking messaging")?;

    let mut vault = vault::Service::default();
    vault.add_resource(secret_client).context("adding azkeyvault")?;
    vault.add_to_linker(&mut rt.linker).context("linking vault")?;

    // run servers
    rt.run(http)?;
    rt.run(messaging)?;

    Ok(rt)
}

// mod generate {
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
// }
