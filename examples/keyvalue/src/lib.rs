use anyhow::Context;
use axum::routing::post;
use axum::{Json, Router};
use bytes::Bytes;
use sdk_http::Result;
use serde_json::{Value, json};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};
use wit_bindings::keyvalue::store;

struct HttpGuest;

impl Guest for HttpGuest {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        let subscriber =
            FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        let router = Router::new().route("/", post(handle));

        let out = sdk_http::serve(router, request);
        ResponseOutparam::set(response, out);
    }
}

async fn handle(body: Bytes) -> Result<Json<Value>> {
    let bucket = store::open("credibil_bucket").context("opening bucket")?;
    bucket.set("my_key", &body).context("storing data")?;

    // check for previous value
    let res = bucket.get("my_key");
    tracing::debug!("found val: {res:?}");

    Ok(Json(json!({
        "message": "Hello, World!"
    })))
}

wasi::http::proxy::export!(HttpGuest);
