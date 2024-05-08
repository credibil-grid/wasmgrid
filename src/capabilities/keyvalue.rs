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
use bytes::Bytes;
use futures::TryStreamExt;
use wasmtime::component::{Linker, Resource};
use wasmtime_wasi::WasiView;

use crate::runtime::{self, Runtime, State};

static JETSTREAM: OnceLock<jetstream::Context> = OnceLock::new();

mod bindings {
    #![allow(clippy::future_not_send)]
    pub use async_nats::jetstream::kv::Store as Bucket;

    wasmtime::component::bindgen!({
        world: "keyvalue",
        path: "wit",
        tracing: true,
        async: true,
        with: {
            "wasi:keyvalue/store/bucket": Bucket,
        },
        // trappable_error_type: {
        //     "wasi:keyvalue/keyvalue-types/error" => Error,
        // },
    });
}

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

impl State {
    // Get underlying JetStream Store from bucket Resource.
    fn store(
        &mut self, bucket: &Resource<bindings::Bucket>,
    ) -> anyhow::Result<jetstream::kv::Store> {
        let bucket = self.table().get_mut(bucket)?;
        Ok(bucket.clone())
    }
}

// Implement the [`wasi_keyvalue::KeyValueView`]` trait for State.
#[async_trait::async_trait]
impl store::Host for State {
    // Open bucket specified by identifier, save to state and return as a resource.
    async fn open(
        &mut self, identifier: String,
    ) -> wasmtime::Result<Result<Resource<bindings::Bucket>, store::Error>> {
        tracing::debug!("StoreView::open {identifier}");

        let Some(jetstream) = JETSTREAM.get() else {
            return Err(anyhow!("JetStream not initialized"));
        };

        // open bucket and save to state
        let bucket = new_bucket(jetstream, identifier.clone()).await?;

        Ok(Ok(self.table().push(bucket)?))
    }
}

#[async_trait::async_trait]
impl store::HostBucket for State {
    async fn get(
        &mut self, bucket: Resource<bindings::Bucket>, key: String,
    ) -> wasmtime::Result<Result<Option<Vec<u8>>, store::Error>> {
        tracing::debug!("BucketView::get {key}");

        let store = self.store(&bucket)?;
        Ok(Ok(store.get(key).await?.map(|v| v.to_vec())))
    }

    async fn set(
        &mut self, bucket: Resource<bindings::Bucket>, key: String, value: Vec<u8>,
    ) -> wasmtime::Result<Result<(), store::Error>, wasmtime::Error> {
        tracing::debug!("BucketView::set {key}");

        let store = self.store(&bucket)?;
        Ok(Ok(store.put(key, Bytes::from(value)).await.map(|_| ())?))
    }

    async fn delete(
        &mut self, bucket: Resource<bindings::Bucket>, key: String,
    ) -> Result<Result<(), store::Error>, wasmtime::Error> {
        tracing::debug!("BucketView::delete {key}");

        let store = self.store(&bucket)?;
        Ok(Ok(store.delete(key).await?))
    }

    async fn exists(
        &mut self, bucket: Resource<bindings::Bucket>, key: String,
    ) -> wasmtime::Result<Result<bool, store::Error>> {
        tracing::debug!("BucketView::exists {key}");

        let store = self.store(&bucket)?;
        Ok(Ok(store.get(key).await?.is_some()))
    }

    async fn list_keys(
        &mut self, bucket: Resource<bindings::Bucket>, cursor: Option<u64>,
    ) -> Result<Result<KeyResponse, store::Error>, wasmtime::Error> {
        tracing::debug!("BucketView::list_keys {cursor:?}");

        let store = self.store(&bucket)?;
        let keys = store.keys().await?.try_collect::<Vec<String>>().await?;
        Ok(Ok(KeyResponse { keys, cursor }))
    }

    // LATER: Can a JetStream bucket be closed?
    fn drop(&mut self, _: Resource<bindings::Bucket>) -> Result<(), wasmtime::Error> {
        tracing::debug!("BucketView::drop");
        Ok(())
    }
}

#[async_trait::async_trait]
impl atomics::Host for State {
    async fn increment(
        &mut self, bucket: Resource<bindings::Bucket>, key: String, delta: u64,
    ) -> wasmtime::Result<Result<u64, Error>> {
        tracing::debug!("AtomicsView::increment {key}, {delta}");

        let store = self.store(&bucket)?;
        let value = store.get(key.clone()).await?.unwrap_or_default();

        // increment value by delta
        let slice: &[u8] = &value;
        let mut buf = [0u8; 8];
        let len = 8.min(slice.len());
        buf[..len].copy_from_slice(&slice[..len]);
        let inc = u64::from_be_bytes(buf) + delta;

        store.put(key, Bytes::from((inc).to_be_bytes().to_vec())).await?;

        Ok(Ok(inc))
    }
}

#[async_trait::async_trait]
impl batch::Host for State {
    async fn get_many(
        &mut self, bucket: Resource<bindings::Bucket>, keys: Vec<String>,
    ) -> wasmtime::Result<Result<Vec<Option<(String, Vec<u8>)>>, store::Error>> {
        tracing::debug!("BatchView::get_many {keys:?}");

        let store = self.store(&bucket)?;

        let mut results = Vec::new();
        for key in keys {
            let value = store.get(&key).await?;
            if let Some(value) = value {
                results.push(Some((key, value.to_vec())));
            }
        }

        Ok(Ok(results))
    }

    async fn set_many(
        &mut self, bucket: Resource<bindings::Bucket>, key_values: Vec<(String, Vec<u8>)>,
    ) -> wasmtime::Result<Result<(), store::Error>> {
        tracing::debug!("BatchView::set_many {key_values:?}");

        let store = self.store(&bucket)?;
        for (key, value) in key_values {
            store.put(key, Bytes::from(value)).await?;
        }

        Ok(Ok(()))
    }

    async fn delete_many(
        &mut self, bucket: Resource<bindings::Bucket>, keys: Vec<String>,
    ) -> wasmtime::Result<Result<(), store::Error>> {
        tracing::debug!("BatchView::delete_many {keys:?}");

        let store = self.store(&bucket)?;
        for key in keys {
            store.delete(key).await?;
        }

        Ok(Ok(()))
    }
}

// Create a new Bucket for the specified NATS server.
async fn new_bucket(
    jetstream: &jetstream::Context, identifier: String,
) -> anyhow::Result<bindings::Bucket> {
    let inner = jetstream
        .create_key_value(jetstream::kv::Config {
            bucket: identifier.clone(),
            history: 10,
            ..Default::default()
        })
        .await?;

    Ok(inner)
}
