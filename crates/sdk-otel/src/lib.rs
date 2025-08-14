//! # WASI OpenTelemetry
//!
//! This module provides bindings for the OpenTelemetry specification in the
//! context of WebAssembly System Interface (WASI) components.

mod init;
mod propagate;

pub use self::init::*;
pub use self::propagate::*;
