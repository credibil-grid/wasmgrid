//! # Trace
//!
//! This module implements the `wasmtime_wasi::StdoutStream` and
//! `wasmtime_wasi::StderrStream` traits to capture the output of the guest's
//! stdout and stderr streams. This is useful for debugging purposes, as it
//! allows us to see the output of the guest's code in the host's console.

use anyhow::anyhow;
use bytes::Bytes;
use wasmtime_wasi::{OutputStream, Pollable, StreamError, StreamResult};

// Implement debug tracing for Guests by capturing stdout.
pub struct Stdout;

impl wasmtime_wasi::StdoutStream for Stdout {
    fn stream(&self) -> Box<dyn OutputStream> {
        Box::new(StdoutStream {})
    }

    fn isatty(&self) -> bool {
        false
    }
}

struct StdoutStream;

#[async_trait::async_trait]
impl Pollable for StdoutStream {
    async fn ready(&mut self) {}
}

impl OutputStream for StdoutStream {
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

impl wasmtime_wasi::StdoutStream for Errout {
    fn stream(&self) -> Box<dyn OutputStream> {
        Box::new(ErroutStream {})
    }

    fn isatty(&self) -> bool {
        false
    }
}

struct ErroutStream;

#[async_trait::async_trait]
impl wasmtime_wasi::Pollable for ErroutStream {
    async fn ready(&mut self) {}
}

impl wasmtime_wasi::OutputStream for ErroutStream {
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
