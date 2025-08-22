//! # OpenTelemetry SDK
//!
//! WASM component (guest) OpenTelemetry SDK.

#[cfg(all(feature = "guest-mode", feature = "host-mode"))]
compile_error!("features \"guest-mode\" and \"host-mode\" cannot both be enabled");

pub mod generated {
    wit_bindgen::generate!({
        world: "otel",
        path: "../../wit",
        generate_all,
    });
}

mod export;
#[cfg(feature = "metrics")]
pub mod metrics;
pub mod tracing;

use opentelemetry::ContextGuard;
use opentelemetry_sdk::Resource;

pub struct ScopeGuard {
    _tracing: ContextGuard,
    #[cfg(feature = "metrics")]
    _metrics: metrics::Reader,
}

// TODO: add .in_span(|| Fn(ctx)) as alternative to guard
// TODO: add xxx_span! macros
pub fn init() -> ScopeGuard {
    let resource = Resource::builder().with_service_name("otel").build();

    ScopeGuard {
        _tracing: tracing::init(resource.clone()).expect("should initialize"),
        #[cfg(feature = "metrics")]
        _metrics: metrics::init(resource).expect("should initialize"),
    }
}

pub fn instrument<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let _guard = init();
    f()
}
