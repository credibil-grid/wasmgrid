use anyhow::anyhow;
use http_shared::{self, Request, Router};
use serde::Serialize;
use serde_json::json;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest as HttpGuest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};
use wasi_bindings::wrpc;

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

#[derive(Serialize)]
pub struct Hello {
    message: String,
}

pub fn hello(request: &Request) -> anyhow::Result<Vec<u8>> {
    let msg = serde_json::to_vec(&Hello {
        message: "Hello, World!".to_string(),
    })?;

    let resp = wrpc::client::call("server", &msg).map_err(|e| anyhow!(e.trace()))?;
    println!("Received response: {:?}", resp);
    // let msg: String = serde_json::from_slice(&resp).unwrap();
    // println!("Received response: {:?}", msg);

    let req: serde_json::Value = serde_json::from_slice(&request.body()?)?;
    println!("json: {:?}", req);

    serde_json::to_vec(&json!({
        "message": "Hello, World!"
    }))
    .map_err(Into::into)
}

wasi::http::proxy::export!(Http);
