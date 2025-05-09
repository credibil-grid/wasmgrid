//! # WebAssembly Runtime

use std::path::PathBuf;

use anyhow::Result;
use wasmtime::component::{Component, InstancePre, Linker};
use wasmtime::{Config, Engine};
use wasmtime_wasi::WasiView;

use crate::compiler;
use crate::service::{Instantiator, Service};

/// Runtime for a wasm component.
pub struct Runtime<T> {
    pub engine: Engine,
    pub component: Component,
    pub linker: Linker<T>,
    instance_pre: Option<InstancePre<T>>,
}

impl<T: WasiView + 'static> Runtime<T> {
    /// Create a new Runtime instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the component cannot be loaded, the linker cannot
    /// be created, or the service cannot be started.
    pub fn new(wasm: PathBuf, compile: bool) -> Result<Self> {
        tracing::trace!("initializing");

        // start engine
        let mut config = Config::new();
        config.async_support(true);
        let engine = Engine::new(&config)?;
        tracing::trace!("engine started");

        // load component (compiling if required)
        let component = if compile {
            let serialized = compiler::serialize(&wasm)?;
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
            instance_pre: None,
        })
    }

    /// Add service dependencies to the linker.
    ///
    /// # Errors
    ///
    /// Returns an error if the service cannot be added to the linker.
    pub fn link(&mut self, service: &impl Service<Ctx = T>) -> Result<()> {
        // let component_type = self.component.component_type();
        // let mut imports = component_type.imports(&self.engine);
        // let mut exports = component_type.exports(&self.engine);

        // let namespace = service.namespace();
        // tracing::trace!("linking {namespace} service");

        // if imports.any(|e| e.0.starts_with(namespace))
        //     || exports.any(|e| e.0.starts_with(namespace))
        // {
        service.add_to_linker(&mut self.linker)?;
        // self.required.push(namespace);
        tracing::trace!("{} linked", service.namespace());
        // }

        Ok(())
    }

    /// Resolve component imports to linked services
    pub async fn instantiate(&mut self) -> Result<()> {
        self.instance_pre = Some(self.linker.instantiate_pre(&self.component)?);
        Ok(())
    }

    /// Initiate service.
    ///
    /// # Errors
    ///
    /// TODO: document errors
    pub  fn run<R:Send+'static>(
        &mut self, service: impl Instantiator<Ctx = T, Resources = R> + 'static, resources: R,
    ) -> Result<()> {
        let namespace = service.namespace();

        let instance_pre = self
            .instance_pre
            .clone()
            .ok_or_else(|| anyhow::anyhow!("service {namespace} not instantiated"))?;

        tokio::spawn(async move {
            tracing::debug!("starting {namespace}");

            if let Err(e) = service.run(instance_pre, resources).await {
                tracing::error!("error starting {namespace}: {e}");
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
        tokio::select! {
            _ = tokio::signal::ctrl_c() => Ok(()),
        }
    }
}
