//! # Tracing

use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::Result;
use futures::executor::block_on;
use opentelemetry::trace::{SpanContext, TraceContextExt};
use opentelemetry::{Context, ContextGuard, global, trace as otel};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::error::{OTelSdkError, OTelSdkResult};
use opentelemetry_sdk::trace::{SdkTracerProvider, Span, SpanData, SpanExporter, SpanProcessor};

use crate::export::tracing::Exporter;
use crate::generated::wasi::otel::tracing as wasi;

pub fn init(resource: Resource) -> Result<SdkTracerProvider> {
    let exporter = Exporter::new()?;
    let processor = Processor::new(exporter);
    let provider =
        SdkTracerProvider::builder().with_resource(resource).with_span_processor(processor).build();
    global::set_tracer_provider(provider.clone());
    Ok(provider)
}

// propagate (inject) host context
#[must_use]
pub fn context() -> ContextGuard {
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
    #[must_use]
    fn new(exporter: Exporter) -> Self {
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
        if let Ok(mut guard) = self.spans.lock() {
            guard.push(span);
        }
    }

    fn force_flush(&self) -> OTelSdkResult {
        Ok(())
    }

    fn shutdown_with_timeout(&self, _: Duration) -> OTelSdkResult {
        let spans =
            self.spans.lock().map_err(|e| OTelSdkError::InternalFailure(e.to_string()))?.to_vec();
        if spans.is_empty() {
            return Ok(());
        }

        block_on(async { self.exporter.export(spans).await })?;
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
