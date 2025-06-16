//! # WASI Key/Value Service
//!
//! This module implements a runtime service for `wasi:vault`
//! (<https://github.com/WebAssembly/wasi-vault>).

/// Wrap generation of wit bindings to simplify exports.
/// See <https://docs.rs/wasmtime/latest/wasmtime/component/macro.bindgen.html>
mod generated {
    #![allow(clippy::trait_duplication_in_bounds)]

    pub use async_nats::jetstream::kv::Store;

    pub use self::wasi::vault::vault::Error;

    wasmtime::component::bindgen!({
        world: "vault",
        path: "../../wit",
        tracing: true,
        async: true,
        trappable_imports: true,
        with: {
            "wasi:vault/vault/locker": Store,
        },
        trappable_error_type: {
            "wasi:vault/vault/error" => Error,
        },
    });
}

use std::time::Duration;

use anyhow::anyhow;
use async_nats::jetstream;
use async_nats::jetstream::kv::{Config, Store};
use futures::TryStreamExt;
use runtime::Linkable;
use wasmtime::component::{Linker, Resource, ResourceTableError};
use wasmtime_wasi::ResourceTable;

use self::generated::wasi::vault::vault;
use self::generated::wasi::vault::vault::Error;
use crate::{Ctx, Resources};

pub type Result<T, E = Error> = anyhow::Result<T, E>;

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

    // Add all the `wasi-vault` world's interfaces to a [`Linker`], and
    // instantiate the `VaultHost` for the component.
    fn add_to_linker(&self, linker: &mut Linker<Self::Ctx>) -> anyhow::Result<()> {
        vault::add_to_linker_get_host(linker, VaultHost::new)?;
        tracing::trace!("added to linker");
        Ok(())
    }
}

// Implement the [`wasi_keyvalue::KeyValueView`]` trait for  VaultHost<'_>.
impl vault::Host for VaultHost<'_> {
    // Open bucket specified by identifier, save to state and return as a resource.
    async fn open(&mut self, identifier: String) -> Result<Resource<Store>> {
        tracing::trace!("vault::Host::open {identifier}");

        let jetstream = jetstream::new(self.resources.nats()?.clone());
        let bucket = if let Ok(bucket) = jetstream.get_key_value(&identifier).await {
            bucket
        } else {
            let result = jetstream
                .create_key_value(Config {
                    bucket: identifier.clone(),
                    history: 1,
                    max_age: Duration::from_secs(10 * 60),
                    max_bytes: 100 * 1024 * 1024, // 100 MiB
                    ..Config::default()
                })
                .await;

            result.map_err(|e| {
                tracing::error!("Failed to create {identifier} bucket: {e}");
                anyhow!("Failed to create {identifier} bucket: {e}")
            })?
        };

        Ok(self.table.push(bucket)?)
    }

    fn convert_error(&mut self, err: Error) -> anyhow::Result<Error> {
        tracing::error!("{err}");
        Ok(err)
    }
}

impl vault::HostLocker for VaultHost<'_> {
    async fn get(
        &mut self, store_ref: Resource<Store>, secret_id: String,
    ) -> Result<Option<Vec<u8>>> {
        tracing::trace!("vault::HostLocker::get {secret_id}");

        let Ok(bucket) = self.table.get(&store_ref) else {
            return Err(Error::NoSuchStore);
        };
        let value =
            bucket.get(secret_id).await.map_err(|e| anyhow!("issue getting secret_id: {e}"))?;
        Ok(value.map(|v| v.to_vec()))
    }

    async fn set(
        &mut self, store_ref: Resource<Store>, secret_id: String, value: Vec<u8>,
    ) -> Result<(), Error> {
        tracing::trace!("vault::HostLocker::set {secret_id}");

        let Ok(bucket) = self.table.get_mut(&store_ref) else {
            return Err(Error::NoSuchStore);
        };
        bucket
            .put(secret_id, value.into())
            .await
            .map_or_else(|e| Err(anyhow!(e).into()), |_| Ok(()))
    }

    async fn delete(&mut self, store_ref: Resource<Store>, secret_id: String) -> Result<()> {
        tracing::trace!("vault::HostLocker::delete {secret_id}");

        let Ok(bucket) = self.table.get_mut(&store_ref) else {
            return Err(Error::NoSuchStore);
        };
        bucket.delete(secret_id).await.map_err(|e| anyhow!("issue deleting value: {e}").into())
    }

    async fn exists(&mut self, store_ref: Resource<Store>, secret_id: String) -> Result<bool> {
        tracing::trace!("vault::HostLocker::exists {secret_id}");

        let Ok(bucket) = self.table.get(&store_ref) else {
            return Err(Error::NoSuchStore);
        };
        let value =
            bucket.get(&secret_id).await.map_err(|e| anyhow!("issue getting secret: {e}"))?;

        Ok(value.is_some())
    }

    async fn list_ids(&mut self, store_ref: Resource<Store>) -> Result<Vec<String>> {
        tracing::trace!("vault::HostLocker::list_keys");

        let Ok(bucket) = self.table.get(&store_ref) else {
            return Err(Error::NoSuchStore);
        };
        let Ok(key_stream) = bucket.keys().await else {
            return Err(anyhow!("Failed to list keys").into());
        };
        let Ok(keys) = key_stream.try_collect().await else {
            return Err(anyhow!("Failed to collect keys").into());
        };

        Ok(keys)
    }

    async fn drop(&mut self, store_ref: Resource<Store>) -> anyhow::Result<()> {
        tracing::trace!("vault::HostLocker::drop");
        self.table.delete(store_ref).map(|_| Ok(()))?
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
