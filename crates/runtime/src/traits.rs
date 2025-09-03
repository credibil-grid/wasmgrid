//! # Service
//!
//! This module contains traits implemented by concrete WASI services.
//!
//! Each service is a module that provides a concrete implementation in support
//! of a specific set of WASI interfaces.

use std::fmt::Debug;

use anyhow::Result;
use futures::FutureExt;
use futures::future::BoxFuture;
use wasmtime::component::{InstancePre, Linker};

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

/// Services implement this trait so that the runtime can link their dependencies
/// and, optionally, run them in the context of the runtime.
pub trait Service: Debug + Sync + Send {
    /// Link the service's dependencies prior to component instantiation.
    ///
    /// This method optionally allows the service to access the runtime
    /// linker's context (`Self::State`).
    ///
    /// # Errors
    ///
    /// Returns an linking error(s) from the service's generated bindings.
    fn add_to_linker(&self, linker: &mut Linker<RunState>) -> Result<()>;

    /// Start the service.
    ///
    /// This is typically implemented by services that instantiate (or run)
    /// wasm components.
    #[allow(unused_variables)]
    fn start(&self, pre: InstancePre<RunState>) -> BoxFuture<'static, Result<()>> {
        async { Ok(()) }.boxed()
    }
}
