//! # Tracing

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, UNIX_EPOCH};

use anyhow::Result;
use opentelemetry::{Context, trace as otel};
use opentelemetry_sdk::error::OTelSdkError;
use opentelemetry_sdk::trace as sdk;

use crate::generated::wasi::otel::tracing as wasi;

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

    fn shutdown(&self) -> Result<(), OTelSdkError> {
        self.force_flush()
    }

    fn shutdown_with_timeout(&self, _: Duration) -> Result<(), OTelSdkError> {
        unimplemented!("`shutdown_with_timeout` is not implemented");
    }
}

impl From<sdk::SpanData> for wasi::SpanData {
    fn from(value: sdk::SpanData) -> Self {
        Self {
            span_context: value.span_context.into(),
            parent_span_id: value.parent_span_id.to_string(),
            span_kind: value.span_kind.into(),
            name: value.name.to_string(),
            start_time: value.start_time.into(),
            end_time: value.end_time.into(),
            attributes: value.attributes.into_iter().map(Into::into).collect(),
            events: value.events.events.into_iter().map(Into::into).collect(),
            links: value.links.links.into_iter().map(Into::into).collect(),
            status: value.status.into(),
            instrumentation_scope: value.instrumentation_scope.into(),
            dropped_attributes: value.dropped_attributes_count,
            dropped_events: value.events.dropped_count,
            dropped_links: value.links.dropped_count,
        }
    }
}

impl From<otel::SpanContext> for wasi::SpanContext {
    fn from(value: otel::SpanContext) -> Self {
        Self {
            trace_id: format!("{:x}", value.trace_id()),
            span_id: format!("{:x}", value.span_id()),
            trace_flags: value.trace_flags().into(),
            is_remote: value.is_remote(),
            trace_state: value
                .trace_state()
                .header()
                .split(',')
                .filter_map(|s| {
                    if let Some((key, value)) = s.split_once('=') {
                        Some((key.to_string(), value.to_string()))
                    } else {
                        None
                    }
                })
                .collect(),
        }
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

impl From<otel::TraceFlags> for wasi::TraceFlags {
    fn from(value: otel::TraceFlags) -> Self {
        if value.is_sampled() { wasi::TraceFlags::SAMPLED } else { wasi::TraceFlags::empty() }
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

impl From<otel::SpanKind> for wasi::SpanKind {
    fn from(value: otel::SpanKind) -> Self {
        match value {
            otel::SpanKind::Client => Self::Client,
            otel::SpanKind::Server => Self::Server,
            otel::SpanKind::Producer => Self::Producer,
            otel::SpanKind::Consumer => Self::Consumer,
            otel::SpanKind::Internal => Self::Internal,
        }
    }
}

impl From<std::time::SystemTime> for wasi::Datetime {
    fn from(value: std::time::SystemTime) -> Self {
        let duration_since_epoch =
            value.duration_since(UNIX_EPOCH).expect("SystemTime should be after UNIX EPOCH");
        Self {
            seconds: duration_since_epoch.as_secs(),
            nanoseconds: duration_since_epoch.subsec_nanos(),
        }
    }
}

impl From<opentelemetry::KeyValue> for wasi::KeyValue {
    fn from(value: opentelemetry::KeyValue) -> Self {
        Self {
            key: value.key.to_string(),
            value: value.value.into(),
        }
    }
}

impl From<&opentelemetry::KeyValue> for wasi::KeyValue {
    fn from(value: &opentelemetry::KeyValue) -> Self {
        Self {
            key: value.key.to_string(),
            value: value.value.clone().into(),
        }
    }
}

impl From<opentelemetry::Value> for wasi::Value {
    fn from(value: opentelemetry::Value) -> Self {
        match value {
            opentelemetry::Value::Bool(v) => Self::Bool(v),
            opentelemetry::Value::I64(v) => Self::S64(v),
            opentelemetry::Value::F64(v) => Self::F64(v),
            opentelemetry::Value::String(v) => Self::String(v.to_string()),
            opentelemetry::Value::Array(v) => match v {
                opentelemetry::Array::Bool(items) => Self::BoolArray(items),
                opentelemetry::Array::I64(items) => Self::S64Array(items),
                opentelemetry::Array::F64(items) => Self::F64Array(items),
                opentelemetry::Array::String(items) => {
                    Self::StringArray(items.into_iter().map(Into::into).collect())
                }
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }
}

impl From<otel::Event> for wasi::Event {
    fn from(value: otel::Event) -> Self {
        Self {
            name: value.name.to_string(),
            time: value.timestamp.into(),
            attributes: value.attributes.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<otel::Link> for wasi::Link {
    fn from(value: otel::Link) -> Self {
        Self {
            span_context: value.span_context.into(),
            attributes: value.attributes.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<otel::Status> for wasi::Status {
    fn from(value: otel::Status) -> Self {
        match value {
            otel::Status::Unset => Self::Unset,
            otel::Status::Error { description } => Self::Error(description.to_string()),
            otel::Status::Ok => Self::Ok,
        }
    }
}

impl From<opentelemetry::InstrumentationScope> for wasi::InstrumentationScope {
    fn from(value: opentelemetry::InstrumentationScope) -> Self {
        Self {
            name: value.name().to_string(),
            version: value.version().map(Into::into),
            schema_url: value.schema_url().map(Into::into),
            attributes: value.attributes().map(Into::into).collect(),
        }
    }
}
