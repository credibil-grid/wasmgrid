//! # WASI Http Capability
//!
//! This module implements a runtime capability for `wasi:http`
//! (<https://github.com/WebAssembly/wasi-http>).

use anyhow::anyhow;
use http_body_util::BodyExt;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Request;
use tokio::net::TcpListener;
use wasmtime::component::Linker;
use wasmtime_wasi::{ResourceTable, WasiView};
use wasmtime_wasi_http::body::HyperOutgoingBody;
use wasmtime_wasi_http::io::TokioIo;
use wasmtime_wasi_http::proxy::Proxy;
use wasmtime_wasi_http::{hyper_response_error, proxy, WasiHttpCtx, WasiHttpView};

use crate::runtime::{self, Runtime, State};

pub struct Capability {
    pub addr: String,
}

pub const fn new(addr: String) -> Capability {
    Capability { addr }
}

#[async_trait::async_trait]
impl runtime::Capability for Capability {
    fn namespace(&self) -> &str {
        "wasi:http"
    }

    fn add_to_linker(&self, linker: &mut Linker<State>) -> anyhow::Result<()> {
        proxy::add_only_http_to_linker(linker)
    }

    /// Provide http proxy capability the specified wasm component.
    async fn run(&self, runtime: Runtime) -> anyhow::Result<()> {
        let listener = TcpListener::bind(&self.addr).await?;
        tracing::info!("listening for http requests on: {}", listener.local_addr()?);

        // listen for requests until terminated
        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let runtime = runtime.clone();

            tokio::spawn(async move {
                if let Err(e) = http1::Builder::new()
                    .keep_alive(true)
                    .serve_connection(io, service_fn(|req| handle_request(&runtime, req)))
                    .await
                {
                    tracing::error!("connection error: {e:?}");
                }
            });
        }
    }
}

// Forward NATS message to the wasm Guest.
async fn handle_request(
    runtime: &Runtime, mut request: Request<Incoming>,
) -> anyhow::Result<hyper::Response<HyperOutgoingBody>> {
    let (sender, receiver) = tokio::sync::oneshot::channel();
    // let req_id = self.next_id.fetch_add(1, Ordering::Relaxed);

    let runtime = runtime.clone();

    let task = tokio::spawn(async move {
        let mut builder = http::Uri::builder();

        // extract scheme and authority from headers
        if let Some(forwarded) = request.headers().get("forwarded") {
            // running behind a proxy (that we have configured)
            for tuple in forwarded.to_str().unwrap().split(';') {
                let tuple = tuple.trim();

                if tuple.starts_with("host=") {
                    let host = tuple.split('=').nth(1).unwrap();
                    builder = builder.authority(host);
                    continue;
                }

                if tuple.starts_with("proto=") {
                    let proto = tuple.split('=').nth(1).unwrap();
                    builder = builder.scheme(proto);
                    continue;
                }
            }
        } else {
            // must be running locally
            let Some(host) = request.headers().get("host") else {
                return Err(anyhow!("host is missing"));
            };
            builder = builder.authority(host.to_str().unwrap());
            builder = builder.scheme("http");
        }

        // update the uri with the new scheme and authority
        let uri = request.uri_mut();
        let Ok(uri) = builder.path_and_query(uri.path_and_query().unwrap().clone()).build() else {
            return Err(anyhow!("failed to build uri"));
        };
        let (mut parts, body) = request.into_parts();
        parts.uri = uri;

        let req = hyper::Request::from_parts(parts, body.map_err(hyper_response_error).boxed());

        let mut store = runtime.new_store();
        store.data_mut().metadata.insert("wasi_http_ctx".to_string(), Box::new(WasiHttpCtx {}));
        let incoming = store.data_mut().new_incoming_request(req)?;
        let outgoing = store.data_mut().new_response_outparam(sender)?;

        // call guest with request
        tracing::debug!("calling guest with request");
        let (proxy, _) = Proxy::instantiate_pre(&mut store, runtime.instance_pre()).await?;
        proxy.wasi_http_incoming_handler().call_handle(&mut store, incoming, outgoing).await
    });

    match receiver.await {
        Ok(Ok(resp)) => Ok(resp),
        Ok(Err(e)) => Err(e.into()),
        Err(_) => {
            // retrieve the inner task error
            let e = match task.await {
                Ok(Err(e)) => e,
                Ok(Ok(())) => anyhow!("task failed without error"),
                Err(e) => e.into(),
            };
            Err(anyhow!("guest did not invoke `response-outparam::set`: {e:?}"))
        }
    }
}

impl WasiHttpView for State {
    fn table(&mut self) -> &mut ResourceTable {
        WasiView::table(self)
    }

    fn ctx(&mut self) -> &mut WasiHttpCtx {
        self.metadata.get_mut("wasi_http_ctx").unwrap().downcast_mut::<WasiHttpCtx>().unwrap()
    }
}
