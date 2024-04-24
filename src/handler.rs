//! # NATS Messaging Runtime
//!
//! This module implements a NATS wasi:messaging runtime.

use std::collections::HashMap;

use wasmtime::component::{Component, InstancePre, Linker};
use wasmtime::StoreLimits; // StoreLimitsBuilder
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::WasiHttpCtx;

pub trait Plugin {
    fn add_to_linker(&self, linker: &mut Linker<State>) -> anyhow::Result<()>;
}

//// HandlerProxy is a proxy for the wasm messaging State, wrapping calls to the Guest's
/// messaging API.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct HandlerProxy {
    engine: Engine,
    instance_pre: InstancePre<State>,
}

impl HandlerProxy {
    /// Create a new HandlerProxy for the specified wasm Guest.
    pub fn new(wasm: String, plugins: Vec<&dyn Plugin>) -> anyhow::Result<Self> {
        let mut config = Config::new();
        config.async_support(true);
        let engine = Engine::new(&config)?;

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_async(&mut linker)?;

        // link plugins
        for p in plugins {
            p.add_to_linker(&mut linker)?;
        }

        let component = Component::from_file(&engine, wasm)?;
        let instance_pre = linker.instantiate_pre(&component)?;

        Ok(Self { engine, instance_pre })
    }

    /// Create [`Store`].
    pub fn store(&self) -> Store<State> {
        let mut store = Store::new(&self.engine, State::new());
        store.limiter(|t| &mut t.limits);
        store
    }

    pub fn instance_pre(&self) -> &InstancePre<State> {
        &self.instance_pre
    }
}

/// State implements messaging host interfaces. In addition, it holds the host-defined
/// state used by the wasm runtime [`Store`].
pub struct State {
    table: ResourceTable,
    ctx: WasiCtx,
    limits: StoreLimits,

    pub http_ctx: WasiHttpCtx,
    pub msg_ctx: HashMap<String, u32>,
}

impl State {
    /// Create a new State instance.
    pub fn new() -> Self {
        Self {
            table: ResourceTable::default(),
            ctx: WasiCtxBuilder::new().inherit_args().inherit_env().inherit_stdio().build(),
            limits: StoreLimits::default(),

            http_ctx: WasiHttpCtx {},
            msg_ctx: HashMap::default(),
        }
    }
}

// Implement the [`wasmtime_wasi::ctx::WasiView`] trait for State.
impl WasiView for State {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
