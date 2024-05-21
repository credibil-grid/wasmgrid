//! # WASI Messaging Capability
//!
//! This module implements a runtime capability for `wasi:messaging`
//! (<https://github.com/WebAssembly/wasi-messaging>).

use std::sync::OnceLock;

use anyhow::anyhow;
use bindings::wasi::wrpc::client::{self, HostError};
use bindings::wasi::wrpc::types;
use bindings::Wrpc;
use futures::stream::StreamExt;
use wasmtime::component::{Linker, Resource};
use wasmtime_wasi::WasiView;

use crate::runtime::{self, Runtime, State};

static CLIENT: OnceLock<async_nats::Client> = OnceLock::new();

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
            "wasi:wrpc/client/error": Error,
        },
    });
}

pub type Error = anyhow::Error;

pub struct Capability {
    addr: String,
}

pub const fn new(addr: String) -> Capability {
    Capability { addr }
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

        let mut store = runtime.new_store();

        // check to see if server is required
        let is_server = runtime
            .instance_pre()
            .component()
            .component_type()
            .exports(store.engine())
            .any(|e| e.0.starts_with(self.namespace()));

        if !is_server {
            return Ok(());
        }

        // get 'server' component's name
        let (wrpc, _) = Wrpc::instantiate_pre(&mut store, runtime.instance_pre()).await?;
        let cfg = wrpc.wasi_wrpc_server().call_configure(&mut store).await??;

        // subscribe to wrpc requests for 'server' endpoints
        tracing::debug!("subscribing to wrpc requests on wrpc:{}.>", cfg.identifier);
        let mut requests = client.subscribe(format!("wrpc:{}.>", cfg.identifier)).await?;

        // process requests
        while let Some(request) = requests.next().await {
            let runtime = runtime.clone();
            let client = client.clone();

            if let Err(e) = tokio::spawn(async move {
                let Some(reply) = request.clone().reply else {
                    return Err(anyhow!("reply subject not found"));
                };

                // convert subject to endpoint
                let endpoint = request.subject.trim_start_matches("wrpc:");
                let endpoint = endpoint.replace('.', "/");

                // forward request to 'server' component
                tracing::debug!("forwarding request to {endpoint}");
                let mut store = runtime.new_store();
                let (wrpc, _) = Wrpc::instantiate_pre(&mut store, runtime.instance_pre()).await?;

                let resp = wrpc
                    .wasi_wrpc_server()
                    .call_handle(&mut store, &endpoint, &request.payload.to_vec())
                    .await??;

                // send reply to 'client' component
                client.publish(reply, resp.into()).await?;

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
        &mut self, endpoint: String, request: Vec<u8>,
    ) -> wasmtime::Result<Result<Vec<u8>, Resource<Error>>> {
        tracing::debug!("client::Host::call for {endpoint}");

        // convert endpoint to safe NATS subject
        let subject = format!("wrpc:{}", endpoint.replacen('/', ".", 1));

        let client = CLIENT.get().ok_or_else(|| anyhow!("CLIENT not initialized"))?;
        let msg = client.request(subject, request.into()).await?;

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
