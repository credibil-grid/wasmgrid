#![feature(let_chains)]

use base64ct::{Base64UrlUnpadded, Encoding};
use serde_json::json;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};
use wasi_bindings::vault::keystore;
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

fn handler(request: &Request) -> anyhow::Result<Vec<u8>> {
    let body = request.body()?;
    // let req: serde_json::Value = serde_json::from_slice(&body)?;

    let key_set = keystore::open("demo-credibil-io")?;
    tracing::debug!("key_set: {:?}", key_set);

    let signer = key_set.get("signing-key")?;
    let bytes = signer.sign(&body)?;
    let encoded = Base64UrlUnpadded::encode_string(&bytes);

    tracing::debug!("signature: {:?}", encoded);
    serde_json::to_vec(&json!({
        "signed": encoded
    }))
    .map_err(Into::into)
}

wasi::http::proxy::export!(HttpGuest);
