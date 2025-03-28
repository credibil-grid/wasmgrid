//! # WASI Key/Value Capability
//!
//! This module implements a runtime capability for `wasi:keyvalue`
//! (<https://github.com/WebAssembly/wasi-keyvalue>).

use std::sync::{Arc, OnceLock};
use std::time::Duration;

use anyhow::anyhow;
use async_nats::{jetstream, AuthError, ConnectOptions};
use bindings::wasi::keyvalue::store::{self, Error, KeyResponse};
use bindings::wasi::keyvalue::{atomics, batch};
use bindings::Keyvalue;
use futures::TryStreamExt;
use jetstream::kv;
use wasmtime::component::{Linker, Resource};
use wasmtime_wasi::WasiView;

use crate::runtime::{self, Runtime, State};

/// Wrap generation of wit bindings to simplify exports.
/// See <https://docs.rs/wasmtime/latest/wasmtime/component/macro.bindgen.html>
mod bindings {
    #![allow(clippy::future_not_send)]
    pub use super::Bucket;

    wasmtime::component::bindgen!({
        world: "keyvalue",
        path: "wit",
        tracing: true,
        async: true,
        trappable_imports: true,
        with: {
            "wasi:keyvalue/store/bucket": Bucket,
        },
        // trappable_error_type: {
        //     "wasi:keyvalue/keyvalue-types/error" => Error,
        // },
    });
}

pub type Bucket = async_nats::jetstream::kv::Store;

static JETSTREAM: OnceLock<jetstream::Context> = OnceLock::new();

pub struct Capability {
    addr: String,
    creds: Option<crate::NatsCreds>,
}

pub const fn new(addr: String, creds: Option<crate::NatsCreds>) -> Capability {
    Capability { addr, creds }
}

#[async_trait::async_trait]
impl runtime::Capability for Capability {
    fn namespace(&self) -> &str {
        "wasi:keyvalue"
    }

    fn add_to_linker(&self, linker: &mut Linker<State>) -> anyhow::Result<()> {
        Keyvalue::add_to_linker(linker, |t| t)
    }

    /// Provide key/value storage capability for the specified wasm component.
    async fn run(&self, _runtime: Runtime) -> anyhow::Result<()> {
        // build connection options
        let opts = if let Some(creds) = &self.creds {
            let key_pair = Arc::new(nkeys::KeyPair::from_seed(&creds.seed)?);
            ConnectOptions::with_jwt(creds.jwt.clone(), move |nonce| {
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

// Implement the [`wasi_keyvalue::KeyValueView`]` trait for State.
#[async_trait::async_trait]
impl store::Host for State {
    // Open bucket specified by identifier, save to state and return as a resource.
    async fn open(
        &mut self, identifier: String,
    ) -> wasmtime::Result<Result<Resource<Bucket>, store::Error>> {
        tracing::trace!("store::Host::open {identifier}");

        let Some(jetstream) = JETSTREAM.get() else {
            return Ok(Err(store::Error::Other("JetStream not initialized".into())));
        };

        let bucket = match jetstream.get_key_value(&identifier).await {
            Ok(bucket) => bucket,
            Err(_) => {
                match jetstream
                    .create_key_value(kv::Config {
                        bucket: identifier.clone(),
                        history: 1,
                        max_age: Duration::from_mins(10),
                        max_bytes: 100 * 1024 * 1024, // 100 MiB
                        // placement: Some(stream::Placement {
                        //     cluster: None,
                        //     // <https://docs.synadia.com/cloud/resources/placement-tags>
                        //     tags: vec!["cloud:az".to_string()],
                        // }),
                        ..kv::Config::default()
                    })
                    .await
                {
                    Ok(bucket) => bucket,
                    Err(e) => {
                        tracing::error!("Failed to create {identifier} bucket: {e}");
                        return Ok(Err(store::Error::Other(format!(
                            "Failed to create {identifier} bucket: {e}"
                        ))));
                    }
                }
            }
        };

        Ok(Ok(self.table().push(bucket)?))
    }
}

#[async_trait::async_trait]
impl store::HostBucket for State {
    async fn get(
        &mut self, rep: Resource<Bucket>, key: String,
    ) -> wasmtime::Result<Result<Option<Vec<u8>>, store::Error>> {
        tracing::trace!("store::HostBucket::get {key}");

        let Ok(bucket) = self.table().get_mut(&rep) else {
            return Ok(Err(store::Error::NoSuchStore));
        };
        Ok(Ok(bucket.get(key).await?.map(|v| v.to_vec())))
    }

    async fn set(
        &mut self, rep: Resource<Bucket>, key: String, value: Vec<u8>,
    ) -> wasmtime::Result<Result<(), store::Error>, wasmtime::Error> {
        tracing::trace!("store::HostBucket::set {key}");

        let Ok(bucket) = self.table().get_mut(&rep) else {
            return Ok(Err(store::Error::NoSuchStore));
        };
        Ok(Ok(bucket.put(key, value.into()).await.map(|_| ())?))
    }

    async fn delete(
        &mut self, rep: Resource<Bucket>, key: String,
    ) -> Result<Result<(), store::Error>, wasmtime::Error> {
        tracing::trace!("store::HostBucket::delete {key}");

        let Ok(bucket) = self.table().get_mut(&rep) else {
            return Ok(Err(store::Error::NoSuchStore));
        };
        Ok(Ok(bucket.delete(key).await?))
    }

    async fn exists(
        &mut self, rep: Resource<Bucket>, key: String,
    ) -> wasmtime::Result<Result<bool, store::Error>> {
        tracing::trace!("store::HostBucket::exists {key}");

        let Ok(bucket) = self.table().get_mut(&rep) else {
            return Ok(Err(store::Error::NoSuchStore));
        };
        Ok(Ok(bucket.get(key).await?.is_some()))
    }

    async fn list_keys(
        &mut self, rep: Resource<Bucket>, cursor: Option<u64>,
    ) -> Result<Result<KeyResponse, store::Error>, wasmtime::Error> {
        tracing::trace!("store::HostBucket::list_keys {cursor:?}");

        let Ok(bucket) = self.table().get_mut(&rep) else {
            return Ok(Err(store::Error::NoSuchStore));
        };
        let Ok(key_stream) = bucket.keys().await else {
            return Ok(Err(store::Error::Other("Failed to list keys".into())));
        };
        let Ok(keys) = key_stream.try_collect::<Vec<String>>().await else {
            return Ok(Err(store::Error::Other("Failed to collect keys".into())));
        };

        Ok(Ok(KeyResponse { keys, cursor }))
    }

    // LATER: Can a JetStream bucket be closed?
    fn drop(&mut self, rep: Resource<Bucket>) -> Result<(), wasmtime::Error> {
        tracing::trace!("store::HostBucket::drop");
        self.table().delete(rep).map_or_else(|e| Err(anyhow!(e)), |_| Ok(()))
    }
}

#[async_trait::async_trait]
impl atomics::Host for State {
    async fn increment(
        &mut self, rep: Resource<Bucket>, key: String, delta: u64,
    ) -> wasmtime::Result<Result<u64, Error>> {
        tracing::trace!("atomics::Host::increment {key}, {delta}");

        let Ok(bucket) = self.table().get_mut(&rep) else {
            return Ok(Err(Error::NoSuchStore));
        };
        let Ok(Some(value)) = bucket.get(key.clone()).await else {
            tracing::error!("no value for {key}");
            return Ok(Err(Error::Other(format!("no value for {key}"))));
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
            return Ok(Err(Error::Other(format!("issue saving increment: {e}"))));
        }

        Ok(Ok(inc))
    }
}

#[async_trait::async_trait]
impl batch::Host for State {
    async fn get_many(
        &mut self, rep: Resource<Bucket>, keys: Vec<String>,
    ) -> wasmtime::Result<Result<Vec<Option<(String, Vec<u8>)>>, store::Error>> {
        tracing::trace!("batch::Host::get_many {keys:?}");

        let Ok(bucket) = self.table().get_mut(&rep) else {
            return Ok(Err(Error::NoSuchStore));
        };

        let mut many = Vec::new();
        for key in keys {
            let value = match bucket.get(key.clone()).await {
                Ok(value) => value,
                Err(e) => {
                    tracing::error!("issue getting value: {e}");
                    return Ok(Err(Error::Other(format!("issue getting value: {key}"))));
                }
            };
            if let Some(value) = value {
                many.push(Some((key, value.to_vec())));
            }
        }

        Ok(Ok(many))
    }

    async fn set_many(
        &mut self, rep: Resource<Bucket>, key_values: Vec<(String, Vec<u8>)>,
    ) -> wasmtime::Result<Result<(), store::Error>> {
        tracing::trace!("batch::Host::set_many {key_values:?}");

        let Ok(bucket) = self.table().get_mut(&rep) else {
            return Ok(Err(Error::NoSuchStore));
        };
        for (key, value) in key_values {
            if let Err(e) = bucket.put(key, value.into()).await {
                tracing::error!("issue Are  value: {e}");
                return Ok(Err(Error::Other(format!("issue saving value: {e}"))));
            }
        }

        Ok(Ok(()))
    }

    async fn delete_many(
        &mut self, rep: Resource<Bucket>, keys: Vec<String>,
    ) -> wasmtime::Result<Result<(), store::Error>> {
        tracing::trace!("batch::Host::delete_many {keys:?}");

        let Ok(bucket) = self.table().get_mut(&rep) else {
            return Ok(Err(Error::NoSuchStore));
        };
        for key in keys {
            if let Err(e) = bucket.delete(key).await {
                tracing::error!("issue deleting value: {e}");
                return Ok(Err(Error::Other(format!("issue deleting value: {e}"))));
            }
        }

        Ok(Ok(()))
    }
}
