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
use wit_bindings::messaging::producer;
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
    producer::send(&client, "a", message).expect("should send");
    Json(json!({"message": "message published"}))
}

wasi::http::proxy::export!(Http);

pub struct Messaging;

impl messaging::incoming_handler::Guest for Messaging {
    // Handle messages to subscribed topics.
    fn handle(message: Message) -> Result<(), Error> {
        let subscriber =
            FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        let data = message.data();
        let data_str =
            String::from_utf8(data.clone()).map_err(|_| Error::Other("not utf8".to_string()))?;

        match message.topic().as_deref() {
            Some("a") => {
                tracing::debug!("message received with topic 'a': {data_str}");

                // send message to topic `b`
                let mut resp = b"topic a says: ".to_vec();
                resp.extend(data);

                let message = Message::new(&resp);
                if let Some(md) = message.metadata() {
                    message.set_metadata(&md);
                }

                // set `content_type` *after* `metadata` otherwise it is overwritten
                if let Some(format) = message.content_type() {
                    message.set_content_type(&format);
                }

                let client = Client::connect("nats")?;
                producer::send(&client, "b", message)?;
            }
            Some("b") => {
                tracing::debug!("message received on topic 'b': {data_str}");
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
            topics: vec!["a".to_string(), "b".to_string()],
        })
    }
}

wit_bindings::messaging::export!(Messaging with_types_in wit_bindings::messaging);
