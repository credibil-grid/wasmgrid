use http_shared::{self, Request, Router};
use serde_json::json;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};

struct HttpGuest;

impl Guest for HttpGuest {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        let subscriber =
            FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        let router = Router::new().route("/", handler);

        let out = http_shared::serve(&router, &request);
        ResponseOutparam::set(response, out);
    }
}

fn handler(request: &Request) -> anyhow::Result<Vec<u8>> {
    let req_val: serde_json::Value = serde_json::from_slice(&request.body()?)?;
    tracing::debug!("request: {:?}", req_val);

    serde_json::to_vec(&json!({
        "message": "Hello, World!"
    }))
    .map_err(Into::into)
}

wasi::http::proxy::export!(HttpGuest);
