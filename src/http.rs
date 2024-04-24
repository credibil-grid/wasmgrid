//! # Http System
//!
//! This module implements a wasi:http runtime.

use anyhow::anyhow;
use http_body_util::BodyExt;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Request;
use tokio::net::TcpListener;
use wasmtime::component::Linker;
use wasmtime_wasi::WasiView;
use wasmtime_wasi_http::body::HyperOutgoingBody;
use wasmtime_wasi_http::io::TokioIo;
use wasmtime_wasi_http::proxy::Proxy;
use wasmtime_wasi_http::{hyper_response_error, proxy, WasiHttpCtx, WasiHttpView};

use crate::system::{self, State, System};

pub struct Runtime {
    pub addr: String,
}

impl Runtime {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
}

#[async_trait::async_trait]
impl system::Runtime for Runtime {
    fn add_to_linker(&self, linker: &mut Linker<State>) -> anyhow::Result<()> {
        proxy::add_only_http_to_linker(linker)
    }

    /// Start and run NATS for the specified wasm component.
    async fn run(&self, system: System) -> anyhow::Result<()> {
        let listener = TcpListener::bind(&self.addr).await?;
        println!("Listening on: {}", listener.local_addr()?);

        // listen for requests until terminated
        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let system = system.clone();

            tokio::spawn(async move {
                if let Err(e) = http1::Builder::new()
                    .keep_alive(true)
                    .serve_connection(io, service_fn(|req| handle_request(&system, req)))
                    .await
                {
                    eprintln!("error: {e:?}");
                }
            });
        }

        // Ok(())
    }
}

// Forward NATS message to the wasm Guest.
async fn handle_request(
    system: &System, request: Request<Incoming>,
) -> anyhow::Result<hyper::Response<HyperOutgoingBody>> {
    let (sender, receiver) = tokio::sync::oneshot::channel();
    // let req_id = self.next_id.fetch_add(1, Ordering::Relaxed);

    let system = system.clone();
    
    let task = tokio::spawn(async move {
        let mut store = system.store();

        let (parts, body) = request.into_parts();
        let req = hyper::Request::from_parts(parts, body.map_err(hyper_response_error).boxed());

        let req = store.data_mut().new_incoming_request(req)?;
        let out = store.data_mut().new_response_outparam(sender)?;

        let (proxy, _) = Proxy::instantiate_pre(&mut store, system.instance_pre()).await?;

        // call guest with request
        proxy.wasi_http_incoming_handler().call_handle(&mut store, req, out).await
    });

    match receiver.await {
        Ok(Ok(resp)) => Ok(resp),
        Ok(Err(e)) => Err(e.into()),
        Err(_) => {
            // An error in the receiver (`RecvError`) only indicates that the
            // task exited before a response was sent (i.e., the sender was
            // dropped); it does not describe the underlying cause of failure.
            // Instead we retrieve and propagate the error from inside the task
            // which should more clearly tell the user what went wrong. Note
            // that we assume the task has already exited at this point so the
            // `await` should resolve immediately.
            let e = match task.await {
                Ok(r) => r.expect_err("if the receiver has an error, the task must have failed"),
                Err(e) => e.into(),
            };

            Err(anyhow!("guest never invoked `response-outparam::set` method: {e:?}"))
        }
    }
}

impl WasiHttpView for State {
    fn table(&mut self) -> &mut wasmtime::component::ResourceTable {
        WasiView::table(self)
    }

    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.http_ctx
    }
}
