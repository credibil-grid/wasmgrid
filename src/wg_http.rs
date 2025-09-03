use anyhow::Result;
use dotenv::dotenv;
use runtime::{Cli, Command, Parser, Runtime};
use wasi_http::Http;
use wasi_otel::Otel;

#[tokio::main]
async fn main() -> Result<()> {
    if cfg!(debug_assertions) {
        dotenv().ok();
    }
    match Cli::parse().command {
        Command::Run { wasm } => Runtime::new(wasm).register(Otel).register(Http).await,
        #[cfg(feature = "compile")]
        Command::Compile { wasm, output } => runtime::compile(&wasm, output),
    }
}
