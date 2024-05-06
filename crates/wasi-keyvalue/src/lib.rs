//! # WASI Key/Value Host

pub mod atomics;
pub mod batch;
pub mod store;

use std::any::Any;

pub type Bucket = Box<dyn RuntimeBucket>;

/// Wrap generation of wit bindings to simplify exports
pub mod bindings {
    #![allow(clippy::future_not_send)]
    pub use super::Bucket;

    wasmtime::component::bindgen!({
        world: "keyvalue",
        path: "wit",
        tracing: true,
        async: true,
        with: {
            "wasi:keyvalue/store/bucket": Bucket,
        },
        // trappable_error_type: {
        //     "wasi:keyvalue/keyvalue-types/error" => Error,
        // },
    });
}

/// RuntimeBucket is implemented by the runtime to provide this host with access
/// to runtime functionality.
#[async_trait::async_trait]
pub trait RuntimeBucket: Sync + Send {
    fn as_any(&self) -> &dyn Any;
}
