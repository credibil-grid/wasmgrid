pub mod http;
pub mod keyvalue;
pub mod rpc;
// pub mod jsondb;
// pub mod messaging;
// pub mod vault;

use runtime::{Errout, Stdout};
use wasmtime::StoreLimits;
use wasmtime_wasi::{IoView, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::WasiHttpCtx;

pub struct Resources {
    pub nats_client: async_nats::Client,
}

/// Ctx implements messaging host interfaces. In addition, it holds the
/// host-defined state used by the wasm runtime [`Store`].
pub struct Ctx {
    table: ResourceTable,
    wasi_ctx: WasiCtx,
    limits: StoreLimits,
    nats_client: async_nats::Client,
    http_ctx: WasiHttpCtx,
}

impl Ctx {
    /// Create a new Ctx instance.
    #[must_use]
    pub async fn new(nats_client: async_nats::Client) -> Self {
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
