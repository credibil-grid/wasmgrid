mod consumer;
mod producer;
pub mod types;

use std::collections::HashMap;

use types::Client;
use wasmtime::component::Resource;
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

use crate::wasi::messaging::messaging_types::{self, Error, HostClient, HostError};

/// Host is the base type used to implement host messaging interfaces.
/// In addition, it holds the "host-defined state" used by the wasm runtime [`Store`].
pub struct Host {
    keys: HashMap<String, u32>,
    pub table: ResourceTable,
    ctx: WasiCtx,
}

impl Default for Host {
    /// Create a default instance of the host state for use in initialisng the [`Store`].
    fn default() -> Self {
        Self {
            keys: HashMap::default(),
            table: ResourceTable::default(),
            ctx: WasiCtxBuilder::new().inherit_env().build(),
        }
    }
}

impl Host {
    pub fn new() -> Self {
        Self::default()
    }
}

impl messaging_types::Host for Host {}

#[async_trait::async_trait]
impl HostClient for Host {
    /// Connect to the NATS server specified by `name` and return a client resource.
    async fn connect(
        &mut self, name: String,
    ) -> wasmtime::Result<anyhow::Result<Resource<Client>, Resource<Error>>> {
        let resource = if let Some(key) = self.keys.get(&name) {
            // Get an existing connection by key
            // let any = self.table.get_any_mut(*key).unwrap();
            // Resource::try_from_resource_any(any, store).unwrap()
            println!("Reusing existing connection");
            Resource::new_own(*key)
        } else {
            // Create a new connection
            println!("New connection");
            let nats_client = async_nats::connect("demo.nats.io").await?;
            let client = Client { inner: nats_client };
            let resource = self.table.push(client)?;
            self.keys.insert(name, resource.rep());
            resource
        };

        Ok(Ok(resource))
    }

    /// Drop the specified NATS client resource.
    fn drop(&mut self, client: Resource<Client>) -> wasmtime::Result<()> {
        self.keys.retain(|_, v| *v != client.rep());
        let _ = self.table.delete(client)?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl HostError for Host {
    async fn trace(&mut self) -> wasmtime::Result<String> {
        Ok(String::from("trace HostError"))
    }

    fn drop(&mut self, err: Resource<Error>) -> wasmtime::Result<()> {
        println!("Implement drop for {err:?}");
        Ok(())
    }
}

impl WasiView for Host {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
