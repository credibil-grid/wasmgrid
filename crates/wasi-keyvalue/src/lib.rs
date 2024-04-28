//! # WASI KeyValue Host

mod atomics;
mod batch;
mod store;

use wasmtime_wasi::WasiView;

/// Wrap generation of wit bindings to simplify exports
pub mod bindings {
    pub use anyhow::Error;

    wasmtime::component::bindgen!({
        world: "keyvalue",
        path: "wit",
        tracing: true,
        async: true,
        // with: {
        //     "wasi:keyvalue/keyvalue-types/client": Client,
        //     "wasi:keyvalue/keyvalue-types/error": Error,
        // },
        // trappable_error_type: {
        //     "wasi:keyvalue/keyvalue-types/error" => Error,
        // },
    });
}

/// KeyValueView is implemented by the keyvalue runtime to provide the host with
/// access to runtime-specific functionality.
#[allow(clippy::module_name_repetitions)]
#[async_trait::async_trait]
pub trait KeyValueView: WasiView + Send {}
