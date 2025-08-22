//! # Trace
//!
//! This module implements the `wasmtime_wasi::StdoutStream` and
//! `wasmtime_wasi::StderrStream` traits to capture the output of the guest's
//! stdout and stderr streams.

use tokio::io::AsyncWrite;
use wasmtime_wasi::cli::{IsTerminal, StdoutStream};

// Capture wasm guest stdout.
pub struct Stdout;

impl StdoutStream for Stdout {
    fn async_stream(&self) -> Box<dyn AsyncWrite + Send + Sync> {
        Box::new(tokio::io::stdout())
    }
}

impl IsTerminal for Stdout {
    fn is_terminal(&self) -> bool {
        false
    }
}

// Implement error tracing for Guests by capturing stderr.
pub struct Errout;

impl StdoutStream for Errout {
    fn async_stream(&self) -> Box<dyn AsyncWrite + Send + Sync> {
        Box::new(tokio::io::stderr())
    }
}

impl IsTerminal for Errout {
    fn is_terminal(&self) -> bool {
        false
    }
}
