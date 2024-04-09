mod consumer;
mod producer;

use anyhow::anyhow;
use wasmtime::component::Resource;
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

use crate::wasi::messaging::consumer::Host;
use crate::wasi::messaging::messaging_types;
use crate::wasi::messaging::messaging_types::{Client, Error, GuestConfiguration};

pub struct HostState {
    client: async_nats::Client,
    pub subscribers: Vec<async_nats::Subscriber>,
    table: ResourceTable,
    ctx: WasiCtx,
}

impl HostState {
    pub async fn new() -> anyhow::Result<Self> {
        let client = async_nats::connect("demo.nats.io").await?;

        Ok(Self {
            client,
            subscribers: Vec::new(),
            table: ResourceTable::new(),
            ctx: WasiCtxBuilder::new().inherit_env().build(),
        })
    }

    pub async fn init(&mut self, gc: GuestConfiguration) -> anyhow::Result<()> {
        match self.update_guest_configuration(gc).await {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => Err(anyhow!("{:?}", e)),
            Err(e) => Err(e),
        }
    }
}

impl messaging_types::Host for HostState {}

#[async_trait::async_trait]
impl messaging_types::HostClient for HostState {
    async fn connect(
        &mut self, name: String,
    ) -> wasmtime::Result<anyhow::Result<Resource<Client>, Resource<Error>>> {
        println!("connect client: {name}");

        let client = async_nats::connect("demo.nats.io").await?;
        self.client = client;

        // let client2 = self.table.push(test).unwrap();
        // self.table.push(client2);
        // let res = self.table.push_child(client, &client2).unwrap();

        Ok(Ok(Resource::new_own(0)))
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
