//! # Tracing

use anyhow::Result;
use opentelemetry::trace::{TraceContextExt, TracerProvider};
use opentelemetry::{Context, ContextGuard, global, trace as otel};
use opentelemetry_otlp::{SpanExporter, WithHttpConfig};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing_opentelemetry::layer as tracing_layer;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::layer as format_layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

use crate::ExportClient;
use crate::generated::wasi::otel::tracing as wasi;

// TODO: add .in_span(|| Fn(ctx)) as alternative to guard
// TODO: add xxx_span! macros
// TODO: handle initialization error
pub(crate) fn init(resource: Resource) -> Result<ContextGuard> {
    // let processor = Processor::new();
    // let provider = SdkTracerProvider::builder().with_span_processor(processor).build();

    // tracing provider
    let exporter = SpanExporter::builder().with_http().with_http_client(ExportClient).build()?;
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

// use std::sync::atomic::{AtomicBool, Ordering};
// use std::time::Duration;

// use anyhow::Result;
// use opentelemetry_sdk::error::OTelSdkError;
// use opentelemetry_sdk::trace as sdk;

// #[derive(Debug, Default)]
// pub struct Processor {
//     is_shutdown: AtomicBool,
// }

// impl Processor {
//     /// Create a new `Processor`.
//     pub fn new() -> Self {
//         Self::default()
//     }
// }

// impl sdk::SpanProcessor for Processor {
//     fn on_start(&self, span: &mut sdk::Span, _: &Context) {
//         if self.is_shutdown.load(Ordering::Relaxed) {
//             return;
//         }
//         if let Some(span_data) = span.exported_data() {
//             let span_data = wasi::SpanData::from(span_data);
//             wasi::on_start(&span_data, &span_data.span_context);
//         }
//     }

//     fn on_end(&self, span_data: sdk::SpanData) {
//         if self.is_shutdown.load(Ordering::Relaxed) {
//             return;
//         }
//         wasi::on_end(&span_data.into());
//     }

//     fn force_flush(&self) -> Result<(), OTelSdkError> {
//         if self.is_shutdown.load(Ordering::Relaxed) {
//             return Err(OTelSdkError::AlreadyShutdown);
//         }
//         Ok(())
//     }

//     fn shutdown_with_timeout(&self, _: Duration) -> Result<(), OTelSdkError> {
//         self.is_shutdown.store(true, Ordering::Relaxed);
//         self.force_flush()
//     }
// }
