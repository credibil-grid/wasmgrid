use std::time::SystemTime;

use axum::routing::post;
use axum::{Json, Router};
use opentelemetry::trace::{TraceContextExt, Tracer};
use opentelemetry::{KeyValue, global};
use sdk_http::Result;
use serde_json::{Value, json};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};

struct HttpGuest;

impl Guest for HttpGuest {
    #[sdk_otel::instrument(name = "http_guest_handle")]
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        // inject remote (host) context
        let now = SystemTime::now();

        tracing::debug!("telemetry initialized {:?}", now.elapsed().unwrap());

        let meter = global::meter("my_meter");
        let counter = meter.u64_counter("my_counter").build();
        counter.add(1, &[KeyValue::new("key1", "value 1")]);
        counter.add(1, &[KeyValue::new("key1", "value 1")]);
        counter.add(1, &[KeyValue::new("key2", "value 2")]);

        // basic span
        let tracer = global::tracer("basic");
        tracer.in_span("main-operation", |cx| {
            let span = cx.span();
            span.set_attribute(KeyValue::new("my-attribute", "my-value"));
            span.add_event("main span event", vec![KeyValue::new("foo", "1")]);
            tracer.in_span("child-operation", |cx| {
                cx.span().add_event("sub span event", vec![KeyValue::new("bar", "1")]);
            });

            tracing::info_span!("child info span").in_scope(|| {
                tracing::info!("info event");
            });
        });

        let out = tracing::info_span!("handler span").in_scope(|| {
            tracing::info!("received request");
            let router = Router::new().route("/", post(handle));
            sdk_http::serve(router, request)
        });

        tracing::info!("request processed {:?}", now.elapsed().unwrap());

        ResponseOutparam::set(response, out);
    }
}

// A simple "Hello, World!" endpoint that returns the client's request.
#[sdk_otel::instrument(name = "handle_fn")]
async fn handle(Json(body): Json<Value>) -> Result<Json<Value>> {
    tracing::info!("handling request: {:?}", body);
    Ok(Json(json!({
        "message": "Hello, World!",
        "request": body
    })))
}

wasi::http::proxy::export!(HttpGuest);
