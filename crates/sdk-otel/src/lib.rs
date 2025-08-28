//! # OpenTelemetry SDK
//!
//! WASM component (guest) OpenTelemetry SDK.

// #[cfg(all(feature = "guest-export", feature = "host-export"))]
// compile_error!("features \"guest-export\" and \"host-export\" cannot both be enabled");

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
#[cfg(feature = "tracing")]
pub mod tracing;

cfg_if! {
    if #[cfg(feature = "metrics" )] {
        use opentelemetry_sdk::metrics::SdkMeterProvider;
    }
}

cfg_if! {
    if #[cfg(feature = "tracing" )] {
        use opentelemetry::global;
        use opentelemetry::trace::Tracer;
        use opentelemetry_sdk::trace::SdkTracerProvider;
    }
}

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(any(feature = "tracing", feature = "metrics"))] {
        use opentelemetry_sdk::Resource;
        pub use sdk_otel_attr::instrument;
    }
}

#[derive(Default)]
pub struct Shutdown {
    #[cfg(feature = "tracing")]
    tracing: SdkTracerProvider,
    #[cfg(feature = "metrics")]
    metrics: SdkMeterProvider,
}

impl Drop for Shutdown {
    fn drop(&mut self) {
        #[cfg(feature = "tracing")]
        if let Err(e) = self.tracing.shutdown() {
            ::tracing::error!("failed to flush tracing: {e}");
        }
        #[cfg(feature = "metrics")]
        if let Err(e) = self.metrics.shutdown() {
            ::tracing::error!("failed to flush metrics: {e}");
        }
    }
}

#[cfg(any(feature = "metrics", feature = "tracing"))]
#[must_use]
pub fn init(name: impl Into<String>) -> Shutdown {
    let resource = Resource::builder().with_service_name(name.into()).build();

    #[cfg(feature = "tracing")]
    let Ok(tracing) = tracing::init(resource.clone()) else {
        ::tracing::error!("failed to initialize tracing");
        return Shutdown::default();
    };
    #[cfg(feature = "metrics")]
    let Ok(metrics) = metrics::init(resource) else {
        ::tracing::error!("failed to initialize metrics");
        return Shutdown::default();
    };

    Shutdown {
        #[cfg(feature = "tracing")]
        tracing,
        #[cfg(feature = "metrics")]
        metrics,
    }
}

#[cfg(feature = "tracing")]
pub fn instrument<F, R>(name: impl Into<String>, f: F) -> R
where
    F: FnOnce() -> R,
{
    let span = ::tracing::Span::current();
    if span.is_none() {
        let name = name.into();
        let _shutdown = init(&name);
        let _ctx = tracing::context();
        let tracer = global::tracer("instrument");
        tracer.in_span(name, |_| f())
    } else {
        span.in_scope(f)
    }
}

#[cfg(all(not(feature = "tracing"), feature = "metrics"))]
pub fn instrument<F, R>(_name: impl Into<String>, f: F) -> R
where
    F: FnOnce() -> R,
{
    let _shutdown = init();
    f()
}
