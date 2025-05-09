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
            // let ctx = wasmgrid::Ctx::new().await;
            let mut rt = runtime::Runtime::new(wasm, compile)?;

            if cfg!(feature = "http") {
                let http = http::new();
                rt.link(&http)?;
            }
            if cfg!(feature = "keyvalue") {
                let keyvalue = keyvalue::new();
                rt.link(&keyvalue)?;
            }

            rt.instantiate().await?;

            let client = async_nats::ConnectOptions::new().connect("demo.nats.io").await.unwrap();

            if cfg!(feature = "http") {
                let http = http::new();
                rt.run(http, client)?;
            }

            // wait for shutdown signal
            rt.shutdown().await
        }
    }
}
