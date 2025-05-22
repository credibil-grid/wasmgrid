use serde_json::json;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};
use wasi_bindings::messaging::types::Client;
use wasi_bindings::messaging::producer::{self, Message};
use wasi_http::{self, Request, Router, post};

pub struct Http;

impl Guest for Http {
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
    tracing::debug!("request.uri: {}", request.uri());

    let client = Client::connect("demo.nats.io").unwrap();
    let message = Message::new(b"Hello");
    producer::send(&client, "b", message).expect("should send");

    let req: serde_json::Value = serde_json::from_slice(&request.body()?)?;
    tracing::debug!("json: {:?}", req);

    let resp = json!({
        "message": "Hello, World!"
    });
    serde_json::to_vec(&resp).map_err(Into::into)
}
