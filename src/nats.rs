mod consumer;
mod producer;

use std::collections::HashMap;

use async_nats::Client;
// use tracing::{event, span, Level};
use wasmtime::component::Resource;
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

use crate::wasi::messaging::messaging_types;
use crate::wasi::messaging::messaging_types::Error; //GuestConfiguration

pub struct HostState {
    keys: HashMap<String, u32>,
    table: ResourceTable,
    ctx: WasiCtx,
}

impl HostState {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            keys: HashMap::new(),
            table: ResourceTable::new(),
            ctx: WasiCtxBuilder::new().inherit_env().build(),
        })
    }

    pub fn client(&self, client: Resource<Client>) -> anyhow::Result<Client> {
        let client = self.table.get(&client).unwrap();
        Ok(client.clone())
    }
}

impl messaging_types::Host for HostState {}

#[async_trait::async_trait]
impl messaging_types::HostClient for HostState {
    async fn connect(
        &mut self, name: String,
    ) -> wasmtime::Result<anyhow::Result<Resource<Client>, Resource<Error>>> {
        // get existing resource entries

        let resource = match self.keys.get(&name) {
            Some(key) => {
                // Get an existing connection by key
                // let any = self.table.get_any_mut(*key).unwrap();
                // Resource::try_from_resource_any(any, store).unwrap()
                Resource::new_own(*key)
            }
            None => {
                // Create a new connection
                let client = async_nats::connect("demo.nats.io").await?;
                let resource = self.table.push(client)?;
                self.keys.insert(name, resource.rep());
                resource
            }
        };

        Ok(Ok(resource))
    }

    fn drop(&mut self, client: Resource<Client>) -> wasmtime::Result<()> {
        self.keys.retain(|_, v| *v != client.rep());
        let _ = self.table.delete(client)?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl messaging_types::HostError for HostState {
    async fn trace(&mut self) -> wasmtime::Result<String> {
        Ok(String::from("trace HostError"))
    }

    fn drop(&mut self, err: Resource<Error>) -> wasmtime::Result<()> {
        println!("Implement drop for {err:?}");
        Ok(())
    }
}

impl WasiView for HostState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
