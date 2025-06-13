//! # WASI Vault Service
//!
//! This module implements a runtime service for `wasi:vault`
//! (<https://github.com/WebAssembly/wasi-vault>).

mod generated {
    #![allow(clippy::trait_duplication_in_bounds)]

    pub use self::wasi::vault::keystore::Error;
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
    });
}

use azure_security_keyvault_keys::models::{
    CurveName, JsonWebKey, KeyType, SignParameters, SignatureAlgorithm,
};
use base64ct::{Base64UrlUnpadded, Encoding};
use runtime::Linkable;
use sha2::{Digest, Sha256};
use wasmtime::component::{Linker, Resource, ResourceTableError};
use wasmtime_wasi::ResourceTable;

use self::generated::wasi::vault;
use self::generated::wasi::vault::keystore::{Algorithm, Error, Jwk};
use crate::{Ctx, Resources};

pub type Result<T, E = Error> = anyhow::Result<T, E>;

#[derive(Clone)]
pub struct KeySet {
    identifier: String,
}

#[derive(Clone)]
pub struct KeyPair {
    name: String,
}

pub struct VaultHost<'a> {
    resources: &'a Resources,
    table: &'a mut ResourceTable,
}

impl VaultHost<'_> {
    const fn new(c: &mut Ctx) -> VaultHost<'_> {
        VaultHost {
            resources: &c.resources,
            table: &mut c.table,
        }
    }
}

pub struct Service;

impl Linkable for Service {
    type Ctx = Ctx;

    // Add all the `wasi-keyvalue` world's interfaces to a [`Linker`], and
    // instantiate the `VaultHost` for the component.
    fn add_to_linker(&self, linker: &mut Linker<Self::Ctx>) -> anyhow::Result<()> {
        vault::keystore::add_to_linker_get_host(linker, VaultHost::new)?;
        tracing::trace!("added to linker");
        Ok(())
    }
}

impl vault::keystore::Host for VaultHost<'_> {
    async fn open(&mut self, owner: String) -> Result<Resource<KeySet>> {
        tracing::trace!("keystore::Host::open {owner}");
        let key_set = KeySet { identifier: owner };
        Ok(self.table.push(key_set)?)
    }

    async fn supported_algorithms(&mut self) -> anyhow::Result<Vec<Algorithm>> {
        Ok(vec![Algorithm::Eddsa])
    }

    fn convert_error(&mut self, error: Error) -> anyhow::Result<Error> {
        Ok(error)
    }
}

impl vault::keystore::HostKeySet for VaultHost<'_> {
    async fn generate(
        &mut self, _rep: Resource<KeySet>, _identifier: String, _alg: Algorithm,
    ) -> Result<Resource<KeyPair>, Error> {
        tracing::trace!("keystore::HostKeySet::generate");
        todo!("generate new key for KeyType")
    }

    async fn get(
        &mut self, rep: Resource<KeySet>, purpose: String,
    ) -> Result<Resource<KeyPair>, Error> {
        tracing::trace!("keystore::HostKeySet::get");

        let Ok(key_set) = self.table.get(&rep) else {
            return Err(Error::NoSuchKeySet);
        };
        tracing::debug!("key: {owner}-{purpose}", owner = key_set.identifier);

        let key_pair = KeyPair {
            name: format!("{owner}-{purpose}", owner = key_set.identifier),
        };

        // check key exists before saving reference
        if let Err(e) = self.resources.azkeyvault()?.get_key(&key_pair.name, "", None).await {
            tracing::error!("key {} cannot be found: {e}", key_pair.name);
            return Err(Error::NoSuchKeyPair);
        }

        Ok(self.table.push(key_pair)?)
    }

    async fn delete(&mut self, _rep: Resource<KeySet>, _owner: String) -> Result<(), Error> {
        tracing::trace!("keystore::HostKeySet::delete");
        todo!("delete key for KeyType")
    }

    async fn drop(&mut self, rep: Resource<KeySet>) -> anyhow::Result<()> {
        tracing::trace!("keystore::HostKeySet::drop");
        self.table.delete(rep).map(|_| Ok(()))?
    }
}

impl vault::keystore::HostKeyPair for VaultHost<'_> {
    async fn sign(&mut self, rep: Resource<KeyPair>, data: Vec<u8>) -> Result<Vec<u8>, Error> {
        tracing::trace!("keystore::HostKeyPair::sign");

        let Ok(key_pair) = self.table.get(&rep) else {
            return Err(Error::NoSuchKeyPair);
        };

        let digest = &Sha256::digest(&data);

        let params: SignParameters = SignParameters {
            algorithm: Some(SignatureAlgorithm::ES256K),
            value: Some(digest.to_vec()),
        };

        let client = self.resources.azkeyvault()?;
        let sig_res = client.sign(&key_pair.name, "", params.try_into()?, None).await?;

        Ok(sig_res.into_body().await?.result.unwrap_or(vec![]))
    }

    async fn public_key(&mut self, rep: Resource<KeyPair>) -> Result<Jwk, Error> {
        tracing::trace!("keystore::HostKeyPair::public_key");

        // retrieve the key from the key vault
        let Ok(key_pair) = self.table.get_mut(&rep) else {
            return Err(Error::NoSuchKeyPair);
        };
        let client = self.resources.azkeyvault()?;
        let kv_key = client.get_key(&key_pair.name, "", None).await?.into_body().await?;
        let Some(key) = kv_key.key else {
            return Err(Error::NoSuchKeyPair);
        };

        Ok(az_to_jwk(key))
    }

    async fn versions(&mut self, _: Resource<KeyPair>) -> Result<Vec<Jwk>, Error> {
        tracing::trace!("keystore::HostKeySet::list_versions");
        todo!("list key versions");
    }

    async fn drop(&mut self, rep: Resource<KeyPair>) -> anyhow::Result<()> {
        tracing::trace!("keystore::HostKeyPair::drop");
        self.table.delete(rep).map(|_| Ok(()))?
    }
}

fn az_to_jwk(key: JsonWebKey) -> Jwk {
    Jwk {
        kid: key.kid.clone(),
        kty: key.kty.unwrap_or(KeyType::EC).to_string(),
        crv: key.crv.unwrap_or(CurveName::P256K).to_string(),
        x: Base64UrlUnpadded::encode_string(&key.x.unwrap_or_default()),
        y: Some(Base64UrlUnpadded::encode_string(&key.y.unwrap_or_default())),
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

impl From<azure_core::error::Error> for Error {
    fn from(err: azure_core::error::Error) -> Self {
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
