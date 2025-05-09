//! # Wasmgrid CLI

use dotenv::dotenv;
// #[cfg(feature = "messaging")]
// use wasmgrid::messaging;
// #[cfg(feature = "rpc")]
// use wasmgrid::rpc;
// #[cfg(feature = "vault")]
// use crate::vault;
use runtime::{Cli, Command, Parser};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
#[cfg(feature = "http")]
use wasmgrid::http;
// #[cfg(feature = "jsondb")]
// use wasmgrid::jsondb;
#[cfg(feature = "keyvalue")]
use wasmgrid::keyvalue;

#[tokio::main]
pub async fn main() -> wasmtime::Result<()> {
    if cfg!(debug_assertions) {
        dotenv().ok();
    }
    let subscriber =
        FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
    tracing::subscriber::set_global_default(subscriber)?;
    tracing::trace!("initializing");

    match Cli::parse().command {
        Command::Compile { wasm, output } => {
            runtime::compile(&wasm, output)?;
            return Ok(());
        }
        Command::Run { wasm, compile } => {
            let mut rt = runtime::Runtime::new(wasm, compile)?;

            if cfg!(feature = "http") {
                let http = http::new();
                rt.link(&http)?.start(http)?;
            }
            if cfg!(feature = "keyvalue") {
                let keyvalue = keyvalue::new();
                rt.link(&keyvalue)?.start(keyvalue)?;
            }
            // if cfg!(feature = "jsondb") {
            //     let jsondb = jsondb::new();
            //     rt.link(&jsondb)?.start(jsondb)?;
            // }
            // if cfg!(feature = "messaging") {
            //     let messaging = messaging::new();
            //     rt.link(&messaging)?.start(messaging)?;
            // }
            // if cfg!(feature = "rpc") {
            //     let rpc = rpc::new();
            //     rt.link(&rpc)?.start(rpc)?;
            // }

            // wait for shutdown signal
            rt.shutdown().await
        }
    }
}
