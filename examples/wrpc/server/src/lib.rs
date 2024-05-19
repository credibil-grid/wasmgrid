use serde::Deserialize;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi_bindings::wrpc::exports::wasi::wrpc::server::Guest as WrpcGuest;
use wasi_bindings::wrpc::types::{Error, ServerConfiguration};

// #[derive(Deserialize)]
pub struct Hello {
    message: String,
}

impl<'de> Deserialize<'de> for Hello {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let message = String::deserialize(deserializer)?;
        Ok(Hello { message })
    }
}

pub struct Server;

impl WrpcGuest for Server {
    fn configure() -> Result<ServerConfiguration, Error> {
        Ok(ServerConfiguration {
            name: "server".to_string(),
        })
    }

    // Whenever a message is received on a subscribed channel, the host will call this
    // function. Once the message has been handled, the host should kill the Wasm
    // instance.
    fn handle(request: Vec<u8>) -> Result<Vec<u8>, Error> {
        let subscriber =
            FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        // let msg: Hello = serde_json::from_slice(&request).unwrap();
        // println!("Received request: {:?}", msg.message);
        println!("Received request: {:?}", request);

        Ok(b"Message received!".to_vec())
    }
}

wasi_bindings::wrpc::export!(Server with_types_in wasi_bindings::wrpc);
