#![allow(dead_code)]

//! # WebAssembly Runtime

use std::any::Any;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use anyhow::{Result, anyhow};
use bytes::Bytes;
use wasmtime::component::{Component, InstancePre, Linker};
use wasmtime::{Config, Engine, StoreLimits};
use wasmtime_wasi::{
    IoView, OutputStream, ResourceTable, StreamError, StreamResult, WasiCtx, WasiCtxBuilder,
    WasiView,
};

#[cfg(feature = "http")]
use crate::service::http;
#[cfg(feature = "keyvalue")]
use crate::service::keyvalue;
// #[cfg(feature = "messaging")]
// use crate::service::messaging;
// #[cfg(feature = "rpc")]
// use crate::service::rpc;
// #[cfg(feature = "vault")]
// use crate::service::vault;
// #[cfg(feature = "jsondb")]
// use crate::service::jsondb;

/// Compile wasm component
pub fn compile(wasm: String) -> Result<()> {
    let mut config = Config::new();
    config.async_support(true);
    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, wasm)?;
    let serialized = component.serialize()?;

    let mut file = File::create("compiled.bin")?;
    file.write_all(&serialized)?;

    Ok(())
}

/// Service represents a particular runtime service depended on by wasm
/// components. For example, an HTTP server or a message broker.
#[async_trait::async_trait]
pub trait Service: Sync + Send {
    /// Returns the wasi namespace the service supports. For example, `wasi:http`.
    fn namespace(&self) -> &'static str;

    /// Add the service to the wasm component linker.
    fn add_to_linker(&self, linker: &mut Linker<Ctx>) -> Result<()>;

    /// Start and run the runtime.
    async fn start(&self, pre: InstancePre<Ctx>) -> Result<()>;
}

/// Runtime for a wasm component.
pub struct Runtime {
    service: Vec<Box<dyn Service + 'static>>,
}

impl Runtime {
    /// Create a new Runtime instance.
    pub fn new() -> Self {
        let mut runtime = Self { service: Vec::new() };
        if cfg!(feature = "http") {
            runtime.with_service(http::new());
        }
        if cfg!(feature = "keyvalue") {
            runtime.with_service(keyvalue::new());
        }
        // if cfg!(feature = "messaging") {
        //      runtime.with_service(messaging::new(nats_cnn.clone()));
        // }
        // if cfg!(feature = "jsondb") {
        //     let mgo_cnn = env::var("MGO_CNN").unwrap_or_else(|_| DEF_MGO_CNN.into());
        //     runtime.with_service(jsondb::new(mgo_cnn));
        // }
        // if cfg!(feature = "rpc") {
        //     runtime.with_service(rpc::new(nats_cnn, nats_creds));
        // }
        // if cfg!(feature = "vault") {
        //     runtime.with_service(vault::new());
        // }
        runtime
    }

    /// Add a service to the wasm runtime.
    fn with_service(&mut self, service: impl Service + 'static) -> &mut Self {
        self.service.push(Box::new(service));
        self
    }

    /// Run the wasm component with the specified service.
    pub fn start(self, wasm: String) -> Result<()> {
        // --------------------------------------
        // Step 1: compile component (~2ms)
        // --------------------------------------
        let mut config = Config::new();
        config.async_support(true);
        let engine = Engine::new(&config)?;
        tracing::trace!("engine started");

        // --------------------------------------
        // Step 2: load compiled component (~9ms)
        // --------------------------------------
        let component = unsafe { Component::deserialize_file(&engine, wasm)? };
        tracing::trace!("component loaded");

        // --------------------------------------
        // Step 3: resolve component imports (~1ms)
        // --------------------------------------
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_async(&mut linker)?;

        let component_type = component.component_type();
        let mut imports = component_type.imports(&engine);
        let mut exports = component_type.exports(&engine);

        let mut required = vec![];
        for c in self.service {
            if imports.any(|e| e.0.starts_with(c.namespace()))
                || exports.any(|e| e.0.starts_with(c.namespace()))
            {
                c.add_to_linker(&mut linker)?;
                required.push(c);
            }
        }
        tracing::trace!("service linked");

        // --------------------------------------
        // Step 3: initiate service (~2ms)
        // --------------------------------------
        // resolve component imports to linked service
        let instance_pre = linker.instantiate_pre(&component)?;

        for c in required {
            let pre = instance_pre.clone();
            tokio::spawn(async move {
                let namespace = c.namespace();
                tracing::debug!("starting {namespace}");
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
