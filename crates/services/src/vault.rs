//! # WASI Vault Service
//!
//! This module implements a runtime service for `wasi:vault`
//! (<https://github.com/WebAssembly/wasi-vault>).

mod generated {
    #![allow(clippy::future_not_send)]

    pub use wasi::vault::keystore::Error;

    pub use super::{KeyPair, KeySet};

    wasmtime::component::bindgen!({
        world: "vault",
        path: "../../wit",
        tracing: true,
        async: true,
        trappable_imports: true,
        with: {
            "wasi:vault/keystore/key-set": KeySet,
            "wasi:vault/keystore/key-pair": KeyPair,
        },
        trappable_error_type: {
            "wasi:vault/keystore/error" => Error,
        },
        // additional_derives: [
        //     Clone,
        // ],
    });
}

// use anyhow::anyhow;
use azure_security_keyvault_keys::KeyClient; //ResourceExt
use azure_security_keyvault_keys::models::{SignParameters, SignatureAlgorithm};
use base64ct::{Base64UrlUnpadded, Encoding};
use runtime::Linkable;
use wasmtime::component::{Linker, Resource, ResourceTableError};
use wasmtime_wasi::ResourceTable;

use self::generated::wasi::vault;
use self::generated::wasi::vault::keystore::{self, Algorithm, Error, Jwk};
use crate::Ctx;

pub type Result<T, E = Error> = anyhow::Result<T, E>;
const ED25519_X: &str = "q6rjRnEH_XK72jvB8FNBJtOl9_gDs6NW49cAz6p2sW4";

#[derive(Clone)]
pub struct KeySet {
    identifier: String,
}

#[derive(Clone)]
pub struct KeyPair {
    name: String,
}

pub struct VaultHost<'a> {
    client: &'a KeyClient,
    table: &'a mut ResourceTable,
}

impl<'a> VaultHost<'a> {
    pub const fn new(client: &'a KeyClient, table: &'a mut ResourceTable) -> Self {
        Self { client, table }
    }
}

pub struct Service;

impl Linkable for Service {
    type Ctx = Ctx;

    // Add all the `wasi-keyvalue` world's interfaces to a [`Linker`], and
    // instantiate the `KeyvalueHost` for the component.
    fn add_to_linker(&self, linker: &mut Linker<Self::Ctx>) -> anyhow::Result<()> {
        add_to_linker(linker, |c: &mut Self::Ctx| {
            VaultHost::new(c.resources.az_client.wait(), &mut c.table)
        })?;
        tracing::trace!("added to linker");
        Ok(())
    }
}

fn add_to_linker<T: Send>(
    l: &mut Linker<T>, f: impl Fn(&mut T) -> VaultHost<'_> + Send + Sync + Copy + 'static,
) -> anyhow::Result<()> {
    vault::keystore::add_to_linker_get_host(l, f)
}

impl vault::keystore::Host for VaultHost<'_> {
    async fn open(&mut self, identifier: String) -> Result<Resource<KeySet>> {
        tracing::trace!("keystore::Host::open {identifier}");
        let key_set = KeySet { identifier };
        Ok(self.table.push(key_set)?)
    }

    async fn supported_algorithms(&mut self) -> anyhow::Result<Vec<Algorithm>> {
        Ok(vec![Algorithm::Eddsa])
    }

    fn convert_error(&mut self, err: Error) -> anyhow::Result<Error> {
        Ok(err)
    }
}

impl vault::keystore::HostKeySet for VaultHost<'_> {
    async fn generate(
        &mut self, _rep: Resource<KeySet>, _identifier: String, _alg: Algorithm,
    ) -> Result<Resource<KeyPair>, keystore::Error> {
        tracing::trace!("keystore::HostKeySet::generate");
        todo!("generate new key for KeyType")
    }

    async fn get(
        &mut self, rep: Resource<KeySet>, identifier: String,
    ) -> Result<Resource<KeyPair>, keystore::Error> {
        tracing::trace!("keystore::HostKeySet::get");

        let Ok(key_set) = self.table.get(&rep) else {
            return Err(keystore::Error::NoSuchKeySet);
        };
        tracing::debug!("key: {}-{identifier}", key_set.identifier);

        let key_pair = KeyPair {
            name: format!("{}-{identifier}", key_set.identifier),
        };

        // check key exists before saving reference
        let Ok(_) = self.client.get_key(&key_pair.name, "1", None).await else {
            return Err(keystore::Error::NoSuchKeyPair);
        };

        Ok(self.table.push(key_pair)?)
    }

    async fn delete(
        &mut self, _rep: Resource<KeySet>, _identifier: String,
    ) -> Result<(), keystore::Error> {
        tracing::trace!("keystore::HostKeySet::delete");
        todo!("generate new key for KeyType")
    }

    async fn drop(&mut self, rep: Resource<KeySet>) -> anyhow::Result<()> {
        tracing::trace!("keystore::HostKeySet::drop");
        self.table.delete(rep).map(|_| Ok(()))?
    }
}

impl vault::keystore::HostKeyPair for VaultHost<'_> {
    async fn sign(
        &mut self, rep: Resource<KeyPair>, data: Vec<u8>,
    ) -> Result<Vec<u8>, keystore::Error> {
        tracing::trace!("keystore::HostKeyPair::sign");

        let Ok(key_pair) = self.table.get(&rep) else {
            return Err(keystore::Error::NoSuchKeyPair);
        };

        let params: SignParameters = SignParameters {
            algorithm: Some(SignatureAlgorithm::ES256K),
            value: Some(data),
        };

        let sig_res = match self
            .client
            .sign(&key_pair.name, &SignatureAlgorithm::ES256K.to_string(), params.try_into()?, None)
            .await
        {
            Ok(digest) => digest,
            Err(e) => {
                return Err(keystore::Error::Other(format!("issue signing data: {e}")));
            }
        };

        Ok(sig_res.into_body().await.unwrap().result.unwrap())
    }

    async fn public_key(&mut self, rep: Resource<KeyPair>) -> Result<Jwk, keystore::Error> {
        tracing::trace!("keystore::HostKeyPair::public_key");

        let Ok(key_pair) = self.table.get_mut(&rep) else {
            return Err(keystore::Error::NoSuchKeyPair);
        };
        let kv_key = self.client.get_key(&key_pair.name, "1", None).await?.into_body().await?; //else {
        //     return Ok(Err(keystore::Error::NoSuchKeyPair));
        // };

        let Some(key) = kv_key.key else {
            return Err(keystore::Error::NoSuchKeyPair);
        };

        Ok(Jwk {
            kid: key.kid.clone(),
            kty: "EC".to_string(),
            crv: "secp256k1".to_string(),
            x: Base64UrlUnpadded::encode_string(&key.x.unwrap_or_default()),
            y: Some(Base64UrlUnpadded::encode_string(&key.y.unwrap_or_default())),
        })
    }

    async fn versions(&mut self, rep: Resource<KeyPair>) -> Result<Vec<Jwk>, keystore::Error> {
        tracing::trace!("keystore::HostKeySet::list_versions");

        let Ok(_key_set) = self.table.get_mut(&rep) else {
            return Err(keystore::Error::NoSuchKeySet);
        };

        Ok(vec![Jwk {
            kid: None,
            kty: "OKP".into(),
            crv: "Ed25519".into(),
            x: ED25519_X.into(),
            y: None,
        }])
    }

    async fn drop(&mut self, rep: Resource<KeyPair>) -> anyhow::Result<()> {
        tracing::trace!("keystore::HostKeyPair::drop");
        self.table.delete(rep).map(|_| Ok(()))?
    }
}

impl From<ResourceTableError> for Error {
    fn from(err: ResourceTableError) -> Self {
        Self::Other(err.to_string())
    }
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Self::Other(err.to_string())
    }
}

impl From<typespec_client_core::error::Error> for Error {
    fn from(err: typespec_client_core::error::Error) -> Self {
        Self::Other(err.to_string())
    }
}

// #[cfg(test)]
// mod tests {
//     use ecdsa::signature::Verifier;
//     use ecdsa::{Signature, VerifyingKey};
//     use k256::Secp256k1;

//     use super::*;

//     // const D: &str = "0Md3MhPaKEpnKAyKE498EdDFerD5NLeKJ5Rb-vC16Gs";
//     // const X: &str = "tXSKB_rubXS7sCjXqupVJEzTcW3MsjmEvq1YpXn96Zg";
//     // const Y: &str = "dOicXqbjFxoGJ-K0-GJ1kHYJqic_D_OMuUwkQ7Ol6nk";

//     #[tokio::test]
//     async fn public_key() {
//         dotenv::dotenv().ok();
//         env::set_var("AZURE_CREDENTIAL_KIND", "environment");

//         let credential = azure_identity::create_credential().expect("should create credential");
//         let client =
//             KeyClient::new("https://kv-credibil-demo.vault.azure.net", credential).unwrap();
//         let kv_key = client.get("funder-signing-key").await.expect("should get key");

//         let jwk = Jwk {
//             kid: kv_key.key.id.clone(),
//             kty: kv_key.key.key_type,
//             crv: "secp256k1".to_string(),
//             x: Base64UrlUnpadded::encode_string(&kv_key.key.x.unwrap()),
//             y: Some(Base64UrlUnpadded::encode_string(&kv_key.key.y.unwrap())),
//         };

//         println!("jwk: {:?}", jwk);
//     }

//     #[tokio::test]
//     async fn sign() {
//         dotenv::dotenv().ok();
//         env::set_var("AZURE_CREDENTIAL_KIND", "environment");

//         let payload = Base64UrlUnpadded::encode_string(b"hello world");
//         let digest = Base64UrlUnpadded::encode_string(&Sha256::digest(&payload));

//         let credential = azure_identity::create_credential().expect("should create credential");
//         let client =
//             KeyClient::new("https://kv-credibil-demo.vault.azure.net", credential).unwrap();
//         let sig_res = client
//             .sign("demo-credibil-io-signing-key", SignatureAlgorithm::ES256K, digest)
//             .await
//             .expect("should sign data");

//         //  verifying key
//         let kv_key = client.get("demo-credibil-io-signing-key").await.expect("should get key");
//         let mut sec1 = vec![0x04]; // uncompressed format
//         sec1.append(&mut kv_key.key.x.unwrap());
//         sec1.append(&mut kv_key.key.y.unwrap());
//         let verifying_key = VerifyingKey::<Secp256k1>::from_sec1_bytes(&sec1).unwrap();

//         let signature: Signature<Secp256k1> =
//             Signature::from_slice(&sig_res.signature).expect("should get signature");
//         let normalised = signature.normalize_s().unwrap_or(signature);

//         match verifying_key.verify(payload.as_bytes(), &normalised) {
//             Ok(_) => println!("VERIFICATION PASSED"),
//             Err(_) => panic!("VERIFICATION FAILED"),
//         }
//     }
// }
