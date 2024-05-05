//! # WASI Signature Capability
//!
//! This module implements a runtime capability for `wasi:signature`
//! (<https://github.com/WebAssembly/wasi-signature>).

use base64ct::{Base64UrlUnpadded, Encoding};
use ecdsa::signature::Signer as _;
use k256::Secp256k1;
use wasi_signature::bindings::wasi::signature::types::{
    Algorithm, SigningSuite,
};
use wasi_signature::bindings::Signature;
use wasi_signature::{self, SignatureView};
use wasmtime::component::Linker;

use crate::runtime::{self, Runtime, State};

// FIXME: replace hard-coded signer with key vault-based signing
const ISSUER_DID: &str ="did:ion:EiDyOQbbZAa3aiRzeCkV7LOx3SERjjH93EXoIM3UoN4oWg:eyJkZWx0YSI6eyJwYXRjaGVzIjpbeyJhY3Rpb24iOiJyZXBsYWNlIiwiZG9jdW1lbnQiOnsicHVibGljS2V5cyI6W3siaWQiOiJwdWJsaWNLZXlNb2RlbDFJZCIsInB1YmxpY0tleUp3ayI6eyJjcnYiOiJzZWNwMjU2azEiLCJrdHkiOiJFQyIsIngiOiJ0WFNLQl9ydWJYUzdzQ2pYcXVwVkpFelRjVzNNc2ptRXZxMVlwWG45NlpnIiwieSI6ImRPaWNYcWJqRnhvR0otSzAtR0oxa0hZSnFpY19EX09NdVV3a1E3T2w2bmsifSwicHVycG9zZXMiOlsiYXV0aGVudGljYXRpb24iLCJrZXlBZ3JlZW1lbnQiXSwidHlwZSI6IkVjZHNhU2VjcDI1NmsxVmVyaWZpY2F0aW9uS2V5MjAxOSJ9XSwic2VydmljZXMiOlt7ImlkIjoic2VydmljZTFJZCIsInNlcnZpY2VFbmRwb2ludCI6Imh0dHA6Ly93d3cuc2VydmljZTEuY29tIiwidHlwZSI6InNlcnZpY2UxVHlwZSJ9XX19XSwidXBkYXRlQ29tbWl0bWVudCI6IkVpREtJa3dxTzY5SVBHM3BPbEhrZGI4Nm5ZdDBhTnhTSFp1MnItYmhFem5qZEEifSwic3VmZml4RGF0YSI6eyJkZWx0YUhhc2giOiJFaUNmRFdSbllsY0Q5RUdBM2RfNVoxQUh1LWlZcU1iSjluZmlxZHo1UzhWRGJnIiwicmVjb3ZlcnlDb21taXRtZW50IjoiRWlCZk9aZE10VTZPQnc4UGs4NzlRdFotMkotOUZiYmpTWnlvYUFfYnFENHpoQSJ9fQ";
const VERIFY_KEY_ID: &str = "publicKeyModel1Id";
const JWK_D: &str = "0Md3MhPaKEpnKAyKE498EdDFerD5NLeKJ5Rb-vC16Gs";

pub struct Capability {}

pub const fn new() -> Capability {
    Capability {}
}

#[async_trait::async_trait]
impl runtime::Capability for Capability {
    fn namespace(&self) -> &str {
        "wasi:signature"
    }

    fn add_to_linker(&self, linker: &mut Linker<State>) -> anyhow::Result<()> {
        Signature::add_to_linker(linker, |t| t)
    }

    /// Provide signature capability for the wasm component.
    async fn run(&self, _runtime: Runtime) -> anyhow::Result<()> {
        Ok(())
    }
}

// Implement the [`wasi_signature::SignatureView`]` trait for State.
#[async_trait::async_trait]
impl SignatureView for State {
    // Open bucket specified by identifier, save to state and return as a resource.
    async fn sign(&mut self, msg: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let decoded = Base64UrlUnpadded::decode_vec(JWK_D)?;
        let signing_key: ecdsa::SigningKey<Secp256k1> = ecdsa::SigningKey::from_slice(&decoded)?;
        let sig: ecdsa::Signature<Secp256k1> = signing_key.sign(&msg);
        Ok(sig.to_vec())
    }

    async fn suite(&mut self) -> anyhow::Result<SigningSuite> {
        Ok(SigningSuite {
            algorithm: Algorithm::Es256k,
            verification_method: format!("{ISSUER_DID}#{VERIFY_KEY_ID}"),
        })
    }

    async fn verify(&mut self, _msg: Vec<u8>, _signature: Vec<u8>) -> anyhow::Result<()> {
        todo!()
    }
}
