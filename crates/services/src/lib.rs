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

use anyhow::{Result, anyhow};
use async_nats::{AuthError, ConnectOptions};
use runtime::{Errout, Stdout};
use tokio::task::JoinHandle;
use wasmtime::StoreLimits;
use wasmtime::component::InstancePre;
use wasmtime_wasi::{IoView, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::WasiHttpCtx;

/// Ctx implements messaging host interfaces. In addition, it holds the
/// host-defined state used by the wasm runtime [`Store`].
#[allow(clippy::struct_field_names)]
#[allow(dead_code)]
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

impl Resources {
    #[must_use]
    pub fn new() -> Self {
        Self {
            #[cfg(any(feature = "keyvalue", feature = "messaging", feature = "rpc"))]
            nats_client: Arc::new(OnceLock::new()),
            #[cfg(feature = "jsondb")]
            mgo_client: Arc::new(OnceLock::new()),
        }
    }

    /// Add a NATS connection using the given address and, optionally,
    /// authenticating using a NATS `nkeys` JWT and seed.
    ///
    /// The method will attempt connect on a separate, returning a
    /// [`tokio::task::JoinHandle`] that can be awaited if desired.
    #[cfg(any(feature = "keyvalue", feature = "messaging", feature = "rpc"))]
    pub fn with_nats(
        &self, addr: impl Into<String> + Send + 'static, jwt: Option<String>, seed: Option<String>,
    ) -> JoinHandle<Result<()>> {
        let resources = self.clone();
        tokio::spawn(async move {
            let opts = if let Some(jwt) = jwt {
                let Ok(key_pair) = nkeys::KeyPair::from_seed(&seed.unwrap_or_default()) else {
                    tracing::error!("failed to create nats KeyPair");
                    return Err(anyhow!("failed to create nats KeyPair"));
                };
                let key_pair = Arc::new(key_pair);
                ConnectOptions::with_jwt(jwt, move |nonce| {
                    let key_pair = key_pair.clone();
                    async move { key_pair.sign(&nonce).map_err(AuthError::new) }
                })
            } else {
                ConnectOptions::new()
            };

            let Ok(client) = opts.connect(addr.into()).await else {
                tracing::error!("failed to connect to nats");
                return Err(anyhow!("failed to connect to nats"));
            };
            resources.nats_client.set(client).map_err(|_| anyhow!("failed to set nats client"))
        })
    }

    /// Add a MongoDB connection from a `mongodb` uri.
    ///
    /// The method will attempt connect on a separate, returning a
    /// [`tokio::task::JoinHandle`] that can be awaited if desired.
    #[cfg(feature = "jsondb")]
    pub fn with_mongo(&self, uri: impl AsRef<str> + Send + 'static) -> JoinHandle<Result<()>> {
        let resources = self.clone();
        tokio::spawn(async move {
            let Ok(client) = mongodb::Client::with_uri_str(uri).await else {
                tracing::error!("failed to connect to mongo");
                return Err(anyhow!("failed to connect to mongo"));
            };
            resources.mgo_client.set(client).map_err(|_| anyhow!("failed to set mongo client"))
        })
    }
}

impl Default for Resources {
    fn default() -> Self {
        Self::new()
    }
}
