//! # WebAssembly Runtime

use std::env;
use std::fmt::{self, Debug, Formatter};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};
use cfg_if::cfg_if;
use credibil_otel::Telemetry;
use futures::future::{BoxFuture, FutureExt};
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine};

use crate::state::RunState;
use crate::traits::Service;

/// Runtime for a wasm component.
pub struct Runtime {
    wasm: PathBuf,
    services: Vec<Box<dyn Service>>,
}

impl Debug for Runtime {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Runtime").finish()
    }
}

impl Runtime {
    /// Create a new Runtime instance from the provided file reference.
    ///
    /// The file can either be a serialized (pre-compiled) wasmtime `Component`
    /// or a standard `wasm32-wasip2` wasm component.
    #[must_use]
    pub fn new(wasm: PathBuf) -> Self {
        Self {
            wasm,
            services: vec![],
        }
    }

    /// Register a service with the runtime.
    ///
    /// The service must have implemented the [`Service`] trait in order to
    /// register.
    #[must_use]
    pub fn register<S: Service + 'static>(mut self, service: S) -> Self {
        self.services.push(Box::new(service));
        self
    }

    /// Start the runtime, instantiating each registered service on its own
    /// thread.
    ///
    /// This function will block until a shutdown signal is received from the OS.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an issue processing the shutdown signal.
    pub async fn serve(self) -> Result<()> {
        init_telemetry(&self.wasm)?;
        tracing::trace!("initializing");

        let mut config = Config::new();
        config.async_support(true);
        let engine = Engine::new(&config)?;

        // TODO: cause executing WebAssembly to periodically yield
        //  1. enable `Config::epoch_interruption`
        //  2. Set `Store::epoch_deadline_async_yield_and_update`
        //  3. Call `Engine::increment_epoch` periodically

        // file can be a serialized component or a wasm file
        cfg_if! {
            if #[cfg(feature = "compile")] {
                // attempt to load as a serialized component with fallback to wasm
                let component = match unsafe { Component::deserialize_file(&engine, &self.wasm) } {
                    Ok(component) => component,
                    Err(_) => Component::from_file(&engine, &self.wasm)?,
                };
            } else {
                // load as a serialized component with no fallback (Cranelift is unavailable)
                let component = unsafe { Component::deserialize_file(&engine, &self.wasm)? };
            }
        }

        // resolve dependencies
        let mut linker: Linker<RunState> = Linker::new(&engine);
        wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;

        for service in &self.services {
            service.add_to_linker(&mut linker)?;
        }

        // start services
        let instance_pre = linker.instantiate_pre(&component)?;

        for service in self.services {
            let instance_pre = instance_pre.clone();
            tokio::spawn(async move {
                tracing::debug!("starting {service:?} service");
                if let Err(e) = service.start(instance_pre).await {
                    tracing::error!("error running {service:?} service: {e}");
                }
            });
        }

        // wait for shutdown signal
        Ok(tokio::signal::ctrl_c().await?)
    }
}

impl IntoFuture for Runtime {
    type IntoFuture = BoxFuture<'static, Result<()>>;
    type Output = Result<()>;

    fn into_future(self) -> Self::IntoFuture {
        self.serve().boxed()
    }
}

fn init_telemetry(file: &Path) -> Result<()> {
    let file_name = file.file_name().and_then(|s| s.to_str()).unwrap_or("unknown");
    let Some((prefix, _)) = file_name.split_once('.') else {
        return Err(anyhow!("file name does not have an extension"));
    };

    // initialize telemetry
    let mut builder = Telemetry::new(prefix);
    if let Ok(endpoint) = env::var("OTEL_GRPC_ADDR") {
        builder = builder.endpoint(endpoint);
    }
    builder.build().context("initializing telemetry")
}

// /// Create a new Runtime instance from a pre-compiled wasm component
// /// serialized as bytes.
// ///
// /// # Errors
// ///
// /// Returns an error if the component cannot be loaded, the linker cannot
// /// be created, or the service cannot be started.
// #[cfg(feature = "compile")]
// pub fn from_wasm(wasm: &PathBuf) -> Result<Self> {
//     tracing::trace!("initializing from wasm file");
//     Self::from_file(wasm)
// }

// /// Create a new Runtime instance from a pre-compiled wasm component
// /// serialized as a file.
// ///
// /// # Errors
// ///
// /// Returns an error if the component cannot be loaded, the linker cannot
// /// be created, or the service cannot be started.
// pub fn from_binary(binary: &PathBuf) -> Result<Self> {
//     tracing::trace!("initializing from serialized component");
//     Self::from_file(binary)
// }
