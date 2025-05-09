//! # Wasmgrid CLI

use std::path::PathBuf;

use anyhow::Error;
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasmgrid::Runtime;
#[cfg(feature = "http")]
use wasmgrid::service::http;
// #[cfg(feature = "jsondb")]
// use wasmgrid::service::jsondb;
#[cfg(feature = "keyvalue")]
use wasmgrid::service::keyvalue;
// #[cfg(feature = "messaging")]
// use wasmgrid::service::messaging;
// #[cfg(feature = "rpc")]
// use wasmgrid::service::rpc;
// #[cfg(feature = "vault")]
// use crate::service::vault;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Compile the specified wasm32-wasip2 component.
    Compile {
        /// The path to the wasm file to compile.
        wasm: PathBuf,

        /// An optional output directory. If not set, the compiled component
        /// will be written to the same location as the input file.
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Run the specified wasm guest.
    Run {
        /// The path to the wasm file to run.
        wasm: PathBuf,

        /// The wasm file requires compiling (leave unset if the file is
        /// pre-compiled).
        #[arg(short, long, default_value_t = false)]
        compile: bool,
    },
}

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
            wasmgrid::compile(&wasm, output)?;
            return Ok(());
        }
        Command::Run { wasm, compile } => {
            let mut rt = Runtime::new(wasm, compile)?;

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

            shutdown().await
        }
    }
}

// Wait for shutdown signal.
async fn shutdown() -> Result<(), Error> {
    tokio::select! {
        _ = tokio::signal::ctrl_c() => Ok(()),
    }
}
