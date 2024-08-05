//! # WASI Vault Capability
//!
//! This module implements a runtime capability for `wasi:vault`
//! (<https://github.com/WebAssembly/wasi-vault>).

use std::sync::OnceLock;
use std::{env, vec};

use anyhow::anyhow;
// use azure_security_keyvault::prelude::SignatureAlgorithm;
use azure_security_keyvault::KeyClient;
use base64ct::{Base64UrlUnpadded, Encoding};
use bindings::wasi::vault::keystore::{self, Algorithm, Jwk};
use bindings::Vault;
// use sha2::{Digest, Sha256};
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
    fn namespace(&self) -> &str {
        "wasi:vault"
    }

    fn add_to_linker(&self, linker: &mut Linker<State>) -> anyhow::Result<()> {
        Vault::add_to_linker(linker, |t| t)
    }

    /// Provide vault capability for the wasm component.
    async fn run(&self, _runtime: Runtime) -> anyhow::Result<()> {
        env::set_var("AZURE_CREDENTIAL_KIND", "environment");

        let credential = azure_identity::create_credential()
            .map_err(|e| anyhow!("could not create credential: {e}"))?;
        let client = KeyClient::new("https://kv-credibil-demo.vault.azure.net", credential)
            .map_err(|e| anyhow!("issue creating client: {e}"))?;
        CLIENT.get_or_init(|| client);

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
        tracing::debug!("key: {}-{identifier}", key_set.identifier);

        let Some(client) = CLIENT.get() else {
            return Ok(Err(keystore::Error::Other("no key client".into())));
        };
        let key_pair = KeyPair {
            name: "demo-credibil-io-signing-key".to_string(),
            // name: format!("{}-{identifier}", key_set.identifier),
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
        tracing::debug!("keystore::HostKeySet::delete");
        todo!("generate new key for KeyType")
    }

    fn drop(&mut self, rep: Resource<KeySet>) -> Result<(), wasmtime::Error> {
        tracing::debug!("keystore::HostKeySet::drop");
        self.table().delete(rep).map_or_else(|e| Err(anyhow!(e)), |_| Ok(()))
    }
}

use ecdsa::signature::Signer;
use ecdsa::Signature;
use k256::ecdsa::SigningKey;
use k256::Secp256k1;

const D: &str = "0Md3MhPaKEpnKAyKE498EdDFerD5NLeKJ5Rb-vC16Gs";
const X: &str = "tXSKB_rubXS7sCjXqupVJEzTcW3MsjmEvq1YpXn96Zg";
const Y: &str = "dOicXqbjFxoGJ-K0-GJ1kHYJqic_D_OMuUwkQ7Ol6nk";

#[async_trait::async_trait]
impl keystore::HostKeyPair for State {
    async fn sign(
        &mut self, _rep: Resource<KeyPair>, data: Vec<u8>,
    ) -> wasmtime::Result<Result<Vec<u8>, keystore::Error>> {
        tracing::debug!("keystore::HostKeyPair::sign");

        // let Ok(key_pair) = self.table().get(&rep) else {
        //     return Ok(Err(keystore::Error::NoSuchKeyPair));
        // };

        // let Some(client) = CLIENT.get() else {
        //     return Ok(Err(keystore::Error::Other("no key client".into())));
        // };

        // // hash data
        // let digest = Base64UrlUnpadded::encode_string(&Sha256::digest(&data));
        // let sig_res = match client.sign(&key_pair.name, SignatureAlgorithm::ES256K, digest).await {
        //     Ok(digest) => digest,
        //     Err(e) => return Ok(Err(keystore::Error::Other(format!("issue signing data: {e}")))),
        // };

        // Ok(Ok(sig_res.signature))

        let decoded = Base64UrlUnpadded::decode_vec(D).expect("should decode");
        let bytes: [u8; 32] = decoded.as_slice().try_into().expect("should convert");
        let signing_key = SigningKey::from_bytes(&bytes.into()).expect("should create signing key");
        let signature: Signature<Secp256k1> = signing_key.sign(&data);

        Ok(Ok(signature.to_vec()))
    }

    async fn public_key(
        &mut self, _rep: Resource<KeyPair>,
    ) -> wasmtime::Result<Result<Jwk, keystore::Error>> {
        tracing::debug!("keystore::HostKeyPair::public_key");

        // let Ok(key_pair) = self.table().get_mut(&rep) else {
        //     return Ok(Err(keystore::Error::NoSuchKeyPair));
        // };
        // let Some(client) = CLIENT.get() else {
        //     return Ok(Err(keystore::Error::Other("no key client".into())));
        // };
        // let Ok(kv_key) = client.get(&key_pair.name).await else {
        //     return Ok(Err(keystore::Error::NoSuchKeyPair));
        // };

        // Ok(Ok(Jwk {
        //     kid: kv_key.key.id.clone(),
        //     kty: kv_key.key.key_type,
        //     crv: "secp256k1".to_string(),
        //     x: Base64UrlUnpadded::encode_string(&kv_key.key.x.unwrap_or_default()),
        //     y: Some(Base64UrlUnpadded::encode_string(&kv_key.key.y.unwrap_or_default())),
        // }))

        Ok(Ok(Jwk {
            kid: None,
            kty: "EC".to_string(),
            crv: "secp256k1".to_string(),
            x: X.to_string(),
            y: Some(Y.to_string()),
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
        tracing::debug!("keystore::HostKeyPair::drop");
        self.table().delete(rep).map_or_else(|e| Err(anyhow!(e)), |_| Ok(()))
    }
}

#[cfg(test)]
mod tests {
    use azure_security_keyvault::prelude::SignatureAlgorithm;
    use ecdsa::signature::{Signer, Verifier};
    use ecdsa::{Signature, VerifyingKey};
    use k256::ecdsa::SigningKey;
    use k256::Secp256k1;
    use sha2::{Digest, Sha256};

    use super::*;

    #[tokio::test]
    async fn test_public_key() {
        dotenv::dotenv().ok();
        env::set_var("AZURE_CREDENTIAL_KIND", "environment");

        let credential = azure_identity::create_credential().expect("should create credential");
        let client =
            KeyClient::new("https://kv-credibil-demo.vault.azure.net", credential).unwrap();
        let kv_key = client.get("demo-credibil-io-signing-key").await.expect("should get key");

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
    async fn test_sign() {
        // d: 0Md3MhPaKEpnKAyKE498EdDFerD5NLeKJ5Rb-vC16Gs
        // x: tXSKB_rubXS7sCjXqupVJEzTcW3MsjmEvq1YpXn96Zg
        // y: dOicXqbjFxoGJ-K0-GJ1kHYJqic_D_OMuUwkQ7Ol6nk

        let payload = "hello world";

        // signing key
        let decoded = Base64UrlUnpadded::decode_vec("0Md3MhPaKEpnKAyKE498EdDFerD5NLeKJ5Rb-vC16Gs")
            .expect("should decode");
        let bytes: [u8; 32] = decoded.as_slice().try_into().unwrap();
        let signing_key = SigningKey::from_bytes(&bytes.into()).unwrap();

        // verifying key
        let mut sec1 = vec![0x04]; // uncompressed format
        sec1.append(
            &mut Base64UrlUnpadded::decode_vec("tXSKB_rubXS7sCjXqupVJEzTcW3MsjmEvq1YpXn96Zg")
                .unwrap(),
        );
        sec1.append(
            &mut Base64UrlUnpadded::decode_vec("dOicXqbjFxoGJ-K0-GJ1kHYJqic_D_OMuUwkQ7Ol6nk")
                .unwrap(),
        );
        let verifying_key = VerifyingKey::<Secp256k1>::from_sec1_bytes(&sec1).unwrap();

        // sign & verify
        let signature: Signature<Secp256k1> = signing_key.sign(payload.as_bytes());

        if let Err(e) = verifying_key.verify(payload.as_bytes(), &signature) {
            println!("signature verification failed: {e}");
        }
    }

    #[tokio::test]
    async fn test_sign2() {
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

        match verifying_key.verify(payload.as_bytes(), &signature) {
            Ok(_) => println!("VERIFICATION PASSED"),
            Err(_) => panic!("VERIFICATION FAILED"),
        }
    }
}
