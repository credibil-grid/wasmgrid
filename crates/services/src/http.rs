//! # WASI Http Service
//!
//! This module implements a runtime service for `wasi:http`
//! (<https://github.com/WebAssembly/wasi-http>).

use std::clone::Clone;
use std::env;

use anyhow::{Result, anyhow};
use http::uri::PathAndQuery;
use http::uri::Uri; // Authority,
use hyper::body::Incoming;
use hyper::header::{
    ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
    FORWARDED, HOST, HeaderValue,
};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use runtime::{Linkable, Runnable};
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use wasmtime::Store;
use wasmtime::component::{InstancePre, Linker};
use wasmtime_wasi_http::bindings::ProxyPre;
use wasmtime_wasi_http::bindings::http::types::Scheme;
use wasmtime_wasi_http::body::HyperOutgoingBody;
use wasmtime_wasi_http::io::TokioIo;
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

use crate::{Ctx, Resources};

const DEF_HTTP_ADDR: &str = "0.0.0.0:8080";

pub struct Service;

impl Linkable for Service {
    type Ctx = Ctx;

    fn add_to_linker(&self, linker: &mut Linker<Self::Ctx>) -> Result<()> {
        wasmtime_wasi_http::add_only_http_to_linker_async(linker)?;
        tracing::trace!("added to linker");
        Ok(())
    }
}

impl Runnable for Service {
    type Resources = Resources;

    /// Provide http proxy service the specified wasm component.
    async fn run(&self, pre: InstancePre<Self::Ctx>, resources: Self::Resources) -> Result<()> {
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

        // listen for requests until terminated
        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let proxy_pre = ProxyPre::new(pre.clone())?;
            let res = resources.clone();

            tokio::spawn(async move {
                if let Err(e) = http1::Builder::new()
                    .keep_alive(true)
                    .serve_connection(io, service_fn(|r| handle(proxy_pre.clone(), res.clone(), r)))
                    .await
                {
                    tracing::error!("connection error: {e:?}");
                }
            });
        }
    }
}

// Forward request to the wasm Guest.
async fn handle(
    proxy_pre: ProxyPre<Ctx>, resources: Resources, request: Request<Incoming>,
) -> Result<Response<HyperOutgoingBody>> {
    // HACK: CORS preflight request for use when testing locally
    let cors = env::var("WITH_CORS").is_ok_and(|v| v.parse().unwrap_or(false));
    if cors && request.method() == Method::OPTIONS {
        return handle_options(&request);
    }

    // prepare wasmtime http request and response
    let mut store =
        Store::new(proxy_pre.engine(), Ctx::new(resources, proxy_pre.instance_pre().clone()));
    store.limiter(|t| &mut t.limits);

    let (request, scheme) = prepare_request(request)?;
    tracing::trace!("sending request: {:#?}", request);

    let (sender, receiver) = oneshot::channel();
    let incoming = store.data_mut().new_incoming_request(scheme, request)?;
    let outgoing = store.data_mut().new_response_outparam(sender)?;

    // call guest with request
    let proxy = proxy_pre.instantiate_async(&mut store).await?;
    let task = proxy.wasi_http_incoming_handler().call_handle(&mut store, incoming, outgoing).await;

    match receiver.await {
        Ok(Ok(mut resp)) => {
            tracing::trace!("request successful: {:#?}", resp);
            if cors {
                resp.headers_mut()
                    .insert(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
            }
            Ok(resp)
        }
        Ok(Err(e)) => Err(e.into()),
        Err(_) => {
            let e = match task {
                Err(e) => e,
                Ok(()) => anyhow!("task failed without error"),
            };
            tracing::trace!("error: {:#?}", e);
            Err(anyhow!("guest did not invoke `response-outparam::set`: {e}"))
        }
    }
}

fn handle_options(_: &Request<Incoming>) -> Result<Response<HyperOutgoingBody>> {
    Response::builder()
        .status(StatusCode::OK)
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(ACCESS_CONTROL_ALLOW_HEADERS, "*")
        .header(ACCESS_CONTROL_ALLOW_METHODS, "DELETE, GET, OPTIONS, PATCH, POST, PUT")
        // .header(CONTENT_TYPE, "application/json")
        .body(HyperOutgoingBody::default())
        .map_err(|e| anyhow!("failed to build response: {e}"))
}

// Prepare the request for the guest.
fn prepare_request(mut request: Request<Incoming>) -> Result<(Request<Incoming>, Scheme)> {
    // let req_id = self.next_id.fetch_add(1, Ordering::Relaxed);

    // rebuild Uri with scheme and authority explicitly set so they are passed to the Guest
    let uri = request.uri_mut();
    let p_and_q = uri.path_and_query().map_or_else(|| PathAndQuery::from_static("/"), Clone::clone);
    let mut builder = Uri::builder().path_and_query(p_and_q);

    if let Some(forwarded) = request.headers().get(FORWARDED) {
        // running behind a proxy (that we have configured)
        for tuple in forwarded.to_str()?.split(';') {
            let tuple = tuple.trim();
            if let Some(host) = tuple.strip_prefix("host=") {
                builder = builder.authority(host);
            } else if let Some(proto) = tuple.strip_prefix("proto=") {
                builder = builder.scheme(proto);
            }
        }
    } else {
        // must be running locally
        let Some(host) = request.headers().get(HOST) else {
            return Err(anyhow!("missing host header"));
        };
        builder = builder.authority(host.to_str()?);
        builder = builder.scheme("http");
    }

    // update the uri with the new scheme and authority
    let (mut parts, body) = request.into_parts();
    parts.uri = builder.build()?;

    tracing::debug!("calling guest");
    tracing::trace!("request headers: {:#?}", parts);
    let request = hyper::Request::from_parts(parts, body);

    let scheme = match request.uri().scheme_str() {
        Some("http") => Scheme::Http,
        Some("https") => Scheme::Https,
        _ => return Err(anyhow!("unsupported scheme")),
    };

    Ok((request, scheme))
}

impl WasiHttpView for Ctx {
    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.http_ctx
    }
}
