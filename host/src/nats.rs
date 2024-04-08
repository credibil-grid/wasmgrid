mod consumer;
mod producer;
mod types;

// use anyhow::{anyhow, bail, Context};
// use wasmtime::StoreLimits;
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

pub struct NatsHost {
    table: ResourceTable,
    ctx: WasiCtx,
    // limits: StoreLimits,
}

impl NatsHost {
    pub fn new() -> Self {
        Self {
            table: ResourceTable::new(),
            ctx: WasiCtxBuilder::new().inherit_env().build(),
            // limits: StoreLimits::default(),
        }
    }
}

impl WasiView for NatsHost {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
