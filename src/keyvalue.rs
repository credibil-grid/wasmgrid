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
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
}

#[async_trait::async_trait]
impl runtime::Capability for Capability {
    fn add_to_linker(&self, linker: &mut Linker<State>) -> anyhow::Result<()> {
        Keyvalue::add_to_linker(linker, |t| t)
    }

    /// Start and run NATS for the specified wasm component.
    async fn run(&self, _runtime: Runtime) -> anyhow::Result<()> {
        // create JetStream context and store in global state
        let client = async_nats::connect(&self.addr).await?;
        JETSTREAM.get_or_init(|| jetstream::new(client));

        tracing::info!(target: "keyvalue", "connected to NATS on {}", self.addr);

        Ok(())
    }
}

// Implement the [`wasi_keyvalue::KeyValueView`]` trait for State.
#[async_trait::async_trait]
impl KeyValueView for State {
    async fn open(
        &mut self, identifier: String,
    ) -> anyhow::Result<Resource<wasi_keyvalue::Bucket>> {
        // open bucket specified by identifier
        let Some(jetstream) = JETSTREAM.get() else {
            return Err(anyhow!("JetStream not initialized"));
        };
        let bucket = Bucket::new(jetstream, identifier.clone()).await?;

        // save opened bucket to state
        let bucket: wasi_keyvalue::Bucket = Box::new(bucket);
        let resource = self.table().push(bucket)?;

        Ok(resource)
    }
}

// Bucket holds a reference to the the NATS bucket. It is used to implement the
// [`wasi_keyvalue::RuntimeBucket`] trait used by the messaging State.
#[derive(Clone)]
pub struct Bucket {
    // identifier: String,
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
        Ok(self.inner.get(key).await?.map(|v| v.to_vec()))
    }

    async fn set(&mut self, key: String, value: Vec<u8>) -> anyhow::Result<()> {
        Ok(self.inner.put(key, Bytes::from(value)).await.map(|_| ())?)
    }

    async fn delete(&mut self, key: String) -> anyhow::Result<()> {
        Ok(self.inner.delete(key).await?)
    }

    async fn exists(&mut self, key: String) -> anyhow::Result<bool> {
        Ok(self.inner.get(key).await?.is_some())
    }

    async fn list_keys(&mut self, cursor: Option<u64>) -> anyhow::Result<KeyResponse> {
        let keys = self.inner.keys().await?.try_collect::<Vec<String>>().await?;
        Ok(KeyResponse { keys, cursor })
    }

    // LATER: Can a JetStream bucket be closed?
    fn close(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Atomics
    // ------------------------------------------------------------------------
    async fn increment(&mut self, key: String, delta: u64) -> anyhow::Result<u64> {
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
        for (key, value) in key_values {
            self.inner.put(key, Bytes::from(value)).await?;
        }
        Ok(())
    }

    async fn delete_many(&mut self, keys: Vec<String>) -> anyhow::Result<()> {
        for key in keys {
            self.inner.delete(key).await?;
        }
        Ok(())
    }
}
