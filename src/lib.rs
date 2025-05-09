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

use runtime::{Errout, Stdout};
use wasmtime::StoreLimits;
use wasmtime_wasi::{IoView, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

pub type Metadata = Box<dyn Any + Send>;

/// Ctx implements messaging host interfaces. In addition, it holds the
/// host-defined state used by the wasm runtime [`Store`].
#[allow(clippy::struct_field_names)]
pub struct Ctx {
    table: ResourceTable,
    wasi_ctx: WasiCtx,
    pub limits: StoreLimits,
    pub data: HashMap<String, Metadata>,
}

impl Ctx {
    /// Create a new Ctx instance.
    #[must_use]
    pub fn new() -> Self {
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
            data: HashMap::default(),
        }
    }
}

impl Default for Ctx {
    fn default() -> Self {
        Self::new()
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
