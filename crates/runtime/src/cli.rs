//! # Command Line Interface

use std::path::PathBuf;

pub use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
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
        /// The path to the wasm file to run. The file can either be a
        /// serialized (pre-compiled) wasmtime `Component` or a standard
        /// `wasm32-wasip2` wasm file.
        wasm: PathBuf,
    },
}
