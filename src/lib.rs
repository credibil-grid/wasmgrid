// #![allow(dead_code)]
#![feature(let_chains)]

//! # WebAssembly Runtime

mod service;

use std::any::Any;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

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

/// Compile `wasm32-wasip2` component.
///
/// For example, to compile the `http` component, run:
///
/// ```
/// cargo build --package http@0.1.0 --target wasm32-wasip2 --release
/// ```
///
/// # Errors
///
/// Returns an error if the WASM component cannot be loaded from the specified
/// path, cannot be compiled, or cannot be serialized to the specified output
/// directory.
pub fn compile(wasm: &PathBuf, output: Option<PathBuf>) -> Result<()> {
    let Some(file_name) = wasm.file_name() else {
        return Err(anyhow!("invalid file name"));
    };

    // compile component
    let serialized = serialize(wasm)?;

    // define output file
    let mut out_path = output.unwrap_or_else(|| PathBuf::from("."));
    if out_path.is_dir() {
        let file_name = file_name.to_string_lossy().to_string();
        let file_name = file_name.replace(".wasm", ".bin");
        out_path.push(file_name);
    }

    // create output directory if it doesn't exist
    if let Some(dir) = out_path.parent()
        && !fs::exists(dir)?
    {
        fs::create_dir_all(dir)?;
    }

    let mut file = File::create(out_path)?;
    file.write_all(&serialized)?;

    Ok(())
}

// Compile and serialize a wasm component.
fn serialize(wasm: &PathBuf) -> Result<Vec<u8>> {
    let mut config = Config::new();
    config.async_support(true);
    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, wasm)?;
    component.serialize()
}

/// Service represents a particular runtime service depended on by wasm
/// components. For example, an HTTP server or a message broker.
#[async_trait::async_trait]
pub trait Service: Sync + Send {
    /// Returns the wasi namespace the service supports. For example, `wasi:http`.
    fn namespace(&self) -> &'static str;

    /// Add the service to the wasm component linker.
    ///
    /// # Errors
    ///
    /// Returns an error if the service encounters an error adding generated
    /// bindings to the linker.
    fn add_to_linker(&self, linker: &mut Linker<Ctx>) -> Result<()>;

    /// Start and run the runtime.
    async fn start(&self, pre: InstancePre<Ctx>) -> Result<()>;
}

/// Runtime for a wasm component.
pub struct Runtime {
    service: Vec<Box<dyn Service + 'static>>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime {
    /// Create a new Runtime instance.
    #[must_use]
    pub fn new() -> Self {
        let mut runtime = Self { service: Vec::new() };
        if cfg!(feature = "http") {
            runtime.with_service(http::new());
        }
        if cfg!(feature = "keyvalue") {
            runtime.with_service(keyvalue::new());
        }
        // if cfg!(feature = "rpc") {
        //     runtime.with_service(rpc::new());
        // }
        // if cfg!(feature = "vault") {
        //     runtime.with_service(vault::new());
        // }
        // if cfg!(feature = "messaging") {
        //      runtime.with_service(messaging::new());
        // }
        // if cfg!(feature = "jsondb") {
        //     runtime.with_service(jsondb::new());
        // }
        runtime
    }

    /// Add a service to the wasm runtime.
    fn with_service(&mut self, service: impl Service + 'static) -> &mut Self {
        self.service.push(Box::new(service));
        self
    }

    /// Run the wasm component with the specified service.
    ///
    /// # Errors
    ///
    /// Returns an error if the component cannot be loaded, the linker cannot
    /// be created, or the service cannot be started.
    #[allow(clippy::cognitive_complexity)]
    pub fn start(self, wasm: PathBuf, compile: bool) -> Result<()> {
        // --------------------------------------
        // Step 1: start engine (~2ms)
        // --------------------------------------
        let mut config = Config::new();
        config.async_support(true);
        let engine = Engine::new(&config)?;
        tracing::trace!("engine started");

        // --------------------------------------
        // Step 2: load component (compiling if required) (~9ms)
        // --------------------------------------
        let component = if compile {
            let serialized = serialize(&wasm)?;
            unsafe { Component::deserialize(&engine, &serialized)? }
        } else {
            unsafe { Component::deserialize_file(&engine, wasm)? }
        };
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
        for svc in self.service {
            if imports.any(|e| e.0.starts_with(svc.namespace()))
                || exports.any(|e| e.0.starts_with(svc.namespace()))
            {
                svc.add_to_linker(&mut linker)?;
                required.push(svc);
            }
        }
        tracing::trace!("service linked");

        // --------------------------------------
        // Step 3: initiate required services (~2ms)
        // --------------------------------------
        // ..resolve component imports to linked services
        let instance_pre = linker.instantiate_pre(&component)?;

        for svc in required {
            let pre = instance_pre.clone();
            tokio::spawn(async move {
                let namespace = svc.namespace();
                tracing::debug!("starting {namespace}");
                if let Err(e) = svc.start(pre).await {
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
#[allow(clippy::struct_field_names)]
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
    #[must_use]
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
