//! # WASI KeyValue Host

mod atomics;
mod batch;
mod store;

use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;

use crate::bindings::wasi::keyvalue::store::KeyResponse;

pub type Bucket = Box<dyn RuntimeBucket>;

/// Wrap generation of wit bindings to simplify exports
pub mod bindings {
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

/// KeyValueView is implemented by the keyvalue runtime to provide the host with
/// access to runtime-specific functionality.
#[allow(clippy::module_name_repetitions)]
#[async_trait::async_trait]
pub trait KeyValueView: WasiView + Send {
    async fn open(&mut self, identifier: String) -> anyhow::Result<Resource<Bucket>>;
}

/// RuntimeBucket is implemented by the runtime to provide this host with access
/// to runtime functionality.
#[async_trait::async_trait]
pub trait RuntimeBucket: Sync + Send {
    // ------------------------------------------------------------------------
    // Store
    // ------------------------------------------------------------------------
    async fn get(&mut self, key: String) -> anyhow::Result<Vec<u8>>;

    async fn set(&mut self, key: String, value: Vec<u8>) -> anyhow::Result<()>;

    async fn delete(&mut self, key: String) -> anyhow::Result<()>;

    async fn exists(&mut self, key: String) -> anyhow::Result<bool>;

    async fn list_keys(&mut self, keys_: Option<u64>) -> anyhow::Result<KeyResponse>;

    fn close(&mut self) -> anyhow::Result<()>;

    // ------------------------------------------------------------------------
    // Atomics
    // ------------------------------------------------------------------------
    async fn increment(&mut self, key: String, delta: u64) -> anyhow::Result<u64>;

    // ------------------------------------------------------------------------
    // Batch
    // ------------------------------------------------------------------------
    async fn get_many(&mut self, keys: Vec<String>) -> anyhow::Result<Vec<(String, Vec<u8>)>>;

    async fn set_many(&mut self, key_values: Vec<(String, Vec<u8>)>) -> anyhow::Result<()>;

    async fn delete_many(&mut self, keys: Vec<String>) -> anyhow::Result<()>;
}
