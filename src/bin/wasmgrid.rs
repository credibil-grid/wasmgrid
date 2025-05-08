//! # Wasmgrid CLI

use std::path::PathBuf;

use anyhow::Error;
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasmgrid::Runtime;

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
            Runtime::new().start(wasm,compile)?;
            shutdown().await
        }
    }
}

// Wait for shutdown signal
async fn shutdown() -> Result<(), Error> {
    tokio::select! {
        _ = tokio::signal::ctrl_c() => Ok(()),
    }
}
