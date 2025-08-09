use anyhow::{Context, Result};
use axum::routing::post;
use axum::{Json, Router};
use serde_json::{Value, json};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};
use wasi_bindings::keyvalue::store;
use wasi_http_ext::{self, AxumError};

struct HttpGuest;

impl Guest for HttpGuest {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        let subscriber =
            FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        let router = Router::new().route("/", post(handle));

        let out = wasi_http_ext::serve(router, request);
        ResponseOutparam::set(response, out);
    }
}

async fn handle(Json(body): Json<Value>) -> Result<Json<Value>, AxumError> {
    tracing::debug!("json: {:?}", body);

    let bucket = store::open("credibil_bucket").context("opening bucket")?;
    let data = serde_json::to_vec(&body).context("serializing body")?;
    bucket.set("my_key", &data).context("storing data")?;

    // check for previous value
    let res = bucket.get("my_key");
    tracing::debug!("found val: {:?}", res);

    Ok(Json(json!({
        "message": "Hello, World!"
    })))
}

wasi::http::proxy::export!(HttpGuest);
