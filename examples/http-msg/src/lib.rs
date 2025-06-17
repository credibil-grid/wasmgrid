use serde_json::json;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http;
use wasi::http::types::{IncomingRequest, ResponseOutparam};
use wasi_bindings::messaging;
use wasi_bindings::messaging::incoming_handler::Configuration;
use wasi_bindings::messaging::producer::{self, Message};
use wasi_bindings::messaging::types::{Client, Error};
use wasi_http::{self, Request, Router, post};

pub struct Http;

impl http::incoming_handler::Guest for Http {
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

pub struct Messaging;

impl messaging::incoming_handler::Guest for Messaging {
    /// Whenever this guest receives a message in one of the subscribed topics, the message is
    /// sent to this handler.
    ///
    /// The guest is responsible for matching on the topic and handling the
    /// message accordingly. Implementors (such as hosts) calling this interface should make their
    /// own decisions on how to handle errors returned from this function.
    ///
    /// Once the message has been handled, the host should kill the Wasm instance.
    fn handle(msg: Message) -> Result<(), Error> {
        let subscriber =
            FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        // get topic
        let Some(topic) = &msg.topic() else {
            return Ok(());
        };

        let data = msg.data();
        let data_str = String::from_utf8(data.clone())
            .unwrap_or("message bytes received but not utf-8".to_string());
        match topic.as_str() {
            "a" => {
                tracing::debug!("message received with topic 'a': {data_str}");
            }
            "b" => {
                tracing::debug!("message received with topic 'b': {data_str}");
                // request-reply from channel d
                // let client = Client::connect("demo.nats.io")?;
                // let msgs = consumer::subscribe_try_receive(client, &Channel::from("d"), 100)?;
                // tracing::debug!("channel d: {:?}", msgs);

                // return consumer::complete_message(&msg);
            }
            "c" => {
                tracing::debug!("message received with topic 'd': {data_str}");

                // send message to temp channel d
                let mut resp = b"channel c: ".to_vec();
                resp.extend(data.clone());

                let client = Client::connect("demo.nats.io")?;

                // No Clone on Message so we need to create a new one manually.
                let message = Message::new(&resp);
                if let Some(md) = msg.metadata().clone() {
                    message.set_metadata(&md);
                }
                // Because NATS doesn't have a concept of content type outside
                // of headers, we need to set this *after* we set the metadata
                // otherwise it will be overwritten.
                if let Some(format) = msg.content_type() {
                    message.set_content_type(&format);
                }

                producer::send(&client, "d", message)?;
            }
            _ => {
                return Ok(());
            }
        }
        Ok(())
    }

    /// Set subscription topics.
    fn configure() -> Result<Configuration, Error> {
        Ok(Configuration {
            topics: vec!["a", "b", "c", "d"].into_iter().map(|s| s.to_string()).collect(),
        })
    }
}

// export guest implementations
wasi::http::proxy::export!(Http);
wasi_bindings::messaging::export!(Messaging with_types_in wasi_bindings::messaging);
