//! # RPC Capability
//!
//! This module implements a runtime capability for `wasi:messaging`
//! (<https://github.com/WebAssembly/wasi-messaging>).

use std::sync::OnceLock;
use std::time::Duration;

use anyhow::anyhow;
use bindings::wasi::rpc::client::{self, HostError};
use bindings::wasi::rpc::types;
use bindings::Rpc;
use futures::stream::StreamExt;
use wasmtime::component::{Linker, Resource};
use wasmtime_wasi::WasiView;

use crate::runtime::{self, Runtime, State};

// TODO: tidy up by creating a client struct with both NATS client and request timeout
static CLIENT: OnceLock<async_nats::Client> = OnceLock::new();
static TIMEOUT: OnceLock<Duration> = OnceLock::new();

/// Wrap generation of wit bindings to simplify exports
mod bindings {
    #![allow(clippy::future_not_send)]

    pub use super::Error;

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

pub type Error = anyhow::Error;

pub struct Capability {
    addr: String,
    timeout: Duration,
}

pub const fn new(addr: String, timeout_s: u64) -> Capability {
    let timeout = Duration::from_secs(timeout_s);
    Capability { addr, timeout }
}

#[async_trait::async_trait]
impl runtime::Capability for Capability {
    fn namespace(&self) -> &str {
        "wasi:rpc"
    }

    fn add_to_linker(&self, linker: &mut Linker<State>) -> anyhow::Result<()> {
        Rpc::add_to_linker(linker, |t| t)
    }

    async fn run(&self, runtime: Runtime) -> anyhow::Result<()> {
        let client = async_nats::connect(&self.addr).await?;
        CLIENT.set(client.clone()).map_err(|_| anyhow!("CLIENT already initialized"))?;
        TIMEOUT.set(self.timeout).map_err(|_| anyhow!("TIMEOUT already initialized"))?;

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
        let (rpc, _) = Rpc::instantiate_pre(&mut store, runtime.instance_pre()).await?;
        let cfg = rpc.wasi_rpc_server().call_configure(&mut store).await??;

        // subscribe to rpc requests for 'server' endpoints
        tracing::debug!("subscribing to rpc requests on rpc:{}.>", cfg.identifier);
        let mut requests = client.subscribe(format!("rpc:{}.>", cfg.identifier)).await?;

        // process requests
        while let Some(request) = requests.next().await {
            let runtime = runtime.clone();
            let client = client.clone();

            if let Err(e) = tokio::spawn(async move {
                let Some(reply) = request.clone().reply else {
                    return Err(anyhow!("reply subject not found"));
                };

                // convert subject to endpoint
                let endpoint = request.subject.trim_start_matches("rpc:");
                let endpoint = endpoint.replace('.', "/");

                // forward request to 'server' component
                tracing::debug!("forwarding request to {endpoint}");
                let mut store = runtime.new_store();
                let (rpc, _) = Rpc::instantiate_pre(&mut store, runtime.instance_pre()).await?;

                let resp = rpc
                    .wasi_rpc_server()
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
        let subject = format!("rpc:{}", endpoint.replacen('/', ".", 1));

        let client = CLIENT.get().ok_or_else(|| anyhow!("CLIENT not initialized"))?;
        let timeout = TIMEOUT.get().ok_or_else(|| anyhow!("TIMEOUT not initialized"))?;
        let nats_request = async_nats::client::Request::new()
            .payload(request.into())
            .timeout(Some(timeout.clone()));
        let msg = client.send_request(subject, nats_request).await?;
        //let msg = client.request(subject, request.into()).await?;

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
