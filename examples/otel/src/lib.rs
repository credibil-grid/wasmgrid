use axum::routing::post;
use axum::{Json, Router};
use opentelemetry_http::HttpClient;
use opentelemetry_otlp::WithHttpConfig;
use sdk_http::{Client, Result};
use serde_json::{Value, json};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};

struct HttpGuest;

#[derive(Debug)]
struct OtelClient;

use std::mem;

use async_trait::async_trait;
use bytes::Bytes;
use http::{Request, Response};
use opentelemetry::trace::{TraceContextExt, Tracer};
use opentelemetry::{Context, KeyValue, global};
use opentelemetry_http::HttpError;
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::SdkTracerProvider;
use sdk_otel::Propagator;

#[async_trait]
impl HttpClient for OtelClient {
    async fn send_bytes(&self, request: Request<Bytes>) -> Result<Response<Bytes>, HttpError> {
        println!("Sending request: {:?}", request);

        let mut response = Client::new()
            .post(request.uri())
            .headers(request.headers())
            .body(request.into_body().to_vec())
            .send::<Vec<u8>>()?;

        let headers = mem::take(response.headers_mut());
        let mut http_response =
            Response::builder().status(response.status()).body(response.body().clone().into())?;
        *http_response.headers_mut() = headers;

        Ok(http_response)
    }
}


impl Guest for HttpGuest {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        let subscriber =
            FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        let resource = Resource::builder()
            .with_service_name("http")
            .with_attributes(vec![
                KeyValue::new("deployment.environment", "unknown"),
                KeyValue::new("service.namespace", "http"),
                KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
                KeyValue::new("service.instance.id", "unknown"),
                KeyValue::new("telemetry.sdk.name", "opentelemetry"),
                KeyValue::new("instrumentation.provider", "opentelemetry"),
            ])
            .build();

        let span_exporter =
            SpanExporter::builder().with_http().with_http_client(OtelClient).build().unwrap();

        // Set up a tracer using the WASI processor
        let processor = sdk_otel::Processor::new();
        let tracer_provider = SdkTracerProvider::builder()
            .with_resource(resource)
            .with_span_processor(processor)
            .with_simple_exporter(span_exporter)
            .build();
        global::set_tracer_provider(tracer_provider);

        // get context from the Wasm host
        let propagator = sdk_otel::ContextPropagator::new();
        let _context = propagator.extract(&Context::current()).attach();

        // Create some spans and events
        let tracer = global::tracer("basic-spin");
        tracer.in_span("main-operation", |cx| {
            let span = cx.span();
            span.set_attribute(KeyValue::new("my-attribute", "my-value"));
            span.add_event("Main span event".to_string(), vec![KeyValue::new("foo", "1")]);
            tracer.in_span("child-operation", |cx| {
                let span = cx.span();
                span.add_event("Sub span event", vec![KeyValue::new("bar", "1")]);
                // let store = Store::open_default().unwrap();
                // store.set("foo", "bar".as_bytes()).unwrap();
            });
        });

        tracing::info!("received request");

        let router = Router::new().route("/", post(handle));
        let out = sdk_http::serve(router, request);
        ResponseOutparam::set(response, out);
    }
}

// A simple "Hello, World!" endpoint that returns the client's request.
async fn handle(Json(body): Json<Value>) -> Result<Json<Value>> {
    Ok(Json(json!({
        "message": "Hello, World!",
        "request": body
    })))
}

wasi::http::proxy::export!(HttpGuest);
