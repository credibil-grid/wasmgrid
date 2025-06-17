//! # Trace
//!
//! This module implements the `wasmtime_wasi::StdoutStream` and
//! `wasmtime_wasi::StderrStream` traits to capture the output of the guest's
//! stdout and stderr streams. This is useful for debugging purposes, as it
//! allows us to see the output of the guest's code in the host's console.

use anyhow::anyhow;
use bytes::Bytes;
use wasmtime_wasi::p2::{OutputStream, Pollable, StdoutStream, StreamError, StreamResult};

// Capture wasm guest stdout.
pub struct Stdout;

impl StdoutStream for Stdout {
    fn stream(&self) -> Box<dyn OutputStream> {
        Box::new(OutStream {})
    }

    fn isatty(&self) -> bool {
        false
    }
}

struct OutStream;

#[async_trait::async_trait]
impl Pollable for OutStream {
    async fn ready(&mut self) {}
}

impl OutputStream for OutStream {
    fn write(&mut self, bytes: Bytes) -> StreamResult<()> {
        let out = String::from_utf8(bytes.to_vec())
            .map_err(|e| StreamError::LastOperationFailed(anyhow!(e)))?;
        println!("{out}");
        Ok(())
    }

    fn flush(&mut self) -> StreamResult<()> {
        Ok(())
    }

    fn check_write(&mut self) -> StreamResult<usize> {
        Ok(1024 * 1024)
    }
}

// Implement error tracing for Guests by capturing stderr.
pub struct Errout;

impl StdoutStream for Errout {
    fn stream(&self) -> Box<dyn OutputStream> {
        Box::new(ErroutStream {})
    }

    fn isatty(&self) -> bool {
        false
    }
}

struct ErroutStream;

#[async_trait::async_trait]
impl Pollable for ErroutStream {
    async fn ready(&mut self) {}
}

impl OutputStream for ErroutStream {
    fn write(&mut self, bytes: Bytes) -> StreamResult<()> {
        let out = String::from_utf8(bytes.to_vec())
            .map_err(|e| StreamError::LastOperationFailed(anyhow!(e)))?;
        println!("{out}");
        Ok(())
    }

    fn flush(&mut self) -> StreamResult<()> {
        Ok(())
    }

    fn check_write(&mut self) -> StreamResult<usize> {
        Ok(1024 * 1024)
    }
}
