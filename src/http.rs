//! # Http Runtime
//!
//! This module implements a wasi:http runtime.

use hyper::server::conn::http1;
use hyper::service::service_fn;
use tokio::net::TcpListener;
use wasmtime_wasi_http::io::TokioIo;

use crate::handler;

/// Start and run NATS for the specified wasm component.
pub async fn serve(handler: handler::HandlerProxy, addr: String) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on: {}", listener.local_addr()?);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let handler = handler.clone();

        tokio::spawn(async move {
            if let Err(e) = http1::Builder::new()
                .keep_alive(true)
                .serve_connection(io, service_fn(|req| handler.clone().request(req)))
                .await
            {
                eprintln!("error: {e:?}");
            }
        });
    }

    Ok(())
}
