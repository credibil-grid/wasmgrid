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
    });
}

use std::time::SystemTime;

use anyhow::Result;
use opentelemetry::trace as otel;
use opentelemetry::trace::TraceContextExt;
use opentelemetry_sdk::trace as sdk;
use resources::Resources;
use runtime::Linkable;
use services::Ctx;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use wasmtime::component::{HasData, Linker};
use wasmtime_wasi::ResourceTable;

use self::generated::wasi::otel as wasi_otel;
use self::generated::wasi::otel::tracing::{self as wasi};

// pub type Result<T, E = Error> = anyhow::Result<T, E>;

pub struct Otel<'a> {
    _resources: &'a Resources,
    _table: &'a mut ResourceTable,
}

impl Otel<'_> {
    const fn new(c: &mut Ctx) -> Otel<'_> {
        Otel {
            _resources: &c.resources,
            _table: &mut c.table,
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
        wasi_otel::tracing::add_to_linker::<_, Data>(linker, Otel::new)
    }
}

impl wasi_otel::tracing::Host for Otel<'_> {
    async fn on_start(&mut self, span: wasi::SpanData, _parent: wasi::SpanContext) -> Result<()> {
        // if self.is_shutdown.load(Ordering::Relaxed) {
        //     return;
        // }

        // let span = sdk::SpanData::from(span);

        println!("on_start: {span:?}\n");
        Ok(())
    }

    async fn on_end(&mut self, span: wasi::SpanData) -> Result<()> {
        // if self.is_shutdown.load(Ordering::Relaxed) {
        //     return;
        // }

        // let _span = sdk::SpanData::from(span);

        println!("on_end: {span:?}\n");
        Ok(())
    }

    async fn current_span_context(&mut self) -> Result<wasi::SpanContext> {
        let ctx = Span::current().context();
        let span = ctx.span();
        Ok(wasi::SpanContext::from(span.span_context()))
    }
}

impl From<wasi::SpanData> for sdk::SpanData {
    fn from(span: wasi::SpanData) -> Self {
        let parent_span_id =
            otel::SpanId::from_hex(&span.parent_span_id).unwrap_or(otel::SpanId::INVALID);

        let mut events = sdk::SpanEvents::default();
        events.events = span.events.into_iter().map(Into::into).collect();

        let mut links = sdk::SpanLinks::default();
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

impl From<&otel::SpanContext> for wasi::SpanContext {
    fn from(ctx: &otel::SpanContext) -> Self {
        Self {
            trace_id: format!("{:x}", ctx.trace_id()),
            span_id: format!("{:x}", ctx.span_id()),
            trace_flags: ctx.trace_flags().into(),
            is_remote: ctx.is_remote(),
            trace_state: ctx
                .trace_state()
                .header()
                .split(',')
                .filter_map(|s| {
                    if let Some((key, ctx)) = s.split_once('=') {
                        Some((key.to_string(), ctx.to_string()))
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }
}

impl From<wasi::SpanContext> for otel::SpanContext {
    fn from(ctx: wasi::SpanContext) -> Self {
        let trace_id = otel::TraceId::from_hex(&ctx.trace_id).unwrap_or(otel::TraceId::INVALID);
        let span_id = otel::SpanId::from_hex(&ctx.span_id).unwrap_or(otel::SpanId::INVALID);
        let trace_state = otel::TraceState::from_key_value(ctx.trace_state)
            .unwrap_or_else(|_| otel::TraceState::default());
        Self::new(trace_id, span_id, ctx.trace_flags.into(), ctx.is_remote, trace_state)
    }
}

impl From<otel::TraceFlags> for wasi::TraceFlags {
    fn from(value: otel::TraceFlags) -> Self {
        if value.is_sampled() { Self::SAMPLED } else { Self::empty() }
    }
}

impl From<wasi::TraceFlags> for otel::TraceFlags {
    fn from(value: wasi::TraceFlags) -> Self {
        if value.contains(wasi::TraceFlags::SAMPLED) { Self::SAMPLED } else { Self::NOT_SAMPLED }
    }
}

impl From<wasi::SpanKind> for otel::SpanKind {
    fn from(value: wasi::SpanKind) -> Self {
        match value {
            wasi::SpanKind::Client => Self::Client,
            wasi::SpanKind::Server => Self::Server,
            wasi::SpanKind::Producer => Self::Producer,
            wasi::SpanKind::Consumer => Self::Consumer,
            wasi::SpanKind::Internal => Self::Internal,
        }
    }
}

impl From<wasi::Datetime> for SystemTime {
    fn from(value: wasi::Datetime) -> Self {
        Self::UNIX_EPOCH
            .checked_add(std::time::Duration::new(value.seconds, value.nanoseconds))
            .unwrap_or(Self::UNIX_EPOCH)
    }
}

impl From<wasi::KeyValue> for opentelemetry::KeyValue {
    fn from(value: wasi::KeyValue) -> Self {
        Self::new(value.key, value.value)
    }
}

impl From<&wasi::KeyValue> for opentelemetry::KeyValue {
    fn from(value: &wasi::KeyValue) -> Self {
        Self::new(value.key.clone(), value.value.clone())
    }
}

impl From<wasi::Value> for opentelemetry::Value {
    fn from(value: wasi::Value) -> Self {
        match value {
            wasi::Value::Bool(v) => Self::Bool(v),
            wasi::Value::S64(v) => Self::I64(v),
            wasi::Value::F64(v) => Self::F64(v),
            wasi::Value::String(v) => Self::String(v.into()),
            wasi::Value::BoolArray(items) => Self::Array(opentelemetry::Array::Bool(items)),
            wasi::Value::S64Array(items) => Self::Array(opentelemetry::Array::I64(items)),
            wasi::Value::F64Array(items) => Self::Array(opentelemetry::Array::F64(items)),
            wasi::Value::StringArray(items) => Self::Array(opentelemetry::Array::String(
                items.into_iter().map(Into::into).collect(),
            )),
        }
    }
}

impl From<wasi::Event> for otel::Event {
    fn from(value: wasi::Event) -> Self {
        let attrs = value.attributes.into_iter().map(Into::into).collect();
        Self::new(value.name, value.time.into(), attrs, 0)
    }
}

impl From<wasi::Link> for otel::Link {
    fn from(value: wasi::Link) -> Self {
        let attrs = value.attributes.into_iter().map(Into::into).collect();
        Self::new(value.span_context.into(), attrs, 0)
    }
}

impl From<wasi::Status> for otel::Status {
    fn from(value: wasi::Status) -> Self {
        match value {
            wasi::Status::Unset => Self::Unset,
            wasi::Status::Error(description) => Self::Error {
                description: description.into(),
            },
            wasi::Status::Ok => Self::Ok,
        }
    }
}

impl From<wasi::InstrumentationScope> for opentelemetry::InstrumentationScope {
    fn from(value: wasi::InstrumentationScope) -> Self {
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
