//! # NATS Messaging Runtime
//!
//! This module implements a NATS wasi:messaging runtime.

use std::collections::HashMap;

use wasmtime::component::{Component, InstancePre, Linker};
use wasmtime::StoreLimits; // StoreLimitsBuilder
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::WasiHttpCtx;

/// Plugin represents a runtime wasm host.
#[async_trait::async_trait]
pub trait Plugin: Send {
    /// Add the plugin to the wasm component linker.
    fn add_to_linker(&self, linker: &mut Linker<State>) -> anyhow::Result<()>;

    /// Start and run the plugin.
    async fn run(&self, handler: Runtime) -> anyhow::Result<()>;
}

/// Runtime for a wasm component.
#[derive(Clone)]
pub struct Runtime {
    engine: Engine,
    instance_pre: InstancePre<State>,
}

impl Runtime {
    /// Returns a [`Store`] for use when calling guests.
    pub fn store(&self) -> Store<State> {
        let mut store = Store::new(&self.engine, State::new());
        store.limiter(|t| &mut t.limits);
        store
    }

    /// Returns a "pre-instantiated" Instance — an efficient form of instantiation
    /// where import type-checking and lookup has been resolved.
    pub fn instance_pre(&self) -> &InstancePre<State> {
        &self.instance_pre
    }
}

#[derive(Default)]
pub struct Builder {
    plugins: Vec<Box<dyn Plugin>>,
}

impl Builder {
    /// Create a new Builder instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a plugin to the wasm runtime.
    pub fn plugin<P>(mut self, plugin: P) -> Self
    where
        P: Plugin + 'static,
    {
        self.plugins.push(Box::new(plugin));
        self
    }

    /// Run the wasm component with the specified plugins.
    pub fn run(self, wasm: String) -> anyhow::Result<()> {
        let mut config = Config::new();
        config.async_support(true);
        let engine = Engine::new(&config)?;

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_async(&mut linker)?;

        // link plugins
        for p in &self.plugins {
            p.add_to_linker(&mut linker)?;
        }

        let component = Component::from_file(&engine, wasm)?;
        let instance_pre = linker.instantiate_pre(&component)?;

        let rt = Runtime { engine, instance_pre };

        // start plugins
        for p in self.plugins {
            let rt = rt.clone();
            tokio::spawn(async move { p.run(rt).await });
        }

        Ok(())
    }
}

/// State implements messaging host interfaces. In addition, it holds the host-defined
/// state used by the wasm runtime [`Store`].
pub struct State {
    table: ResourceTable,
    ctx: WasiCtx,
    limits: StoreLimits,

    // TODO factor out http_ctx and msg_ctx into respective plugins
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
