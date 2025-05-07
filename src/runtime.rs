#![allow(dead_code)]

//! # WebAssembly Runtime

use std::any::Any;
use std::collections::HashMap;

use anyhow::anyhow;
use bytes::Bytes;
use wasmtime::component::{Component, InstancePre, Linker};
use wasmtime::{Config, Engine, StoreLimits};
use wasmtime_wasi::{
    IoView, OutputStream, ResourceTable, StreamError, StreamResult, WasiCtx, WasiCtxBuilder,
    WasiView,
};

/// Capability represents a particular runtime capability depended on by wasm
/// components. For example, an HTTP server or a message broker.
#[async_trait::async_trait]
pub trait Capability: Send {
    /// Returns the wasi namespace the capability supports. For example, `wasi:http`.
    fn namespace(&self) -> &'static str;

    /// Add the capability to the wasm component linker.
    fn add_to_linker(&self, linker: &mut Linker<Ctx>) -> anyhow::Result<()>;

    /// Start and run the runtime.
    async fn start(&self, pre: InstancePre<Ctx>) -> anyhow::Result<()>;
}

/// Runtime for a wasm component.
pub struct Runtime {
    capabilities: Vec<Box<dyn Capability>>,
}

impl Runtime {
    /// Create a new Builder instance.
    pub fn new() -> Self {
        Self {
            capabilities: Vec::new(),
        }
    }

    /// Add a capability to the wasm runtime.
    pub fn capability(mut self, capability: impl Capability + 'static) -> Self {
        self.capabilities.push(Box::new(capability));
        self
    }

    /// Run the wasm component with the specified capabilities.
    pub fn start(self, wasm: String) -> anyhow::Result<()> {
        tracing::debug!("starting runtime");

        let mut config = Config::new();
        config.async_support(true);
        let engine = Engine::new(&config)?;
        let component = Component::from_file(&engine, wasm)?;

        // link dependencies
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_async(&mut linker)?;
        for c in &self.capabilities {
            c.add_to_linker(&mut linker)?;
        }

        // pre-instantiate wasm component
        let instance_pre = linker.instantiate_pre(&component)?;
        let component_type = component.component_type();

        // start capabilities
        for c in self.capabilities {
            // check whether capability is required
            let namespace = c.namespace();
            if !component_type.imports(&engine).any(|e| e.0.starts_with(namespace))
                && !component_type.exports(&engine).any(|e| e.0.starts_with(namespace))
            {
                continue;
            }

            // start capability
            tracing::debug!("starting {namespace} capability");
            let pre = instance_pre.clone();
            let namespace = namespace.to_string();

            tokio::spawn(async move {
                tracing::debug!("{namespace} starting");
                if let Err(e) = c.start(pre).await {
                    tracing::error!("error starting {namespace}: {e}");
                }
            });
        }

        Ok(())
    }
}

pub type Metadata = Box<dyn Any + Send>;

/// Ctx implements messaging host interfaces. In addition, it holds the host-defined
/// state used by the wasm runtime [`Store`].
pub struct Ctx {
    table: ResourceTable,
    ctx: WasiCtx,
    pub limits: StoreLimits,
    pub metadata: HashMap<String, Metadata>,
}

impl Default for Ctx {
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

impl Ctx {
    /// Create a new Ctx instance.
    pub fn new() -> Self {
        Self::default()
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
        &mut self.ctx
    }
}

// Implement debug tracing for Guests by capturing stdout.
struct Stdout;

impl wasmtime_wasi::StdoutStream for Stdout {
    fn stream(&self) -> Box<dyn OutputStream> {
        Box::new(StdoutStream {})
    }

    fn isatty(&self) -> bool {
        false
    }
}

struct StdoutStream;

#[async_trait::async_trait]
impl wasmtime_wasi::Pollable for StdoutStream {
    async fn ready(&mut self) {}
}

impl OutputStream for StdoutStream {
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
    fn stream(&self) -> Box<dyn OutputStream> {
        Box::new(ErroutStream {})
    }

    fn isatty(&self) -> bool {
        false
    }
}

struct ErroutStream;

#[async_trait::async_trait]
impl wasmtime_wasi::Pollable for ErroutStream {
    async fn ready(&mut self) {}
}

impl wasmtime_wasi::OutputStream for ErroutStream {
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
