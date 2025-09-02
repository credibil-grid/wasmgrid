//! # Export
//!
//! Convert OpenTelemetry types in `wasi-otel` types.

#[cfg(feature = "metrics")]
pub mod metrics;
#[cfg(feature = "tracing")]
pub mod tracing;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(all(feature = "guest-export", any(feature = "metrics", feature = "tracing")))] {
        use std::mem;
        use anyhow::Result;
        use async_trait::async_trait;
        use bytes::Bytes;
        use http::{Request, Response};
        use opentelemetry_http::{HttpClient, HttpError};
        use sdk_http::Client;

        #[derive(Debug)]
        pub struct ExportClient;

        #[async_trait]
        impl HttpClient for ExportClient {
            async fn send_bytes(&self, request: Request<Bytes>) -> Result<Response<Bytes>, HttpError> {
                let mut response = Client::new()
                    .post(request.uri())
                    .headers(request.headers())
                    .body(request.into_body().to_vec())
                    .send()?;

                let headers = mem::take(response.headers_mut());
                let mut http_response =
                    Response::builder().status(response.status()).body(response.body().clone())?;
                *http_response.headers_mut() = headers;

                Ok(http_response)
            }
        }
    } else {
        use std::time::{SystemTime, UNIX_EPOCH};
        use opentelemetry::{Array, InstrumentationScope, Key, KeyValue, Value};
        use crate::generated::wasi::clocks::wall_clock::Datetime;
        use crate::generated::wasi::otel::types as wasi;

        impl From<Value> for wasi::Value {
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

        impl From<KeyValue> for wasi::KeyValue {
            fn from(kv: KeyValue) -> Self {
                Self {
                    key: kv.key.to_string(),
                    value: kv.value.into(),
                }
            }
        }

        impl From<&KeyValue> for wasi::KeyValue {
            fn from(kv: &KeyValue) -> Self {
                kv.clone().into()
            }
        }

        impl From<(&Key, &Value)> for wasi::KeyValue {
            fn from((key, value): (&Key, &Value)) -> Self {
                Self {
                    key: key.to_string(),
                    value: value.clone().into(),
                }
            }
        }

        impl From<&Value> for wasi::Value {
            fn from(value: &Value) -> Self {
                value.clone().into()
            }
        }

        impl From<InstrumentationScope> for wasi::InstrumentationScope {
            fn from(scope: InstrumentationScope) -> Self {
                Self {
                    name: scope.name().to_string(),
                    version: scope.version().map(Into::into),
                    schema_url: scope.schema_url().map(Into::into),
                    attributes: scope.attributes().map(Into::into).collect(),
                }
            }
        }

        impl From<&InstrumentationScope> for wasi::InstrumentationScope {
            fn from(scope: &InstrumentationScope) -> Self {
                scope.clone().into()
            }
        }

        impl From<SystemTime> for Datetime {
            fn from(st: SystemTime) -> Self {
                let since_epoch = st.duration_since(UNIX_EPOCH).unwrap_or_default();
                Self {
                    seconds: since_epoch.as_secs(),
                    nanoseconds: since_epoch.subsec_nanos(),
                }
            }
        }
    }
}
