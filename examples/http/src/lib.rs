use axum::routing::post;
use axum::{Json, Router};
use http_server::Result;
use opentelemetry::trace::{TraceContextExt, Tracer};
use opentelemetry::{Context, KeyValue, global};
use opentelemetry_sdk::trace::SdkTracerProvider;
use otel_client::WasiPropagator;
use serde_json::{Value, json};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};

struct HttpGuest;

impl Guest for HttpGuest {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        // let subscriber =
        //     FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        // tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        // Set up a tracer using the WASI processor
        let wasi_processor = otel_client::Processor::new();
        let tracer_provider =
            SdkTracerProvider::builder().with_span_processor(wasi_processor).build();
        global::set_tracer_provider(tracer_provider);
        let tracer = global::tracer("basic-spin");

        // Extract context from the Wasm host
        let wasi_propagator = otel_client::TraceContextPropagator::new();
        let _context_guard = wasi_propagator.extract(&Context::current()).attach();

        // Create some spans and events
        tracer.in_span("main-operation", |cx| {
            let span = cx.span();
            span.set_attribute(KeyValue::new("my-attribute", "my-value"));
            span.add_event("Main span event".to_string(), vec![KeyValue::new("foo", "1")]);
            tracer.in_span("child-operation", |cx| {
                let span = cx.span();
                span.add_event("Sub span event", vec![KeyValue::new("bar", "1")]);

                // let store = Store::open_default().unwrap();
                // store.set("foo", "bar".as_bytes()).unwrap();
                tracing::info!("received request");
            });
        });

        tracing::info!("received request");

        let router = Router::new().route("/", post(handle));
        let out = http_server::serve(router, request);
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
