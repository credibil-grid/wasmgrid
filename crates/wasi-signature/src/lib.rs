//! # WASI Key/Value Host

use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;

use crate::bindings::wasi::signature::signature_types::{self, Error, SigningSuite};
use crate::bindings::wasi::signature::{signer, verifier};

/// Wrap generation of wit bindings to simplify exports
pub mod bindings {
    #![allow(clippy::future_not_send)]

    pub use anyhow::Error;

    wasmtime::component::bindgen!({
        world: "signature",
        path: "wit",
        tracing: true,
        async: true,
        // with: {
        //     "wasi:signature/signature-types/error": Error,
        // },
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

// Implement the [`signer::Host`]` trait for SignatureView impls.
#[async_trait::async_trait]
impl<T: SignatureView> signer::Host for T {
    async fn sign(&mut self, msg: Vec<u8>) -> wasmtime::Result<Result<Vec<u8>, Resource<Error>>> {
        tracing::debug!("Host::sign");
        Ok(Ok(T::sign(self, msg).await?))
    }

    async fn suite(&mut self) -> wasmtime::Result<SigningSuite> {
        tracing::debug!("Host::suite");
        T::suite(self).await
    }
}

// Implement the [`verifier::Host`]` trait for SignatureView impls.
#[async_trait::async_trait]
impl<T: SignatureView> verifier::Host for T {
    async fn verify(
        &mut self, msg: Vec<u8>, signature: Vec<u8>,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::debug!("Host::verify");
        Ok(Ok(T::verify(self, msg, signature).await?))
    }
}

// Implement the [`signature_types::Host`] trait for SignatureView impls.
impl<T: SignatureView> signature_types::Host for T {}

// Implement the [`signature_types::HostError`] trait for SignatureView impls.
#[async_trait::async_trait]
impl<T: SignatureView> signature_types::HostError for T {
    async fn trace(&mut self) -> wasmtime::Result<String> {
        tracing::warn!("FIXME: trace HostError");
        Ok(String::from("trace HostError"))
    }

    fn drop(&mut self, err: Resource<Error>) -> wasmtime::Result<()> {
        tracing::debug!("drop for Resource<Error>");
        self.table().delete(err)?;
        Ok(())
    }
}
