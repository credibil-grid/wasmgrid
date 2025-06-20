use anyhow::Result;
use serde_json::json;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http;
use wasi::http::types::{IncomingRequest, ResponseOutparam};
use wasi_bindings::messaging;
use wasi_bindings::messaging::incoming_handler::Configuration;
use wasi_bindings::messaging::producer::{self, Message};
use wasi_bindings::messaging::types::{Client, Error};
use wasi_http_ext::{self, Router, post};

pub struct Http;

impl http::incoming_handler::Guest for Http {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        let subscriber =
            FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        let router = Router::new().route(
            "/",
            post(|request| {
                // publish HTTP request body to NATS topic `a`
                let client = Client::connect("nats").unwrap();
                let message = Message::new(&request.body()?);
                producer::send(&client, "a", message).expect("should send");

                let resp = json!({"message": "message published"});
                serde_json::to_vec(&resp).map_err(Into::into)
            }),
        );

        let out = wasi_http_ext::serve(&router, &request);
        ResponseOutparam::set(response, out);
    }
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
                resp.extend(data.clone());

                let message = Message::new(&resp);
                if let Some(md) = message.metadata().clone() {
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
            topics: vec!["a", "b"].into_iter().map(|s| s.to_string()).collect(),
        })
    }
}

wasi_bindings::messaging::export!(Messaging with_types_in wasi_bindings::messaging);
