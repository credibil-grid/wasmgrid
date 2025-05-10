//! # WebAssembly Runtime

pub mod http;
pub mod keyvalue;
pub mod rpc;
// pub mod jsondb;
pub mod messaging;
// pub mod vault;

use std::any::Any;
use std::collections::HashMap;

use anyhow::{Result, anyhow};
use async_nats::Client;
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
    nats_client: Client,
    http_ctx: WasiHttpCtx,
    instance_pre: InstancePre<Ctx>,
    // resources: Resources,
}

impl Ctx {
    /// Create a new Ctx instance.
    #[must_use]
    pub fn new(nats_client: Client, instance_pre: InstancePre<Self>) -> Self {
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
            nats_client,
            http_ctx: WasiHttpCtx::new(),
            instance_pre,
            // resources: Resources::new(),
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

pub struct Resources {
    table: HashMap<String, Box<dyn Any + Send + Sync>>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn insert<T: Send + Sync + 'static>(&mut self, key: &str, value: T) {
        self.table.insert(key.to_string(), Box::new(value));
    }

    /// Get an immutable reference to a resource of a given type for a
    /// given key.
    pub fn get<T: Any + Sized>(&self, key: &str) -> Result<&T> {
        self.table
            .get(key)
            .ok_or(anyhow!("no value for {key}"))?
            .downcast_ref()
            .ok_or(anyhow!("failed to downcast"))
    }
}
