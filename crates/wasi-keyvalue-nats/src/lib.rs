//! # WASI Host Service using NATS
//!
//! This module implements a runtime service for `wasi:keyvalue`
//! (<https://github.com/WebAssembly/wasi-keyvalue>).

mod generated {
    #![allow(clippy::trait_duplication_in_bounds)]

    pub use async_nats::jetstream::kv::Store;

    pub use self::wasi::keyvalue::store::Error;
    pub use super::Cas;

    wasmtime::component::bindgen!({
        world: "keyvalue",
        path: "../../wit",
        imports: {
            default: async | tracing | trappable,
        },
        with: {
            "wasi:keyvalue/store/bucket": Store,
            "wasi:keyvalue/atomics/cas": Cas,
        },
        trappable_error_type: {
            "wasi:keyvalue/store/error" => Error,
        },
    });
}

use std::sync::OnceLock;
use std::time::Duration;

use anyhow::anyhow;
use async_nats::jetstream;
use async_nats::jetstream::kv::{Config, Store};
use base64ct::{Base64UrlUnpadded, Encoding};
use futures::TryStreamExt;
use runtime::{AddResource, RunState};
use wasmtime::component::{HasData, Linker, Resource, ResourceTableError};
use wasmtime_wasi::ResourceTable;

use self::generated::wasi::keyvalue::atomics::CasError;
use self::generated::wasi::keyvalue::store::{Error, KeyResponse};
use self::generated::wasi::keyvalue::{atomics, batch, store};

pub type Result<T, E = Error> = anyhow::Result<T, E>;

pub struct Cas {
    pub key: String,
    pub current: Option<Vec<u8>>,
}

static NATS_CLIENT: OnceLock<async_nats::Client> = OnceLock::new();

#[derive(Debug)]
pub struct KeyValue;

impl runtime::Service for KeyValue {
    fn add_to_linker(&self, l: &mut Linker<RunState>) -> anyhow::Result<()> {
        store::add_to_linker::<_, Data>(l, Host::new)?;
        atomics::add_to_linker::<_, Data>(l, Host::new)?;
        batch::add_to_linker::<_, Data>(l, Host::new)?;
        Ok(())
    }
}

impl AddResource<async_nats::Client> for KeyValue {
    fn resource(self, resource: async_nats::Client) -> anyhow::Result<Self> {
        NATS_CLIENT.set(resource).map_err(|_| anyhow!("client already set"))?;
        Ok(self)
    }
}

struct Data;
impl HasData for Data {
    type Data<'a> = Host<'a>;
}

pub struct Host<'a> {
    table: &'a mut ResourceTable,
}

impl Host<'_> {
    const fn new(c: &mut RunState) -> Host<'_> {
        Host { table: &mut c.table }
    }
}

fn nats() -> anyhow::Result<&'static async_nats::Client> {
    NATS_CLIENT.get().ok_or_else(|| anyhow!("NATS client not initialized."))
}

// Implement the [`wasi_keyvalue::KeyValueView`]` trait for  Host<'_>.
impl store::Host for Host<'_> {
    // Open bucket specified by identifier, save to state and return as a resource.
    async fn open(&mut self, identifier: String) -> Result<Resource<Store>> {
        let jetstream = jetstream::new(nats()?.clone());

        let bucket_id = Base64UrlUnpadded::encode_string(identifier.as_bytes());
        let bucket = if let Ok(bucket) = jetstream.get_key_value(&bucket_id).await {
            bucket
        } else {
            let result = jetstream
                .create_key_value(Config {
                    bucket: bucket_id,
                    history: 1,
                    max_age: Duration::from_secs(10 * 60),
                    max_bytes: 100 * 1024 * 1024, // 100 MiB
                    ..Config::default()
                })
                .await;

            result.map_err(|e| anyhow!("failed to create bucket: {e}"))?
        };

        Ok(self.table.push(bucket)?)
    }

    fn convert_error(&mut self, err: Error) -> anyhow::Result<Error> {
        tracing::error!("{err}");
        Ok(err)
    }
}

impl store::HostBucket for Host<'_> {
    async fn get(&mut self, store_ref: Resource<Store>, key: String) -> Result<Option<Vec<u8>>> {
        let Ok(bucket) = self.table.get(&store_ref) else {
            return Err(Error::NoSuchStore);
        };
        let key_enc = Base64UrlUnpadded::encode_string(key.as_bytes());
        let value = bucket.get(key_enc).await.map_err(|e| anyhow!("issue getting key: {e}"))?;
        Ok(value.map(|v| v.to_vec()))
    }

    async fn set(
        &mut self, store_ref: Resource<Store>, key: String, value: Vec<u8>,
    ) -> Result<(), Error> {
        let Ok(bucket) = self.table.get_mut(&store_ref) else {
            return Err(Error::NoSuchStore);
        };
        let key_enc = Base64UrlUnpadded::encode_string(key.as_bytes());
        bucket.put(key_enc, value.into()).await.map_or_else(|e| Err(anyhow!(e).into()), |_| Ok(()))
    }

    async fn delete(&mut self, store_ref: Resource<Store>, key: String) -> Result<()> {
        let Ok(bucket) = self.table.get_mut(&store_ref) else {
            return Err(Error::NoSuchStore);
        };
        let key_enc = Base64UrlUnpadded::encode_string(key.as_bytes());
        bucket.delete(key_enc).await.map_err(|e| anyhow!("issue deleting value: {e}").into())
    }

    async fn exists(&mut self, store_ref: Resource<Store>, key: String) -> Result<bool> {
        let Ok(bucket) = self.table.get(&store_ref) else {
            return Err(Error::NoSuchStore);
        };
        let key_enc = Base64UrlUnpadded::encode_string(key.as_bytes());
        let value =
            bucket.get(key_enc).await.map_err(|e| anyhow!("issue checking for {key}: {e}"))?;
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
            return Err(anyhow!("failed to list keys").into());
        };
        let Ok(keys) = key_stream
            .try_filter_map(|enc| async move {
                let Ok(decoded) = Base64UrlUnpadded::decode_vec(&enc) else {
                    return Ok(None);
                };
                Ok(Some(String::from_utf8_lossy(&decoded).into_owned()))
            })
            .try_collect()
            .await
        else {
            return Err(anyhow!("failed to collect keys").into());
        };

        Ok(KeyResponse { keys, cursor })
    }

    async fn drop(&mut self, store_ref: Resource<Store>) -> anyhow::Result<()> {
        self.table.delete(store_ref).map(|_| Ok(()))?
    }
}

impl atomics::HostCas for Host<'_> {
    /// Construct a new CAS operation. Implementors can map the underlying functionality
    /// (transactions, versions, etc) as desired.
    async fn new(&mut self, store_ref: Resource<Store>, key: String) -> Result<Resource<Cas>> {
        let Ok(bucket) = self.table.get(&store_ref) else {
            return Err(Error::NoSuchStore);
        };
        let key_enc = Base64UrlUnpadded::encode_string(key.as_bytes());
        let value = bucket.get(key_enc).await.map_err(|e| anyhow!("issue getting key: {e}"))?;
        let cas = Cas {
            key,
            current: value.map(|v| v.to_vec()),
        };

        Ok(self.table.push(cas)?)
    }

    /// Get the current value of the CAS handle.
    async fn current(&mut self, cas_ref: Resource<Cas>) -> Result<Option<Vec<u8>>> {
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

impl atomics::Host for Host<'_> {
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
        let Ok(bucket) = self.table.get_mut(&store_ref) else {
            return Err(Error::NoSuchStore);
        };

        let key_enc = Base64UrlUnpadded::encode_string(key.as_bytes());
        let Ok(Some(value)) = bucket.get(&key_enc).await else {
            return Err(anyhow!("no value for {key}").into());
        };

        // increment value by delta
        let slice: &[u8] = &value;
        let mut buf = [0u8; 8];
        let len = 8.min(slice.len());
        buf[..len].copy_from_slice(&slice[..len]);
        let inc = i64::from_be_bytes(buf) + delta;

        // update value in bucket
        if let Err(e) = bucket.put(key_enc, inc.to_be_bytes().to_vec().into()).await {
            return Err(anyhow!("issue saving increment: {e}").into());
        }

        Ok(inc)
    }

    /// Perform the swap on a CAS operation. This consumes the CAS handle and
    /// returns an error if the CAS operation failed.
    async fn swap(
        &mut self, _cas_ref: Resource<Cas>, _value: Vec<u8>,
    ) -> anyhow::Result<Result<(), CasError>> {
        Err(anyhow!("not implemented"))
    }
}

impl batch::Host for Host<'_> {
    async fn get_many(
        &mut self, store_ref: Resource<Store>, keys: Vec<String>,
    ) -> Result<Vec<Option<(String, Vec<u8>)>>> {
        let Ok(bucket) = self.table.get(&store_ref) else {
            return Err(Error::NoSuchStore);
        };

        let mut many = Vec::new();
        for key in keys {
            let key_enc = Base64UrlUnpadded::encode_string(key.as_bytes());
            let value =
                bucket.get(key_enc).await.map_err(|e| anyhow!("issue getting value: {e}"))?;
            if let Some(value) = value {
                many.push(Some((key, value.to_vec())));
            }
        }

        Ok(many)
    }

    async fn set_many(
        &mut self, store_ref: Resource<Store>, key_values: Vec<(String, Vec<u8>)>,
    ) -> Result<()> {
        let Ok(bucket) = self.table.get_mut(&store_ref) else {
            return Err(Error::NoSuchStore);
        };

        for (key, value) in key_values {
            let key_enc = Base64UrlUnpadded::encode_string(key.as_bytes());
            if let Err(e) = bucket.put(key_enc, value.into()).await {
                return Err(anyhow!("issue saving value: {e}").into());
            }
        }

        Ok(())
    }

    async fn delete_many(&mut self, store_ref: Resource<Store>, keys: Vec<String>) -> Result<()> {
        let Ok(bucket) = self.table.get_mut(&store_ref) else {
            return Err(Error::NoSuchStore);
        };

        for key in keys {
            let key_enc = Base64UrlUnpadded::encode_string(key.as_bytes());
            if let Err(e) = bucket.delete(key_enc).await {
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
