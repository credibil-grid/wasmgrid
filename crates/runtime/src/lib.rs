// #![allow(dead_code)]
#![feature(let_chains)]

//! # WebAssembly Runtime

mod service;
mod trace;

use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use anyhow::{Result, anyhow};
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine};
use wasmtime_wasi::WasiView;

// use wasmtime_wasi_http::WasiHttpView;
pub use crate::service::Service;

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

/// Runtime for a wasm component.
pub struct Runtime<T> {
    engine: Engine,
    component: Component,
    linker: Linker<T>,
    required: Vec<&'static str>,
}

impl<T: WasiView + 'static> Runtime<T> {
    /// Create a new Runtime instance.

    /// Run the wasm component with the specified service.
    ///
    /// # Errors
    ///
    /// Returns an error if the component cannot be loaded, the linker cannot
    /// be created, or the service cannot be started.
    #[must_use]
    pub fn new(wasm: PathBuf, compile: bool) -> Result<Self> {
        // start engine
        let mut config = Config::new();
        config.async_support(true);
        let engine = Engine::new(&config)?;
        tracing::trace!("engine started");

        // load component (compiling if required)
        let component = if compile {
            let serialized = serialize(&wasm)?;
            unsafe { Component::deserialize(&engine, &serialized)? }
        } else {
            unsafe { Component::deserialize_file(&engine, wasm)? }
        };
        tracing::trace!("component loaded");

        // resolve component dependencies
        let mut linker: Linker<T> = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_async(&mut linker)?;

        Ok(Self {
            engine,
            linker,
            component,
            required: Vec::new(),
        })
    }

    /// Add service dependencies to the linker.
    pub fn link(&mut self, service: &impl Service<Ctx = T>) -> Result<&Self> {
        let component_type = self.component.component_type();
        let mut imports = component_type.imports(&self.engine);
        let mut exports = component_type.exports(&self.engine);

        let namespace = service.namespace();

        if imports.any(|e| e.0.starts_with(namespace))
            || exports.any(|e| e.0.starts_with(namespace))
        {
            service.add_to_linker(&mut self.linker)?;
            self.required.push(namespace);
            tracing::trace!("{namespace} service linked");
        }

        Ok(self)
    }

    // pub fn instantiate(&self)->Result<()>{
    //     let instance_pre = self.linker.instantiate_pre(&self.component)?;
    //     Ok(())
    // }

    // Initiate service
    pub fn start(&self, service: impl Service<Ctx = T> + 'static) -> Result<()> {
        let namespace = service.namespace();

        if !self.required.contains(&namespace) {
            tracing::warn!("skipping {namespace} service");
            return Ok(());
        }

        // resolve component imports to linked services
        let instance_pre = self.linker.instantiate_pre(&self.component)?;

        let pre = instance_pre.clone();
        tokio::spawn(async move {
            tracing::debug!("starting {namespace}");
            if let Err(e) = service.start(pre).await {
                tracing::error!("error starting {namespace}: {e}");
            }
        });

        Ok(())
    }
}
