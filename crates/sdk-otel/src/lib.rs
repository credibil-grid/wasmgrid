//! # WASI OpenTelemetry
//!
//! This module provides bindings for the OpenTelemetry specification in the
//! context of WebAssembly System Interface (WASI) components.

pub mod tracing;

// pub use self::tracing::*;

pub mod generated {
    wit_bindgen::generate!({
        world: "otel",
        path: "../../wit",
        generate_all,
    });
}
