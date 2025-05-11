//! # Wasmgrid CLI

use std::sync::LazyLock;

use async_nats::ConnectOptions;
use dotenv::dotenv;
use runtime::{Cli, Parser};
use services::{Resources, http, keyvalue, messaging, rpc};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

// const DEF_MGO_CNN: &str = "mongodb://localhost:27017";
// env::var("MGO_CNN").unwrap_or_else(|_| DEF_MGO_CNN.into()),

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
            rt.link(&messaging::Service)?;

            // TODO: load all required resources (lazy instantiate?)
            let resources = Resources {
                nats_client: LazyLock::new(||ConnectOptions::new().connect("demo.nats.io").await.unwrap()),
            };
            // tokio::spawn(async move {
            //     let client = ConnectOptions::new().connect("demo.nats.io").await.unwrap();
            //     resources.nats_client.set(client).unwrap();
            // });
            resources.nats_client.get().unwrap();

            // let mut opts = ClientOptions::parse(&self.addr).await?;
            // opts.app_name = Some("Credibil Grid".into());
            // let client = Client::with_options(opts)?;
            // // redact password from connection string
            // let mut redacted = url::Url::parse(&self.addr).unwrap();
            // redacted.set_password(Some("*****")).map_err(|()| anyhow!("issue redacting password"))?;
            // tracing::info!("connected to: {redacted}");
            // MONGODB.set(client).map_err(|_| anyhow!("MongoDB already initialized"))

            // start `Runnable` services (servers)
            rt.run(http::Service, resources.clone())?;
            rt.run(rpc::Service, resources.clone())?;
            rt.run(messaging::Service, resources)?;

            rt.shutdown().await
        }
    }
}
