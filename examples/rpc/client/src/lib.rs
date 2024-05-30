use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest as HttpGuest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};
use wasi_bindings::rpc;
use wasi_http::{self, Request, Router};

pub struct Http;

impl HttpGuest for Http {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        let subscriber =
            FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        let router = Router::new().route("/", hello);

        let out = wasi_http::serve(&router, &request);
        ResponseOutparam::set(response, out);
    }
}

#[derive(Deserialize, Debug)]
pub struct HttpRequest {
    text: String,
}

#[derive(Serialize)]
pub struct WrpcRequest {
    message: String,
}

#[derive(Deserialize)]
pub struct WrpcResponse {
    message: String,
}

fn hello(request: &Request) -> anyhow::Result<Vec<u8>> {
    // extract http request
    let req: HttpRequest = serde_json::from_slice(&request.body()?)?;
    println!("http request: {:?}", req);

    // send http request message to wrpc server
    let msg = serde_json::to_vec(&WrpcRequest {
        message: format!("client says: {}", req.text),
    })?;

    // call server and deserialize response
    let ser_resp = rpc::client::call("server/Request", &msg).map_err(|e| anyhow!(e.trace()))?;
    let wrpc_resp: WrpcResponse = serde_json::from_slice(ser_resp.as_slice())?;

    // return http response
    serde_json::to_vec(&json!( {
        "message": format!("server says: {}", wrpc_resp.message),
    }))
    .map_err(Into::into)
}

wasi::http::proxy::export!(Http);
