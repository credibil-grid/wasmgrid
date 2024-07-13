//! # WASI Vault Capability
//!
//! This module implements a runtime capability for `wasi:vault`
//! (<https://github.com/WebAssembly/wasi-vault>).

use anyhow::anyhow;
use base64ct::{Base64UrlUnpadded, Encoding};
use bindings::wasi::vault::enclave::{self, KeyOp};
use bindings::wasi::vault::types::{self, Algorithm, Jwk, VerificationMethod};
use bindings::wasi::vault::{signer, verifier};
use bindings::Vault;
use ecdsa::signature::Signer as _;
use k256::Secp256k1;
use wasmtime::component::{Linker, Resource};
use wasmtime_wasi::WasiView;

use crate::runtime::{self, Runtime, State};

mod bindings {
    #![allow(clippy::future_not_send)]

    pub use super::Error;

    wasmtime::component::bindgen!({
        world: "vault",
        path: "wit",
        tracing: true,
        async: true,
        trappable_imports: true,
        with: {
            "wasi:vault/types/error": Error,
        },
    });
}

pub type Error = anyhow::Error;

pub struct Capability {}

pub const fn new() -> Capability {
    Capability {}
}

#[async_trait::async_trait]
impl runtime::Capability for Capability {
    fn namespace(&self) -> &str {
        "wasi:vault"
    }

    fn add_to_linker(&self, linker: &mut Linker<State>) -> anyhow::Result<()> {
        Vault::add_to_linker(linker, |t| t)
    }

    /// Provide vault capability for the wasm component.
    async fn run(&self, _runtime: Runtime) -> anyhow::Result<()> {
        Ok(())
    }
}

#[async_trait::async_trait]
impl enclave::Host for State {
    // Sign the provided message using the signing key.
    async fn sign(&mut self, data: Vec<u8>) -> wasmtime::Result<Result<Vec<u8>, Resource<Error>>> {
        // FIXME: replace hard-coded signer with key enclave-based signing
        const JWK_D: &str = "0Md3MhPaKEpnKAyKE498EdDFerD5NLeKJ5Rb-vC16Gs";

        let decoded = match Base64UrlUnpadded::decode_vec(JWK_D) {
            Ok(decoded) => decoded,
            Err(e) => {
                tracing::debug!("issue decoding JWK_D: {e}");
                return Ok(Err(self.table().push(anyhow!("issue decoding JWK_D: {e}"))?));
            }
        };
        let signing_key: ecdsa::SigningKey<Secp256k1> =
            match ecdsa::SigningKey::from_slice(&decoded) {
                Ok(signing_key) => signing_key,
                Err(e) => {
                    tracing::debug!("issue deserializing signing key: {e}");
                    return Ok(Err(self
                        .table()
                        .push(anyhow!("issue deserializing signing key: {e}"))?));
                }
            };
        let sig: ecdsa::Signature<Secp256k1> = signing_key.sign(&data);

        Ok(Ok(sig.to_vec()))
    }

    async fn active_key(&mut self, _op: KeyOp) -> wasmtime::Result<Result<Jwk, Resource<Error>>> {
        // FIXME: replace hard-coded public key with enclave-based key
        Ok(Ok(Jwk {
            kid: None,
            kty: "EC".into(),
            crv: "secp256k1".into(),
            x: "tXSKB_rubXS7sCjXqupVJEzTcW3MsjmEvq1YpXn96Zg".into(),
            y: Some("dOicXqbjFxoGJ-K0-GJ1kHYJqic_D_OMuUwkQ7Ol6nk".into()),
        }))
    }

    async fn next_key(&mut self, _op: KeyOp) -> wasmtime::Result<Result<Jwk, Resource<Error>>> {
        // FIXME: replace hard-coded public key with enclave-based key
        Ok(Ok(Jwk {
            kid: None,
            kty: "EC".into(),
            crv: "secp256k1".into(),
            x: "tXSKB_rubXS7sCjXqupVJEzTcW3MsjmEvq1YpXn96Zg".into(),
            y: Some("dOicXqbjFxoGJ-K0-GJ1kHYJqic_D_OMuUwkQ7Ol6nk".into()),
        }))
    }
}

#[async_trait::async_trait]
impl signer::Host for State {
    // Sign the provided message using the vault suite referenced by the
    // verification-method.
    async fn sign(&mut self, msg: Vec<u8>) -> wasmtime::Result<Result<Vec<u8>, Resource<Error>>> {
        enclave::Host::sign(self, msg).await
    }

    async fn verification(&mut self) -> wasmtime::Result<VerificationMethod> {
        // FIXME: get Issuer DID and Key ID from ...?
        const ISSUER_DID: &str ="did:ion:EiDyOQbbZAa3aiRzeCkV7LOx3SERjjH93EXoIM3UoN4oWg:eyJkZWx0YSI6eyJwYXRjaGVzIjpbeyJhY3Rpb24iOiJyZXBsYWNlIiwiZG9jdW1lbnQiOnsicHVibGljS2V5cyI6W3siaWQiOiJwdWJsaWNLZXlNb2RlbDFJZCIsInB1YmxpY0tleUp3ayI6eyJjcnYiOiJzZWNwMjU2azEiLCJrdHkiOiJFQyIsIngiOiJ0WFNLQl9ydWJYUzdzQ2pYcXVwVkpFelRjVzNNc2ptRXZxMVlwWG45NlpnIiwieSI6ImRPaWNYcWJqRnhvR0otSzAtR0oxa0hZSnFpY19EX09NdVV3a1E3T2w2bmsifSwicHVycG9zZXMiOlsiYXV0aGVudGljYXRpb24iLCJrZXlBZ3JlZW1lbnQiXSwidHlwZSI6IkVjZHNhU2VjcDI1NmsxVmVyaWZpY2F0aW9uS2V5MjAxOSJ9XSwic2VydmljZXMiOlt7ImlkIjoic2VydmljZTFJZCIsInNlcnZpY2VFbmRwb2ludCI6Imh0dHA6Ly93d3cuc2VydmljZTEuY29tIiwidHlwZSI6InNlcnZpY2UxVHlwZSJ9XX19XSwidXBkYXRlQ29tbWl0bWVudCI6IkVpREtJa3dxTzY5SVBHM3BPbEhrZGI4Nm5ZdDBhTnhTSFp1MnItYmhFem5qZEEifSwic3VmZml4RGF0YSI6eyJkZWx0YUhhc2giOiJFaUNmRFdSbllsY0Q5RUdBM2RfNVoxQUh1LWlZcU1iSjluZmlxZHo1UzhWRGJnIiwicmVjb3ZlcnlDb21taXRtZW50IjoiRWlCZk9aZE10VTZPQnc4UGs4NzlRdFotMkotOUZiYmpTWnlvYUFfYnFENHpoQSJ9fQ";
        const VERIFY_KEY_ID: &str = "publicKeyModel1Id";

        Ok(VerificationMethod {
            algorithm: Algorithm::Es256k,
            key_id: format!("{ISSUER_DID}#{VERIFY_KEY_ID}"),
            jwk: None,
        })
    }
}

#[async_trait::async_trait]
impl verifier::Host for State {
    async fn verify(
        &mut self, _msg: Vec<u8>, _signature: Vec<u8>,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        todo!()
    }
}

impl types::Host for State {}

#[async_trait::async_trait]
impl types::HostError for State {
    async fn trace(&mut self, rep: Resource<Error>) -> wasmtime::Result<String> {
        tracing::warn!("FIXME: trace HostError");
        let error = self.table().get(&rep)?;
        Ok(error.to_string())
    }

    fn drop(&mut self, rep: Resource<Error>) -> wasmtime::Result<()> {
        tracing::debug!("drop for Resource<Error>");
        self.table().delete(rep)?;
        Ok(())
    }
}
