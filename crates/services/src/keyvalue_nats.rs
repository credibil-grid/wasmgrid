//! # WASI Key/Value Service
//!
//! This module implements a runtime service for `wasi:keyvalue`
//! (<https://github.com/WebAssembly/wasi-keyvalue>).

/// Wrap generation of wit bindings to simplify exports.
/// See <https://docs.rs/wasmtime/latest/wasmtime/component/macro.bindgen.html>
mod generated {
    #![allow(clippy::trait_duplication_in_bounds)]

    pub use async_nats::jetstream::kv::Store;

    pub use self::wasi::keyvalue::store::Error;
    pub use super::Cas;

    wasmtime::component::bindgen!({
        world: "keyvalue",
        path: "../../wit",
        tracing: true,
        async: true,
        trappable_imports: true,
        with: {
            "wasi:keyvalue/store/bucket": Store,
            "wasi:keyvalue/atomics/cas": Cas,
        },
        trappable_error_type: {
            "wasi:keyvalue/store/error" => Error,
        },
    });
}

use std::time::Duration;

use anyhow::anyhow;
use async_nats::jetstream;
use async_nats::jetstream::kv::{Config, Store};
use futures::TryStreamExt;
use generated::wasi::keyvalue::atomics::CasError;
use runtime::Linkable;
use wasmtime::component::{Linker, Resource, ResourceTableError};
use wasmtime_wasi::ResourceTable;

use self::generated::wasi::keyvalue::store::{Error, KeyResponse};
use self::generated::wasi::keyvalue::{atomics, batch, store};
use crate::{Ctx, Resources};

pub type Result<T, E = Error> = anyhow::Result<T, E>;

pub struct KeyvalueHost<'a> {
    resources: &'a Resources,
    table: &'a mut ResourceTable,
}

impl KeyvalueHost<'_> {
    const fn new(c: &mut Ctx) -> KeyvalueHost<'_> {
        KeyvalueHost {
            resources: &c.resources,
            table: &mut c.table,
        }
    }
}

/// Compare and Swap (CAS) handle.
pub struct Cas {
    /// Key of the stored value.
    pub key: String,

    /// Current value.
    pub current: Option<Vec<u8>>,
}

pub struct Service;

impl Linkable for Service {
    type Ctx = Ctx;

    // Add all the `wasi-keyvalue` world's interfaces to a [`Linker`], and
    // instantiate the `KeyvalueHost` for the component.
    fn add_to_linker(&self, linker: &mut Linker<Self::Ctx>) -> anyhow::Result<()> {
        store::add_to_linker_get_host(linker, KeyvalueHost::new)?;
        atomics::add_to_linker_get_host(linker, KeyvalueHost::new)?;
        batch::add_to_linker_get_host(linker, KeyvalueHost::new)?;
        tracing::trace!("added to linker");
        Ok(())
    }
}

// Implement the [`wasi_keyvalue::KeyValueView`]` trait for  KeyvalueHost<'_>.
impl store::Host for KeyvalueHost<'_> {
    // Open bucket specified by identifier, save to state and return as a resource.
    async fn open(&mut self, identifier: String) -> Result<Resource<Store>> {
        tracing::trace!("store::Host::open {identifier}");

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

impl store::HostBucket for KeyvalueHost<'_> {
    async fn get(&mut self, store_ref: Resource<Store>, key: String) -> Result<Option<Vec<u8>>> {
        tracing::trace!("store::HostBucket::get {key}");

        let Ok(bucket) = self.table.get(&store_ref) else {
            return Err(Error::NoSuchStore);
        };
        let value = bucket.get(key).await.map_err(|e| anyhow!("issue getting key: {e}"))?;
        Ok(value.map(|v| v.to_vec()))
    }

    async fn set(
        &mut self, store_ref: Resource<Store>, key: String, value: Vec<u8>,
    ) -> Result<(), Error> {
        tracing::trace!("store::HostBucket::set {key}");

        let Ok(bucket) = self.table.get_mut(&store_ref) else {
            return Err(Error::NoSuchStore);
        };
        bucket.put(key, value.into()).await.map_or_else(|e| Err(anyhow!(e).into()), |_| Ok(()))
    }

    async fn delete(&mut self, store_ref: Resource<Store>, key: String) -> Result<()> {
        tracing::trace!("store::HostBucket::delete {key}");

        let Ok(bucket) = self.table.get_mut(&store_ref) else {
            return Err(Error::NoSuchStore);
        };
        bucket.delete(key).await.map_err(|e| anyhow!("issue deleting value: {e}").into())
    }

    async fn exists(&mut self, store_ref: Resource<Store>, key: String) -> Result<bool> {
        tracing::trace!("store::HostBucket::exists {key}");

        let Ok(bucket) = self.table.get(&store_ref) else {
            return Err(Error::NoSuchStore);
        };
        let value = bucket.get(&key).await.map_err(|e| {
            tracing::error!("issue getting value: {e}");
            anyhow!("issue getting value: {key}")
        })?;

        Ok(value.is_some())
    }

    async fn list_keys(
        &mut self, store_ref: Resource<Store>, cursor: Option<String>,
    ) -> Result<KeyResponse> {
        tracing::trace!("store::HostBucket::list_keys {cursor:?}");

        let Ok(bucket) = self.table.get(&store_ref) else {
            return Err(Error::NoSuchStore);
        };
        let Ok(key_stream) = bucket.keys().await else {
            return Err(anyhow!("Failed to list keys").into());
        };
        let Ok(keys) = key_stream.try_collect().await else {
            return Err(anyhow!("Failed to collect keys").into());
        };
        Ok(KeyResponse { keys, cursor })
    }

    async fn drop(&mut self, store_ref: Resource<Store>) -> anyhow::Result<()> {
        tracing::trace!("store::HostBucket::drop");
        self.table.delete(store_ref).map(|_| Ok(()))?
    }
}

impl atomics::HostCas for KeyvalueHost<'_> {
    /// Construct a new CAS operation. Implementors can map the underlying functionality
    /// (transactions, versions, etc) as desired.
    async fn new(&mut self, store_ref: Resource<Store>, key: String) -> Result<Resource<Cas>> {
        tracing::trace!("atomics::HostCas::new {key}");

        let Ok(bucket) = self.table.get(&store_ref) else {
            return Err(Error::NoSuchStore);
        };
        let value = bucket.get(key.clone()).await.map_err(|e| anyhow!("issue getting key: {e}"))?;
        let cas = Cas {
            key: key.clone(),
            current: value.map(|v| v.to_vec()),
        };

        Ok(self.table.push(cas)?)
    }

    /// Get the current value of the CAS handle.
    async fn current(&mut self, cas_ref: Resource<Cas>) -> Result<Option<Vec<u8>>> {
        tracing::trace!("atomics::HostCas::current");

        let Ok(cas) = self.table.get(&cas_ref) else {
            return Err(Error::NoSuchStore);
        };
        let value = cas.current.clone();
        Ok(value)
    }

    /// Drop the CAS handle.
    async fn drop(&mut self, cas_ref: Resource<Cas>) -> anyhow::Result<()> {
        tracing::trace!("atomics::HostCas::drop");
        self.table.delete(cas_ref).map(|_| Ok(()))?
    }
}

impl atomics::Host for KeyvalueHost<'_> {
    /// Atomically increment the value associated with the key in the store by
    /// the given delta. It returns the new value.
    ///
    /// If the key does not exist in the store, it creates a new key-value pair
    /// with the value set to the given delta.
    ///
    /// If any other error occurs, it returns an `Err(error)`.
    async fn increment(
        &mut self, store_ref: Resource<Store>, key: String, delta: i64,
    ) -> Result<i64> {
        tracing::trace!("atomics::Host::increment {key}, {delta}");

        let Ok(bucket) = self.table.get_mut(&store_ref) else {
            return Err(Error::NoSuchStore);
        };
        let Ok(Some(value)) = bucket.get(key.clone()).await else {
            tracing::error!("no value for {key}");
            return Err(anyhow!("no value for {key}").into());
        };

        // increment value by delta
        let slice: &[u8] = &value;
        let mut buf = [0u8; 8];
        let len = 8.min(slice.len());
        buf[..len].copy_from_slice(&slice[..len]);
        let inc = i64::from_be_bytes(buf) + delta;

        // update value in bucket
        if let Err(e) = bucket.put(key, inc.to_be_bytes().to_vec().into()).await {
            tracing::error!("issue saving increment: {e}");
            return Err(anyhow!("issue saving increment: {e}").into());
        }

        Ok(inc)
    }

    /// Perform the swap on a CAS operation. This consumes the CAS handle and
    /// returns an error if the CAS operation failed.
    async fn swap(
        &mut self, _cas_ref: Resource<Cas>, _value: Vec<u8>,
    ) -> anyhow::Result<Result<(), CasError>> {
        tracing::trace!("atomics::Host::swap");
        Err(anyhow!("not implemented"))
    }
}

impl batch::Host for KeyvalueHost<'_> {
    async fn get_many(
        &mut self, store_ref: Resource<Store>, keys: Vec<String>,
    ) -> Result<Vec<Option<(String, Vec<u8>)>>> {
        tracing::trace!("batch::Host::get_many {keys:?}");

        let Ok(bucket) = self.table.get(&store_ref) else {
            return Err(Error::NoSuchStore);
        };

        let mut many = Vec::new();
        for key in keys {
            let value = bucket.get(&key).await.map_err(|e| {
                tracing::error!("issue getting value: {e}");
                anyhow!("issue getting value: {key}")
            })?;
            if let Some(value) = value {
                many.push(Some((key, value.to_vec())));
            }
        }

        Ok(many)
    }

    async fn set_many(
        &mut self, store_ref: Resource<Store>, key_values: Vec<(String, Vec<u8>)>,
    ) -> Result<()> {
        tracing::trace!("batch::Host::set_many {key_values:?}");

        let Ok(bucket) = self.table.get_mut(&store_ref) else {
            return Err(Error::NoSuchStore);
        };
        for (key, value) in key_values {
            if let Err(e) = bucket.put(key, value.into()).await {
                tracing::error!("issue saving value: {e}");
                return Err(anyhow!("issue saving value: {e}").into());
            }
        }

        Ok(())
    }

    async fn delete_many(&mut self, store_ref: Resource<Store>, keys: Vec<String>) -> Result<()> {
        tracing::trace!("batch::Host::delete_many {keys:?}");

        let Ok(bucket) = self.table.get_mut(&store_ref) else {
            return Err(Error::NoSuchStore);
        };
        for key in keys {
            if let Err(e) = bucket.delete(key).await {
                tracing::error!("issue deleting value: {e}");
                return Err(anyhow!("issue deleting value: {e}").into());
            }
        }

        Ok(())
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
