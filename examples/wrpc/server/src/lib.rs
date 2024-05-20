use serde::{Deserialize, Serialize};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi_bindings::wrpc::exports::wasi::wrpc::server::{self, Error};
use wasi_bindings::wrpc::types::ServerConfiguration;

#[derive(Deserialize, Debug)]
pub struct WrpcRequest {
    message: String,
}

#[derive(Serialize)]
pub struct WrpcResponse {
    message: String,
}

struct Server;

impl server::Guest for Server {
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

        let msg: WrpcRequest = match serde_json::from_slice(request.as_slice()) {
            Ok(msg) => msg,
            Err(e) => {
                tracing::error!("Error deserializing request: {:?}", e);
                WrpcRequest {
                    message: "Error deserializing request".to_string(),
                }
            }
        };
        println!("request: {:?}", msg);

        // return response
        Ok(serde_json::to_vec(&WrpcResponse {
            message: format!("Thank you for your message: {}", msg.message),
        })
        .unwrap())
    }
}

wasi_bindings::wrpc::export!(Server with_types_in wasi_bindings::wrpc);
