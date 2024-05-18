#![feature(let_chains)]

use anyhow::anyhow;
use http_shared::{self, Request, Router};
use serde_json::json;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest as HttpGuest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};
use wasi_bindings::wrpc;
use wasi_bindings::wrpc::exports::wasi::wrpc::server::Guest as WrpcGuest;
use wasi_bindings::wrpc::types::Error;

pub struct Http;

impl HttpGuest for Http {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        let subscriber =
            FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        let router = Router::new().route("/", hello);

        let out = http_shared::serve(&router, &request);
        ResponseOutparam::set(response, out);
    }
}

pub fn hello(request: &Request) -> anyhow::Result<Vec<u8>> {
    tracing::debug!("request.uri: {}", request.uri());

    let resp = wrpc::client::call("holder", b"hello").map_err(|e| anyhow!(e.trace()))?;
    println!("response: {:?}", resp);

    let req: serde_json::Value = serde_json::from_slice(&request.body()?)?;
    tracing::debug!("json: {:?}", req);

    let resp = json!({
        "message": "Hello, World!"
    });
    serde_json::to_vec(&resp).map_err(Into::into)
}

wasi::http::proxy::export!(Http);

pub struct Server;

impl WrpcGuest for Server {
    // Whenever a message is received on a subscribed channel, the host will call this
    // function. Once the message has been handled, the host should kill the Wasm
    // instance.
    fn handle(request: Vec<u8>) -> Result<Vec<u8>, Error> {
        println!("Received request: {:?}", request);
        let resp = b"Hello, World!";
        Ok(resp.to_vec())
    }
}

wasi_bindings::wrpc::export!(Server with_types_in wasi_bindings::wrpc);
