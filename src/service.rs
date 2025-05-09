//! # Service
//!
//! This module contains runtime service providers. Each service is a module
//! that provides a concrete implementation in support of a specific set of WASI
//! interfaces.

#[cfg(feature = "http")]
pub mod http;
// #[cfg(feature = "jsondb")]
// pub mod jsondb;
#[cfg(feature = "keyvalue")]
pub mod keyvalue;
// #[cfg(feature = "messaging")]
// pub mod messaging;
// #[cfg(feature = "rpc")]
// pub mod rpc;
// #[cfg(feature = "vault")]
// pub mod vault;

use std::any::Any;
use std::collections::HashMap;

use anyhow::Result;
use wasmtime::StoreLimits;
use wasmtime::component::{InstancePre, Linker};
use wasmtime_wasi::{IoView, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

// use wasmtime_wasi_http::WasiHttpView;
use crate::trace::{Errout, Stdout};

pub type Metadata = Box<dyn Any + Send>;

/// Service represents a particular runtime service depended on by wasm
/// components. For example, an HTTP server or a message broker.
pub trait Service: Sync + Send {
    type Ctx: IoView + WasiView;

    /// Returns the wasi namespace the service supports. For example, `wasi:http`.
    fn namespace(&self) -> &'static str;

    /// Add the service to the wasm component linker.
    ///
    /// # Errors
    ///
    /// Returns an error if the service encounters an error adding generated
    /// bindings to the linker.
    fn add_to_linker(&self, linker: &mut Linker<Self::Ctx>) -> Result<()>;

    /// Start and run the runtime.
    fn start(&self, pre: InstancePre<Self::Ctx>) -> impl Future<Output = Result<()>> + Send;
}

/// Ctx implements messaging host interfaces. In addition, it holds the
/// host-defined state used by the wasm runtime [`Store`].
#[allow(clippy::struct_field_names)]
pub struct Ctx {
    table: ResourceTable,
    ctx: WasiCtx,
    pub limits: StoreLimits,
    pub metadata: HashMap<String, Metadata>,
}

impl Default for Ctx {
    fn default() -> Self {
        let mut ctx = WasiCtxBuilder::new();
        ctx.inherit_args();
        ctx.inherit_env();
        ctx.inherit_stdin();
        ctx.stdout(Stdout {});
        ctx.stderr(Errout {});

        Self {
            table: ResourceTable::default(),
            ctx: ctx.build(),
            limits: StoreLimits::default(),

            // TODO: wrap Hashmap in custom type to create accessors
            metadata: HashMap::default(),
        }
    }
}

impl Ctx {
    /// Create a new Ctx instance.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
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
        &mut self.ctx
    }
}
