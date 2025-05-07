//! # WASI Vault Capability
//!
//! This module implements a runtime capability for `wasi:vault`
//! (<https://github.com/WebAssembly/wasi-vault>).

/// Wrap generation of wit bindings to simplify exports.
/// See <https://docs.rs/wasmtime/latest/wasmtime/component/macro.bindgen.html>
mod generated {
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

use std::sync::OnceLock;
use std::{env, vec};

use anyhow::anyhow;
use azure_security_keyvault::KeyClient;
use azure_security_keyvault::prelude::SignatureAlgorithm;
use base64ct::{Base64UrlUnpadded, Encoding};
use sha2::{Digest, Sha256};
use wasmtime::component::{InstancePre, Linker, Resource};
use wasmtime_wasi::IoView;

use self::generated::Vault;
use self::generated::wasi::vault::keystore::{self, Algorithm, Jwk};
use crate::runtime::{self, Ctx};

const ED25519_X: &str = "q6rjRnEH_XK72jvB8FNBJtOl9_gDs6NW49cAz6p2sW4";

static CLIENT: OnceLock<KeyClient> = OnceLock::new();

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
    fn namespace(&self) -> &'static str {
        "wasi:vault"
    }

    fn add_to_linker(&self, linker: &mut Linker<Ctx>) -> anyhow::Result<()> {
        Vault::add_to_linker(linker, |t| t)
    }

    /// Provide vault capability for the wasm component.
    async fn start(&self, _runtime: Runtime) -> anyhow::Result<()> {
        env::set_var("AZURE_CREDENTIAL_KIND", "environment");

        let credential = azure_identity::create_credential()
            .map_err(|e| anyhow!("could not create credential: {e}"))?;
        let client = KeyClient::new("https://kv-credibil-demo.vault.azure.net", credential)
            .map_err(|e| anyhow!("issue creating client: {e}"))?;
        CLIENT.get_or_init(|| client);

        Ok(())
    }
}

impl keystore::Host for Ctx {
    async fn open(
        &mut self, identifier: String,
    ) -> wasmtime::Result<Result<Resource<KeySet>, keystore::Error>> {
        tracing::trace!("keystore::Host::open {identifier}");

        let key_set = KeySet { identifier };
        Ok(Ok(self.table().push(key_set)?))
    }

    async fn supported_algorithms(&mut self) -> wasmtime::Result<Vec<Algorithm>> {
        Ok(vec![Algorithm::Eddsa])
    }
}

impl keystore::HostKeySet for Ctx {
    async fn generate(
        &mut self, _rep: Resource<KeySet>, _identifier: String, _alg: Algorithm,
    ) -> wasmtime::Result<Result<Resource<KeyPair>, keystore::Error>> {
        tracing::trace!("keystore::HostKeySet::generate");
        todo!("generate new key for KeyType")
    }

    async fn get(
        &mut self, rep: Resource<KeySet>, identifier: String,
    ) -> wasmtime::Result<Result<Resource<KeyPair>, keystore::Error>> {
        tracing::trace!("keystore::HostKeySet::get");

        let Ok(key_set) = self.table().get(&rep) else {
            return Ok(Err(keystore::Error::NoSuchKeySet));
        };
        tracing::debug!("key: {}-{identifier}", key_set.identifier);

        let Some(client) = CLIENT.get() else {
            return Ok(Err(keystore::Error::Other("no key client".into())));
        };
        let key_pair = KeyPair {
            name: format!("{}-{identifier}", key_set.identifier),
        };

        // check key exists before saving reference
        let Ok(_) = client.get(&key_pair.name).await else {
            return Ok(Err(keystore::Error::NoSuchKeyPair));
        };

        Ok(Ok(self.table().push(key_pair)?))
    }

    async fn delete(
        &mut self, _rep: Resource<KeySet>, _identifier: String,
    ) -> wasmtime::Result<Result<(), keystore::Error>> {
        tracing::trace!("keystore::HostKeySet::delete");
        todo!("generate new key for KeyType")
    }

    async fn drop(&mut self, rep: Resource<KeySet>) -> Result<(), wasmtime::Error> {
        tracing::trace!("keystore::HostKeySet::drop");
        self.table().delete(rep).map_or_else(|e| Err(anyhow!(e)), |_| Ok(()))
    }
}

impl keystore::HostKeyPair for Ctx {
    async fn sign(
        &mut self, rep: Resource<KeyPair>, data: Vec<u8>,
    ) -> wasmtime::Result<Result<Vec<u8>, keystore::Error>> {
        tracing::trace!("keystore::HostKeyPair::sign");

        let Ok(key_pair) = self.table().get(&rep) else {
            return Ok(Err(keystore::Error::NoSuchKeyPair));
        };

        let Some(client) = CLIENT.get() else {
            return Ok(Err(keystore::Error::Other("no key client".into())));
        };

        // hash data
        let digest = Base64UrlUnpadded::encode_string(&Sha256::digest(&data));
        let sig_res = match client.sign(&key_pair.name, SignatureAlgorithm::ES256K, digest).await {
            Ok(digest) => digest,
            Err(e) => return Ok(Err(keystore::Error::Other(format!("issue signing data: {e}")))),
        };

        Ok(Ok(sig_res.signature))
    }

    async fn public_key(
        &mut self, rep: Resource<KeyPair>,
    ) -> wasmtime::Result<Result<Jwk, keystore::Error>> {
        tracing::trace!("keystore::HostKeyPair::public_key");

        let Ok(key_pair) = self.table().get_mut(&rep) else {
            return Ok(Err(keystore::Error::NoSuchKeyPair));
        };
        let Some(client) = CLIENT.get() else {
            return Ok(Err(keystore::Error::Other("no key client".into())));
        };
        let Ok(kv_key) = client.get(&key_pair.name).await else {
            return Ok(Err(keystore::Error::NoSuchKeyPair));
        };

        Ok(Ok(Jwk {
            kid: kv_key.key.id.clone(),
            kty: kv_key.key.key_type,
            crv: "secp256k1".to_string(),
            x: Base64UrlUnpadded::encode_string(&kv_key.key.x.unwrap_or_default()),
            y: Some(Base64UrlUnpadded::encode_string(&kv_key.key.y.unwrap_or_default())),
        }))
    }

    async fn versions(
        &mut self, rep: Resource<KeyPair>,
    ) -> wasmtime::Result<Result<Vec<Jwk>, keystore::Error>> {
        tracing::trace!("keystore::HostKeySet::list_versions");

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
        tracing::trace!("keystore::HostKeyPair::drop");
        self.table().delete(rep).map_or_else(|e| Err(anyhow!(e)), |_| Ok(()))
    }
}

#[cfg(test)]
mod tests {
    use ecdsa::signature::Verifier;
    use ecdsa::{Signature, VerifyingKey};
    use k256::Secp256k1;

    use super::*;

    // const D: &str = "0Md3MhPaKEpnKAyKE498EdDFerD5NLeKJ5Rb-vC16Gs";
    // const X: &str = "tXSKB_rubXS7sCjXqupVJEzTcW3MsjmEvq1YpXn96Zg";
    // const Y: &str = "dOicXqbjFxoGJ-K0-GJ1kHYJqic_D_OMuUwkQ7Ol6nk";

    #[tokio::test]
    async fn public_key() {
        dotenv::dotenv().ok();
        env::set_var("AZURE_CREDENTIAL_KIND", "environment");

        let credential = azure_identity::create_credential().expect("should create credential");
        let client =
            KeyClient::new("https://kv-credibil-demo.vault.azure.net", credential).unwrap();
        let kv_key = client.get("funder-signing-key").await.expect("should get key");

        let jwk = Jwk {
            kid: kv_key.key.id.clone(),
            kty: kv_key.key.key_type,
            crv: "secp256k1".to_string(),
            x: Base64UrlUnpadded::encode_string(&kv_key.key.x.unwrap()),
            y: Some(Base64UrlUnpadded::encode_string(&kv_key.key.y.unwrap())),
        };

        println!("jwk: {:?}", jwk);
    }

    #[tokio::test]
    async fn sign() {
        dotenv::dotenv().ok();
        env::set_var("AZURE_CREDENTIAL_KIND", "environment");

        let payload = Base64UrlUnpadded::encode_string(b"hello world");
        let digest = Base64UrlUnpadded::encode_string(&Sha256::digest(&payload));

        let credential = azure_identity::create_credential().expect("should create credential");
        let client =
            KeyClient::new("https://kv-credibil-demo.vault.azure.net", credential).unwrap();
        let sig_res = client
            .sign("demo-credibil-io-signing-key", SignatureAlgorithm::ES256K, digest)
            .await
            .expect("should sign data");

        //  verifying key
        let kv_key = client.get("demo-credibil-io-signing-key").await.expect("should get key");
        let mut sec1 = vec![0x04]; // uncompressed format
        sec1.append(&mut kv_key.key.x.unwrap());
        sec1.append(&mut kv_key.key.y.unwrap());
        let verifying_key = VerifyingKey::<Secp256k1>::from_sec1_bytes(&sec1).unwrap();

        let signature: Signature<Secp256k1> =
            Signature::from_slice(&sig_res.signature).expect("should get signature");
        let normalised = signature.normalize_s().unwrap_or(signature);

        match verifying_key.verify(payload.as_bytes(), &normalised) {
            Ok(_) => println!("VERIFICATION PASSED"),
            Err(_) => panic!("VERIFICATION FAILED"),
        }
    }
}
