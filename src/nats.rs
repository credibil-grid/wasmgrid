mod consumer;
mod producer;

use std::collections::HashMap;

use async_nats::Client;
use wasmtime::component::Resource;
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

use crate::wasi::messaging::messaging_types;
use crate::wasi::messaging::messaging_types::Error; //GuestConfiguration

pub struct HostState {
    cnns: HashMap<String, u32>,
    table: ResourceTable,
    ctx: WasiCtx,
}

impl HostState {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            cnns: HashMap::new(),
            table: ResourceTable::new(),
            ctx: WasiCtxBuilder::new().inherit_env().build(),
        })
    }

    pub async fn client(&self, client: Resource<Client>) -> anyhow::Result<Client> {
        let client = self.table.get(&client).unwrap();
        Ok(client.clone())
    }

    // pub async fn init(&mut self, gc: GuestConfiguration) -> anyhow::Result<()> {
    //     match self.update_guest_configuration(gc).await {
    //         Ok(Ok(_)) => Ok(()),
    //         Ok(Err(e)) => Err(anyhow!("{:?}", e)),
    //         Err(e) => Err(e),
    //     }
    // }
}

impl messaging_types::Host for HostState {}

#[async_trait::async_trait]
impl messaging_types::HostClient for HostState {
    async fn connect(
        &mut self, name: String,
    ) -> wasmtime::Result<anyhow::Result<Resource<Client>, Resource<Error>>> {
        // get existing resource entries

        let resource = match self.cnns.get(&name) {
            Some(key) => {
                // Get existing connection by saved key
                // let any = self.table.get_any_mut(*key).unwrap();
                // Resource::try_from_resource_any(any, store).unwrap()
                Resource::new_own(*key)
            }
            None => {
                // Create new connection
                let client = async_nats::connect("demo.nats.io").await?;
                let resource = self.table.push(client)?;
                self.cnns.insert(name, resource.rep());
                resource
            }
        };

        Ok(Ok(resource))
    }

    fn drop(&mut self, client: Resource<Client>) -> wasmtime::Result<()> {
        todo!("Implement drop for {client:?}")
    }
}

#[async_trait::async_trait]
impl messaging_types::HostError for HostState {
    async fn trace(&mut self) -> wasmtime::Result<String> {
        todo!("Implement trace")
    }

    fn drop(&mut self, err: Resource<Error>) -> wasmtime::Result<()> {
        todo!("Implement drop for {err:?}")
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
