use serde_json::json;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};
use wasi_http::{self, Request, Router};

struct HttpGuest;

impl Guest for HttpGuest {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        let subscriber =
            FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        for (name, value) in request.headers().entries() {
            println!("guest {}: {}", name, String::from_utf8(value).unwrap());
        }

        println!("request.scheme(): {:?}", request.scheme());
        println!("request.authority(): {:?}", request.authority());

        let router = Router::new().route("/", handler);

        let out = wasi_http::serve(&router, &request);
        ResponseOutparam::set(response, out);
    }
}

fn handler(request: &Request) -> anyhow::Result<Vec<u8>> {
    let req_val: serde_json::Value = serde_json::from_slice(&request.body()?)?;
    tracing::debug!("request received: {:?}", req_val);

    serde_json::to_vec(&json!({
        "message": "Hello, World!"
    }))
    .map_err(Into::into)
}

wasi::http::proxy::export!(HttpGuest);
