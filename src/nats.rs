use std::collections::HashMap;

use bytes::Bytes;
use futures::stream::{self, StreamExt};
use messaging::bindings::messaging_types::{FormatSpec, Message};
use messaging::bindings::Messaging;
use messaging::{self, Client, MessagingClient, MessagingView};
use wasmtime::component::{Component, InstancePre, Linker, Resource};
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
impl MessagingView for Host {
    async fn connect(&mut self, name: String) -> anyhow::Result<Resource<Client>> {
        let resource = if let Some(key) = self.keys.get(&name) {
            // Get an existing connection by key
            // let any = self.table.get_any_mut(*key).unwrap();
            // Resource::try_from_resource_any(any, store).unwrap()
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

pub async fn serve(engine: &Engine, wasm: String) -> anyhow::Result<()> {
    let component = Component::from_file(engine, wasm)?;

    let mut linker = Linker::new(engine);
    command::add_to_linker(&mut linker)?;
    Messaging::add_to_linker(&mut linker, |t| t)?;

    let instance_pre = linker.instantiate_pre(&component)?;

    // Guest channels to subscribe to
    let channels = {
        let mut store = Store::new(engine, Host::new());
        let (messaging, _) = Messaging::instantiate_pre(&mut store, &instance_pre).await?;
        let Ok(gc) = messaging.wasi_messaging_messaging_guest().call_configure(&mut store).await?
        else {
            return Err(anyhow::anyhow!("Failed to configure NATS client"));
        };
        gc.channels
    };

    // connect to NATS
    let client = async_nats::connect("demo.nats.io").await?;

    // subscribe to channels
    let mut subscribers = vec![];
    for ch in &channels {
        let subscriber = client.subscribe(ch.to_owned()).await?;
        subscribers.push(subscriber);
    }

    // process messages until terminated
    let mut messages = stream::select_all(subscribers);
    while let Some(message) = messages.next().await {
        let engine = engine.clone();
        let instance_pre = instance_pre.clone();
        let client = client.clone();
        tokio::spawn(async move { handle_request(engine, instance_pre, client, message).await });
    }

    Ok(())
}

async fn handle_request(
    engine: Engine, instance_pre: InstancePre<Host>, client: async_nats::Client,
    message: async_nats::Message,
) -> anyhow::Result<()> {
    // set up host state
    let mut host = Host::new();
    let client = Client::new(Box::new(MyClient { inner: client }));
    let resource = host.table.push(client)?;
    host.keys.insert("demo.nats.io".to_string(), resource.rep());

    let mut store = Store::new(&engine, host);
    let (messaging, _) = Messaging::instantiate_pre(&mut store, &instance_pre).await?;

    let msg = Message {
        data: message.payload.to_vec(),
        metadata: Some(vec![(String::from("channel"), message.subject.to_string())]),
        format: FormatSpec::Raw,
    };

    let _ = messaging.wasi_messaging_messaging_guest().call_handler(&mut store, &[msg]).await?;

    Ok(())
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
