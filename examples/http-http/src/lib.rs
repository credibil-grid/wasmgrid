#![feature(let_chains)]

use serde_json::json;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};
use wasi_http::{self, client, get, post, Request, Router};

struct HttpGuest;

impl Guest for HttpGuest {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        let subscriber =
            FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        let router = Router::new().route("/", get(get_handler)).route("/", post(post_handler));

        let out = wasi_http::serve(&router, &request);
        ResponseOutparam::set(response, out);
    }
}

// Forward request to external service and return the response
fn get_handler(_: &Request) -> anyhow::Result<Vec<u8>> {
    let resp = client::Client::new().get("https://jsonplaceholder.cypress.io/posts/1").send()?;

    serde_json::to_vec(&json!({
        "response": resp.json::<serde_json::Value>()?
    }))
    .map_err(Into::into)
}

// Forward request to external service and return the response
fn post_handler(request: &Request) -> anyhow::Result<Vec<u8>> {
    let body: serde_json::Value = serde_json::from_slice(&request.body()?)?;

    let resp = client::Client::new()
        .post("https://jsonplaceholder.cypress.io/posts")
        .bearer_auth("some token") // not required, but shown for example
        .json(&body)
        .send()?;

    serde_json::to_vec(&json!({
        "response": resp.json::<serde_json::Value>()?
    }))
    .map_err(Into::into)
}

wasi::http::proxy::export!(HttpGuest);
