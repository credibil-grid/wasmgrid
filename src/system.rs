//! # NATS Messaging System
//!
//! This module implements a NATS wasi:messaging runtime.

use std::collections::HashMap;

use wasmtime::component::{Component, InstancePre, Linker};
use wasmtime::{Config, Engine, Store, StoreLimits};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::WasiHttpCtx;

/// Runtime represents a particular runtime capability depended on by wasm
/// components. For example, an HTTP server or a message broker.
#[async_trait::async_trait]
pub trait Runtime: Send {
    /// Add the runtime to the wasm component linker.
    fn add_to_linker(&self, linker: &mut Linker<State>) -> anyhow::Result<()>;

    /// Start and run the runtime.
    async fn run(&self, handler: System) -> anyhow::Result<()>;
}

/// System for a wasm component.
#[derive(Clone)]
pub struct System {
    engine: Engine,
    instance_pre: InstancePre<State>,
}

impl System {
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
    runtimes: Vec<Box<dyn Runtime>>,
}

impl Builder {
    /// Create a new Builder instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a runtime to the wasm runtime.
    pub fn runtime(mut self, runtime: impl Runtime + 'static) -> Self {
        self.runtimes.push(Box::new(runtime));
        self
    }

    /// Run the wasm component with the specified runtimes.
    pub fn run(self, wasm: String) -> anyhow::Result<()> {
        let mut config = Config::new();
        config.async_support(true);
        let engine = Engine::new(&config)?;

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_async(&mut linker)?;

        // link each runtime
        for rt in &self.runtimes {
            rt.add_to_linker(&mut linker)?;
        }

        // pre-instantiate component
        let component = Component::from_file(&engine, wasm)?;
        let instance_pre = linker.instantiate_pre(&component)?;
        let system = System { engine, instance_pre };

        // start runtimes
        for rt in self.runtimes {
            let system = system.clone();
            tokio::spawn(async move { rt.run(system).await });
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

    // TODO factor out http_ctx and msg_ctx into respective runtimes
    pub http_ctx: WasiHttpCtx,
    pub msg_ctx: HashMap<String, u32>,
    // pub metadata: HashMap<String, WasiHttpCtx>,
}

impl State {
    /// Create a new State instance.
    fn new() -> Self {
        Self {
            table: ResourceTable::default(),
            ctx: WasiCtxBuilder::new().inherit_args().inherit_env().inherit_stdio().build(),
            limits: StoreLimits::default(),

            http_ctx: WasiHttpCtx {},
            msg_ctx: HashMap::default(),
            // metadata: HashMap::default(),
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
