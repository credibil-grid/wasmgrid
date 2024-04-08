use anyhow::{anyhow, bail, Context};
use clap::Parser;
use wasmtime::component::{bindgen, Component, Linker, Resource};
use wasmtime::{AsContextMut, Config, Engine, Store, StoreLimits};
use wasmtime_wasi::{command, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
// use bindings::exports::wasi::messaging::messaging_guest::Guest;

// use crate::guest::types::{Location, RequestAdd};

bindgen!({ path: "../wit", world: "messaging", async: true });

use wasi::messaging::messaging_types::{Client, Error, FormatSpec, GuestConfiguration, Message};
use wasi::messaging::{consumer, messaging_types, producer};

struct State {
    table: ResourceTable,
    ctx: WasiCtx,
    limits: StoreLimits,
}

impl messaging_types::Host for State {}

#[async_trait::async_trait]
impl messaging_types::HostClient for State {
    async fn connect(
        &mut self, ch: String,
    ) -> wasmtime::Result<anyhow::Result<Resource<Client>, Resource<Error>>> {
        println!("connect");
        todo!()
    }

    fn drop(&mut self, err: Resource<Client>) -> wasmtime::Result<()> {
        todo!()
    }
}

#[async_trait::async_trait]
impl messaging_types::HostError for State {
    async fn trace(&mut self) -> wasmtime::Result<String> {
        println!("trace");
        todo!()
    }

    fn drop(&mut self, err: Resource<Error>) -> wasmtime::Result<()> {
        println!("drop");
        todo!()
    }
}

#[async_trait::async_trait]
impl producer::Host for State {
    async fn send(
        &mut self, client: Resource<Client>, ch: String, msg: Vec<Message>,
    ) -> wasmtime::Result<anyhow::Result<(), Resource<Error>>> {
        println!("send");
        todo!();
    }
}

#[async_trait::async_trait]
impl consumer::Host for State {
    async fn subscribe_try_receive(
        &mut self, client: Resource<Client>, ch: String, t_milliseconds: u32,
    ) -> wasmtime::Result<anyhow::Result<Option<Vec<Message>>, Resource<Error>>> {
        println!("subscribe_try_receive");
        todo!();
    }

    async fn subscribe_receive(
        &mut self, client: Resource<Client>, ch: String,
    ) -> wasmtime::Result<anyhow::Result<Vec<Message>, Resource<Error>>> {
        println!("subscribe_receive");
        todo!();
    }

    async fn update_guest_configuration(
        &mut self, gc: GuestConfiguration,
    ) -> wasmtime::Result<anyhow::Result<(), Resource<Error>>> {
        println!("update_guest_configuration");
        todo!();
    }

    async fn complete_message(
        &mut self, msg: Message,
    ) -> wasmtime::Result<anyhow::Result<(), Resource<Error>>> {
        println!("complete_message");
        todo!();
    }

    async fn abandon_message(
        &mut self, msg: Message,
    ) -> wasmtime::Result<anyhow::Result<(), Resource<Error>>> {
        println!("abandon_message");
        todo!();
    }
}

impl WasiView for State {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

/// Host wasm runtime for a vault service that stores signing keys and credentials for a Verifiable
/// Credential wallet.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to the wasm file to run.
    wasm_file: String,
}

#[tokio::main]
pub async fn main() -> wasmtime::Result<()> {
    // let args = Args::parse();
    let file = "target/wasm32-wasi/release/guest.wasm";

    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = Engine::new(&config)?;

    let component = Component::from_file(&engine, file)?;
    let mut store = Store::new(
        &engine,
        State {
            table: ResourceTable::new(),
            ctx: WasiCtxBuilder::new().inherit_env().build(),
            limits: StoreLimits::default(),
        },
    );

    let mut linker: Linker<State> = Linker::new(&engine);
    command::add_to_linker(&mut linker).context("failed to add wasi to linker")?;
    messaging_types::add_to_linker(&mut linker, |t| t)?;
    producer::add_to_linker(&mut linker, |t| t)?;
    consumer::add_to_linker(&mut linker, |t| t)?;

    let (messaging, _) = Messaging::instantiate_async(&mut store, &component, &linker).await?;

    let cfg =
        messaging.wasi_messaging_messaging_guest().call_configure(store.as_context_mut()).await;
    println!("{:?}", cfg);

    let msg = Message {
        data: b"test".to_vec(),
        metadata: None,
        format: FormatSpec::Raw,
    };

    let result = messaging
        .wasi_messaging_messaging_guest()
        .call_handler(store.as_context_mut(), &[msg])
        .await?;
    println!("{:?}", result);

    Ok(())
}
