//! # WASI OpenTelemetry
//!
//! This module provides bindings for the OpenTelemetry specification in the
//! context of WebAssembly System Interface (WASI) components.

mod generated {
    #![allow(clippy::trait_duplication_in_bounds)]

    // pub use super::{Container, IncomingValue, OutgoingValue, StreamObjectNames};

    wasmtime::component::bindgen!({
        world: "otel",
        path: "../../wit",
        tracing: true,
        async: true,
        trappable_imports: true,
        // with: {
        //     "wasi:io": wasmtime_wasi::p2::bindings::io,

        //     "wasi:blobstore/types/incoming-value": IncomingValue,
        //     "wasi:blobstore/types/outgoing-value": OutgoingValue,
        //     "wasi:blobstore/container/container": Container,
        //     "wasi:blobstore/container/stream-object-names": StreamObjectNames,
        // },
        // trappable_error_type: {
        //     "wasi:blobstore/types/error" => anyhow::Error,
        // },
    });
}

use anyhow::Result;
use resources::Resources;
use runtime::Linkable;
use services::Ctx;
use wasmtime::component::{HasData, Linker};
use wasmtime_wasi::ResourceTable;

use self::generated::wasi::otel;
use self::generated::wasi::otel::tracing::{
    Datetime, Event, InstrumentationScope, KeyValue, Link, SpanContext, SpanData, SpanKind, Status,
    TraceFlags, Value,
};

// pub type Result<T, E = Error> = anyhow::Result<T, E>;

pub struct Otel<'a> {
    resources: &'a Resources,
    table: &'a mut ResourceTable,
}

impl Otel<'_> {
    const fn new(c: &mut Ctx) -> Otel<'_> {
        Otel {
            resources: &c.resources,
            table: &mut c.table,
        }
    }
}

struct Data;
impl HasData for Data {
    type Data<'a> = Otel<'a>;
}

pub struct Service;

impl Linkable for Service {
    type Ctx = Ctx;

    // Add the `wasi-otel` world's interfaces to a [`Linker`]
    fn add_to_linker(&self, linker: &mut Linker<Self::Ctx>) -> Result<()> {
        otel::tracing::add_to_linker::<_, Data>(linker, Otel::new)
    }
}

impl otel::tracing::Host for Otel<'_> {
    async fn on_start(&mut self, _span: SpanData, _parent: SpanContext) -> Result<()> {
        // if self.is_shutdown.load(Ordering::Relaxed) {
        //     return;
        // }

        // let span = opentelemetry_sdk::trace::SpanData::from(span);

        Ok(())
    }

    async fn on_end(&mut self, span: SpanData) -> Result<()> {
        // if self.is_shutdown.load(Ordering::Relaxed) {
        //     return;
        // }

        let _span = opentelemetry_sdk::trace::SpanData::from(span);

        Ok(())
    }

    async fn current_span_context(&mut self) -> Result<SpanContext> {
        todo!()
    }
}

impl From<SpanData> for opentelemetry_sdk::trace::SpanData {
    fn from(span: SpanData) -> Self {
        let parent_span_id = opentelemetry::SpanId::from_hex(&span.parent_span_id)
            .unwrap_or(opentelemetry::SpanId::INVALID);

        let mut events = opentelemetry_sdk::trace::SpanEvents::default();
        events.events = span.events.into_iter().map(Into::into).collect();

        let mut links = opentelemetry_sdk::trace::SpanLinks::default();
        links.links = span.links.into_iter().map(Into::into).collect();

        Self {
            span_context: span.span_context.into(),
            parent_span_id,
            span_kind: span.span_kind.into(),
            name: span.name.into(),
            start_time: span.start_time.into(),
            end_time: span.end_time.into(),
            attributes: span.attributes.into_iter().map(Into::into).collect(),
            dropped_attributes_count: span.dropped_attributes,
            events,
            links,
            status: span.status.into(),
            instrumentation_scope: span.instrumentation_scope.into(),
        }
    }
}

impl From<opentelemetry::trace::SpanContext> for SpanContext {
    fn from(value: opentelemetry::trace::SpanContext) -> Self {
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

impl From<SpanContext> for opentelemetry::trace::SpanContext {
    fn from(value: SpanContext) -> Self {
        let trace_id = opentelemetry::trace::TraceId::from_hex(&value.trace_id)
            .unwrap_or(opentelemetry::trace::TraceId::INVALID);
        let span_id = opentelemetry::trace::SpanId::from_hex(&value.span_id)
            .unwrap_or(opentelemetry::trace::SpanId::INVALID);
        let trace_state = opentelemetry::trace::TraceState::from_key_value(value.trace_state)
            .unwrap_or_else(|_| opentelemetry::trace::TraceState::default());
        Self::new(trace_id, span_id, value.trace_flags.into(), value.is_remote, trace_state)
    }
}

impl From<opentelemetry::trace::TraceFlags> for TraceFlags {
    fn from(value: opentelemetry::trace::TraceFlags) -> Self {
        if value.is_sampled() { Self::SAMPLED } else { Self::empty() }
    }
}

impl From<TraceFlags> for opentelemetry::trace::TraceFlags {
    fn from(value: TraceFlags) -> Self {
        if value.contains(TraceFlags::SAMPLED) {
            Self::SAMPLED
        } else {
            Self::NOT_SAMPLED
        }
    }
}

impl From<SpanKind> for opentelemetry::trace::SpanKind {
    fn from(value: SpanKind) -> Self {
        match value {
            SpanKind::Client => Self::Client,
            SpanKind::Server => Self::Server,
            SpanKind::Producer => Self::Producer,
            SpanKind::Consumer => Self::Consumer,
            SpanKind::Internal => Self::Internal,
        }
    }
}

impl From<Datetime> for std::time::SystemTime {
    fn from(value: Datetime) -> Self {
        Self::UNIX_EPOCH
            .checked_add(std::time::Duration::new(value.seconds, value.nanoseconds))
            .unwrap_or(Self::UNIX_EPOCH)
    }
}

impl From<KeyValue> for opentelemetry::KeyValue {
    fn from(value: KeyValue) -> Self {
        Self::new(value.key, value.value)
    }
}

impl From<&KeyValue> for opentelemetry::KeyValue {
    fn from(value: &KeyValue) -> Self {
        Self::new(value.key.clone(), value.value.clone())
    }
}

impl From<Value> for opentelemetry::Value {
    fn from(value: Value) -> Self {
        match value {
            Value::Bool(v) => Self::Bool(v),
            Value::S64(v) => Self::I64(v),
            Value::F64(v) => Self::F64(v),
            Value::String(v) => Self::String(v.into()),
            Value::BoolArray(items) => Self::Array(opentelemetry::Array::Bool(items)),
            Value::S64Array(items) => Self::Array(opentelemetry::Array::I64(items)),
            Value::F64Array(items) => Self::Array(opentelemetry::Array::F64(items)),
            Value::StringArray(items) => Self::Array(opentelemetry::Array::String(
                items.into_iter().map(Into::into).collect(),
            )),
        }
    }
}

impl From<Event> for opentelemetry::trace::Event {
    fn from(value: Event) -> Self {
        let attrs = value.attributes.into_iter().map(Into::into).collect();
        Self::new(value.name, value.time.into(), attrs, 0)
    }
}

impl From<Link> for opentelemetry::trace::Link {
    fn from(value: Link) -> Self {
        let attrs = value.attributes.into_iter().map(Into::into).collect();
        Self::new(value.span_context.into(), attrs, 0)
    }
}

impl From<Status> for opentelemetry::trace::Status {
    fn from(value: Status) -> Self {
        match value {
            Status::Unset => Self::Unset,
            Status::Error(description) => Self::Error {
                description: description.into(),
            },
            Status::Ok => Self::Ok,
        }
    }
}

impl From<InstrumentationScope> for opentelemetry::InstrumentationScope {
    fn from(value: InstrumentationScope) -> Self {
        let mut builder = Self::builder(value.name);
        if let Some(version) = value.version {
            builder = builder.with_version(version);
        }
        if let Some(schema_url) = value.schema_url {
            builder = builder.with_schema_url(schema_url);
        }
        builder = builder.with_attributes(value.attributes.iter().map(Into::into));
        builder.build()
    }
}
