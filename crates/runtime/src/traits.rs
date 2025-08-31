//! # Service
//!
//! This module contains traits implemented by concrete WASI services.
//!
//! Each service is a module that provides a concrete implementation in support
//! of a specific set of WASI interfaces.

use std::any::Any;

use anyhow::Result;
use wasmtime::component::{InstancePre, Linker};
use wasmtime_wasi::WasiView;

/// The `Interface` trait is implemented by every service so that the runtime
/// can link the service's dependencies prior to instantiation of a component.
pub trait Interface: Sync + Send {
    type State: WasiView;

    /// Link the service's dependencies prior to component instantiation.
    ///
    /// This method optionally allows the service to access the runtime
    /// linker's context (`Self::State`).
    ///
    /// # Errors
    ///
    /// Returns an linking error(s) from the service's generated bindings.
    fn add_to_linker(&self, linker: &mut Linker<Self::State>) -> Result<()>;
}

/// The `Instantiator` trait is implemented by services that can instantiate
/// components. For example, an http service or a messaging service.
pub trait Instantiator: Interface {
    type Resources;

    /// Run the service.
    fn run(
        &self, pre: InstancePre<Self::State>, resources: Self::Resources,
    ) -> impl Future<Output = Result<()>> + Send;
}

pub trait Resource: Send + Sync {
    /// A static identifier for the resource.
    fn identifier(&self) -> &'static str;

    /// Get a reference to the underlying resource.
    fn as_any(&self) -> &dyn Any;
}
