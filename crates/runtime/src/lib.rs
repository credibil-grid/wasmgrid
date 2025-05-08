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

pub use crate::service::*;


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
        Self { service: Vec::new() }
    }

    /// Add a service to the wasm runtime.
    pub fn with_service(&mut self, service: impl Service + 'static) -> &mut Self {
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
