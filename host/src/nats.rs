mod consumer;
mod producer;

use wasmtime::component::Resource;
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

use crate::wasi::messaging::messaging_types;
use crate::wasi::messaging::messaging_types::{Client, Error};

pub struct NatsHost {
    client: async_nats::Client,
    subscriber: Option<async_nats::Subscriber>,
    table: ResourceTable,
    ctx: WasiCtx,
    // limits: StoreLimits,
}

#[allow(clippy::module_name_repetitions)]
impl NatsHost {
    pub async fn new() -> anyhow::Result<Self> {
        let client = async_nats::connect("demo.nats.io").await?;

        Ok(Self {
            client,
            subscriber: None,
            table: ResourceTable::new(),
            ctx: WasiCtxBuilder::new().inherit_env().build(),
            // limits: StoreLimits::default(),
        })
    }
}

impl messaging_types::Host for super::NatsHost {}

#[async_trait::async_trait]
impl messaging_types::HostClient for super::NatsHost {
    async fn connect(
        &mut self, ch: String,
    ) -> wasmtime::Result<anyhow::Result<Resource<Client>, Resource<Error>>> {
        let subscriber = self.client.subscribe(ch).await?;
        self.subscriber = Some(subscriber);

        Ok(Ok(Resource::new_own(1)))
    }

    fn drop(&mut self, client: Resource<Client>) -> wasmtime::Result<()> {
        todo!("Implement drop for {client:?}")
    }
}

#[async_trait::async_trait]
impl messaging_types::HostError for super::NatsHost {
    async fn trace(&mut self) -> wasmtime::Result<String> {
        todo!("Implement trace")
    }

    fn drop(&mut self, err: Resource<Error>) -> wasmtime::Result<()> {
        todo!("Implement drop for {err:?}")
    }
}

impl WasiView for NatsHost {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
