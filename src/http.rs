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
use hyper::{Method, Request, StatusCode};
use tokio::net::TcpListener;
use wasmtime::Store;
use wasmtime::component::{InstancePre, Linker};
use wasmtime_wasi_http::bindings::ProxyPre;
use wasmtime_wasi_http::bindings::http::types::Scheme;
use wasmtime_wasi_http::body::HyperOutgoingBody;
use wasmtime_wasi_http::io::TokioIo;
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

use crate::Ctx;

const DEF_HTTP_ADDR: &str = "0.0.0.0:8080";

pub struct Service {
    pub addr: String,
}

#[must_use]
pub fn new() -> Service {
    let addr = env::var("HTTP_ADDR").unwrap_or_else(|_| DEF_HTTP_ADDR.into());
    Service { addr }
}

impl runtime::Service for Service {
    type Ctx = Ctx;

    fn namespace(&self) -> &'static str {
        "wasi:http"
    }

    fn add_to_linker(&self, linker: &mut Linker<Self::Ctx>) -> Result<()> {
        wasmtime_wasi_http::add_only_http_to_linker_async(linker)
    }
}

impl runtime::Instantiator for Service {
    /// Provide http proxy service the specified wasm component.
    async fn run(&self, pre: InstancePre<Ctx>) -> Result<()> {
        let listener = TcpListener::bind(&self.addr).await?;
        tracing::info!("http server listening on: {}", listener.local_addr()?);

        // listen for requests until terminated
        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let proxy_pre = ProxyPre::new(pre.clone())?;

            tokio::spawn(async move {
                if let Err(e) = http1::Builder::new()
                    .keep_alive(true)
                    .serve_connection(io, service_fn(|req| handle_request(proxy_pre.clone(), req)))
                    .await
                {
                    tracing::error!("connection error: {e:?}");
                }
            });
        }
    }
}

// Forward request to the wasm Guest.
async fn handle_request(
    proxy_pre: ProxyPre<Ctx>, mut request: Request<Incoming>,
) -> Result<hyper::Response<HyperOutgoingBody>> {
    let (sender, receiver) = tokio::sync::oneshot::channel();

    // HACK: CORS preflight request for use when testing locally
    let cors = env::var("WITH_CORS").is_ok_and(|val| val.parse().unwrap_or(false));
    if cors && request.method() == Method::OPTIONS {
        let resp = hyper::Response::builder()
            .status(StatusCode::OK)
            .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .header(ACCESS_CONTROL_ALLOW_HEADERS, "*")
            .header(ACCESS_CONTROL_ALLOW_METHODS, "DELETE, GET, OPTIONS, PATCH, POST, PUT")
            // .header(CONTENT_TYPE, "application/json")
            .body(HyperOutgoingBody::default())?;
        return Ok(resp);
    }

    // let req_id = self.next_id.fetch_add(1, Ordering::Relaxed);

    let task = tokio::spawn(async move {
        // rebuild Uri with scheme and authority explicitly set so they are passed to the Guest
        let uri = request.uri_mut();
        let p_and_q =
            uri.path_and_query().map_or_else(|| PathAndQuery::from_static("/"), Clone::clone);
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

        tracing::debug!("calling guest with request: {parts:?}");
        let req = hyper::Request::from_parts(parts, body);

        let scheme = match req.uri().scheme_str() {
            Some("http") => Scheme::Http,
            Some("https") => Scheme::Https,
            _ => return Err(anyhow!("unsupported scheme")),
        };

        // prepare wasmtime http request and response
        let mut store = Store::new(proxy_pre.engine(), Ctx::new().await);
        store.limiter(|t| &mut t.limits);

        store.data_mut().data.insert("wasi_http_ctx".into(), Box::new(WasiHttpCtx::new()));
        let incoming = store.data_mut().new_incoming_request(scheme, req)?;
        let outgoing = store.data_mut().new_response_outparam(sender)?;

        // call guest with request
        let proxy = proxy_pre.instantiate_async(&mut store).await?;
        proxy.wasi_http_incoming_handler().call_handle(&mut store, incoming, outgoing).await
    });

    match receiver.await {
        Ok(Ok(mut resp)) => {
            if cors {
                resp.headers_mut()
                    .insert(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
            }
            Ok(resp)
        }
        Ok(Err(e)) => Err(e.into()),
        Err(_) => {
            // retrieve the inner task error
            let e = match task.await {
                Ok(Err(e)) => e,
                Ok(Ok(())) => anyhow!("task failed without error"),
                Err(e) => e.into(),
            };
            Err(anyhow!("guest did not invoke `response-outparam::set`: {e}"))
        }
    }
}

impl WasiHttpView for Ctx {
    fn ctx(&mut self) -> &mut WasiHttpCtx {
        self.data.get_mut("wasi_http_ctx").unwrap().downcast_mut::<WasiHttpCtx>().unwrap()
    }
}
