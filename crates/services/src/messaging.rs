//! # WASI Messaging Service
//!
//! This module implements a runtime service for `wasi:messaging`
//! (<https://github.com/WebAssembly/wasi-messaging>).

mod host;
mod server;

mod generated {
    #![allow(clippy::trait_duplication_in_bounds)]

    pub use anyhow::Error;
    pub use async_nats::Client;

    wasmtime::component::bindgen!({
        world: "messaging",
        path: "../../wit",
        tracing: true,
        async: true,
        trappable_imports: true,
        with: {
            "wasi:messaging/messaging-types/client": Client,
            "wasi:messaging/messaging-types/error": Error,
        },
        // trappable_error_type: {
        //     "wasi:messaging/messaging-types/error" => Error,
        // },
    });
}

use anyhow::Result;
use async_nats::Client;
use runtime::{Linkable, Runnable};
use wasmtime::component::{InstancePre, Linker};

use crate::Ctx;

pub struct Service;

impl Linkable for Service {
    type Ctx = Ctx;

    fn add_to_linker(&self, linker: &mut Linker<Self::Ctx>) -> Result<()> {
        host::add_to_linker(linker)?;
        tracing::trace!("added to linker");
        Ok(())
    }
}

impl Runnable for Service {
    type Resources = Client;

    async fn run(&self, pre: InstancePre<Self::Ctx>, resources: Self::Resources) -> Result<()> {
        server::run(pre, resources).await
    }
}
