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
#[cfg(feature = "vault")]
pub mod vault;

use std::sync::{Arc, OnceLock};
use std::thread::sleep;
use std::time::Duration;

use anyhow::{Result, anyhow};
use async_nats::{AuthError, ConnectOptions};
#[cfg(feature = "vault")]
use azure_identity::DefaultAzureCredential;
#[cfg(feature = "vault")]
use azure_security_keyvault_keys::KeyClient;
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
    instance_pre: InstancePre<Ctx>,
    resources: Resources,
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
            instance_pre,
            resources,
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
    nats: Arc<OnceLock<async_nats::Client>>,
    #[cfg(feature = "jsondb")]
    mongo: Arc<OnceLock<mongodb::Client>>,
    #[cfg(feature = "vault")]
    azkeyvault: Arc<OnceLock<KeyClient>>,
}

impl Resources {
    #[must_use]
    pub fn new() -> Self {
        Self {
            #[cfg(any(feature = "keyvalue", feature = "messaging", feature = "rpc"))]
            nats: Arc::new(OnceLock::new()),
            #[cfg(feature = "jsondb")]
            mongo: Arc::new(OnceLock::new()),
            #[cfg(feature = "vault")]
            azkeyvault: Arc::new(OnceLock::new()),
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
            let client = nats_connect(addr.into(), jwt, seed).await.map_err(|e| {
                tracing::error!("failed to connect to nats: {e}");
                anyhow!("failed to connect to nats: {e}")
            })?;
            tracing::info!("connected to nats");
            resources.nats.set(client).map_err(|_| {
                tracing::error!("failed to initialize nats context");
                anyhow!("failed to initialize nats context")
            })
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
            let client = mongodb::Client::with_uri_str(uri).await.map_err(|e| {
                tracing::error!("failed to connect to mongo: {e}");
                anyhow!("failed to connect to mongo: {e}")
            })?;
            tracing::info!("connected to mongo");
            resources.mongo.set(client).map_err(|_| {
                tracing::error!("failed to initialize mongo context");
                anyhow!("failed to initialize mongo context")
            })
        })
    }

    /// Add an `Azure KeyVault` connection.
    ///
    /// The method will attempt connect on a separate, returning a
    /// [`tokio::task::JoinHandle`] that can be awaited if desired.
    #[cfg(feature = "vault")]
    pub fn with_azkeyvault(
        &self, addr: impl AsRef<str> + Send + 'static,
    ) -> JoinHandle<Result<()>> {
        let resources = self.clone();
        tokio::spawn(async move {
            let credential = if cfg!(debug_assertions) {
                DefaultAzureCredential::new()
                    .map_err(|e| anyhow!("could not create credential: {e}"))?
            } else {
                // let credential = ClientSecretCredential::new()?;
                DefaultAzureCredential::new()
                    .map_err(|e| anyhow!("could not create credential: {e}"))?
            };

            let client = KeyClient::new(addr.as_ref(), credential, None).map_err(|e| {
                tracing::error!("failed to connect to azure keyvault: {e}");
                anyhow!("failed to connect to azure keyvault: {e}")
            })?;
            tracing::info!("connected to azure keyvault");
            resources.azkeyvault.set(client).map_err(|_| {
                tracing::error!("failed to initialize mongo context");
                anyhow!("failed to set az keyvault client")
            })
        })
    }

    /// Get the NATS client.
    ///
    /// This method will block until the client is available, timing out after
    /// 100ms.
    ///
    /// # Panics
    ///
    /// This method panics if the client is not available before the method
    /// times out.
    pub(crate) fn nats(&self) -> Result<&async_nats::Client> {
        timeout(&self.nats)
    }

    /// Get the MongoDB client.
    ///
    /// This method will block until the client is available, timing out after
    /// 100ms.
    ///
    /// # Panics
    ///
    /// This method panics if the client is not available before the method
    /// times out.
    pub(crate) fn mongo(&self) -> Result<&mongodb::Client> {
        timeout(&self.mongo)
    }

    /// Get the Azure Keyvault client.
    ///
    /// This method will block until the client is available, timing out after
    /// 100ms.
    ///
    /// # Panics
    ///
    /// This method panics if the client is not available before the method
    /// times out.
    #[cfg(feature = "vault")]
    pub(crate) fn azkeyvault(&self) -> Result<&KeyClient> {
        timeout(&self.azkeyvault)
    }
}

impl Default for Resources {
    fn default() -> Self {
        Self::new()
    }
}

fn timeout<T>(once_lock: &OnceLock<T>) -> Result<&T> {
    for _ in 0..10 {
        if let Some(client) = once_lock.get() {
            return Ok(client);
        }
        sleep(Duration::from_millis(10));
    }
    tracing::error!("failed to get resource");
    Err(anyhow!("failed to get resource"))
}

async fn nats_connect(
    addr: String, jwt: Option<String>, seed: Option<String>,
) -> Result<async_nats::Client> {
    let mut opts = ConnectOptions::new();
    if let Some(jwt) = jwt {
        let key_pair = nkeys::KeyPair::from_seed(&seed.unwrap_or_default())
            .map_err(|e| anyhow!("failed to create KeyPair: {e}"))?;
        let key_pair = Arc::new(key_pair);
        opts = opts.jwt(jwt, move |nonce| {
            let key_pair = key_pair.clone();
            async move { key_pair.sign(&nonce).map_err(AuthError::new) }
        });
    }
    opts.connect(addr).await.map_err(|e| anyhow!("{e}"))
}
