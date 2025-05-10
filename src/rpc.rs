//! # RPC Service
//!
//! This module implements a runtime service for `wasi:messaging`
//! (<https://github.com/WebAssembly/wasi-messaging>).

mod host;
mod server;

mod generated {
    #![allow(clippy::future_not_send)]
    #![allow(clippy::trait_duplication_in_bounds)]
    pub use anyhow::Error;

    wasmtime::component::bindgen!({
        world: "rpc",
        path: "wit",
        tracing: true,
        async: true,
        trappable_imports: true,
        with: {
            "wasi:rpc/client/error": Error,
        },
    });
}

use anyhow::Result;
use wasmtime::component::{InstancePre, Linker};

use self::host::RpcHost;
use crate::Ctx;

pub struct Service;

impl runtime::Linkable for Service {
    type Ctx = Ctx;

    fn add_to_linker(&self, linker: &mut Linker<Self::Ctx>) -> Result<()> {
        self::host::add_to_linker(linker, |c: &mut Self::Ctx| {
            RpcHost::new(&c.nats_client, &mut c.table)
        })?;
        tracing::trace!("added to linker");
        Ok(())
    }
}

impl runtime::Runnable for Service {
    type Resources = async_nats::Client;

    async fn run(&self, pre: InstancePre<Self::Ctx>, resources: Self::Resources) -> Result<()> {
        server::run(pre, resources).await
    }
}
