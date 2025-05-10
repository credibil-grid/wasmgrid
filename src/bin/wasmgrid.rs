//! # Wasmgrid CLI

use async_nats::ConnectOptions;
use dotenv::dotenv;
use runtime::{Cli, Parser};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasmgrid::{http, keyvalue, rpc};

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

            // link services
            rt.link(&http::Service)?;
            rt.link(&rpc::Service)?;
            rt.link(&keyvalue::Service)?;

            // TODO: load all required resources (lazy instantiate?)
            let resources = ConnectOptions::new().connect("demo.nats.io").await?;

            // start `Runnable` services (servers)
            rt.run(http::Service, resources.clone())?;
            rt.run(rpc::Service, resources)?;

            rt.shutdown().await
        }
    }
}
