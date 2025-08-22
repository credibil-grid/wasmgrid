//! # WASI Messaging NATS

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

use anyhow::Result;
use resources::Resources;
use runtime::{Linkable, Runnable};
use wasi_core::Ctx;
use wasmtime::component::{InstancePre, Linker};

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

#[derive(Default)]
pub struct RequestOptions {
    pub timeout: Option<std::time::Duration>,
}
