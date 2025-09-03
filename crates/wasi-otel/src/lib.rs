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
use std::sync::OnceLock;

use anyhow::Result;
use credibil_otel::init;
use futures::FutureExt;
use futures::future::BoxFuture;
use opentelemetry::{Array, Key, Value};
use opentelemetry_sdk::Resource;
use runtime::RunState;
use wasmtime::component::{HasData, Linker};

use self::generated::wasi::otel as wasi_otel;
use self::generated::wasi::otel::types;

const DEF_HTTP_ADDR: &str = "http://localhost:4318";

static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

#[derive(Debug)]
pub struct Otel;

impl runtime::Service for Otel {
    fn add_to_linker(&self, l: &mut Linker<RunState>) -> Result<()> {
        wasi_otel::tracing::add_to_linker::<_, Data>(l, Host::new)?;
        wasi_otel::metrics::add_to_linker::<_, Data>(l, Host::new)?;
        wasi_otel::types::add_to_linker::<_, Data>(l, Host::new)?;
        wasi_otel::resource::add_to_linker::<_, Data>(l, Host::new)?;
        Ok(())
    }

    fn start(
        &self, _: wasmtime::component::InstancePre<RunState>,
    ) -> BoxFuture<'static, Result<()>> {
        async {
            let client = reqwest::Client::new();
            let _ = HTTP_CLIENT.set(client);
            Ok(())
        }
        .boxed()
    }
}

struct Data;
impl HasData for Data {
    type Data<'a> = Host<'a>;
}

struct Host<'a> {
    http_client: &'a reqwest::Client,
    _phantom: PhantomData<&'a ()>,
}

impl Host<'_> {
    fn new(_: &mut RunState) -> Self {
        Host {
            http_client: HTTP_CLIENT.wait(),
            _phantom: PhantomData,
        }
    }
}

impl wasi_otel::resource::Host for Host<'_> {
    async fn resource(&mut self) -> Result<types::Resource> {
        Ok(init::resource().into())
    }
}

impl From<&Resource> for types::Resource {
    fn from(resource: &Resource) -> Self {
        Self {
            attributes: resource.iter().map(Into::into).collect(),
            schema_url: resource.schema_url().map(Into::into),
        }
    }
}

impl From<(&Key, &Value)> for types::KeyValue {
    fn from((key, value): (&Key, &Value)) -> Self {
        Self {
            key: key.to_string(),
            value: value.clone().into(),
        }
    }
}

impl From<Value> for types::Value {
    fn from(value: Value) -> Self {
        match value {
            Value::Bool(v) => Self::Bool(v),
            Value::I64(v) => Self::S64(v),
            Value::F64(v) => Self::F64(v),
            Value::String(v) => Self::String(v.to_string()),
            Value::Array(v) => match v {
                Array::Bool(items) => Self::BoolArray(items),
                Array::I64(items) => Self::S64Array(items),
                Array::F64(items) => Self::F64Array(items),
                Array::String(items) => {
                    Self::StringArray(items.into_iter().map(Into::into).collect())
                }
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
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
