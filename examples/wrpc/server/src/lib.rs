use serde::{Deserialize, Serialize};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi_bindings::wrpc::exports::wasi::wrpc::server::{self, Error, ServerConfiguration};

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
            identifier: "server".into(),
        })
    }

    // Whenever a message is received on a subscribed channel, the host will call this
    // function. Once the message has been handled, the host should kill the Wasm
    // instance.
    fn handle(endpoint: String, request: Vec<u8>) -> Result<Vec<u8>, Error> {
        let subscriber =
            FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        // route request to appropriate handler
        tracing::debug!("received request for {endpoint}");
        match endpoint.as_str() {
            "server/Request" => handle_request(&request),
            _ => return Err(Error::UnknownEndpoint),
        }
    }
}

fn handle_request(request: &[u8]) -> Result<Vec<u8>, Error> {
    let msg: WrpcRequest = serde_json::from_slice(request).map_err(|_| Error::InvalidRequest)?;
    println!("request: {:?}", msg);

    // return response
    Ok(serde_json::to_vec(&WrpcResponse {
        message: format!("Thank you for your message: {}", msg.message),
    })
    .map_err(|e| Error::Other(e.to_string()))?)
}

wasi_bindings::wrpc::export!(Server with_types_in wasi_bindings::wrpc);
