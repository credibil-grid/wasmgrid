//! # WASI Key/Value Capability
//!
//! This module implements a runtime capability for `wasi:keyvalue`
//! (<https://github.com/WebAssembly/wasi-keyvalue>).

use std::sync::OnceLock;

use anyhow::anyhow;
use async_nats::jetstream;
use bindings::wasi::keyvalue::store::{self, Error, KeyResponse};
use bindings::wasi::keyvalue::{atomics, batch};
use bindings::Keyvalue;
use futures::TryStreamExt;
use wasmtime::component::{Linker, Resource};
use wasmtime_wasi::WasiView;

use crate::runtime::{self, Runtime, State};

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
    pub addr: String,
}

pub const fn new(addr: String) -> Capability {
    Capability { addr }
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
        // create JetStream context and store in global state
        let client = async_nats::connect(&self.addr).await?;
        tracing::info!("connected to JetStream");

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
        tracing::debug!("store::Host::open {identifier}");

        let Some(jetstream) = JETSTREAM.get() else {
            return Err(anyhow!("JetStream not initialized"));
        };

        let bucket = jetstream
            .create_key_value(jetstream::kv::Config {
                bucket: identifier.clone(),
                history: 10,
                ..Default::default()
            })
            .await?;

        Ok(Ok(self.table().push(bucket)?))
    }
}

#[async_trait::async_trait]
impl store::HostBucket for State {
    async fn get(
        &mut self, rep: Resource<Bucket>, key: String,
    ) -> wasmtime::Result<Result<Option<Vec<u8>>, store::Error>> {
        tracing::debug!("store::HostBucket::get {key}");
        let bucket = self.table().get_mut(&rep)?;
        Ok(Ok(bucket.get(key).await?.map(|v| v.to_vec())))
    }

    async fn set(
        &mut self, rep: Resource<Bucket>, key: String, value: Vec<u8>,
    ) -> wasmtime::Result<Result<(), store::Error>, wasmtime::Error> {
        tracing::debug!("store::HostBucket::set {key}");
        let bucket = self.table().get_mut(&rep)?;
        Ok(Ok(bucket.put(key, value.into()).await.map(|_| ())?))
    }

    async fn delete(
        &mut self, rep: Resource<Bucket>, key: String,
    ) -> Result<Result<(), store::Error>, wasmtime::Error> {
        tracing::debug!("store::HostBucket::delete {key}");
        let bucket = self.table().get_mut(&rep)?;
        Ok(Ok(bucket.delete(key).await?))
    }

    async fn exists(
        &mut self, rep: Resource<Bucket>, key: String,
    ) -> wasmtime::Result<Result<bool, store::Error>> {
        tracing::debug!("store::HostBucket::exists {key}");
        let bucket = self.table().get_mut(&rep)?;
        Ok(Ok(bucket.get(key).await?.is_some()))
    }

    async fn list_keys(
        &mut self, rep: Resource<Bucket>, cursor: Option<u64>,
    ) -> Result<Result<KeyResponse, store::Error>, wasmtime::Error> {
        tracing::debug!("store::HostBucket::list_keys {cursor:?}");
        let bucket = self.table().get_mut(&rep)?;
        let keys = bucket.keys().await?.try_collect::<Vec<String>>().await?;
        Ok(Ok(KeyResponse { keys, cursor }))
    }

    // LATER: Can a JetStream bucket be closed?
    fn drop(&mut self, rep: Resource<Bucket>) -> Result<(), wasmtime::Error> {
        tracing::debug!("store::HostBucket::drop");
        self.table().delete(rep).map_or_else(|e| Err(anyhow!(e)), |_| Ok(()))
    }
}

#[async_trait::async_trait]
impl atomics::Host for State {
    async fn increment(
        &mut self, rep: Resource<Bucket>, key: String, delta: u64,
    ) -> wasmtime::Result<Result<u64, Error>> {
        tracing::debug!("atomics::Host::increment {key}, {delta}");

        let bucket = self.table().get_mut(&rep)?;
        let value = bucket.get(key.clone()).await?.unwrap_or_default();

        // increment value by delta
        let slice: &[u8] = &value;
        let mut buf = [0u8; 8];
        let len = 8.min(slice.len());
        buf[..len].copy_from_slice(&slice[..len]);
        let inc = u64::from_be_bytes(buf) + delta;

        bucket.put(key, inc.to_be_bytes().to_vec().into()).await?;

        Ok(Ok(inc))
    }
}

#[async_trait::async_trait]
impl batch::Host for State {
    async fn get_many(
        &mut self, rep: Resource<Bucket>, keys: Vec<String>,
    ) -> wasmtime::Result<Result<Vec<Option<(String, Vec<u8>)>>, store::Error>> {
        tracing::debug!("batch::Host::get_many {keys:?}");

        let bucket = self.table().get_mut(&rep)?;

        let mut many = Vec::new();
        for key in keys {
            let value = bucket.get(&key).await?;
            if let Some(value) = value {
                many.push(Some((key, value.to_vec())));
            }
        }

        Ok(Ok(many))
    }

    async fn set_many(
        &mut self, rep: Resource<Bucket>, key_values: Vec<(String, Vec<u8>)>,
    ) -> wasmtime::Result<Result<(), store::Error>> {
        tracing::debug!("batch::Host::set_many {key_values:?}");

        let bucket = self.table().get_mut(&rep)?;
        for (key, value) in key_values {
            bucket.put(key, value.into()).await?;
        }

        Ok(Ok(()))
    }

    async fn delete_many(
        &mut self, rep: Resource<Bucket>, keys: Vec<String>,
    ) -> wasmtime::Result<Result<(), store::Error>> {
        tracing::debug!("batch::Host::delete_many {keys:?}");

        let bucket = self.table().get_mut(&rep)?;
        for key in keys {
            bucket.delete(key).await?;
        }

        Ok(Ok(()))
    }
}
