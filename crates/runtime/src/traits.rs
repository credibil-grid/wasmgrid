//! # Service
//!
//! This module contains traits implemented by concrete WASI services.
//!
//! Each service is a module that provides a concrete implementation in support
//! of a specific set of WASI interfaces.

use std::fmt::Debug;

use anyhow::Result;
use futures::future::BoxFuture;
use wasmtime::component::{InstancePre, Linker};

use crate::runtime::Runtime;
use crate::state::RunState;

pub trait ResourceBuilder<T>: Sized {
    /// Create a new resource builder.
    fn new() -> Self;

    /// Set one or more attributes for use in building a resource.
    #[must_use]
    fn attribute(self, _key: &str, _value: &str) -> Self {
        self
    }

    /// Get a reference to the requested resource.
    ///
    /// # Errors
    ///
    /// Returns an error if the resource is not available.
    fn connect(self) -> impl Future<Output = Result<T>> + Send;
}

/// The `ServiceBuilder` trait is required to be implemented by a service so
/// that the runtime can link the service's dependencies prior to instantiation
/// of a component.
pub trait ServiceBuilder: Sized + Sync + Send {
    fn new() -> Self;

    /// Link the service's dependencies prior to component instantiation.
    ///
    /// This method optionally allows the service to access the runtime
    /// linker's context (`Self::State`).
    ///
    /// # Errors
    ///
    /// Returns an linking error(s) from the service's generated bindings.
    fn add_to_linker(self, linker: &mut Linker<RunState>) -> Result<Self>;
}

/// `AddResource` is implemented by services that require external resources.
///
/// This trait can be implemented multiple times for different resource types.
///
/// # Example
///
/// ```rust,ignore
/// impl AddResource<reqwest::Client> for Service {
///     fn resource(&mut self, resource: reqwest::Client) -> Result<()> {
///         self.http_client = resource;
///         Ok(())
///     }
/// }
pub trait AddResource<T>: Sized {
    /// Add a resource to the service.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an issue adding the resource.
    fn resource(self, resource: T) -> Result<Self>;
}

/// The `Run` trait is implemented by services that instantiate (or run)
/// components. For example, an `http` or `messaging` service.
pub trait Run: Debug + Send + Sync {
    /// Register the service as runnable with the runtime.
    fn register(self, rt: &mut Runtime);

    /// Run the service.
    fn run(&self, pre: InstancePre<RunState>) -> BoxFuture<'static, Result<()>>;
}
