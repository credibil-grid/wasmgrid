//! # WASI Key/Value Host

mod signer;
mod verifier;

// use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;

use crate::bindings::wasi::signature::signature_types::SigningSuite;

/// Wrap generation of wit bindings to simplify exports
pub mod bindings {
    #![allow(clippy::future_not_send)]

    pub use anyhow::Error;

    wasmtime::component::bindgen!({
        world: "signature",
        path: "wit",
        tracing: true,
        async: true,
        with: {
            "wasi:signature/signature-types/error": Error,
        },
        // trappable_error_type: {
        //     "wasi:keyvalue/keyvalue-types/error" => Error,
        // },
    });
}

/// SignatureView is implemented by the signature runtime to provide the host with
/// access to runtime-specific functionality.
#[allow(clippy::module_name_repetitions)]
#[async_trait::async_trait]
pub trait SignatureView: WasiView + Send {
    async fn sign(&mut self, msg: Vec<u8>) -> anyhow::Result<Vec<u8>>;

    async fn suite(&mut self) -> anyhow::Result<SigningSuite>;

    async fn verify(&mut self, msg: Vec<u8>, signature: Vec<u8>) -> anyhow::Result<()>;
}

// /// RuntimeBucket is implemented by the runtime to provide this host with access
// /// to runtime functionality.
// #[async_trait::async_trait]
// pub trait RuntimeBucket: Sync + Send {
//     // ------------------------------------------------------------------------
//     // Store
//     // ------------------------------------------------------------------------
//     async fn get(&mut self, key: String) -> anyhow::Result<Option<Vec<u8>>>;

//     async fn set(&mut self, key: String, value: Vec<u8>) -> anyhow::Result<()>;

//     async fn delete(&mut self, key: String) -> anyhow::Result<()>;

//     async fn exists(&mut self, key: String) -> anyhow::Result<bool>;

//     async fn list_keys(&mut self, cursor: Option<u64>) -> anyhow::Result<KeyResponse>;

//     /// Close the bucket. This may be a no-op for some backends.
//     ///
//     /// # Errors
//     fn close(&mut self) -> anyhow::Result<()>;

//     // ------------------------------------------------------------------------
//     // Atomics
//     // ------------------------------------------------------------------------
//     async fn increment(&mut self, key: String, delta: u64) -> anyhow::Result<u64>;

//     // ------------------------------------------------------------------------
//     // Batch
//     // ------------------------------------------------------------------------
//     async fn get_many(&mut self, keys: Vec<String>) -> anyhow::Result<Vec<(String, Vec<u8>)>>;

//     async fn set_many(&mut self, key_values: Vec<(String, Vec<u8>)>) -> anyhow::Result<()>;

//     async fn delete_many(&mut self, keys: Vec<String>) -> anyhow::Result<()>;
// }
