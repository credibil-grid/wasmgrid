mod consumer;
mod producer;

use std::collections::HashMap;

use async_nats::Client;
use futures::stream::{self, StreamExt};
use wasmtime::component::Resource;
use wasmtime::Store;
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

use crate::exports::wasi::messaging::messaging_guest::Guest;
use crate::wasi::messaging::messaging_types::{
    Error, FormatSpec, Host, HostClient, HostError, Message,
};

/// Nats is the base type used to implement host messaging interfaces.
/// In addition, it holds the "host-defined state" used by the wasm runtime [`Store`].
pub struct Nats {
    keys: HashMap<String, u32>,
    table: ResourceTable,
    ctx: WasiCtx,
}

impl Nats {
    /// Create a default instance of the host state for use in initialisng the [`Store`].
    pub fn default() -> Self {
        Self {
            keys: HashMap::default(),
            table: ResourceTable::default(),
            ctx: WasiCtxBuilder::new().inherit_env().build(),
        }
    }

    /// Run the NATS messaging service. The method subscribes to configured channels and processes
    /// messages blocking the current thread until terminated.
    pub async fn run(store: &mut Store<Nats>, guest: &Guest) -> wasmtime::Result<()> {
        // connect to NATS server
        let host_state = store.data_mut();
        let client = HostClient::connect(host_state, "demo.nats.io".to_string()).await?.unwrap();
        let client = host_state.table.get(&client)?.clone();

        // subscribe to configured channels
        let gc = guest.call_configure(&mut *store).await?;
        let mut subscribers = vec![];
        for ch in &gc.unwrap().channels {
            let subscriber = client.subscribe(ch.to_owned()).await?;
            subscribers.push(subscriber);
        }

        // process messages until terminated
        let mut messages = stream::select_all(subscribers);
        while let Some(message) = messages.next().await {
            let msg = Message {
                data: message.payload.to_vec(),
                metadata: Some(vec![(String::from("channel"), message.subject.to_string())]),
                format: FormatSpec::Raw,
            };
            let _ = guest.call_handler(&mut *store, &[msg]).await?;
        }

        Ok(())
    }
}

impl Host for Nats {}

#[async_trait::async_trait]
impl HostClient for Nats {
    /// Connect to the NATS server specified by `name` and return a client resource.
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

    /// Drop the specified NATS client resource.
    fn drop(&mut self, client: Resource<Client>) -> wasmtime::Result<()> {
        self.keys.retain(|_, v| *v != client.rep());
        let _ = self.table.delete(client)?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl HostError for Nats {
    async fn trace(&mut self) -> wasmtime::Result<String> {
        Ok(String::from("trace HostError"))
    }

    fn drop(&mut self, err: Resource<Error>) -> wasmtime::Result<()> {
        println!("Implement drop for {err:?}");
        Ok(())
    }
}

impl WasiView for Nats {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
