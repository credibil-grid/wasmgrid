use anyhow::{Context, anyhow};
use axum::routing::post;
use axum::{Json, Router};
use bytes::Bytes;
use sdk_http::Result;
use serde_json::Value;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};
use wit_bindings::blobstore::blobstore;
use wit_bindings::blobstore::types::{IncomingValue, OutgoingValue};

struct HttpGuest;

impl Guest for HttpGuest {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        let subscriber =
            FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        let router = Router::new().route("/", post(handler));

        let out = sdk_http::serve(router, request);
        ResponseOutparam::set(response, out);
    }
}

async fn handler(body: Bytes) -> Result<Json<Value>> {
    // write to blobstore
    let outgoing = OutgoingValue::new_outgoing_value();
    let stream =
        outgoing.outgoing_value_write_body().map_err(|_| anyhow!("failed create stream"))?;
    stream.blocking_write_and_flush(&body).context("writing body")?;

    let container = blobstore::create_container("container")
        .map_err(|e| anyhow!("failed to create container: {e}"))?;
    container.write_data("request", &outgoing).map_err(|e| anyhow!("failed to write data: {e}"))?;
    OutgoingValue::finish(outgoing).map_err(|e| anyhow!("issue finishing: {e}"))?;

    // read from blobstore
    let incoming =
        container.get_data("request", 0, 0).map_err(|e| anyhow!("failed to read data: {e}"))?;
    let data = IncomingValue::incoming_value_consume_sync(incoming)
        .map_err(|_| anyhow!("failed to create incoming value"))?;

    assert_eq!(data, body);

    let response = serde_json::from_slice::<Value>(&data).context("deserializing data")?;
    Ok(Json(response))
}

wasi::http::proxy::export!(HttpGuest);
