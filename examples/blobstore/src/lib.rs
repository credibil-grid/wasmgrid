#![feature(let_chains)]

use anyhow::{Result, anyhow};
use serde_json::json;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};
use wasi_bindings::blobstore::blobstore;
use wasi_bindings::blobstore::types::OutgoingValue;
use wasi_http::{self, Request, Router, post};

struct HttpGuest;

impl Guest for HttpGuest {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        let subscriber =
            FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        let router = Router::new().route("/", post(handler));

        let out = wasi_http::serve(&router, &request);
        ResponseOutparam::set(response, out);
    }
}

fn handler(request: &Request) -> Result<Vec<u8>> {
    let body = request.body()?;
    let req: serde_json::Value = serde_json::from_slice(&body)?;
    tracing::debug!("json: {:?}", req);

    // write to blobstore
    let data = serde_json::to_vec(&body)?;

    let value = OutgoingValue::new_outgoing_value();
    let stream = value.outgoing_value_write_body().map_err(|_| anyhow!("failed to write body"))?;
    stream.blocking_write_and_flush(&data)?;

    let container = blobstore::create_container("credibil_bucket")
        .map_err(|e| anyhow!("failed to create container: {e}"))?;
    container.write_data("request", &value).map_err(|e| anyhow!("failed to write data: {e}"))?;

    OutgoingValue::finish(value).map_err(|e| anyhow!("issue finishing: {e}"))?;

    // // read from blobstore
    // let read_value =
    //     container.get_data("request", 0, 0).map_err(|e| anyhow!("failed to read data: {e}"))?;
    // let data = IncomingValue::incoming_value_consume_sync(read_value)
    //     .map_err(|_| anyhow!("failed to create incoming value"))?;

    // tracing::debug!("read from container");

    // let request = serde_json::from_slice::<serde_json::Value>(&data)?;
    // tracing::debug!("request: {request:?}");

    serde_json::to_vec(&json!({
        "message": "Hello, World!"
    }))
    .map_err(Into::into)
}

wasi::http::proxy::export!(HttpGuest);
