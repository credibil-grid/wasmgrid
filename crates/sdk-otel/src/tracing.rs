//! # Tracing

use anyhow::Result;
use opentelemetry::trace::{TraceContextExt, TracerProvider};
use opentelemetry::{Context, ContextGuard, global, trace as otel};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing_opentelemetry::layer as tracing_layer;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::layer as format_layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

use crate::export::tracing;
use crate::generated::wasi::otel::tracing as wasi;

// TODO: add xxx_span! macros
// TODO: handle initialization error
pub(crate) fn init(resource: Resource) -> Result<ContextGuard> {
    let exporter = tracing::exporter()?;
    let provider =
        SdkTracerProvider::builder().with_resource(resource).with_simple_exporter(exporter).build();

    // tracing layers
    let filter = EnvFilter::from_default_env()
        .add_directive("hyper=off".parse()?)
        .add_directive("h2=off".parse()?)
        .add_directive("tonic=off".parse()?);
    let format = format_layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE);
    let tracer = provider.tracer("global");
    let tracing = tracing_layer().with_tracer(tracer);

    Registry::default().with(filter).with(format).with(tracing).try_init()?;
    global::set_tracer_provider(provider);

    // propagate (inject) host context
    let host_ctx = wasi::current_span_context();
    let guest_ctx = Context::current().with_remote_span_context(host_ctx.into());
    let guard = guest_ctx.attach();

    Ok(guard)
}

impl From<wasi::SpanContext> for otel::SpanContext {
    fn from(value: wasi::SpanContext) -> Self {
        let trace_id = otel::TraceId::from_hex(&value.trace_id).unwrap_or(otel::TraceId::INVALID);
        let span_id = otel::SpanId::from_hex(&value.span_id).unwrap_or(otel::SpanId::INVALID);
        let trace_state = otel::TraceState::from_key_value(value.trace_state)
            .unwrap_or_else(|_| otel::TraceState::default());
        Self::new(trace_id, span_id, value.trace_flags.into(), value.is_remote, trace_state)
    }
}

impl From<wasi::TraceFlags> for otel::TraceFlags {
    fn from(value: wasi::TraceFlags) -> Self {
        if value.contains(wasi::TraceFlags::SAMPLED) {
            otel::TraceFlags::SAMPLED
        } else {
            otel::TraceFlags::default()
        }
    }
}
