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

use opentelemetry::trace::Tracer;
use opentelemetry::{ContextGuard, global};
use opentelemetry_sdk::Resource;
pub use sdk_otel_attr::instrument;

pub struct ScopeGuard {
    _tracing: ContextGuard,
    #[cfg(feature = "metrics")]
    _metrics: metrics::Reader,
}

pub fn init() -> ScopeGuard {
    let resource = Resource::builder().with_service_name("otel").build();
    ScopeGuard {
        _tracing: tracing::init(resource.clone()).expect("should initialize"),
        #[cfg(feature = "metrics")]
        _metrics: metrics::init(resource).expect("should initialize"),
    }
}

pub fn instrument<F, R>(name: impl Into<String>, f: F) -> R
where
    F: FnOnce() -> R,
{
    let _guard = init();
    let tracer = global::tracer("instrument");
    tracer.in_span(name.into(), |_| f())
}
