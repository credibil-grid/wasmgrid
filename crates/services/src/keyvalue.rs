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

    wasmtime::component::bindgen!({
        world: "keyvalue",
        path: "../../wit",
        tracing: true,
        async: true,
        trappable_imports: true,
        with: {
            "wasi:keyvalue/store/bucket": Store,
        },
        trappable_error_type: {
            "wasi:keyvalue/store/error" => Error,
        },
    });
}

use std::time::Duration;

use anyhow::anyhow;
use async_nats::jetstream::kv::{self, Store};
use async_nats::{Client, jetstream};
use futures::TryStreamExt;
use runtime::Linkable;
use wasmtime::component::{Linker, Resource, ResourceTableError};
use wasmtime_wasi::ResourceTable;

use self::generated::wasi::keyvalue;
use self::generated::wasi::keyvalue::store::{Error, KeyResponse};
use crate::Ctx;

pub type Result<T, E = Error> = anyhow::Result<T, E>;

pub struct KeyvalueHost<'a> {
    client: &'a Client,
    table: &'a mut ResourceTable,
}

impl KeyvalueHost<'_> {
    fn new(c: &mut Ctx) -> KeyvalueHost<'_> {
        KeyvalueHost {
            client: c.resources.nats(),
            table: &mut c.table,
        }
    }
}

pub struct Service;

impl Linkable for Service {
    type Ctx = Ctx;

    // Add all the `wasi-keyvalue` world's interfaces to a [`Linker`], and
    // instantiate the `KeyvalueHost` for the component.
    fn add_to_linker(&self, linker: &mut Linker<Self::Ctx>) -> anyhow::Result<()> {
        // add_to_linker(linker, link)?;
        keyvalue::store::add_to_linker_get_host(linker, KeyvalueHost::new)?;
        keyvalue::atomics::add_to_linker_get_host(linker, KeyvalueHost::new)?;
        keyvalue::batch::add_to_linker_get_host(linker, KeyvalueHost::new)?;
        tracing::trace!("added to linker");
        Ok(())
    }
}

// fn add_to_linker<T: Send>(
//     l: &mut Linker<T>, f: impl Fn(&mut T) -> KeyvalueHost<'_> + Send + Sync + Copy + 'static,
// ) -> anyhow::Result<()> {
//     keyvalue::store::add_to_linker_get_host(l, f)?;
//     keyvalue::atomics::add_to_linker_get_host(l, f)?;
//     keyvalue::batch::add_to_linker_get_host(l, f)
// }

// Implement the [`wasi_keyvalue::KeyValueView`]` trait for  KeyvalueHost<'_>.
impl keyvalue::store::Host for KeyvalueHost<'_> {
    // Open bucket specified by identifier, save to state and return as a resource.
    async fn open(&mut self, identifier: String) -> Result<Resource<Store>> {
        tracing::trace!("store::Host::open {identifier}");

        let jetstream = jetstream::new(self.client.clone());

        let bucket = if let Ok(bucket) = jetstream.get_key_value(&identifier).await {
            bucket
        } else {
            let result = jetstream
                .create_key_value(kv::Config {
                    bucket: identifier.clone(),
                    history: 1,
                    max_age: Duration::from_secs(10 * 60),
                    max_bytes: 100 * 1024 * 1024, // 100 MiB
                    ..kv::Config::default()
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
        Ok(err)
    }
}

impl keyvalue::store::HostBucket for KeyvalueHost<'_> {
    async fn get(&mut self, rep: Resource<Store>, key: String) -> Result<Option<Vec<u8>>> {
        tracing::trace!("store::HostBucket::get {key}");

        let Ok(bucket) = self.table.get_mut(&rep) else {
            return Err(Error::NoSuchStore);
        };
        let value = bucket.get(key).await.map_err(|e| anyhow!("issue getting key: {e}"))?;
        Ok(value.map(|v| v.to_vec()))
    }

    async fn set(
        &mut self, rep: Resource<Store>, key: String, value: Vec<u8>,
    ) -> Result<(), Error> {
        tracing::trace!("store::HostBucket::set {key}");

        let Ok(bucket) = self.table.get_mut(&rep) else {
            return Err(Error::NoSuchStore);
        };
        bucket.put(key, value.into()).await.map_or_else(|e| Err(anyhow!(e).into()), |_| Ok(()))
    }

    async fn delete(&mut self, rep: Resource<Store>, key: String) -> Result<()> {
        tracing::trace!("store::HostBucket::delete {key}");

        let Ok(bucket) = self.table.get_mut(&rep) else {
            return Err(Error::NoSuchStore);
        };
        bucket.delete(key).await.map_err(|e| anyhow!("issue deleting value: {e}").into())
    }

    async fn exists(&mut self, rep: Resource<Store>, key: String) -> Result<bool> {
        tracing::trace!("store::HostBucket::exists {key}");

        let Ok(bucket) = self.table.get_mut(&rep) else {
            return Err(Error::NoSuchStore);
        };
        let value = bucket.get(&key).await.map_err(|e| {
            tracing::error!("issue getting value: {e}");
            anyhow!("issue getting value: {key}")
        })?;

        Ok(value.is_some())
    }

    async fn list_keys(
        &mut self, rep: Resource<Store>, cursor: Option<u64>,
    ) -> Result<KeyResponse> {
        tracing::trace!("store::HostBucket::list_keys {cursor:?}");

        let Ok(bucket) = self.table.get_mut(&rep) else {
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

    async fn drop(&mut self, rep: Resource<Store>) -> anyhow::Result<()> {
        tracing::trace!("store::HostBucket::drop");
        self.table.delete(rep).map(|_| Ok(()))?
    }
}

impl keyvalue::atomics::Host for KeyvalueHost<'_> {
    async fn increment(&mut self, rep: Resource<Store>, key: String, delta: u64) -> Result<u64> {
        tracing::trace!("atomics::Host::increment {key}, {delta}");

        let Ok(bucket) = self.table.get_mut(&rep) else {
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
        let inc = u64::from_be_bytes(buf) + delta;

        // update value in bucket
        if let Err(e) = bucket.put(key, inc.to_be_bytes().to_vec().into()).await {
            tracing::error!("issue saving increment: {e}");
            return Err(anyhow!("issue saving increment: {e}").into());
        }

        Ok(inc)
    }
}

impl keyvalue::batch::Host for KeyvalueHost<'_> {
    async fn get_many(
        &mut self, rep: Resource<Store>, keys: Vec<String>,
    ) -> Result<Vec<Option<(String, Vec<u8>)>>> {
        tracing::trace!("batch::Host::get_many {keys:?}");

        let Ok(bucket) = self.table.get_mut(&rep) else {
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
        &mut self, rep: Resource<Store>, key_values: Vec<(String, Vec<u8>)>,
    ) -> Result<()> {
        tracing::trace!("batch::Host::set_many {key_values:?}");

        let Ok(bucket) = self.table.get_mut(&rep) else {
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

    async fn delete_many(&mut self, rep: Resource<Store>, keys: Vec<String>) -> Result<()> {
        tracing::trace!("batch::Host::delete_many {keys:?}");

        let Ok(bucket) = self.table.get_mut(&rep) else {
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
