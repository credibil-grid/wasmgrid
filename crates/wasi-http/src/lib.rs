//! # WebAssembly Runtime

pub mod http;

pub use resources::Resources;
use runtime::{Errout, Stdout};
use wasmtime::StoreLimits;
use wasmtime::component::InstancePre;
use wasmtime_wasi::ResourceTable;
use wasmtime_wasi::p2::{IoView, WasiCtx, WasiCtxBuilder, WasiView};
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
