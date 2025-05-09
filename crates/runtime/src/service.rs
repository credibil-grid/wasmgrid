//! # Service
//!
//! This module contains runtime service providers. Each service is a module
//! that provides a concrete implementation in support of a specific set of WASI
//! interfaces.

use anyhow::Result;
use wasmtime::component::{InstancePre, Linker};
use wasmtime_wasi::{IoView, WasiView};

/// Service represents a particular runtime service depended on by wasm
/// components. For example, an HTTP server or a message broker.
pub trait Service: Sync + Send {
    type Ctx: IoView + WasiView;

    /// Returns the wasi namespace the service supports. For example, `wasi:http`.
    fn namespace(&self) -> &'static str;

    /// Add the service to the wasm component linker.
    ///
    /// # Errors
    ///
    /// Returns an error if the service encounters an error adding generated
    /// bindings to the linker.
    fn add_to_linker(&self, linker: &mut Linker<Self::Ctx>) -> Result<()>;
}

/// Service represents a particular runtime service depended on by wasm
/// components. For example, an HTTP server or a message broker.
pub trait Instantiator: Service + Sync + Send {
    type Resources;

    /// Start and run the runtime.
    fn run(
        &self, pre: InstancePre<Self::Ctx>, resources: Self::Resources,
    ) -> impl Future<Output = Result<()>> + Send;
}
