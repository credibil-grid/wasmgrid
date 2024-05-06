//! # WASI Key/Value Capability
//!
//! This module implements a runtime capability for `wasi:keyvalue`
//! (<https://github.com/WebAssembly/wasi-keyvalue>).

use std::sync::OnceLock;

use anyhow::anyhow;
use async_nats::jetstream;
use bytes::Bytes;
use futures::TryStreamExt;
use wasi_keyvalue::atomics::AtomicsView;
use wasi_keyvalue::batch::BatchView;
use wasi_keyvalue::bindings::wasi::keyvalue::store::KeyResponse;
use wasi_keyvalue::bindings::Keyvalue;
use wasi_keyvalue::store::{BucketView, StoreView};
use wasi_keyvalue::{self, RuntimeBucket};
use wasmtime::component::{Linker, Resource};
use wasmtime_wasi::WasiView;

use crate::runtime::{self, Runtime, State};

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
impl StoreView for State {
    // Open bucket specified by identifier, save to state and return as a resource.
    async fn open(
        &mut self, identifier: String,
    ) -> anyhow::Result<Resource<wasi_keyvalue::Bucket>> {
        let Some(jetstream) = JETSTREAM.get() else {
            return Err(anyhow!("JetStream not initialized"));
        };

        // open bucket and save to state
        let bucket = Bucket::new(jetstream, identifier.clone()).await?;
        let boxed: wasi_keyvalue::Bucket = Box::new(bucket);

        Ok(self.table().push(boxed)?)
    }
}

#[async_trait::async_trait]
impl BucketView for State {
    async fn get(
        &mut self, bucket: Resource<wasi_keyvalue::Bucket>, key: String,
    ) -> anyhow::Result<Option<Vec<u8>>> {
        tracing::debug!("RuntimeBucket::get {key}");

        let bucket = self.table().get_mut(&bucket)?;
        let Some(bkt) = bucket.as_ref().as_any().downcast_ref::<Bucket>() else {
            return Err(anyhow!("invalid connection"));
        };

        Ok(bkt.inner.get(key).await?.map(|v| v.to_vec()))
    }

    async fn set(
        &mut self, bucket: Resource<wasi_keyvalue::Bucket>, key: String, value: Vec<u8>,
    ) -> anyhow::Result<()> {
        tracing::debug!("RuntimeBucket::set {key}");

        let bucket = self.table().get_mut(&bucket)?;
        let Some(bkt) = bucket.as_ref().as_any().downcast_ref::<Bucket>() else {
            return Err(anyhow!("invalid connection"));
        };

        Ok(bkt.inner.put(key, Bytes::from(value)).await.map(|_| ())?)
    }

    async fn delete(
        &mut self, bucket: Resource<wasi_keyvalue::Bucket>, key: String,
    ) -> anyhow::Result<()> {
        tracing::debug!("RuntimeBucket::delete {key}");

        let bucket = self.table().get_mut(&bucket)?;
        let Some(bkt) = bucket.as_ref().as_any().downcast_ref::<Bucket>() else {
            return Err(anyhow!("invalid connection"));
        };

        Ok(bkt.inner.delete(key).await?)
    }

    async fn exists(
        &mut self, bucket: Resource<wasi_keyvalue::Bucket>, key: String,
    ) -> anyhow::Result<bool> {
        tracing::debug!("RuntimeBucket::exists {key}");

        let bucket = self.table().get_mut(&bucket)?;
        let Some(bkt) = bucket.as_ref().as_any().downcast_ref::<Bucket>() else {
            return Err(anyhow!("invalid connection"));
        };

        Ok(bkt.inner.get(key).await?.is_some())
    }

    async fn list_keys(
        &mut self, bucket: Resource<wasi_keyvalue::Bucket>, cursor: Option<u64>,
    ) -> anyhow::Result<KeyResponse> {
        tracing::debug!("RuntimeBucket::list_keys {cursor:?}");

        let bucket = self.table().get_mut(&bucket)?;
        let Some(bkt) = bucket.as_ref().as_any().downcast_ref::<Bucket>() else {
            return Err(anyhow!("invalid connection"));
        };

        let keys = bkt.inner.keys().await?.try_collect::<Vec<String>>().await?;
        Ok(KeyResponse { keys, cursor })
    }

    // LATER: Can a JetStream bucket be closed?
    fn drop(&mut self, _: Resource<wasi_keyvalue::Bucket>) -> anyhow::Result<()> {
        tracing::debug!("RuntimeBucket::drop");
        Ok(())
    }
}

#[async_trait::async_trait]
impl AtomicsView for State {
    async fn increment(
        &mut self, bucket: Resource<wasi_keyvalue::Bucket>, key: String, delta: u64,
    ) -> anyhow::Result<u64> {
        tracing::debug!("RuntimeBucket::increment {key}, {delta}");

        let bucket = self.table().get_mut(&bucket)?;
        let Some(bkt) = bucket.as_ref().as_any().downcast_ref::<Bucket>() else {
            return Err(anyhow!("invalid connection"));
        };
        let value = bkt.inner.get(key.clone()).await?.unwrap_or_default();

        // increment value by delta
        let slice: &[u8] = &value;
        let mut buf = [0u8; 8];
        let len = 8.min(slice.len());
        buf[..len].copy_from_slice(&slice[..len]);
        let inc = u64::from_be_bytes(buf) + delta;

        bkt.inner.put(key, Bytes::from((inc).to_be_bytes().to_vec())).await?;

        Ok(inc)
    }
}

#[async_trait::async_trait]
impl BatchView for State {
    async fn get_many(
        &mut self, bucket: Resource<wasi_keyvalue::Bucket>, keys: Vec<String>,
    ) -> anyhow::Result<Vec<Option<(String, Vec<u8>)>>> {
        tracing::debug!("RuntimeBucket::get_many {keys:?}");

        // get concrete bucket impl
        let bucket = self.table().get_mut(&bucket)?;
        let Some(bkt) = bucket.as_ref().as_any().downcast_ref::<Bucket>() else {
            return Err(anyhow!("invalid connection"));
        };

        let mut results = Vec::new();
        for key in keys {
            let value = bkt.inner.get(&key).await?;
            if let Some(value) = value {
                results.push(Some((key, value.to_vec())));
            }
        }

        Ok(results)
    }

    async fn set_many(
        &mut self, bucket: Resource<wasi_keyvalue::Bucket>, key_values: Vec<(String, Vec<u8>)>,
    ) -> anyhow::Result<()> {
        tracing::debug!("RuntimeBucket::set_many {key_values:?}");

        // get concrete bucket impl
        let bucket = self.table().get_mut(&bucket)?;
        let Some(bkt) = bucket.as_ref().as_any().downcast_ref::<Bucket>() else {
            return Err(anyhow!("invalid connection"));
        };

        for (key, value) in key_values {
            bkt.inner.put(key, Bytes::from(value)).await?;
        }
        
        Ok(())
    }

    async fn delete_many(
        &mut self, bucket: Resource<wasi_keyvalue::Bucket>, keys: Vec<String>,
    ) -> anyhow::Result<()> {
        tracing::debug!("RuntimeBucket::delete_many {keys:?}");

        // get concrete bucket impl
        let bucket = self.table().get_mut(&bucket)?;
        let Some(bkt) = bucket.as_ref().as_any().downcast_ref::<Bucket>() else {
            return Err(anyhow!("invalid connection"));
        };

        for key in keys {
            bkt.inner.delete(key).await?;
        }

        Ok(())
    }
}

// Bucket holds a reference to the the NATS bucket. It is used to implement the
// [`wasi_keyvalue::RuntimeBucket`] trait used by the messaging State.
struct Bucket {
    inner: jetstream::kv::Store,
}

impl Bucket {
    // Create a new Bucket for the specified NATS server.
    async fn new(jetstream: &jetstream::Context, identifier: String) -> anyhow::Result<Self> {
        let inner = jetstream
            .create_key_value(jetstream::kv::Config {
                bucket: identifier.clone(),
                history: 10,
                ..Default::default()
            })
            .await?;

        Ok(Self { inner })
    }
}

// Implement the [`wasi_keyvalue::RuntimeBucket`] trait. The implementation
// allows the wasi-keyvalue host to interact with the JetStream KV store.
#[async_trait::async_trait]
impl RuntimeBucket for Bucket {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
