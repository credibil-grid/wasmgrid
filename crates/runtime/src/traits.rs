//! # Service
//!
//! This module contains traits implemented by concrete WASI services.
//!
//! Each service is a module that provides a concrete implementation in support
//! of a specific set of WASI interfaces.

use anyhow::Result;
use wasmtime::component::{InstancePre, Linker};

use crate::state::RunState;

/// `AddResource` is implemented by services that require external resources.
///
/// This trait can be implemented multiple times for different resource types.
///
/// # Example
///
/// ```rust,ignore
/// impl AddResource<reqwest::Client> for Service {
///     fn add_resource(&mut self, resource: reqwest::Client) -> Result<()> {
///         self.http_client = resource;
///         Ok(())
///     }
/// }
/// ```
pub trait AddResource<T> {
    /// Add a resource to the service.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an issue adding the resource.
    fn add_resource(&mut self, resource: T) -> Result<()>;
}

/// The `AddToLinker` trait is required to be implemented by a service so that
/// the runtime can link the service's dependencies prior to instantiation of
/// a component.
pub trait AddToLinker: Sync + Send {
    /// Link the service's dependencies prior to component instantiation.
    ///
    /// This method optionally allows the service to access the runtime
    /// linker's context (`Self::State`).
    ///
    /// # Errors
    ///
    /// Returns an linking error(s) from the service's generated bindings.
    fn add_to_linker(&self, linker: &mut Linker<RunState>) -> Result<()>;
}

/// The `Run` trait is implemented by services that instantiate (or run)
/// components. For example, an http or messaging service.
pub trait Run {
    /// Run the service.
    fn run(&self, pre: InstancePre<RunState>) -> impl Future<Output = Result<()>> + Send;
}

pub trait ResourceBuilder<T>: Sized {
    fn new() -> Self;

    /// Set a resource on the resource builder.
    #[allow(unused)]
    #[must_use]
    fn attribute(mut self, _key: &str, _value: &str) -> Self {
        self
    }

    /// Get a reference to the requested resource.
    ///
    /// # Errors
    ///
    /// Returns an error if the resource is not available.
    fn connect(self) -> impl Future<Output = Result<T>> + Send;
}

// pub trait Service<T> {
//     fn new() -> Self;

//     /// Set a resource on the service.
//     fn resource(&mut self, _key: &str, _value: &str) -> &mut Self {
//         self
//     }

//     /// Register the service with the runtime Linker.
//     ///
//     /// # Errors
//     ///
//     /// Returns an linking error(s) from the service's generated bindings.
//     fn add_to_linker(&self, linker: &mut Linker<RunState>) -> Result<()>;
// }
