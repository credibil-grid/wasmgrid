//! # WASI Http Service
//!
//! This module implements a runtime service for `wasi:http`
//! (<https://github.com/WebAssembly/wasi-http>).

use std::clone::Clone;
use std::env;

use anyhow::{Result, anyhow};
use futures::future::{BoxFuture, FutureExt};
use http::uri::{PathAndQuery, Uri};
use hyper::body::Incoming;
use hyper::header::{FORWARDED, HOST};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use runtime::RunState;
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tracing::{Instrument, info_span};
use wasmtime::Store;
use wasmtime::component::{InstancePre, Linker};
use wasmtime_wasi_http::WasiHttpView;
use wasmtime_wasi_http::bindings::ProxyPre;
use wasmtime_wasi_http::bindings::http::types::Scheme;
use wasmtime_wasi_http::body::HyperOutgoingBody;
use wasmtime_wasi_http::io::TokioIo;

const DEF_HTTP_ADDR: &str = "0.0.0.0:8080";

#[derive(Debug)]
pub struct Http;

impl runtime::Service for Http {
    fn add_to_linker(&self, l: &mut Linker<RunState>) -> Result<()> {
        wasmtime_wasi_http::add_only_http_to_linker_async(l)
    }

    /// Provide http proxy service the specified wasm component.
    fn start(&self, pre: InstancePre<RunState>) -> BoxFuture<'static, Result<()>> {
        Self::run(pre).boxed()
    }
}

impl Http {
    /// Provide http proxy service the specified wasm component.
    async fn run(pre: InstancePre<RunState>) -> Result<()> {
        // bail if server is not required
        let component_type = pre.component().component_type();
        let mut exports = component_type.imports(pre.engine());
        if !exports.any(|e| e.0.starts_with("wasi:http")) {
            tracing::debug!("http server not required");
            return Ok(());
        }

        let addr = env::var("HTTP_ADDR").unwrap_or_else(|_| DEF_HTTP_ADDR.into());
        let listener = TcpListener::bind(&addr).await?;
        tracing::info!("http server listening on: {}", listener.local_addr()?);

        let handler = Handler {
            proxy_pre: ProxyPre::new(pre.clone())?,
        };

        // listen for requests until terminated
        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let handler = handler.clone();

            tokio::spawn(async move {
                let mut http1 = http1::Builder::new();
                http1.keep_alive(true);

                if let Err(e) = http1
                    .serve_connection(
                        io,
                        service_fn(|request| {
                            handler.handle(request).instrument(info_span!("http-request"))
                        }),
                    )
                    .await
                {
                    tracing::error!("connection error: {e:?}");
                }
            });
        }
    }
}

#[derive(Clone)]
struct Handler {
    proxy_pre: ProxyPre<RunState>,
}

impl Handler {
    // Forward request to the wasm Guest.
    async fn handle(&self, request: Request<Incoming>) -> Result<Response<HyperOutgoingBody>> {
        tracing::debug!("handling request: {request:?}");

        // prepare wasmtime http request and response
        let mut store = Store::new(self.proxy_pre.engine(), RunState::new());

        let (request, scheme) = prepare_request(request)?;
        tracing::trace!("sending request: {request:#?}");

        let (sender, receiver) = oneshot::channel();
        let incoming = store.data_mut().new_incoming_request(scheme, request)?;
        let outgoing = store.data_mut().new_response_outparam(sender)?;

        // call guest with request
        let proxy = self.proxy_pre.instantiate_async(&mut store).await?;
        let task =
            proxy.wasi_http_incoming_handler().call_handle(&mut store, incoming, outgoing).await;

        match receiver.await {
            Ok(Ok(resp)) => {
                tracing::debug!("request success: {resp:?}");
                Ok(resp)
            }
            Ok(Err(e)) => {
                tracing::debug!("request error: {e:?}");
                Err(e.into())
            }
            Err(_) => {
                let e = match task {
                    Err(e) => e,
                    Ok(()) => anyhow!("task failed without error"),
                };
                tracing::debug!("request error: {e:?}");
                Err(anyhow!("guest did not invoke `response-outparam::set`: {e}"))
            }
        }
    }
}

// Prepare the request for the guest.
fn prepare_request(mut request: Request<Incoming>) -> Result<(Request<Incoming>, Scheme)> {
    // let req_id = self.next_id.fetch_add(1, Ordering::Relaxed);

    // rebuild Uri with scheme and authority explicitly set so they are passed to the Guest
    let uri = request.uri_mut();
    let p_and_q = uri.path_and_query().map_or_else(|| PathAndQuery::from_static("/"), Clone::clone);
    let mut uri_builder = Uri::builder().path_and_query(p_and_q);

    if let Some(forwarded) = request.headers().get(FORWARDED) {
        // running behind a proxy (that we have configured)
        for tuple in forwarded.to_str()?.split(';') {
            let tuple = tuple.trim();
            if let Some(host) = tuple.strip_prefix("host=") {
                uri_builder = uri_builder.authority(host);
            } else if let Some(proto) = tuple.strip_prefix("proto=") {
                uri_builder = uri_builder.scheme(proto);
            }
        }
    } else {
        // should be running locally
        let Some(host) = request.headers().get(HOST) else {
            return Err(anyhow!("missing host header"));
        };
        uri_builder = uri_builder.authority(host.to_str()?);
        uri_builder = uri_builder.scheme("http");
    }

    // update the uri with the new scheme and authority
    let (mut parts, body) = request.into_parts();
    parts.uri = uri_builder.build()?;
    let request = hyper::Request::from_parts(parts, body);

    let scheme = match request.uri().scheme_str() {
        Some("http") => Scheme::Http,
        Some("https") => Scheme::Https,
        _ => return Err(anyhow!("unsupported scheme")),
    };

    Ok((request, scheme))
}
