//! # WASI OpenTelemetry
//!
//! This module provides bindings for the OpenTelemetry specification in the
//! context of WebAssembly System Interface (WASI) components.

pub mod otel {
    wit_bindgen::generate!({
        world: "otel",
        path: "../../wit",
        generate_all,
        // pub_export_macro: true
    });
}

mod convert;
mod process;
mod propagate;

pub use process::Processor;
pub use propagate::{TraceContextPropagator, WasiPropagator};
