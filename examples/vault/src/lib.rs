use anyhow::{Context, anyhow};
use axum::routing::post;
use axum::{Json, Router};
use bytes::Bytes;
use sdk_http::Result;
use serde_json::Value;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};
use wit_bindings::vault::vault;

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
    // write secret to vault
    let locker =
        vault::open("credibil-locker").map_err(|e| anyhow!("failed to open vault locker: {e}"))?;
    locker.set("secret-id", &body).map_err(|e| anyhow!("issue setting secret: {e}"))?;

    // read secret from vault
    let secret = locker.get("secret-id").map_err(|e| anyhow!("issue retriving secret: {e}"))?;
    assert_eq!(secret.unwrap(), body);

    let response = serde_json::from_slice::<Value>(&body).context("deserializing data")?;
    tracing::debug!("sending response: {response:?}");
    Ok(Json(response))
}

wasi::http::proxy::export!(HttpGuest);
