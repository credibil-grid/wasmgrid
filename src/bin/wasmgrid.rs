//! # Wasmgrid CLI

use dotenv::dotenv;
use runtime::{Cli, Parser};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasmgrid::{http, keyvalue};

#[tokio::main]
pub async fn main() -> wasmtime::Result<()> {
    if cfg!(debug_assertions) {
        dotenv().ok();
    }
    let subscriber =
        FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
    tracing::subscriber::set_global_default(subscriber)?;

    match Cli::parse().command {
        runtime::Command::Compile { wasm, output } => {
            runtime::compile(&wasm, output)?;
            return Ok(());
        }
        runtime::Command::Run { wasm, compile } => {
            let compiled = if compile { runtime::compile(&wasm, None)? } else { wasm };

            let mut rt = runtime::Runtime::new(compiled)?;

            if cfg!(feature = "http") {
                rt.link(&http::Service)?;
            }
            if cfg!(feature = "keyvalue") {
                rt.link(&keyvalue::Service)?;
            }

            let client = async_nats::ConnectOptions::new().connect("demo.nats.io").await.unwrap();

            if cfg!(feature = "http") {
                rt.run(http::Service, client)?;
            }

            rt.shutdown().await
        }
    }
}
