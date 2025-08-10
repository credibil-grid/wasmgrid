use axum::routing::post;
use axum::{Json, Router};
use http_server::Result;
use serde_json::{Value, json};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};

struct HttpGuest;

impl Guest for HttpGuest {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        let subscriber =
            FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        tracing::info!("Received request: {:?}", request);
        
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
