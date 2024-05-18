//! # WASI Messaging Capability
//!
//! This module implements a runtime capability for `wasi:messaging`
//! (<https://github.com/WebAssembly/wasi-messaging>).

use std::sync::OnceLock;

use anyhow::anyhow;
use bindings::wasi::wrpc::client;
use bindings::wasi::wrpc::types::{self, HostError};
use bindings::Wrpc;
use bytes::Bytes;
use futures::stream::StreamExt;
use wasmtime::component::{Linker, Resource};
use wasmtime_wasi::WasiView;

use crate::runtime::{self, Runtime, State};

/// Wrap generation of wit bindings to simplify exports
mod bindings {
    #![allow(clippy::future_not_send)]

    pub use super::Error;

    wasmtime::component::bindgen!({
        world: "wrpc",
        path: "wit",
        tracing: true,
        async: true,
        with: {
            "wasi:wrpc/types/error": Error,
        },
    });
}

// pub type Client = async_nats::Client;
pub type Error = anyhow::Error;

static CLIENT: OnceLock<async_nats::Client> = OnceLock::new();

pub struct Capability {
    addr: String,
    server: String,
}

pub fn new(addr: String, server: impl Into<String>) -> Capability {
    Capability {
        addr,
        server: server.into(),
    }
}

#[async_trait::async_trait]
impl runtime::Capability for Capability {
    fn namespace(&self) -> &str {
        "wasi:wrpc"
    }

    fn add_to_linker(&self, linker: &mut Linker<State>) -> anyhow::Result<()> {
        Wrpc::add_to_linker(linker, |t| t)
    }

    async fn run(&self, runtime: Runtime) -> anyhow::Result<()> {
        let client = async_nats::connect(&self.addr).await?;
        CLIENT.set(client.clone()).map_err(|_| anyhow!("CLIENT already initialized"))?;

        // subscribe to wrpc requests for 'server'
        let mut requests = client.subscribe(format!("wrpc:{}", self.server)).await?;

        // process requests
        while let Some(request) = requests.next().await {
            let runtime = runtime.clone();

            if let Err(e) = tokio::spawn(async move {
                let Some(reply) = request.clone().reply else {
                    return Err(anyhow!("reply subject not found"));
                };

                // call 'server' component
                let mut store = runtime.new_store();
                let (wrpc, _) = Wrpc::instantiate_pre(&mut store, runtime.instance_pre()).await?;

                let resp = match wrpc
                    .wasi_wrpc_server()
                    .call_handle(&mut store, &request.payload)
                    .await?
                {
                    Ok(resp) => resp,
                    Err(e) => {
                        let error = store.data_mut().table().get(&e)?;
                        return Err(anyhow!(error.to_string()));
                    }
                };

                // publish response as reply to 'client' request
                let client = CLIENT.get().ok_or_else(|| anyhow!("CLIENT not initialized"))?;
                let data = Bytes::from(resp);
                client.publish(reply, data).await?;

                Ok(())
            })
            .await
            {
                tracing::error!("error processing request {e:?}");
            }
        }

        Ok(())
    }
}

impl types::Host for State {}

#[async_trait::async_trait]
impl client::Host for State {
    async fn call(
        &mut self, server: String, request: Vec<u8>,
    ) -> wasmtime::Result<Result<Vec<u8>, Resource<Error>>> {
        tracing::debug!("client::Host::call");

        let client = CLIENT.get().ok_or_else(|| anyhow!("CLIENT not initialized"))?;
        let data = Bytes::from(request);
        let msg = client.request(format!("wrpc:{server}"), data).await?;

        Ok(Ok(msg.payload.to_vec()))
    }
}

#[async_trait::async_trait]
impl HostError for State {
    async fn trace(&mut self, rep: Resource<Error>) -> wasmtime::Result<String> {
        tracing::debug!("HostError::trace");
        let error = self.table().get(&rep)?;
        Ok(error.to_string())
    }

    fn drop(&mut self, rep: Resource<Error>) -> wasmtime::Result<()> {
        tracing::debug!("HostError::drop");
        self.table().delete(rep)?;
        Ok(())
    }
}
