//! # WASI Messaging Service
//!
//! This module implements a runtime service for `wasi:messaging`
//! (<https://github.com/WebAssembly/wasi-messaging>).

mod host;
mod server;

mod generated {
    #![allow(clippy::trait_duplication_in_bounds)]

    pub use wasi::messaging::types::Error;
    pub use async_nats::{Client, Message};

    wasmtime::component::bindgen!({
        world: "messaging",
        path: "../../wit",
        tracing: true,
        async: true,
        trappable_imports: true,
        with: {
            "wasi:messaging/types/client": Client,
            "wasi:messaging/types/message": Message,
        },
        trappable_error_type: {
            "wasi:messaging/types/error" => Error,
        },
    });
}

use anyhow::Result;
use runtime::{Linkable, Runnable};
use wasmtime::component::{InstancePre, Linker};

use crate::{Ctx, Resources};

pub struct Service;

impl std::fmt::Debug for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("messaging").finish()
    }
}

impl Linkable for Service {
    type Ctx = Ctx;

    fn add_to_linker(&self, linker: &mut Linker<Self::Ctx>) -> Result<()> {
        host::add_to_linker(linker)?;
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
