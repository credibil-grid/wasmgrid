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
        /// The path to the wasm file to run.
        wasm: PathBuf,

        /// The wasm file requires compiling (leave unset if the file is
        /// pre-compiled).
        #[arg(short, long, default_value_t = false)]
        compile: bool,
    },
}
