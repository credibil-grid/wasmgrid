//! # WebAssembly Runtime

#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "jsondb")]
pub mod jsondb;
#[cfg(feature = "keyvalue")]
pub mod keyvalue;
#[cfg(feature = "messaging")]
pub mod messaging;
#[cfg(feature = "rpc")]
pub mod rpc;
// pub mod vault;

use std::sync::{Arc, OnceLock};

use mongodb::options::ClientOptions;
use runtime::{Errout, Stdout};
use wasmtime::StoreLimits;
use wasmtime::component::InstancePre;
use wasmtime_wasi::{IoView, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::WasiHttpCtx;

/// Ctx implements messaging host interfaces. In addition, it holds the
/// host-defined state used by the wasm runtime [`Store`].
#[allow(clippy::struct_field_names)]
pub struct Ctx {
    table: ResourceTable,
    wasi_ctx: WasiCtx,
    limits: StoreLimits,
    http_ctx: WasiHttpCtx,
    resources: Resources,
    instance_pre: InstancePre<Ctx>,
}

impl Ctx {
    /// Create a new Ctx instance.
    #[must_use]
    pub fn new(resources: Resources, instance_pre: InstancePre<Self>) -> Self {
        let mut ctx = WasiCtxBuilder::new();
        ctx.inherit_args();
        ctx.inherit_env();
        ctx.inherit_stdin();
        ctx.stdout(Stdout {});
        ctx.stderr(Errout {});

        Self {
            table: ResourceTable::default(),
            wasi_ctx: ctx.build(),
            limits: StoreLimits::default(),
            http_ctx: WasiHttpCtx::new(),
            resources,
            instance_pre,
        }
    }
}

impl IoView for Ctx {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

// Implement the [`wasmtime_wasi::ctx::WasiView`] trait for Ctx.
impl WasiView for Ctx {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

#[derive(Clone)]
pub struct Resources {
    #[cfg(any(feature = "keyvalue", feature = "messaging", feature = "rpc"))]
    pub nats_client: Arc<OnceLock<async_nats::Client>>,
    #[cfg(feature = "jsondb")]
    pub mgo_client: Arc<OnceLock<mongodb::Client>>,
}

use async_nats::ConnectOptions;

impl Resources {
    pub fn new() -> Self {
        Self {
            #[cfg(any(feature = "keyvalue", feature = "messaging", feature = "rpc"))]
            nats_client: Arc::new(OnceLock::new()),
            #[cfg(feature = "jsondb")]
            mgo_client: Arc::new(OnceLock::new()),
        }
    }

    #[cfg(any(feature = "keyvalue", feature = "messaging", feature = "rpc"))]
    pub fn with_nats(&self, addr: impl Into<String>, opts: ConnectOptions) {
        let res = self.clone();
        let addr = addr.into();
        tokio::spawn(async move {
            let client = opts.connect(addr).await.expect("should connect to nats");
            res.nats_client.set(client).unwrap();
        });
    }

    #[cfg(feature = "jsondb")]
    pub fn with_mongo(&self, opts: ClientOptions) {
        let res = self.clone();
        tokio::spawn(async move {
            let client = mongodb::Client::with_options(opts).expect("should connect to mongodb");
            res.mgo_client.set(client).unwrap();

            // // redact password from connection string
            // let mut redacted = url::Url::parse(&self.addr).unwrap();
            // redacted.set_password(Some("*****")).map_err(|()| anyhow!("issue redacting password"))?;
            // tracing::info!("connected to: {redacted}");
            // MONGODB.set(client).map_err(|_| anyhow!("MongoDB already initialized"))
        });
    }
}
