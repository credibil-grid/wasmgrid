use anyhow::{Result, anyhow};
use serde_json::{Value, json};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};
use wasi_bindings::vault::vault;
use wasi_http_ext::{self, Request, Response, Router, post};

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

fn handler(request: &Request) -> Result<Response> {
    let body = request.body()?;
    let request: Value = serde_json::from_slice(&body)?;
    tracing::debug!("received request: {request:?}");

    let locker =
        vault::open("credibil-locker").map_err(|e| anyhow!("failed to open vault locker: {e}"))?;

    // write secret to vault
    locker.set("secret-id", &body).map_err(|e| anyhow!("issue setting secret: {e}"))?;

    // read secret from vault
    let secret = locker.get("secret-id").map_err(|e| anyhow!("issue retriving secret: {e}"))?;
    assert_eq!(secret.unwrap(), body);

    let response = serde_json::from_slice::<Value>(&body)?;
    tracing::debug!("sending response: {:?}", json!(response));
    Ok(serde_json::to_vec(&response)?.into())
}

wasi::http::proxy::export!(HttpGuest);
