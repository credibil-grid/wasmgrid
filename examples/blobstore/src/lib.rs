use anyhow::{Result, anyhow};
use serde_json::{Value, json};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};
use wasi_bindings::blobstore::blobstore;
use wasi_bindings::blobstore::types::{IncomingValue, OutgoingValue};
use wasi_http_ext::{self, Request, Router, post};

struct HttpGuest;

impl Guest for HttpGuest {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        let subscriber =
            FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        let router = Router::new().route("/", post(handler));

        let out = wasi_http_ext::serve(&router, &request);
        ResponseOutparam::set(response, out);
    }
}

fn handler(request: &Request) -> Result<Vec<u8>> {
    let body = request.body()?;
    let request: Value = serde_json::from_slice(&body)?;
    tracing::debug!("received request: {request:?}");

    // write to blobstore
    let outgoing = OutgoingValue::new_outgoing_value();
    let stream =
        outgoing.outgoing_value_write_body().map_err(|_| anyhow!("failed create stream"))?;
    stream.blocking_write_and_flush(&body)?;

    let container = blobstore::create_container("credibil_bucket")
        .map_err(|e| anyhow!("failed to create container: {e}"))?;
    container.write_data("request", &outgoing).map_err(|e| anyhow!("failed to write data: {e}"))?;
    OutgoingValue::finish(outgoing).map_err(|e| anyhow!("issue finishing: {e}"))?;

    // read from blobstore
    let incoming =
        container.get_data("request", 0, 0).map_err(|e| anyhow!("failed to read data: {e}"))?;
    let data = IncomingValue::incoming_value_consume_sync(incoming)
        .map_err(|_| anyhow!("failed to create incoming value"))?;

    assert_eq!(data, body);

    let response = serde_json::from_slice::<Value>(&data)?;
    tracing::debug!("sending response: {:?}", json!(response));
    serde_json::to_vec(&response).map_err(Into::into)
}

wasi::http::proxy::export!(HttpGuest);
