//! # JetStream Key/Value Runtime
//!
//! This module implements a NATS wasi:messaging runtime.

use std::sync::OnceLock;

use anyhow::anyhow;
use async_nats::jetstream;
use bytes::Bytes;
use futures::TryStreamExt;
use wasi_keyvalue::bindings::wasi::keyvalue::store::KeyResponse;
use wasi_keyvalue::bindings::Keyvalue;
use wasi_keyvalue::{self, KeyValueView, RuntimeBucket};
use wasmtime::component::{Linker, Resource};
use wasmtime_wasi::WasiView;

use crate::runtime::{self, Runtime, State};

static JETSTREAM: OnceLock<jetstream::Context> = OnceLock::new();

pub struct Capability {
    pub addr: String,
}

impl Capability {
    pub const fn new(addr: String) -> Self {
        Self { addr }
    }
}

#[async_trait::async_trait]
impl runtime::Capability for Capability {
        fn component_type(&self) -> &str {
        "wasi:keyvalue"
    }

    fn add_to_linker(&self, linker: &mut Linker<State>) -> anyhow::Result<()> {
        Keyvalue::add_to_linker(linker, |t| t)
    }

    /// Start and run NATS for the specified wasm component.
    async fn run(&self, runtime: Runtime) -> anyhow::Result<()> {
        // create JetStream context and store in global state
        let client = async_nats::connect(&self.addr).await?;
        tracing::info!("connected to NATS on {}", self.addr);

        JETSTREAM.get_or_init(|| jetstream::new(client));
        Ok(())
    }
}

// Implement the [`wasi_keyvalue::KeyValueView`]` trait for State.
#[async_trait::async_trait]
impl KeyValueView for State {
    // Open bucket specified by identifier, save to state and return as a resource.
    async fn open(
        &mut self, identifier: String,
    ) -> anyhow::Result<Resource<wasi_keyvalue::Bucket>> {
        let Some(jetstream) = JETSTREAM.get() else {
            return Err(anyhow!("JetStream not initialized"));
        };

        // open bucket and save to state
        let bucket = Bucket::new(jetstream, identifier.clone()).await?;
        let bucket: wasi_keyvalue::Bucket = Box::new(bucket);

        Ok(self.table().push(bucket)?)
    }
}

// Bucket holds a reference to the the NATS bucket. It is used to implement the
// [`wasi_keyvalue::RuntimeBucket`] trait used by the messaging State.
#[derive(Clone)]
pub struct Bucket {
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
    // ------------------------------------------------------------------------
    // Store
    // ------------------------------------------------------------------------
    async fn get(&mut self, key: String) -> anyhow::Result<Option<Vec<u8>>> {
        tracing::debug!("RuntimeBucket::get {key}");
        Ok(self.inner.get(key).await?.map(|v| v.to_vec()))
    }

    async fn set(&mut self, key: String, value: Vec<u8>) -> anyhow::Result<()> {
        tracing::debug!("RuntimeBucket::set {key}");
        Ok(self.inner.put(key, Bytes::from(value)).await.map(|_| ())?)
    }

    async fn delete(&mut self, key: String) -> anyhow::Result<()> {
        tracing::debug!("RuntimeBucket::delete {key}");
        Ok(self.inner.delete(key).await?)
    }

    async fn exists(&mut self, key: String) -> anyhow::Result<bool> {
        tracing::debug!("RuntimeBucket::exists {key}");
        Ok(self.inner.get(key).await?.is_some())
    }

    async fn list_keys(&mut self, cursor: Option<u64>) -> anyhow::Result<KeyResponse> {
        tracing::debug!("RuntimeBucket::list_keys {cursor:?}");
        let keys = self.inner.keys().await?.try_collect::<Vec<String>>().await?;
        Ok(KeyResponse { keys, cursor })
    }

    // LATER: Can a JetStream bucket be closed?
    fn close(&mut self) -> anyhow::Result<()> {
        tracing::debug!("RuntimeBucket::close");
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Atomics
    // ------------------------------------------------------------------------
    async fn increment(&mut self, key: String, delta: u64) -> anyhow::Result<u64> {
        tracing::debug!("RuntimeBucket::increment {key}, {delta}");

        let value = self.inner.get(&key).await?.unwrap_or_default();

        // increment value by delta
        let slice: &[u8] = &value;
        let mut buf = [0u8; 8];
        let len = 8.min(slice.len());
        buf[..len].copy_from_slice(&slice[..len]);
        let inc = u64::from_be_bytes(buf) + delta;

        self.inner.put(&key, Bytes::from((inc).to_be_bytes().to_vec())).await?;

        Ok(inc)
    }

    // ------------------------------------------------------------------------
    // Batch
    // ------------------------------------------------------------------------
    async fn get_many(&mut self, keys: Vec<String>) -> anyhow::Result<Vec<(String, Vec<u8>)>> {
        tracing::debug!("RuntimeBucket::get_many {keys:?}");

        let mut results = Vec::new();
        for key in keys {
            let value = self.inner.get(&key).await?;
            if let Some(value) = value {
                results.push((key, value.to_vec()));
            }
        }
        Ok(results)
    }

    async fn set_many(&mut self, key_values: Vec<(String, Vec<u8>)>) -> anyhow::Result<()> {
        tracing::debug!("RuntimeBucket::set_many {key_values:?}");

        for (key, value) in key_values {
            self.inner.put(key, Bytes::from(value)).await?;
        }
        Ok(())
    }

    async fn delete_many(&mut self, keys: Vec<String>) -> anyhow::Result<()> {
        tracing::debug!("RuntimeBucket::delete_many {keys:?}");

        for key in keys {
            self.inner.delete(key).await?;
        }
        Ok(())
    }
}
