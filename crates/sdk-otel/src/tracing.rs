//! # Tracing

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use anyhow::Result;
use opentelemetry::trace::{TraceContextExt, TracerProvider};
use opentelemetry::{Context, ContextGuard, global, trace as otel};
use opentelemetry_sdk::error::OTelSdkError;
use opentelemetry_sdk::trace as sdk;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing_subscriber::Registry;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::generated::wasi::otel::tracing as wasi;

// TODO: handle initialization error
pub fn init() {
    let processor = Processor::new();
    let provider = SdkTracerProvider::builder().with_span_processor(processor).build();

    // tracing layer is required by `tracer::xxx_span!` macros
    let tracer = provider.tracer("otel-tracing");
    let layer = tracing_opentelemetry::layer().with_tracer(tracer);
    Registry::default().with(layer).try_init().expect("should initialize tracing");
    global::set_tracer_provider(provider);
}

// TODO: add .in_span(|| Fn(ctx)) as alternative to guard
// TODO: add xxx_span! macros
pub fn init_with_context() -> ContextGuard {
    init();

    // propagate (inject) host context
    let host_ctx = wasi::current_span_context();
    let guest_ctx = Context::current().with_remote_span_context(host_ctx.into());
    guest_ctx.attach()
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

#[derive(Debug, Default)]
pub struct Processor {
    is_shutdown: AtomicBool,
}

impl Processor {
    /// Create a new `Processor`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl sdk::SpanProcessor for Processor {
    fn on_start(&self, span: &mut sdk::Span, _: &Context) {
        if self.is_shutdown.load(Ordering::Relaxed) {
            return;
        }
        if let Some(span_data) = span.exported_data() {
            let span_data = wasi::SpanData::from(span_data);
            wasi::on_start(&span_data, &span_data.span_context);
        }
    }

    fn on_end(&self, span: sdk::SpanData) {
        if self.is_shutdown.load(Ordering::Relaxed) {
            return;
        }
        wasi::on_end(&span.into());
    }

    fn force_flush(&self) -> Result<(), OTelSdkError> {
        if self.is_shutdown.load(Ordering::Relaxed) {
            return Err(OTelSdkError::AlreadyShutdown);
        }
        Ok(())
    }

    fn shutdown_with_timeout(&self, _: Duration) -> Result<(), OTelSdkError> {
        self.is_shutdown.store(true, Ordering::Relaxed);
        self.force_flush()
    }
}
