//! # OpenTelemetry SDK
//!
//! Wasm component (guest) OpenTelemetry SDK.

mod generated {
    #![allow(clippy::collection_is_never_read)]

    wit_bindgen::generate!({
        world: "otel",
        path: "../../wit",
        generate_all,
    });
}

mod export;
mod init;

pub use sdk_otel_attr::instrument;

use crate::init::Shutdown;

/// Initialize OpenTelemetry SDK and tracing subscriber.
pub fn init() -> Shutdown {
    let Ok(shutdown) = init::init() else {
        ::tracing::error!("failed to initialize");
        return Shutdown::default();
    };
    shutdown
}
