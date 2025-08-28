//! # Tracing

use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::Result;
use futures::executor::block_on;
use opentelemetry::trace::{SpanContext, TraceContextExt, TracerProvider};
use opentelemetry::{Context, ContextGuard, global, trace as otel};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::error::OTelSdkResult;
use opentelemetry_sdk::trace::{SdkTracerProvider, Span, SpanData, SpanExporter, SpanProcessor};
use tracing_opentelemetry::layer as tracing_layer;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::layer as format_layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

use crate::export::tracing::Exporter;
use crate::generated::wasi::otel::tracing as wasi;

pub(crate) fn init(resource: Resource) -> Result<SdkTracerProvider> {
    let exporter = Exporter::new()?;
    let processor = Processor::new(exporter);
    let provider =
        SdkTracerProvider::builder().with_resource(resource).with_span_processor(processor).build();

    // tracing layers
    let filter = EnvFilter::from_default_env()
        .add_directive("hyper=off".parse()?)
        .add_directive("h2=off".parse()?)
        .add_directive("tonic=off".parse()?);
    let format = format_layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE);
    let tracer = provider.tracer("global");
    let tracing = tracing_layer().with_tracer(tracer);

    Registry::default().with(filter).with(format).with(tracing).try_init()?;
    global::set_tracer_provider(provider.clone());

    Ok(provider)
}

// propagate (inject) host context
pub(crate) fn context() -> ContextGuard {
    let host_ctx = wasi::context();
    let context: SpanContext = host_ctx.into();
    let current = Context::current();

    // use current context if remote context is invalid
    if !context.is_valid() {
        return current.attach();
    }
    current.with_remote_span_context(context).attach()
}

#[derive(Debug)]
struct Processor {
    exporter: Exporter,
    spans: Arc<Mutex<Vec<SpanData>>>,
}

impl Processor {
    /// Create a new span processor.
    #[must_use]
    pub fn new(exporter: Exporter) -> Self {
        Self {
            exporter,
            spans: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl SpanProcessor for Processor {
    fn on_start(&self, _: &mut Span, _cx: &Context) {}

    fn on_end(&self, span: SpanData) {
        if !span.span_context.is_sampled() {
            return;
        }
        self.spans.lock().unwrap().push(span);
    }

    fn force_flush(&self) -> OTelSdkResult {
        Ok(())
    }

    fn shutdown_with_timeout(&self, _: Duration) -> OTelSdkResult {
        for span in self.spans.lock().unwrap().drain(..) {
            block_on(async { self.exporter.export(vec![span]).await })?;
        }
        Ok(())
    }

    fn set_resource(&mut self, resource: &Resource) {
        self.exporter.set_resource(resource);
    }
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
        if value.contains(wasi::TraceFlags::SAMPLED) { Self::SAMPLED } else { Self::default() }
    }
}
