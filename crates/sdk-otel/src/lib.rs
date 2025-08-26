//! # OpenTelemetry SDK
//!
//! WASM component (guest) OpenTelemetry SDK.

#[cfg(all(feature = "guest-mode", feature = "host-mode"))]
compile_error!("features \"guest-mode\" and \"host-mode\" cannot both be enabled");

pub mod generated {
    #![allow(clippy::collection_is_never_read)]

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

use opentelemetry::global;
use opentelemetry::trace::Tracer;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_sdk::trace::SdkTracerProvider;
pub use sdk_otel_attr::instrument;

pub struct Shutdown {
    tracing: SdkTracerProvider,
    #[cfg(feature = "metrics")]
    metrics: SdkMeterProvider,
}

impl Drop for Shutdown {
    fn drop(&mut self) {
        if let Err(e) = self.tracing.shutdown() {
            ::tracing::error!("failed to flush tracing: {e}");
        }
        #[cfg(feature = "metrics")]
        if let Err(e) = self.metrics.shutdown() {
            ::tracing::error!("failed to flush metrics: {e}");
        }
    }
}

#[must_use]
pub fn init() -> Shutdown {
    let resource = Resource::builder().with_service_name("otel").build();
    let tracing = tracing::init(resource.clone()).expect("should initialize");
    Shutdown {
        tracing,
        #[cfg(feature = "metrics")]
        metrics: metrics::init(resource).expect("should initialize"),
    }
}

pub fn instrument<F, R>(name: impl Into<String>, f: F) -> R
where
    F: FnOnce() -> R,
{
    let span = ::tracing::Span::current();
    if span.is_none() {
        let _shutdown = init();
        let _ctx = tracing::context();
        let tracer = global::tracer("instrument");
        tracer.in_span(name.into(), |_| f())
    } else {
        span.in_scope(f)
    }
}
