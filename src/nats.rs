use std::collections::HashMap;

use bytes::Bytes;
use futures::stream::{self, StreamExt};
use messaging::bindings::messaging_types::{FormatSpec, Message};
use messaging::bindings::Messaging;
use messaging::{self, Client, MessagingClient, MessagingView};
use wasmtime::component::{Component, Linker, Resource};
use wasmtime::{Engine, Store};
use wasmtime_wasi::{command, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

/// Host is the base type used to implement host messaging interfaces.
/// In addition, it holds the "host-defined state" used by the wasm runtime [`Store`].
pub struct Host {
    keys: HashMap<String, u32>,
    pub table: ResourceTable,
    ctx: WasiCtx,
}

impl Host {
    pub fn new() -> Self {
        Self {
            keys: HashMap::default(),
            table: ResourceTable::default(),
            ctx: WasiCtxBuilder::new().inherit_env().build(),
        }
    }
}

#[async_trait::async_trait]
impl messaging::MessagingView for Host {
    async fn connect(&mut self, name: String) -> anyhow::Result<Resource<Client>> {
        let resource = if let Some(key) = self.keys.get(&name) {
            // Get an existing connection by key
            // let any = self.table.get_any_mut(*key).unwrap();
            // Resource::try_from_resource_any(any, store).unwrap()
            println!("Reusing connection");
            Resource::new_own(*key)
        } else {
            // Create a new connection
            let client = Client::new(Box::new(MyClient {
                inner: async_nats::connect(&name).await?,
            }));
            let resource = self.table.push(client)?;
            self.keys.insert(name, resource.rep());
            resource
        };

        Ok(resource)
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

#[derive(Clone)]
pub struct MyClient {
    pub inner: async_nats::Client,
}

#[async_trait::async_trait]
impl MessagingClient for MyClient {
    async fn subscribe(&self, ch: String) -> anyhow::Result<async_nats::Subscriber> {
        Ok(self.inner.subscribe(ch).await?)
    }

    async fn publish(&self, ch: String, data: Bytes) -> anyhow::Result<()> {
        Ok(self.inner.publish(ch, data).await?)
    }
}

pub async fn serve(engine: &Engine, wasm: String) -> anyhow::Result<()> {
    let mut store = Store::new(engine, Host::new());
    let component = Component::from_file(engine, wasm)?;

    let mut linker = Linker::new(engine);
    command::add_to_linker(&mut linker)?;
    Messaging::add_to_linker(&mut linker, |t| t)?;

    let instance_pre = linker.instantiate_pre(&component)?;

    let (messaging, _) = Messaging::instantiate_pre(&mut store, &instance_pre).await?;
    let guest = messaging.wasi_messaging_messaging_guest();

    // connect to NATS server
    let host = store.data_mut();
    let client_res = host.connect("demo.nats.io".to_string()).await?;
    let client = host.table.get(&client_res)?;

    // get channels to subscribe to
    // let Ok(gc) = guest.call_configure(&mut store).await? else {
    //     return Err(anyhow::anyhow!("Failed to configure NATS client"));
    // };

    // subscribe to channels
    let mut subscribers = vec![];
    // for ch in &gc.channels {
    for ch in ["a", "b", "c"] {
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
        let _ = guest.call_handler(&mut store, &[msg]).await?;
    }

    Ok(())
}
