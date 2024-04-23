//! # NATS Messaging Runtime
//!
//! This module implements a NATS wasi:messaging runtime.

use std::collections::HashMap;

use wasi_messaging::bindings::Messaging;
use wasi_messaging::{self};
use wasmtime::component::{Component, InstancePre, Linker};
use wasmtime::StoreLimits; // StoreLimitsBuilder
use wasmtime::{Config, Engine};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::proxy::{self};
use wasmtime_wasi_http::WasiHttpCtx;

// HandlerProxy is a proxy for the wasm messaging Host, wrapping calls to the Guest's
// messaging API.
#[derive(Clone)]
pub struct HandlerProxy {
    pub engine: Engine,
    pub instance_pre: InstancePre<Host>,
}

impl HandlerProxy {
    // Create a new HandlerProxy for the specified wasm Guest.
    pub fn new(wasm: String) -> anyhow::Result<Self> {
        let mut config = Config::new();
        config.async_support(true);
        let engine = Engine::new(&config)?;

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_async(&mut linker)?;

        // link specific runtime modules
        Messaging::add_to_linker(&mut linker, |t| t)?;
        proxy::add_only_http_to_linker(&mut linker)?;

        let component = Component::from_file(&engine, wasm)?;
        let instance_pre = linker.instantiate_pre(&component)?;

        Ok(Self { engine, instance_pre })
    }
}

// Host implements messaging host interfaces. In addition, it holds the host-defined
// state used by the wasm runtime [`Store`].
pub struct Host {
    pub keys: HashMap<String, u32>,
    pub table: ResourceTable,
    ctx: WasiCtx,
    pub http_ctx: WasiHttpCtx,
    pub limits: StoreLimits,
}

impl Host {
    // Create a new Host instance.
    pub fn new() -> Self {
        Self {
            keys: HashMap::default(),
            table: ResourceTable::default(),
            ctx: WasiCtxBuilder::new().inherit_args().inherit_env().inherit_stdio().build(),
            http_ctx: WasiHttpCtx {},
            limits: StoreLimits::default(),
        }
    }
}

// Implement the [`wasmtime_wasi::ctx::WasiView`] trait for Host.
impl WasiView for Host {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
