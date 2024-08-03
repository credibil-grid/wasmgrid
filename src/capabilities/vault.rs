//! # WASI Vault Capability
//!
//! This module implements a runtime capability for `wasi:vault`
//! (<https://github.com/WebAssembly/wasi-vault>).

use std::sync::OnceLock;
use std::vec;

use anyhow::anyhow;
use azure_security_keyvault::prelude::SignatureAlgorithm;
use azure_security_keyvault::KeyvaultClient;
use base64ct::{Base64UrlUnpadded, Encoding};
use bindings::wasi::vault::keystore::{self, Algorithm, Jwk};
use bindings::Vault;
use sha2::{Digest, Sha256};
use wasmtime::component::{Linker, Resource};
use wasmtime_wasi::WasiView;

use crate::runtime::{self, Runtime, State};

/// Wrap generation of wit bindings to simplify exports.
/// See <https://docs.rs/wasmtime/latest/wasmtime/component/macro.bindgen.html>
mod bindings {
    #![allow(clippy::future_not_send)]

    pub use super::{KeyPair, KeySet};

    wasmtime::component::bindgen!({
        world: "vault",
        path: "wit",
        tracing: true,
        async: true,
        trappable_imports: true,
        with: {
            "wasi:vault/keystore/key-set": KeySet,
            "wasi:vault/keystore/key-pair": KeyPair,
        },
        additional_derives: [
            Clone,
        ],
        // include_generated_code_from_file: true,
    });
}

const ED25519_X: &str = "q6rjRnEH_XK72jvB8FNBJtOl9_gDs6NW49cAz6p2sW4";

static KV_CLIENT: OnceLock<KeyvaultClient> = OnceLock::new();

#[derive(Clone)]
pub struct KeySet {
    identifier: String,
}

#[derive(Clone)]
pub struct KeyPair {
    name: String,
}

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
        let credential = azure_identity::create_credential()
            .map_err(|e| anyhow!("could not create credential: {e}"))?;

        let client = KeyvaultClient::new("https://kv-credibil-demo.vault.azure.net", credential)
            .map_err(|e| anyhow!("issue creating client: {e}"))?;
        KV_CLIENT.get_or_init(|| client);

        Ok(())
    }
}

#[async_trait::async_trait]
impl keystore::Host for State {
    async fn open(
        &mut self, identifier: String,
    ) -> wasmtime::Result<Result<Resource<KeySet>, keystore::Error>> {
        tracing::debug!("keystore::Host::open {identifier}");

        // sanitise the identifier so it can be used as a key name
        let identifier = identifier
            .strip_prefix("https://")
            .map_or(identifier.as_str(), |sanitised| sanitised)
            .replace(['.', '/'], "-");

        let key_set = KeySet { identifier };
        Ok(Ok(self.table().push(key_set)?))
    }

    async fn supported_algorithms(&mut self) -> wasmtime::Result<Vec<Algorithm>> {
        Ok(vec![Algorithm::Eddsa])
    }
}

#[async_trait::async_trait]
impl keystore::HostKeySet for State {
    async fn generate(
        &mut self, _rep: Resource<KeySet>, _identifier: String, _alg: Algorithm,
    ) -> wasmtime::Result<Result<Resource<KeyPair>, keystore::Error>> {
        tracing::debug!("keystore::HostKeySet::generate");

        todo!("generate new key for KeyType")
    }

    async fn get(
        &mut self, rep: Resource<KeySet>, identifier: String,
    ) -> wasmtime::Result<Result<Resource<KeyPair>, keystore::Error>> {
        tracing::debug!("keystore::HostKeySet::get");

        let Ok(key_set) = self.table().get(&rep) else {
            return Ok(Err(keystore::Error::NoSuchKeySet));
        };

        let name = format!("{}-{identifier}", key_set.identifier);
        let key_pair = KeyPair { name };
        Ok(Ok(self.table().push(key_pair)?))
    }

    async fn delete(
        &mut self, _rep: Resource<KeySet>, _identifier: String,
    ) -> wasmtime::Result<Result<(), keystore::Error>> {
        tracing::debug!("keystore::HostKeySet::delete");
        todo!("generate new key for KeyType")
    }

    fn drop(&mut self, rep: Resource<KeySet>) -> Result<(), wasmtime::Error> {
        tracing::debug!("keystore::HostKeySet::drop");
        self.table().delete(rep).map_or_else(|e| Err(anyhow!(e)), |_| Ok(()))
    }
}

#[async_trait::async_trait]
impl keystore::HostKeyPair for State {
    async fn sign(
        &mut self, rep: Resource<KeyPair>, data: Vec<u8>,
    ) -> wasmtime::Result<Result<Vec<u8>, keystore::Error>> {
        tracing::debug!("keystore::HostKeySet::sign");

        let Ok(key_pair) = self.table().get(&rep) else {
            return Ok(Err(keystore::Error::NoSuchKeySet));
        };
        let Some(client) = KV_CLIENT.get() else {
            return Ok(Err(keystore::Error::NoSuchKeySet));
        };

        // hash data
        let mut hasher = Sha256::new();
        hasher.update(data);
        let bytes = hasher.finalize();
        let Ok(digest) = std::str::from_utf8(&bytes) else {
            return Ok(Err(keystore::Error::NoSuchKeySet));
        };

        let Ok(sig_res) =
            client.key_client().sign(&key_pair.name, SignatureAlgorithm::ES256K, digest).await
        else {
            return Ok(Err(keystore::Error::NoSuchKeySet));
        };

        Ok(Ok(sig_res.signature))
    }

    async fn public_key(
        &mut self, rep: Resource<KeyPair>,
    ) -> wasmtime::Result<Result<Jwk, keystore::Error>> {
        tracing::debug!("keystore::HostKeySet::verifying_key");

        let Ok(_key_pair) = self.table().get_mut(&rep) else {
            return Ok(Err(keystore::Error::NoSuchKeySet));
        };
        let Some(client) = KV_CLIENT.get() else {
            return Ok(Err(keystore::Error::NoSuchKeySet));
        };

        let Ok(key_pair) = client.key_client().get("demo-credibil-io-supplier-signing-key").await
        else {
            return Ok(Err(keystore::Error::NoSuchKeyPair));
        };

        Ok(Ok(Jwk {
            kid: key_pair.key.id.clone(),
            kty: key_pair.key.key_type,
            crv: "secp256k1".to_string(),
            x: Base64UrlUnpadded::encode_string(&key_pair.key.x.unwrap_or_default()),
            y: Some(Base64UrlUnpadded::encode_string(&key_pair.key.y.unwrap_or_default())),
        }))
    }

    async fn versions(
        &mut self, rep: Resource<KeyPair>,
    ) -> wasmtime::Result<Result<Vec<Jwk>, keystore::Error>> {
        tracing::debug!("keystore::HostKeySet::list_versions");

        let Ok(_key_set) = self.table().get_mut(&rep) else {
            return Ok(Err(keystore::Error::NoSuchKeySet));
        };

        Ok(Ok(vec![Jwk {
            kid: None,
            kty: "OKP".into(),
            crv: "Ed25519".into(),
            x: ED25519_X.into(),
            y: None,
        }]))
    }

    fn drop(&mut self, rep: Resource<KeyPair>) -> Result<(), wasmtime::Error> {
        tracing::debug!("keystore::HostKeySet::drop");
        self.table().delete(rep).map_or_else(|e| Err(anyhow!(e)), |_| Ok(()))
    }
}

#[cfg(test)]
mod tests {


    use super::*;

    #[tokio::test]
    async fn test_connect() {
        dotenv::dotenv().ok();

        let credential = azure_identity::create_credential().expect("should create credential");
        let client =
            KeyvaultClient::new("https://kv-credibil-demo.vault.azure.net", credential).unwrap();
        let kv_key = client
            .key_client()
            .get("demo-credibil-io-supplier-signing-key")
            .await
            .expect("should get key");

        let jwk = Jwk {
            kid: kv_key.key.id.clone(),
            kty: kv_key.key.key_type,
            crv: "secp256k1".to_string(),
            x: Base64UrlUnpadded::encode_string(&kv_key.key.x.unwrap()),
            y: Some(Base64UrlUnpadded::encode_string(&kv_key.key.y.unwrap())),
        };

        println!("jwk: {:?}", jwk);
    }
}
