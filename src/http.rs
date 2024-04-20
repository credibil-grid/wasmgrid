//! # NATS Messaging Runtime
//!
//! This module implements a NATS wasi:messaging runtime.

use std::net::SocketAddr;
use std::str::FromStr;

// use std::sync::Arc;

// use std::sync::atomic::{AtomicU64}, Ordering};
use anyhow::anyhow;
use http_body_util::BodyExt;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Request;
use wasmtime::component::{Component, InstancePre, Linker, ResourceTable};
use wasmtime::{Engine, Store, StoreLimits};
use wasmtime_wasi::{command, WasiCtx, WasiCtxBuilder, WasiView};
// use wasmtime_wasi_http::bindings::http::types as http_types;
use wasmtime_wasi_http::body::HyperOutgoingBody;
use wasmtime_wasi_http::io::TokioIo;
use wasmtime_wasi_http::proxy::Proxy;
use wasmtime_wasi_http::{hyper_response_error, WasiHttpCtx, WasiHttpView};

/// Start and run NATS for the specified wasm component.
pub async fn serve(engine: Engine, wasm: String, host: String) -> anyhow::Result<()> {
    let handler = HandlerProxy::new(engine.clone(), wasm)?;

    let addr = SocketAddr::from_str(&host)?;
    let socket = match &addr {
        SocketAddr::V4(_) => tokio::net::TcpSocket::new_v4()?,
        SocketAddr::V6(_) => tokio::net::TcpSocket::new_v6()?,
    };
    socket.set_reuseaddr(false)?;
    socket.bind(addr)?;
    let listener = socket.listen(100)?;

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

    // Ok(())
}

// HandlerProxy is a proxy for the wasm messaging Host, wrapping calls to the Guest's
// messaging API.
#[derive(Clone)]
struct HandlerProxy {
    engine: Engine,
    instance_pre: InstancePre<Host>,
    // next_id: Arc<AtomicU64>,
}

impl HandlerProxy {
    // Create a new HandlerProxy for the specified wasm Guest.
    fn new(engine: Engine, wasm: String) -> anyhow::Result<Self> {
        let mut linker = Linker::new(&engine);
        command::add_to_linker(&mut linker)?;
        wasmtime_wasi_http::proxy::add_to_linker(&mut linker)?;

        let component = Component::from_file(&engine, wasm)?;
        let instance_pre = linker.instantiate_pre(&component)?;

        Ok(Self {
            engine,
            instance_pre,
            // next_id: Arc::new(AtomicU64::new(0)),
        })
    }

    // Forward NATS message to the wasm Guest.
    async fn request(
        self, request: Request<Incoming>,
    ) -> anyhow::Result<hyper::Response<HyperOutgoingBody>> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let engine = self.engine.clone();
        // let req_id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let instance_pre = self.instance_pre.clone();

        let task = tokio::spawn(async move {
            let mut store = Store::new(&engine, Host::new());

            let (parts, body) = request.into_parts();
            let req = hyper::Request::from_parts(parts, body.map_err(hyper_response_error).boxed());

            let req = store.data_mut().new_incoming_request(req)?;
            let out = store.data_mut().new_response_outparam(sender)?;

            let (proxy, _) = Proxy::instantiate_pre(&mut store, &instance_pre).await?;

            // call guest with request
            if let Err(e) =
                proxy.wasi_http_incoming_handler().call_handle(&mut store, req, out).await
            {
                // log::error!("[{req_id}] :: {:#?}", e);
                return Err(e);
            }

            Ok(())
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
                    Ok(r) => {
                        r.expect_err("if the receiver has an error, the task must have failed")
                    }
                    Err(e) => e.into(),
                };

                return Err(anyhow!("guest never invoked `response-outparam::set` method: {e:?}"));
            }
        }
    }
}

struct Host {
    table: wasmtime::component::ResourceTable,
    ctx: WasiCtx,
    http: WasiHttpCtx,
    limits: StoreLimits,
}

impl Host {
    fn new() -> Self {
        Self {
            table: ResourceTable::default(),
            ctx: WasiCtxBuilder::new().inherit_args().inherit_env().inherit_stdio().build(),
            http: WasiHttpCtx {},
            limits: StoreLimits::default(),
        }
    }
}

impl WasiView for Host {
    fn table(&mut self) -> &mut wasmtime::component::ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

impl WasiHttpView for Host {
    fn table(&mut self) -> &mut wasmtime::component::ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.http
    }
}
