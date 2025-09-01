//! # WASI Messaging NATS
//!
//! This module implements a runtime service for `wasi:messaging`
//! (<https://github.com/WebAssembly/wasi-messaging>).

mod host;
mod server;

mod generated {
    #![allow(clippy::trait_duplication_in_bounds)]

    pub use async_nats::{Client, Message};
    pub use wasi::messaging::types::Error;

    pub use super::RequestOptions;

    wasmtime::component::bindgen!({
        world: "messaging",
        path: "../../wit",
        imports: {
            default: async | tracing | trappable,
        },
        exports: {
            default: async | tracing | trappable,
        },
        with: {
            "wasi:messaging/request-reply/request-options": RequestOptions,
            "wasi:messaging/types/client": Client,
            "wasi:messaging/types/message": Message,
        },
        trappable_error_type: {
            "wasi:messaging/types/error" => Error,
        },
    });
}

use std::sync::OnceLock;

use anyhow::{Result, anyhow};
use runtime::{AddResource, Run, RunState, ServiceBuilder};
use wasmtime::component::{HasData, InstancePre, Linker};
use wasmtime_wasi::ResourceTable;

use self::generated::wasi::messaging::{producer, request_reply, types};

static NATS_CLIENT: OnceLock<async_nats::Client> = OnceLock::new();

#[derive(Debug)]
pub struct Service;

impl ServiceBuilder for Service {
    fn new() -> Self {
        Self
    }

    fn add_to_linker(self, l: &mut Linker<RunState>) -> anyhow::Result<Self> {
        // host::add_to_linker::<_, Data>(l, Messaging::new)?;
        producer::add_to_linker::<_, Data>(l, Messaging::new)?;
        request_reply::add_to_linker::<_, Data>(l, Messaging::new)?;
        types::add_to_linker::<_, Data>(l, Messaging::new)?;
        Ok(self)
    }
}

impl AddResource<async_nats::Client> for Service {
    fn resource(self, resource: async_nats::Client) -> anyhow::Result<Self> {
        NATS_CLIENT.set(resource).map_err(|_| anyhow!("client already set"))?;
        Ok(self)
    }
}

impl Run for Service {
    async fn run(&self, pre: InstancePre<RunState>) -> Result<()> {
        server::run(pre).await
    }
}

pub struct Messaging<'a> {
    table: &'a mut ResourceTable,
}

impl Messaging<'_> {
    const fn new(c: &mut RunState) -> Messaging<'_> {
        Messaging { table: &mut c.table }
    }
}

fn nats() -> anyhow::Result<&'static async_nats::Client> {
    NATS_CLIENT.get().ok_or_else(|| anyhow!("NATS client not initialized."))
}

struct Data;
impl HasData for Data {
    type Data<'a> = Messaging<'a>;
}

#[derive(Default)]
pub struct RequestOptions {
    pub timeout: Option<std::time::Duration>,
}
