//! # NATS Messaging Runtime
//!
//! This module implements a NATS wasi:messaging runtime.

// use std::net::SocketAddr;
// use std::str::FromStr;

// use std::sync::Arc;

// use std::sync::atomic::{AtomicU64}, Ordering};
use anyhow::anyhow;
use http_body_util::BodyExt;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Request;
use tokio::net::TcpListener;
use wasmtime::component::{Component, InstancePre, Linker, ResourceTable};
use wasmtime::{Engine, Store, StoreLimits}; // StoreLimitsBuilder
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::body::HyperOutgoingBody;
use wasmtime_wasi_http::io::TokioIo;
use wasmtime_wasi_http::proxy::{self, Proxy};
use wasmtime_wasi_http::{hyper_response_error, WasiHttpCtx, WasiHttpView};

/// Start and run http server for the specified wasm component.
pub async fn serve(engine: Engine, addr: String, wasm: String) -> anyhow::Result<()> {
    let handler = HandlerProxy::new(engine.clone(), wasm)?;
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
        wasmtime_wasi::add_to_linker_async(&mut linker)?;
        proxy::add_only_http_to_linker(&mut linker)?;

        let component = Component::from_file(&engine, wasm)?;
        let instance_pre = linker.instantiate_pre(&component).expect("should instantiate");

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
            store.limiter(|t| &mut t.limits);

            let (parts, body) = request.into_parts();
            let req = hyper::Request::from_parts(parts, body.map_err(hyper_response_error).boxed());

            let req = store.data_mut().new_incoming_request(req)?;
            let out = store.data_mut().new_response_outparam(sender)?;

            let (proxy, _) = Proxy::instantiate_pre(&mut store, &instance_pre).await?;

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
                    Ok(r) => {
                        r.expect_err("if the receiver has an error, the task must have failed")
                    }
                    Err(e) => e.into(),
                };

                Err(anyhow!("guest never invoked `response-outparam::set` method: {e:?}"))
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
