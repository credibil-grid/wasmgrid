use anyhow::Result;
use axum::routing::post;
use axum::{Json, Router};
use bytes::Bytes;
use serde_json::{Value, json};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http;
use wasi::http::types::{IncomingRequest, ResponseOutparam};
use wit_bindings::messaging;
use wit_bindings::messaging::incoming_handler::Configuration;
use wit_bindings::messaging::request_reply;
use wit_bindings::messaging::types::{Client, Error, Message};

pub struct Http;

impl http::incoming_handler::Guest for Http {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        let subscriber =
            FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        let router = Router::new().route("/", post(handle));

        let out = sdk_http::serve(router, request);
        ResponseOutparam::set(response, out);
    }
}

async fn handle(body: Bytes) -> Json<Value> {
    let client = Client::connect("nats").unwrap();
    let message = Message::new(&body);
    let reply = request_reply::request(&client, "a", &message, None).expect("should reply");

    // process first reply
    let data = reply[0].data();
    let data_str = String::from_utf8_lossy(&data);

    Json(json!({"reply": data_str}))
}

wasi::http::proxy::export!(Http);

pub struct RequestReply;

impl messaging::incoming_handler::Guest for RequestReply {
    // Handle messages to subscribed topics.
    fn handle(message: Message) -> Result<(), Error> {
        let subscriber =
            FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        match message.topic().as_deref() {
            Some("a") => {
                let data = message.data();
                let data_str = String::from_utf8(data.clone())
                    .map_err(|_| Error::Other("not utf8".to_string()))?;
                tracing::debug!("message received on topic 'a': {data_str}");

                // send message to topic `b`
                let mut resp = b"reply from topic a: ".to_vec();
                resp.extend(data);

                let reply = Message::new(&resp);
                request_reply::reply(&message, reply)?;
            }
            _ => {
                return Ok(());
            }
        }
        Ok(())
    }

    // Subscribe to topics.
    fn configure() -> Result<Configuration, Error> {
        Ok(Configuration {
            topics: vec!["a".to_string()],
        })
    }
}

wit_bindings::messaging::export!(RequestReply  with_types_in wit_bindings::messaging);
