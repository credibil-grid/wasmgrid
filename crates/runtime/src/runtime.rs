//! # WebAssembly Runtime

use std::path::PathBuf;

use anyhow::Result;
use cfg_if::cfg_if;
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine};
use wasmtime_wasi::p2::WasiView;

use crate::service::{Linkable, Runnable};

/// Runtime for a wasm component.
pub struct Runtime<T: 'static> {
    pub component: Component,
    pub linker: Linker<T>,
}

impl<T: WasiView + 'static> Runtime<T> {
    /// Create a new Runtime instance from the provided file reference.
    ///
    /// The file can either be a serialized (pre-compiled) wasmtime `Component`
    /// or a standard `wasm32-wasip2` wasm component.
    ///
    /// # Errors
    ///
    /// Returns an error if the component cannot be loaded, the linker cannot
    /// be created, or the service cannot be started.
    pub fn from_file(file: &PathBuf) -> Result<Self> {
        tracing::trace!("initializing from file");

        let mut config = Config::new();
        config.async_support(true);
        let engine = Engine::new(&config)?;

        // file can be a serialized component or a wasm file
        cfg_if! {
            if #[cfg(feature = "compile")] {
                // attempt to load as a serialized component with fallback to wasm
                let component = match unsafe { Component::deserialize_file(&engine, file) } {
                    Ok(component) => component,
                    Err(_) => Component::from_file(&engine, file)?,
                };
            } else {
                // load as a serialized component with no fallback (cranelift is unavailable)
                let component = unsafe { Component::deserialize_file(&engine, file)? };
            }
        }

        // resolve dependencies
        let mut linker: Linker<T> = Linker::new(&engine);
        wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;

        tracing::trace!("initialized");
        Ok(Self { component, linker })
    }

    /// Create a new Runtime instance from a pre-compiled wasm component
    /// serialized as bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the component cannot be loaded, the linker cannot
    /// be created, or the service cannot be started.
    #[cfg(feature = "compile")]
    pub fn from_wasm(wasm: &PathBuf) -> Result<Self> {
        tracing::trace!("initializing from wasm file");
        Self::from_file(wasm)
    }

    /// Create a new Runtime instance from a pre-compiled wasm component
    /// serialized as a file.
    ///
    /// # Errors
    ///
    /// Returns an error if the component cannot be loaded, the linker cannot
    /// be created, or the service cannot be started.
    pub fn from_binary(binary: &PathBuf) -> Result<Self> {
        tracing::trace!("initializing from serialized component");
        Self::from_file(binary)
    }

    /// Add each service's dependency linker.
    ///
    /// # Errors
    ///
    /// Returns an error if the service cannot be added to the linker.
    pub fn link(&mut self, service: &impl Linkable<Ctx = T>) -> Result<()> {
        service.add_to_linker(&mut self.linker)
    }

    /// Initiate the service on it's own thread.
    ///
    /// # Errors
    ///
    /// TODO: document errors
    pub fn run<R: Send + 'static>(
        &mut self, service: impl Runnable<Ctx = T, Resources = R> + 'static + std::fmt::Debug,
        resources: R,
    ) -> Result<()> {
        let instance_pre = self.linker.instantiate_pre(&self.component)?;
        tokio::spawn(async move {
            tracing::debug!("starting {service:?} service");
            if let Err(e) = service.run(instance_pre, resources).await {
                tracing::error!("error running {service:?} service: {e}");
            }
        });
        Ok(())
    }

    /// Wait for a shutdown signal from the OS.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an issue processing the shutdown signal.
    pub async fn shutdown(&self) -> Result<()> {
        Ok(tokio::signal::ctrl_c().await?)
    }
}
