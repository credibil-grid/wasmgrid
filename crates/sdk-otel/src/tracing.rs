//! # Tracing

pub mod processor;

use anyhow::Result;
use opentelemetry::trace::{TraceContextExt, TracerProvider};
use opentelemetry::{Context, ContextGuard, global};
use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing_subscriber::Registry;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub use self::processor::*;
use crate::generated::wasi::otel::tracing as wasi;

pub fn init() {
    let processor = Processor::new();
    let provider = SdkTracerProvider::builder().with_span_processor(processor).build();

    // tracing layer is required by `tracer::xxx_span!` macros
    let tracer = provider.tracer("otel-tracing");
    let layer = tracing_opentelemetry::layer().with_tracer(tracer);
    Registry::default().with(layer).try_init().unwrap();

    // set a global tracer provider
    global::set_tracer_provider(provider);
}

pub fn init_with_context() -> Result<ContextGuard> {
    init();

    // inject remote (host) context
    let guard = Propagator::new().extract(&Context::current()).attach();
    Ok(guard)
}

/// Propagator to extract remote (host) tracing context.
#[derive(Debug, Clone)]
pub struct Propagator;

impl Propagator {
    pub fn new() -> Self {
        Self
    }

    pub fn extract(&self, ctx: &Context) -> Context {
        ctx.with_remote_span_context(wasi::current_span_context().into())
    }
}
