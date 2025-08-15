//! # OpenTelemetry SDK
//!
//! WASM component (guest) OpenTelemetry SDK.

pub mod tracing;

pub mod generated {
    wit_bindgen::generate!({
        world: "otel",
        path: "../../wit",
        generate_all,
    });
}
