//! # RPC Service
//!
//! This module implements a runtime service for `wasi:messaging`
//! (<https://github.com/WebAssembly/wasi-messaging>).

mod host;
mod server;

mod generated {
    #![allow(clippy::trait_duplication_in_bounds)]
    pub use anyhow::Error;

    wasmtime::component::bindgen!({
        world: "rpc",
        path: "../../wit",
        tracing: true,
        async: true,
        trappable_imports: true,
        with: {
            "wasi:rpc/client/error": Error,
        },
    });
}

use anyhow::Result;
use runtime::{Linkable, Runnable};
use wasmtime::component::{InstancePre, Linker};

use crate::{Ctx, Resources};

pub struct Service;

impl Linkable for Service {
    type Ctx = Ctx;

    fn add_to_linker(&self, linker: &mut Linker<Self::Ctx>) -> Result<()> {
        self::host::add_to_linker(linker)?;
        tracing::trace!("added to linker");
        Ok(())
    }
}

impl Runnable for Service {
    type Resources = Resources;

    async fn run(&self, pre: InstancePre<Self::Ctx>, resources: Self::Resources) -> Result<()> {
        server::run(pre, resources).await
    }
}
