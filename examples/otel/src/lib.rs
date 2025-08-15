use axum::routing::post;
use axum::{Json, Router};
use opentelemetry::global;
use sdk_http::Result;
use serde_json::{Value, json};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};

struct HttpGuest;

use opentelemetry::trace::{TraceContextExt, Tracer, TracerProvider};
use opentelemetry::{Context, KeyValue};
use opentelemetry_sdk::trace::SdkTracerProvider;
use sdk_otel::Propagator;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry;
use tracing_subscriber::util::SubscriberInitExt;

impl Guest for HttpGuest {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        let processor = sdk_otel::Processor::new();
        let provider = SdkTracerProvider::builder().with_span_processor(processor).build();
        // tracing layer is required by tracer::xxx_span! macros
        let tracer = provider.tracer("otel-tracing");
        let layer = tracing_opentelemetry::layer().with_tracer(tracer);
        registry().with(layer).try_init().unwrap();
        // set a global tracer provider
        global::set_tracer_provider(provider);

        // basic span
        let tracer = global::tracer("basic");
        tracer.in_span("main-operation", |cx| {
            let span = cx.span();
            span.set_attribute(KeyValue::new("my-attribute", "my-value"));
            span.add_event("main span event", vec![KeyValue::new("foo", "1")]);
            tracer.in_span("child-operation", |cx| {
                cx.span().add_event("sub span event", vec![KeyValue::new("bar", "1")]);
            });
        });

        // inject remote (host) context
        let propagator = sdk_otel::ContextPropagator::new();
        let _guard = propagator.extract(&Context::current()).attach();

        let out = tracing::debug_span!("handle request").in_scope(|| {
            tracing::info!("received request");
            let router = Router::new().route("/", post(handle));
            sdk_http::serve(router, request)
        });

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
