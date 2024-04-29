//! # JetStream KeyValue Runtime
//!
//! This module implements a NATS wasi:messaging runtime.

// use anyhow::anyhow;
use async_nats::jetstream;
use wasi_keyvalue::bindings::wasi::keyvalue::store::KeyResponse;
use wasi_keyvalue::bindings::Keyvalue;
use wasi_keyvalue::{self, KeyValueView, RuntimeBucket};
use wasmtime::component::{Linker, Resource};
use wasmtime_wasi::WasiView;

use crate::runtime::{self, Runtime, State};

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
    async fn run(&self, system: Runtime) -> anyhow::Result<()> {
        // create JetStream context
        let client = async_nats::connect(&self.addr).await?;
        let jetstream = jetstream::new(client);

        // save context to state
        let store = &mut system.store();
        store.data_mut().metadata.insert("context".to_string(), Box::new(jetstream));

        println!("Connected to NATS: {}", self.addr);

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
        // let jetstream =
        //     self.metadata.get("context").unwrap().downcast_ref::<jetstream::Context>().unwrap();
        let client = async_nats::connect("demo.nats.io").await?;
        let jetstream = jetstream::new(client);
        let bucket = Bucket::new(&jetstream, identifier.clone()).await?;

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
    identifier: String,
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

        Ok(Self { identifier, inner })
    }
}

// Implement the [`wasi_keyvalue::RuntimeBucket`] trait for Bucket. This trait
// implementation is used by the messaging State to interact with the NATS bucket.
#[async_trait::async_trait]
impl RuntimeBucket for Bucket {
    // ------------------------------------------------------------------------
    // Store
    // ------------------------------------------------------------------------
    async fn get(&mut self, key: String) -> anyhow::Result<Vec<u8>> {
        unimplemented!("get")
    }

    async fn set(&mut self, key: String, value: Vec<u8>) -> anyhow::Result<()> {
        unimplemented!("set")
    }

    async fn delete(&mut self, key: String) -> anyhow::Result<()> {
        unimplemented!("delete")
    }

    async fn exists(&mut self, key: String) -> anyhow::Result<bool> {
        unimplemented!("exists")
    }

    async fn list_keys(&mut self, keys_: Option<u64>) -> anyhow::Result<KeyResponse> {
        unimplemented!("list_keys")
    }

    fn close(&mut self) -> anyhow::Result<()> {
        unimplemented!("close")
    }

    // ------------------------------------------------------------------------
    // Atomics
    // ------------------------------------------------------------------------
    async fn increment(&mut self, key: String, delta: u64) -> anyhow::Result<u64> {
        unimplemented!("increment")
    }

    // ------------------------------------------------------------------------
    // Batch
    // ------------------------------------------------------------------------
    async fn get_many(&mut self, keys: Vec<String>) -> anyhow::Result<Vec<(String, Vec<u8>)>> {
        unimplemented!("get_many")
    }

    async fn set_many(&mut self, key_values: Vec<(String, Vec<u8>)>) -> anyhow::Result<()> {
        unimplemented!("set_many")
    }

    async fn delete_many(&mut self, keys: Vec<String>) -> anyhow::Result<()> {
        unimplemented!("delete_many")
    }
}
