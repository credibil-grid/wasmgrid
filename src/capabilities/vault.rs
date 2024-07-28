//! # WASI Vault Capability
//!
//! This module implements a runtime capability for `wasi:vault`
//! (<https://github.com/WebAssembly/wasi-vault>).

use anyhow::anyhow;
use base64ct::{Base64UrlUnpadded, Encoding};
use bindings::wasi::vault::keystore::{self, Algorithm, Jwk, KeyType};
use bindings::Vault;
// use ecdsa::{Signature, Signer as _, SigningKey};
use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;
// use k256::Secp256k1;
use wasmtime::component::{Linker, Resource};
use wasmtime_wasi::WasiView;

use crate::runtime::{self, Runtime, State};

/// Wrap generation of wit bindings to simplify exports.
/// See <https://docs.rs/wasmtime/latest/wasmtime/component/macro.bindgen.html>
mod bindings {
    #![allow(clippy::future_not_send)]

    pub use super::{Error, KeySet};

    wasmtime::component::bindgen!({
        world: "vault",
        path: "wit",
        tracing: true,
        async: true,
        trappable_imports: true,
        with: {
            "wasi:vault/keystore/error": Error,
            "wasi:vault/keystore/key-set": KeySet,
        },
        additional_derives: [
            Clone,
        ],
        // include_generated_code_from_file: true,
    });
}

pub type Error = anyhow::Error;

// const SECP1_X: &str = "tXSKB_rubXS7sCjXqupVJEzTcW3MsjmEvq1YpXn96Zg";
// const SECP1_Y: &str = "dOicXqbjFxoGJ-K0-GJ1kHYJqic_D_OMuUwkQ7Ol6nk";
// const SECP1_SECRET: &str = "0Md3MhPaKEpnKAyKE498EdDFerD5NLeKJ5Rb-vC16Gs";

const ED25519_X: &str = "q6rjRnEH_XK72jvB8FNBJtOl9_gDs6NW49cAz6p2sW4";
const ED25519_SECRET: &str = "cCxmHfFfIJvP74oNKjAuRC3zYoDMo0pFsAs19yKMowY";

#[derive(Clone)]
pub struct KeySet {}

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
impl keystore::Host for State {
    async fn open(
        &mut self, _identifier: String,
    ) -> wasmtime::Result<Result<Resource<KeySet>, Resource<Error>>> {
        todo!()
    }

    async fn supported_algorithms(&mut self) -> wasmtime::Result<Vec<Algorithm>> {
        todo!()
    }
}

#[async_trait::async_trait]
impl keystore::HostKeySet for State {
    async fn generate(
        &mut self, _rep: Resource<KeySet>, _: KeyType, _: Algorithm,
    ) -> wasmtime::Result<Result<Jwk, Resource<Error>>> {
        todo!("generate new key for KeyType")
    }

    async fn sign(
        &mut self, _rep: Resource<KeySet>, _: KeyType, data: Vec<u8>,
    ) -> wasmtime::Result<Result<Vec<u8>, Resource<Error>>> {
        let decoded = match Base64UrlUnpadded::decode_vec(ED25519_SECRET) {
            Ok(decoded) => decoded,
            Err(e) => {
                tracing::debug!("issue decoding ED25519_SECRET: {e}");
                return Ok(Err(self
                    .table()
                    .push(anyhow!("issue decoding ED25519_SECRET: {e}"))?));
            }
        };
        let Ok(secret_key) = decoded.try_into() else {
            tracing::debug!("issue deserializing signing key");
            return Ok(Err(self.table().push(anyhow!("issue deserializing signing key"))?));
        };

        let signing_key: SigningKey = SigningKey::from_bytes(&secret_key);
        Ok(Ok(signing_key.sign(&data).to_bytes().to_vec()))

        // let decoded = match Base64UrlUnpadded::decode_vec(SECP1_SECRET) {
        //     Ok(decoded) => decoded,
        //     Err(e) => {
        //         tracing::debug!("issue decoding SECP1_SECRET: {e}");
        //         return Ok(Err(self.table().push(anyhow!("issue decoding SECP1_SECRET: {e}"))?));
        //     }
        // };

        // let signing_key: ecdsa::SigningKey<Secp256k1> =
        //     match ecdsa::SigningKey::from_slice(&decoded) {
        //         Ok(signing_key) => signing_key,
        //         Err(e) => {
        //             tracing::debug!("issue deserializing signing key: {e}");
        //             return Ok(Err(self
        //                 .table()
        //                 .push(anyhow!("issue deserializing signing key: {e}"))?));
        //         }
        //     };

        // let sig: ecdsa::Signature<Secp256k1> = signing_key.sign(&data);

        // Ok(Ok(sig.to_vec()))
    }

    async fn verifying_key(
        &mut self, _rep: Resource<KeySet>, _: KeyType,
    ) -> wasmtime::Result<Result<Jwk, Resource<Error>>> {
        Ok(Ok(Jwk {
            kid: None,
            kty: "OKP".into(),
            crv: "Ed25519".into(),
            x: ED25519_X.into(),
            y: None,
        }))

        // Ok(Ok(Jwk {
        //     kid: None,
        //     kty: "EC".into(),
        //     crv: "secp256k1".into(),
        //     x: SECP1_X.into(),
        //     y: Some(SECP1_Y.into(),),
        // }))
    }

    async fn delete(
        &mut self, _rep: Resource<KeySet>, _: KeyType,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        todo!()
    }

    async fn list_versions(
        &mut self, _rep: Resource<KeySet>, _: KeyType,
    ) -> wasmtime::Result<Result<Vec<Jwk>, Resource<Error>>> {
        Ok(Ok(vec![Jwk {
            kid: None,
            kty: "OKP".into(),
            crv: "Ed25519".into(),
            x: ED25519_X.into(),
            y: None,
        }]))

        // Ok(Ok(vec![Jwk {
        //     kid: None,
        //     kty: "EC".into(),
        //     crv: "secp256k1".into(),
        //     x: "tXSKB_rubXS7sCjXqupVJEzTcW3MsjmEvq1YpXn96Zg".into(),
        //     y: Some("dOicXqbjFxoGJ-K0-GJ1kHYJqic_D_OMuUwkQ7Ol6nk".into()),
        // }]))
    }

    fn drop(&mut self, rep: Resource<KeySet>) -> Result<(), wasmtime::Error> {
        tracing::debug!("keystore::HostKeySet::drop");
        self.table().delete(rep).map_or_else(|e| Err(anyhow!(e)), |_| Ok(()))
    }
}

#[async_trait::async_trait]
impl keystore::HostError for State {
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
