use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi_bindings::wrpc::exports::wasi::wrpc::server::Guest as WrpcGuest;
use wasi_bindings::wrpc::types::{Error, ServerConfiguration};

pub struct Server;

impl WrpcGuest for Server {
    fn configure() -> Result<ServerConfiguration, Error> {
        Ok(ServerConfiguration {
            name: "holder".to_string(),
        })
    }

    // Whenever a message is received on a subscribed channel, the host will call this
    // function. Once the message has been handled, the host should kill the Wasm
    // instance.
    fn handle(request: Vec<u8>) -> Result<Vec<u8>, Error> {
        let subscriber =
            FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        println!("Received request: {:?}", request);
        let resp = b"Hello, World!";
        Ok(resp.to_vec())
    }
}

wasi_bindings::wrpc::export!(Server with_types_in wasi_bindings::wrpc);
