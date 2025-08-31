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

use anyhow::Result;
use resources::Resources;
use runtime::{Interface, RunState, Instantiator};
use wasmtime::component::{InstancePre, Linker};

#[derive(Debug)]
pub struct Service;

impl Interface for Service {
    type State = RunState;

    fn add_to_linker(&self, linker: &mut Linker<Self::State>) -> Result<()> {
        host::add_to_linker(linker)?;
        tracing::trace!("added to linker");
        Ok(())
    }
}

impl Instantiator for Service {
    type Resources = Resources;

    async fn run(&self, pre: InstancePre<Self::State>, resources: Self::Resources) -> Result<()> {
        server::run(pre, resources).await
    }
}

#[derive(Default)]
pub struct RequestOptions {
    pub timeout: Option<std::time::Duration>,
}
