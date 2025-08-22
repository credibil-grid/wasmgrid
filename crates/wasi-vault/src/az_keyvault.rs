//! # WASI Key/Value Service
//!
//! This module implements a runtime service for `wasi:vault`
//! (<https://github.com/WebAssembly/wasi-vault>).

/// Wrap generation of wit bindings to simplify exports.
/// See <https://docs.rs/wasmtime/latest/wasmtime/component/macro.bindgen.html>
mod generated {
    #![allow(clippy::trait_duplication_in_bounds)]

    pub use self::wasi::vault::vault::Error;
    pub use super::Locker;

    wasmtime::component::bindgen!({
        world: "vault",
        path: "../../wit",
        tracing: true,
        async: true,
        trappable_imports: true,
        with: {
            "wasi:vault/vault/locker": Locker,
        },
        trappable_error_type: {
            "wasi:vault/vault/error" => Error,
        },
    });
}

use anyhow::Context;
use azure_security_keyvault_secrets::models::{Secret, SetSecretParameters};
use base64ct::{Base64UrlUnpadded, Encoding};
use futures::TryStreamExt;
use http::StatusCode;
use resources::Resources;
use runtime::Linkable;
use wasi_core::Ctx;
use wasmtime::component::{HasData, Linker, Resource, ResourceTableError};
use wasmtime_wasi::ResourceTable;

use self::generated::wasi::vault::vault;
use self::generated::wasi::vault::vault::Error;

pub type Result<T, E = Error> = anyhow::Result<T, E>;

pub struct Vault<'a> {
    resources: &'a Resources,
    table: &'a mut ResourceTable,
}

impl Vault<'_> {
    const fn new(c: &mut Ctx) -> Vault<'_> {
        Vault {
            resources: &c.resources,
            table: &mut c.table,
        }
    }
}

struct Data;
impl HasData for Data {
    type Data<'a> = Vault<'a>;
}

pub struct Service;

impl Linkable for Service {
    type Ctx = Ctx;

    // Add all the `wasi-vault` world's interfaces to a [`Linker`], and
    // instantiate the `Vault` for the component.
    fn add_to_linker(&self, linker: &mut Linker<Self::Ctx>) -> anyhow::Result<()> {
        vault::add_to_linker::<_, Data>(linker, Vault::new)
    }
}

pub struct Locker {
    identifier: String,
}

// Implement the [`wasi_vault::Host`]` trait for  Vault<'_>.
impl vault::Host for Vault<'_> {
    // Open locker specified by identifier, save to state and return as a resource.
    async fn open(&mut self, identifier: String) -> Result<Resource<Locker>> {
        let locker = Locker { identifier };
        Ok(self.table.push(locker)?)
    }

    fn convert_error(&mut self, err: Error) -> anyhow::Result<Error> {
        tracing::error!("{err}");
        Ok(err)
    }
}

impl vault::HostLocker for Vault<'_> {
    async fn get(
        &mut self, locker_ref: Resource<Locker>, secret_id: String,
    ) -> Result<Option<Vec<u8>>> {
        let Ok(locker) = self.table.get(&locker_ref) else {
            return Err(Error::NoSuchStore);
        };
        let secret_name = format!("{}-{secret_id}", locker.identifier);
        tracing::debug!("getting secret named: {secret_name}");
        let secret_id = Base64UrlUnpadded::encode_string(secret_name.as_bytes());

        let kv = self.resources.azkeyvault().context("connecting to Azure KeyVault")?;
        let result = kv.get_secret(&secret_id, "", None).await;
        let response = match result {
            Ok(resp) => resp,
            Err(e) => {
                if let Some(code) = e.http_status()
                    && code == StatusCode::NOT_FOUND.as_u16()
                {
                    return Ok(None);
                }
                return Err(Error::Other(format!("issue getting secret: {e}")));
            }
        };

        let secret: Secret = response.into_body().await.context("issue deserializing secret")?;
        let Some(value) = secret.value else {
            return Ok(None);
        };
        let decoded = Base64UrlUnpadded::decode_vec(&value).context("issue decoding secret")?;

        Ok(Some(decoded))
    }

    async fn set(
        &mut self, locker_ref: Resource<Locker>, secret_id: String, value: Vec<u8>,
    ) -> Result<(), Error> {
        let Ok(locker) = self.table.get(&locker_ref) else {
            return Err(Error::NoSuchStore);
        };
        let secret_name = format!("{}-{secret_id}", locker.identifier);
        tracing::debug!("setting secret named: {secret_name}");
        let secret_id = Base64UrlUnpadded::encode_string(secret_name.as_bytes());

        let params = SetSecretParameters {
            value: Some(Base64UrlUnpadded::encode_string(&value)),
            ..SetSecretParameters::default()
        };
        let content = params.try_into().context("issue converting params to content")?;

        let kv = self.resources.azkeyvault().context("connecting to Azure KeyVault")?;
        kv.set_secret(&secret_id, content, None).await.context("issue setting secret")?;

        Ok(())
    }

    async fn delete(&mut self, locker_ref: Resource<Locker>, secret_id: String) -> Result<()> {
        let Ok(locker) = self.table.get(&locker_ref) else {
            return Err(Error::NoSuchStore);
        };
        let secret_name = format!("{}-{secret_id}", locker.identifier);
        tracing::debug!("deleting secret named: {secret_name}");
        let secret_id = Base64UrlUnpadded::encode_string(secret_name.as_bytes());

        let kv = self.resources.azkeyvault().context("connecting to Azure KeyVault")?;
        kv.delete_secret(&secret_id, None).await.context("issue deleting secret")?;

        Ok(())
    }

    async fn exists(&mut self, locker_ref: Resource<Locker>, secret_id: String) -> Result<bool> {
        vault::HostLocker::get(self, locker_ref, secret_id).await.map(|opt| opt.is_some())
    }

    async fn list_ids(&mut self, locker_ref: Resource<Locker>) -> Result<Vec<String>> {
        let Ok(locker) = self.table.get(&locker_ref) else {
            return Err(Error::NoSuchStore);
        };
        let identifier = &locker.identifier;
        tracing::debug!("listing secrets for: {identifier}");

        // get all secret properties from Azure KeyVault
        let kv = self.resources.azkeyvault().context("connecting to Azure KeyVault")?;
        let iter = kv.list_secret_properties(None).context("issue listing secrets")?;

        // filter and collect secret IDs for this 'locker'
        let secret_ids: Vec<String> = iter
            .try_filter_map(|props| async move {
                let Some(id) = props.id else {
                    return Ok(None);
                };
                Ok(id.strip_prefix(&format!("{identifier}-")).map(ToString::to_string))
            })
            .try_collect()
            .await
            .context("issue collecting secrets")?;

        Ok(secret_ids)
    }

    async fn drop(&mut self, locker_ref: Resource<Locker>) -> anyhow::Result<()> {
        tracing::trace!("vault::HostLocker::drop");
        self.table.delete(locker_ref).map(|_| Ok(()))?
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
