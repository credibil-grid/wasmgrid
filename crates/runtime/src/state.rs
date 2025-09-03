//! # WebAssembly Runtime

use tokio::io;
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

/// `RunState` is used to share host state between the Wasm runtime and hosts
/// each time they are instantiated.
pub struct RunState {
    pub wasi_ctx: WasiCtx,
    pub table: ResourceTable,
    pub http_ctx: WasiHttpCtx,
}

impl Default for RunState {
    fn default() -> Self {
        Self::new()
    }
}

impl RunState {
    /// Create a new [`RunState`] instance.
    #[must_use]
    pub fn new() -> Self {
        let mut ctx = WasiCtxBuilder::new();
        ctx.inherit_args();
        ctx.inherit_env();
        ctx.inherit_stdin();
        ctx.stdout(io::stdout());
        ctx.stderr(io::stderr());

        Self {
            table: ResourceTable::default(),
            wasi_ctx: ctx.build(),
            http_ctx: WasiHttpCtx::new(),
        }
    }
}

impl WasiView for RunState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi_ctx,
            table: &mut self.table,
        }
    }
}

impl WasiHttpView for RunState {
    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.http_ctx
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}
