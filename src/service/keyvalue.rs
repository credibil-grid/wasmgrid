//! # WASI Key/Value Service
//!
//! This module implements a runtime service for `wasi:keyvalue`
//! (<https://github.com/WebAssembly/wasi-keyvalue>).

/// Wrap generation of wit bindings to simplify exports.
/// See <https://docs.rs/wasmtime/latest/wasmtime/component/macro.bindgen.html>
mod generated {
    #![allow(clippy::future_not_send)]
    #![allow(clippy::trait_duplication_in_bounds)]

    use super::bindgen;
    pub use super::{Bucket, Error};

    bindgen!({
        world: "keyvalue",
        path: "wit",
        tracing: true,
        async: true,
        trappable_imports: true,
        with: {
            "wasi:keyvalue/store/bucket": Bucket,
        },
        trappable_error_type: {
            "wasi:keyvalue/store/error" => Error,
        },
        // trappable_error_type: {
        //     "wasi:keyvalue/keyvalue-types/error" => Error,
        // },
    });
}

use std::env;
use std::sync::{Arc, OnceLock};
use std::time::Duration;

use anyhow::{Result, anyhow};
use async_nats::{AuthError, ConnectOptions, jetstream};
use futures::TryStreamExt;
use jetstream::kv;
use wasmtime::component::{InstancePre, Linker, Resource, ResourceTableError, bindgen};
use wasmtime_wasi::IoView;

use self::generated::Keyvalue;
use self::generated::wasi::keyvalue;
use self::generated::wasi::keyvalue::store::KeyResponse;
use crate::service::Ctx;

pub type Bucket = async_nats::jetstream::kv::Store;

const DEF_NATS_ADDR: &str = "demo.nats.io";
static JETSTREAM: OnceLock<jetstream::Context> = OnceLock::new();

#[derive(Debug)]
pub enum Error {
    NoSuchStore,
    // AccessDenied,
    Other(String),
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

pub struct Service {
    addr: String,
    jwt: Option<String>,
    seed: Option<String>,
}

pub fn new() -> Service {
    Service {
        addr: env::var("NATS_ADDR").unwrap_or_else(|_| DEF_NATS_ADDR.into()),
        jwt: env::var("NATS_JWT").ok(),
        seed: env::var("NATS_SEED").ok(),
    }
}

impl runtime::Service for Service {
    type Ctx = Ctx;

    fn namespace(&self) -> &'static str {
        "wasi:keyvalue"
    }

    fn add_to_linker(&self, linker: &mut Linker<Self::Ctx>) -> anyhow::Result<()> {
        Keyvalue::add_to_linker(linker, |t| t)
    }

    /// Provide key/value storage service for the specified wasm component.
    async fn start(&self, _: InstancePre<Self::Ctx>) -> anyhow::Result<()> {
        // build connection options
        let opts = if let Some(jwt) = &self.jwt
            && let Some(seed) = &self.seed
        {
            let key_pair = Arc::new(nkeys::KeyPair::from_seed(seed)?);
            ConnectOptions::with_jwt(jwt.clone(), move |nonce| {
                let key_pair = key_pair.clone();
                async move { key_pair.sign(&nonce).map_err(AuthError::new) }
            })
            .name("wasmgrid")
        } else {
            ConnectOptions::new()
        };

        // create JetStream context and store in global state
        let client = opts.connect(&self.addr).await?;
        tracing::info!("connected to JetStream on {}", self.addr);
        JETSTREAM.get_or_init(|| jetstream::new(client));

        Ok(())
    }
}

// Implement the [`wasi_keyvalue::KeyValueView`]` trait for Ctx.
impl keyvalue::store::Host for Ctx {
    // Open bucket specified by identifier, save to state and return as a resource.
    async fn open(&mut self, identifier: String) -> Result<Resource<Bucket>, Error> {
        tracing::trace!("store::Host::open {identifier}");

        let Some(jetstream) = JETSTREAM.get() else {
            return Err(anyhow!("JetStream not initialized").into());
        };

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

        Ok(self.table().push(bucket)?)
    }

    fn convert_error(&mut self, err: Error) -> Result<keyvalue::store::Error> {
        match err {
            Error::NoSuchStore => Ok(keyvalue::store::Error::NoSuchStore),
            // Error::AccessDenied => Ok(keyvalue::store::Error::AccessDenied),
            Error::Other(e) => Ok(keyvalue::store::Error::Other(e)),
        }
    }
}

impl keyvalue::store::HostBucket for Ctx {
    async fn get(&mut self, rep: Resource<Bucket>, key: String) -> Result<Option<Vec<u8>>, Error> {
        tracing::trace!("store::HostBucket::get {key}");

        let Ok(bucket) = self.table().get_mut(&rep) else {
            return Err(Error::NoSuchStore);
        };
        let value = bucket.get(key).await.map_err(|e| anyhow!("issue getting key: {e}"))?;
        Ok(value.map(|v| v.to_vec()))
    }

    async fn set(
        &mut self, rep: Resource<Bucket>, key: String, value: Vec<u8>,
    ) -> Result<(), Error> {
        tracing::trace!("store::HostBucket::set {key}");

        let Ok(bucket) = self.table().get_mut(&rep) else {
            return Err(Error::NoSuchStore);
        };
        bucket.put(key, value.into()).await.map_or_else(|e| Err(anyhow!(e).into()), |_| Ok(()))
    }

    async fn delete(&mut self, rep: Resource<Bucket>, key: String) -> Result<(), Error> {
        tracing::trace!("store::HostBucket::delete {key}");

        let Ok(bucket) = self.table().get_mut(&rep) else {
            return Err(Error::NoSuchStore);
        };
        bucket.delete(key).await.map_err(|e| anyhow!("issue deleting value: {e}").into())
    }

    async fn exists(&mut self, rep: Resource<Bucket>, key: String) -> Result<bool, Error> {
        tracing::trace!("store::HostBucket::exists {key}");

        let Ok(bucket) = self.table().get_mut(&rep) else {
            return Err(Error::NoSuchStore);
        };
        let value = bucket.get(&key).await.map_err(|e| {
            tracing::error!("issue getting value: {e}");
            anyhow!("issue getting value: {key}")
        })?;

        Ok(value.is_some())
    }

    async fn list_keys(
        &mut self, rep: Resource<Bucket>, cursor: Option<u64>,
    ) -> Result<KeyResponse, Error> {
        tracing::trace!("store::HostBucket::list_keys {cursor:?}");

        let Ok(bucket) = self.table().get_mut(&rep) else {
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

    // LATER: Can a JetStream bucket be closed?
    async fn drop(&mut self, rep: Resource<Bucket>) -> Result<(), wasmtime::Error> {
        tracing::trace!("store::HostBucket::drop");
        self.table().delete(rep).map_or_else(|e| Err(anyhow!(e)), |_| Ok(()))
    }
}

impl keyvalue::atomics::Host for Ctx {
    async fn increment(
        &mut self, rep: Resource<Bucket>, key: String, delta: u64,
    ) -> Result<u64, Error> {
        tracing::trace!("atomics::Host::increment {key}, {delta}");

        let Ok(bucket) = self.table().get_mut(&rep) else {
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

impl keyvalue::batch::Host for Ctx {
    async fn get_many(
        &mut self, rep: Resource<Bucket>, keys: Vec<String>,
    ) -> Result<Vec<Option<(String, Vec<u8>)>>, Error> {
        tracing::trace!("batch::Host::get_many {keys:?}");

        let Ok(bucket) = self.table().get_mut(&rep) else {
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
        &mut self, rep: Resource<Bucket>, key_values: Vec<(String, Vec<u8>)>,
    ) -> Result<(), Error> {
        tracing::trace!("batch::Host::set_many {key_values:?}");

        let Ok(bucket) = self.table().get_mut(&rep) else {
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

    async fn delete_many(&mut self, rep: Resource<Bucket>, keys: Vec<String>) -> Result<(), Error> {
        tracing::trace!("batch::Host::delete_many {keys:?}");

        let Ok(bucket) = self.table().get_mut(&rep) else {
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
