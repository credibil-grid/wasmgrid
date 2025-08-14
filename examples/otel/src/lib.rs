use axum::routing::post;
use axum::{Json, Router};
use sdk_http::Result;
use sdk_otel::Otel;
use serde_json::{Value, json};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};

struct HttpGuest;

impl Guest for HttpGuest {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        let timer=std::time::SystemTime::now();
        // get host context
        let _guard = Otel::new("otel").with_host_context().expect("initializing telemetry");
        println!("time: {:?}", timer.elapsed());

        // let tracer = global::tracer("basic-spin");
        // tracer.in_span("main-operation", |cx| {
        //     let span = cx.span();
        //     span.set_attribute(KeyValue::new("my-attribute", "my-value"));
        //     span.add_event("Main span event".to_string(), vec![KeyValue::new("foo", "1")]);
        //     tracer.in_span("child-operation", |cx| {
        //         let span = cx.span();
        //         span.add_event("Sub span event", vec![KeyValue::new("bar", "1")]);
        //         // let store = Store::open_default().unwrap();
        //         // store.set("foo", "bar".as_bytes()).unwrap();
        //     });
        // });
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
