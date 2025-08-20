//! # OpenTelemetry SDK
//!
//! WASM component (guest) OpenTelemetry SDK.

pub mod generated {
    wit_bindgen::generate!({
        world: "otel",
        path: "../../wit",
        generate_all,
    });
}

mod convert;
pub mod metrics;
pub mod tracing;

use std::mem;

use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;
use http::{Request, Response};
use opentelemetry::ContextGuard;
use opentelemetry_http::{HttpClient, HttpError};
use opentelemetry_sdk::Resource;
use sdk_http::Client;

use self::metrics::Reader;

pub struct ScopeGuard {
    _tracing: ContextGuard,
    _metrics: Reader,
}

// TODO: add .in_span(|| Fn(ctx)) as alternative to guard
// TODO: add xxx_span! macros
pub fn init() -> ScopeGuard {
    let resource = Resource::builder().with_service_name("otel").build();
    
    ScopeGuard {
        _tracing: tracing::init(resource.clone()).expect("should initialize"),
        _metrics: metrics::init(resource).expect("should initialize"),
    }
}

pub fn instrument<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let _guard = init();
    f()
}

#[derive(Debug)]
struct ExportClient;

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
            Response::builder().status(response.status()).body(response.body().clone().into())?;
        *http_response.headers_mut() = headers;

        Ok(http_response)
    }
}
