//! # Wasmgrid CLI

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
            init(rt).await?.run().await
        }

        #[cfg(feature = "compile")]
        Command::Compile { wasm, output } => {
            runtime::compile(&wasm, output).map_err(|e| anyhow!(e))
        }
    }
}

// Start the runtime for the specified wasm file.
#[instrument(skip(rt))]
async fn init(rt: Runtime) -> Result<Runtime> {
    // create resources (in parallel)
    let (Ok(mongodb_client), Ok(secret_client), Ok(nats_client)) =
        tokio::join!(MongoDb::new(), AzKeyVault::new(), Nats::new())
    else {
        return Err(anyhow!("failed to create clients"));
    };

    // initialize services
    let mut rt = rt;

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

    Ok(rt)
}

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
