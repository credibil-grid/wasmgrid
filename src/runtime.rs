//! # WebAssembly Runtime

use std::any::Any;
use std::collections::HashMap;

use anyhow::anyhow;
use bytes::Bytes;
use wasmtime::component::{Component, InstancePre, Linker};
use wasmtime::{Config, Engine, StoreLimits};
use wasmtime_wasi::{
    HostOutputStream, ResourceTable, StreamError, StreamResult, WasiCtx, WasiCtxBuilder, WasiView,
};

/// Capability represents a particular runtime capability depended on by wasm
/// components. For example, an HTTP server or a message broker.
#[async_trait::async_trait]
pub trait Capability: Send {
    /// Returns the wasi namespace the capability supports. For example, `wasi:http`.
    fn namespace(&self) -> &str;

    /// Add the capability to the wasm component linker.
    fn add_to_linker(&self, linker: &mut Linker<State>) -> anyhow::Result<()>;

    /// Start and run the runtime.
    async fn run(&self, runtime: Runtime) -> anyhow::Result<()>;
}

/// Runtime for a wasm component.
#[derive(Clone)]
pub struct Runtime {
    instance_pre: InstancePre<State>,
}

impl Runtime {
    /// Returns a "pre-instantiated" Instance — an efficient form of instantiation
    /// where import type-checking and lookup has been resolved.
    pub const fn instance_pre(&self) -> &InstancePre<State> {
        &self.instance_pre
    }
}

#[derive(Default)]
pub struct Builder {
    capabilities: Vec<Box<dyn Capability>>,
}

impl Builder {
    /// Create a new Builder instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a capability to the wasm runtime.
    pub fn capability(mut self, capability: impl Capability + 'static) -> Self {
        self.capabilities.push(Box::new(capability));
        self
    }

    /// Run the wasm component with the specified capabilities.
    pub fn run(self, wasm: String) -> anyhow::Result<()> {
        tracing::debug!("starting runtime");

        let mut config = Config::new();
        config.async_support(true);
        let engine = Engine::new(&config)?;

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_async(&mut linker)?;
        for cap in &self.capabilities {
            cap.add_to_linker(&mut linker)?;
        }

        // pre-instantiate component
        let component = Component::from_file(&engine, wasm)?;
        let instance_pre = linker.instantiate_pre(&component)?;
        let component_type = component.component_type();

        let runtime = Runtime { instance_pre };

        // start capabilities
        for cap in self.capabilities {
            // check whether capability is required by the wasm component
            let namespace = cap.namespace();
            if !component_type.imports(&engine).any(|e| e.0.starts_with(namespace))
                && !component_type.exports(&engine).any(|e| e.0.starts_with(namespace))
            {
                continue;
            }

            // start capability
            tracing::debug!("starting {namespace} capability");
            let runtime = runtime.clone();
            let namespace = namespace.to_string();

            tokio::spawn(async move {
                tracing::debug!("{namespace} starting");
                if let Err(e) = cap.run(runtime).await {
                    tracing::error!("error starting {namespace}: {e}");
                }
            });
        }

        Ok(())
    }
}

pub type Metadata = Box<dyn Any + Send>;

/// State implements messaging host interfaces. In addition, it holds the host-defined
/// state used by the wasm runtime [`Store`].
pub struct State {
    table: ResourceTable,
    ctx: WasiCtx,
    pub limits: StoreLimits,

    pub metadata: HashMap<String, Metadata>,
}

impl Default for State {
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

impl State {
    /// Create a new State instance.
    pub fn new() -> Self {
        Self::default()
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

// Implement debug tracing for Guests by capturing stdout.
struct Stdout;

impl wasmtime_wasi::StdoutStream for Stdout {
    fn stream(&self) -> Box<dyn HostOutputStream> {
        Box::new(StdoutStream {})
    }

    fn isatty(&self) -> bool {
        false
    }
}

struct StdoutStream;

#[async_trait::async_trait]
impl wasmtime_wasi::Subscribe for StdoutStream {
    async fn ready(&mut self) {}
}

impl wasmtime_wasi::HostOutputStream for StdoutStream {
    fn write(&mut self, bytes: Bytes) -> StreamResult<()> {
        let out = String::from_utf8(bytes.to_vec())
            .map_err(|e| StreamError::LastOperationFailed(anyhow!(e)))?;
        println!("{out}");
        Ok(())
    }

    fn flush(&mut self) -> StreamResult<()> {
        Ok(())
    }

    fn check_write(&mut self) -> StreamResult<usize> {
        Ok(1024 * 1024)
    }
}

// Implement error tracing for Guests by capturing stderr.
struct Errout;

impl wasmtime_wasi::StdoutStream for Errout {
    fn stream(&self) -> Box<dyn HostOutputStream> {
        Box::new(ErroutStream {})
    }

    fn isatty(&self) -> bool {
        false
    }
}

struct ErroutStream;

#[async_trait::async_trait]
impl wasmtime_wasi::Subscribe for ErroutStream {
    async fn ready(&mut self) {}
}

impl wasmtime_wasi::HostOutputStream for ErroutStream {
    fn write(&mut self, bytes: Bytes) -> StreamResult<()> {
        let out = String::from_utf8(bytes.to_vec())
            .map_err(|e| StreamError::LastOperationFailed(anyhow!(e)))?;
        println!("{out}");
        Ok(())
    }

    fn flush(&mut self) -> StreamResult<()> {
        Ok(())
    }

    fn check_write(&mut self) -> StreamResult<usize> {
        Ok(1024 * 1024)
    }
}
