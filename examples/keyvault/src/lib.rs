#![feature(let_chains)]

use serde_json::json;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};
use wasi_bindings::keyvalue::store;
use wasi_http::{self, post, Request, Router};

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

fn handler(request: &Request) -> anyhow::Result<Vec<u8>> {
    let body = request.body()?;
    let req: serde_json::Value = serde_json::from_slice(&body)?;
    tracing::debug!("json: {:?}", req);

    let bucket = match store::open("credibil_bucket") {
        Ok(bucket) => bucket,
        Err(err) => {
            tracing::debug!("error opening bucket: {:?}", err);
            return Err(err.into());
        }
    };

    bucket.set("my_key", &body)?;

    // check for previous value
    let res = bucket.get("my_key");
    tracing::debug!("found val: {:?}", res);

    serde_json::to_vec(&json!({
        "message": "Hello, World!"
    }))
    .map_err(Into::into)
}

wasi::http::proxy::export!(HttpGuest);
