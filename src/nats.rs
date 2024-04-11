mod consumer;
mod producer;

use std::collections::HashMap;

use async_nats::Client;
use futures::stream::{self, StreamExt};
// use tracing::{event, span, Level};
use wasmtime::component::Resource;
use wasmtime::component::{Component, Linker};
use wasmtime::{AsContextMut, Engine, Store};
use wasmtime_wasi::{command, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

use crate::wasi::messaging::messaging_types::Error; //GuestConfiguration
use crate::wasi::messaging::messaging_types::{FormatSpec, Message};
use crate::wasi::messaging::{self, messaging_types};

pub struct HostState {
    keys: HashMap<String, u32>,
    table: ResourceTable,
    ctx: WasiCtx,
}

impl HostState {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            table: ResourceTable::new(),
            ctx: WasiCtxBuilder::new().inherit_env().build(),
        }
    }

    pub async fn run(
        self, engine: &Engine, component: &Component, client: &Resource<Client>,
    ) -> wasmtime::Result<()> {
        
        let client = self.table.get(client)?.clone();

        let mut linker = Linker::new(&engine);
        command::add_to_linker(&mut linker)?;
        messaging_types::add_to_linker(&mut linker, |t| t)?;
        messaging::producer::add_to_linker(&mut linker, |t| t)?;
        messaging::consumer::add_to_linker(&mut linker, |t| t)?;

        let mut store = Store::new(&engine, self);
        let (messaging, _instance) =
            crate::Messaging::instantiate_async(&mut store, &component, &linker).await?;

        let guest = messaging.wasi_messaging_messaging_guest();
        let gc = guest.call_configure(store.as_context_mut()).await?;
        let mut subscribers = vec![];
        for ch in &gc.unwrap().channels {
            let subscriber = client.subscribe(ch.to_owned()).await?;
            subscribers.push(subscriber);
        }

        let mut messages = stream::select_all(subscribers);
        while let Some(message) = messages.next().await {
            let msg = Message {
                data: message.payload.to_vec(),
                metadata: Some(vec![(String::from("channel"), message.subject.to_string())]),
                format: FormatSpec::Raw,
            };
            let _ = guest.call_handler(store.as_context_mut(), &[msg]).await?;
        }

        Ok(())
    }
}

impl messaging_types::Host for HostState {}

#[async_trait::async_trait]
impl messaging_types::HostClient for HostState {
    async fn connect(
        &mut self, name: String,
    ) -> wasmtime::Result<anyhow::Result<Resource<Client>, Resource<Error>>> {
        // get existing resource entries

        let resource = if let Some(key) = self.keys.get(&name) {
            // Get an existing connection by key
            // let any = self.table.get_any_mut(*key).unwrap();
            // Resource::try_from_resource_any(any, store).unwrap()
            Resource::new_own(*key)
        } else {
            // Create a new connection
            let client = async_nats::connect("demo.nats.io").await?;
            let resource = self.table.push(client)?;
            self.keys.insert(name, resource.rep());
            resource
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
