//! # WASI OpenTelemetry
//!
//! This module provides bindings for the OpenTelemetry specification
//! (wasi:otel) in the context of WebAssembly System Interface (WASI)
//! components.

mod metrics;
mod tracing;

mod generated {
    #![allow(clippy::trait_duplication_in_bounds)]
    pub use self::wasi::otel::types::Error;

    wasmtime::component::bindgen!({
        world: "otel",
        path: "../../wit",
        tracing: true,
        async: true,
        trappable_imports: true,
        trappable_error_type: {
            "wasi:otel/types/error" => Error,
        },
    });
}

use std::marker::PhantomData;
use std::time::{Duration, SystemTime};

use anyhow::Result;
use credibil_otel::init;
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::trace::SpanExporter as _;
use runtime::Linkable;
use wasi_core::Ctx;
use wasmtime::component::{HasData, Linker};

use self::generated::wasi::otel as wasi_otel;
use self::generated::wasi::otel::types;

pub struct Otel<'a> {
    exporter: SpanExporter,
    _phantom: PhantomData<&'a ()>,
}

impl Otel<'_> {
    fn new(_: &mut Ctx) -> Otel<'_> {
        let mut exporter =
            SpanExporter::builder().with_tonic().build().expect("should build exporter");
        exporter.set_resource(init::resource());

        Otel {
            exporter,
            _phantom: PhantomData,
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
        wasi_otel::tracing::add_to_linker::<_, Data>(linker, Otel::new)?;
        wasi_otel::metrics::add_to_linker::<_, Data>(linker, Otel::new)?;
        wasi_otel::types::add_to_linker::<_, Data>(linker, Otel::new)
    }
}

impl From<types::KeyValue> for opentelemetry::KeyValue {
    fn from(value: types::KeyValue) -> Self {
        Self::new(value.key, value.value)
    }
}

impl From<&types::KeyValue> for opentelemetry::KeyValue {
    fn from(value: &types::KeyValue) -> Self {
        Self::new(value.key.clone(), value.value.clone())
    }
}

impl From<types::Value> for opentelemetry::Value {
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

impl From<types::InstrumentationScope> for opentelemetry::InstrumentationScope {
    fn from(value: types::InstrumentationScope) -> Self {
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

impl From<types::Datetime> for SystemTime {
    fn from(value: types::Datetime) -> Self {
        Self::UNIX_EPOCH
            .checked_add(Duration::new(value.seconds, value.nanoseconds))
            .unwrap_or(Self::UNIX_EPOCH)
    }
}

impl From<types::Datetime> for u64 {
    fn from(value: types::Datetime) -> Self {
        value.seconds * 1_000_000_000 + value.nanoseconds as u64
    }
}
