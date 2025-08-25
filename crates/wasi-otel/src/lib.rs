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
        imports: {
            default: async | tracing | trappable,
        },
        trappable_error_type: {
            "wasi:otel/types/error" => Error,
        }
    });
}

use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use anyhow::Result;
use runtime::Linkable;
use wasi_core::Ctx;
use wasmtime::component::{HasData, Linker};

use self::generated::wasi::otel as wasi_otel;
use self::generated::wasi::otel::types;

const DEF_HTTP_ADDR: &str = "http://localhost:4318";

pub struct Otel<'a> {
    http_client: reqwest::Client,
    _phantom: PhantomData<&'a ()>,
}

impl Otel<'_> {
    fn new(_: &mut Ctx) -> Otel<'_> {
        Otel {
            http_client: reqwest::Client::new(),
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

impl PartialEq for types::KeyValue {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value == other.value
    }
}

impl Hash for types::KeyValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
        self.value.hash(state);
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

impl PartialEq for types::Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::S64(a), Self::S64(b)) => a == b,
            (Self::F64(a), Self::F64(b)) => a == b,
            (Self::String(a), Self::String(b)) => a == b,
            (Self::BoolArray(a), Self::BoolArray(b)) => a == b,
            (Self::S64Array(a), Self::S64Array(b)) => a == b,
            (Self::F64Array(a), Self::F64Array(b)) => a == b,
            (Self::StringArray(a), Self::StringArray(b)) => a == b,
            _ => false,
        }
    }
}

impl Hash for types::Value {
    #[allow(clippy::cast_possible_truncation)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Bool(v) => v.hash(state),
            Self::S64(v) => v.hash(state),
            Self::F64(v) => v.to_bits().hash(state),
            Self::String(v) => v.hash(state),
            Self::BoolArray(items) => items.hash(state),
            Self::S64Array(items) => items.hash(state),
            Self::F64Array(items) => {
                items.iter().map(|v| *v as i64).collect::<Vec<_>>().hash(state);
            }
            Self::StringArray(items) => items.hash(state),
        }
    }
}

impl From<types::InstrumentationScope> for opentelemetry::InstrumentationScope {
    fn from(scope: types::InstrumentationScope) -> Self {
        let mut builder = Self::builder(scope.name);
        if let Some(version) = scope.version {
            builder = builder.with_version(version);
        }
        if let Some(schema_url) = scope.schema_url {
            builder = builder.with_schema_url(schema_url);
        }
        builder = builder.with_attributes(scope.attributes.iter().map(Into::into));
        builder.build()
    }
}

impl Eq for types::InstrumentationScope {}

impl PartialEq for types::InstrumentationScope {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.version == other.version
            && self.schema_url == other.schema_url
            && self.attributes == other.attributes
    }
}

impl Hash for types::InstrumentationScope {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.version.hash(state);
        self.schema_url.hash(state);
        self.attributes.hash(state);
    }
}

impl From<types::Datetime> for u64 {
    fn from(dt: types::Datetime) -> Self {
        (dt.seconds * 1_000_000_000) + Self::from(dt.nanoseconds)
    }
}
