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

use cfg_if::cfg_if;

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
cfg_if! {
    if #[cfg(any(feature = "tracing", feature = "metrics"))] {
        use opentelemetry_sdk::Resource;
        use opentelemetry::{KeyValue, Value};
        use self::generated::wasi::otel::resource;
        use self::generated::wasi::otel::types;
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
pub fn init() -> Shutdown {
    let resource: Resource = resource::resource().into();

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
        let _shutdown = init();
        let _ctx = tracing::context();
        let tracer = global::tracer("instrument");
        tracer.in_span(name.into(), |_| f())
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

cfg_if! {
    if #[cfg(any(feature = "tracing", feature = "metrics"))]{
        impl From<types::Resource> for Resource {
            fn from(value: types::Resource) -> Self {
                let attrs = value.attributes.into_iter().map(Into::into).collect::<Vec<_>>();
                let builder = Self::builder();

                if let Some(schema_url) = value.schema_url {
                    builder.with_schema_url(attrs, schema_url).build()
                } else {
                    builder.with_attributes(attrs).build()
                }
            }
        }

        impl From<types::KeyValue> for KeyValue {
            fn from(value: types::KeyValue) -> Self {
                Self::new(value.key, value.value)
            }
        }

        impl From<types::Value> for Value {
            fn from(value: types::Value) -> Self {
                match value {
                    types::Value::Bool(v) => Self::Bool(v),
                    types::Value::S64(v) => Self::I64(v),
                    types::Value::F64(v) => Self::F64(v),
                    types::Value::String(v) => Self::String(v.into()),
                    types::Value::BoolArray(items) => Self::Array(opentelemetry::Array::Bool(items)),
                    types::Value::S64Array(items) => Self::Array(opentelemetry::Array::I64(items)),
                    types::Value::F64Array(items) => Self::Array(opentelemetry::Array::F64(items)),
                    types::Value::StringArray(items) => Self::Array(opentelemetry::Array::String(
                        items.into_iter().map(Into::into).collect(),
                    )),
                }
            }
        }
    }
}
