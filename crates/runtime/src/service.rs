//! # Service
//!
//! This module contains traits implemented by concrete WASI services.
//!
//! Each service is a module that provides a concrete implementation in support
//! of a specific set of WASI interfaces.

use anyhow::Result;
use wasmtime::component::{InstancePre, Linker};
use wasmtime_wasi::WasiView;
// use wasmtime_wasi::p2::IoView;

/// The `Linkable` trait is implemented by every service so that the runtime
/// can link the service's dependencies prior to instantiation of a component.
pub trait Linkable: Sync + Send {
    type Ctx: WasiView;

    /// Link the service's dependencies prior to component instantiation.
    ///
    /// This method optionally allows the service to access the runtime
    /// linker's context (`Self::Ctx`).
    ///
    /// # Errors
    ///
    /// Returns an linking error(s) from the service's generated bindings.
    fn add_to_linker(&self, linker: &mut Linker<Self::Ctx>) -> Result<()>;
}

/// The `Runnable` trait is implemented by services that can instantiate
/// components. For example, an http service or a messaging service.
pub trait Runnable: Linkable + Sync + Send {
    type Resources;

    /// Run the service.
    fn run(
        &self, pre: InstancePre<Self::Ctx>, resources: Self::Resources,
    ) -> impl Future<Output = Result<()>> + Send;
}
