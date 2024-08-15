//! # RPC Capability
//!
//! This module implements a runtime capability for `wasi:messaging`
//! (<https://github.com/WebAssembly/wasi-messaging>).

use std::sync::{Arc, OnceLock};

use anyhow::anyhow;
use async_nats::{AuthError, ConnectOptions, HeaderMap, Message};
use bindings::wasi::rpc::client::{self, HostError};
use bindings::wasi::rpc::types;
use bindings::{Rpc, RpcPre};
use bytes::Bytes;
use futures::stream::StreamExt;
use wasmtime::component::{Linker, Resource};
use wasmtime::Store;
use wasmtime_wasi::WasiView;
use tracing::Level;

use crate::runtime::{self, Runtime, State};

// TODO: create a client struct with both NATS client and request timeout
static CLIENT: OnceLock<async_nats::Client> = OnceLock::new();

/// Wrap generation of wit bindings to simplify exports.
/// See <https://docs.rs/wasmtime/latest/wasmtime/component/macro.bindgen.html>
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
        // additional_derives: [
        //     Hash,
        //     serde::Deserialize,
        //     serde::Serialize,
        // ],
    });
}

pub type Error = anyhow::Error;

#[derive(Debug)]
pub struct Capability {
    addr: String,
    creds: Option<crate::NatsCreds>,
}

pub const fn new(addr: String, creds: Option<crate::NatsCreds>) -> Capability {
    Capability { addr, creds }
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
        // build connection options
        let opts = if let Some(creds) = &self.creds {
            let key_pair = Arc::new(nkeys::KeyPair::from_seed(&creds.seed)?);
            ConnectOptions::with_jwt(creds.jwt.clone(), move |nonce| {
                let key_pair = key_pair.clone();
                async move { key_pair.sign(&nonce).map_err(AuthError::new) }
            })
            .name("wasmgrid")
        } else {
            ConnectOptions::new()
        };

        // connect
        let client = opts.connect(&self.addr).await?;
        tracing::info!("connected to: {}", &self.addr);
        CLIENT.set(client.clone()).map_err(|_| anyhow!("CLIENT already initialized"))?;

        // check to see if server is required
        if !runtime
            .instance_pre()
            .component()
            .component_type()
            .exports(runtime.instance_pre().engine())
            .any(|e| e.0.starts_with(self.namespace()))
        {
            tracing::warn!("rpc server not required");
            return Ok(());
        }

        // get 'server' component's name
        let pre = RpcPre::new(runtime.instance_pre().clone())?;
        let mut store = Store::new(pre.engine(), State::new());
        let rpc = pre.instantiate_async(&mut store).await?;
        let cfg = rpc.wasi_rpc_server().call_configure(&mut store).await??;

        // subscribe to rpc requests for 'server' endpoints
        tracing::debug!("subscribing to rpc requests on rpc:{}.>", cfg.identifier);
        let mut requests = client.subscribe(format!("rpc:{}.>", cfg.identifier)).await?;

        // process requests
        while let Some(request) = requests.next().await {
            // ensure we have a reply subject
            let Some(subject) = request.clone().reply else {
                return Err(anyhow!("reply subject not found"));
            };

            match handle_request(pre.clone(), request).await {
                Ok(resp) => client.publish(subject, resp.into()).await?,
                Err(e) => {
                    tracing::error!("rpc server error: {e:?}");

                    // forward RPC server error to Guest where it will be processed
                    // in the `client::Host::call` method (below)
                    let mut headers = HeaderMap::new();
                    headers.insert("Error", &*format!("rpc server error: {e:?}"));
                    client.publish_with_headers(subject, headers, Bytes::new()).await?;
                }
            }
        }

        Ok(())
    }
}

// Forward request to the wasm Guest.
async fn handle_request(pre: RpcPre<State>, request: Message) -> anyhow::Result<Vec<u8>> {
    tokio::spawn(async move {
        // convert subject to endpoint
        let endpoint = request.subject.trim_start_matches("rpc:").replace('.', "/");

        // forward request to 'server' component
        tracing::span!(Level::INFO, "forwarding request to {endpoint}");

        let mut store = Store::new(pre.engine(), State::new());
        store.limiter(|t| &mut t.limits);

        let rpc = pre.instantiate_async(&mut store).await?;
        rpc.wasi_rpc_server()
            .call_handle(&mut store, &endpoint, &request.payload.to_vec())
            .await?
            .map_err(|e| anyhow!(e))
    })
    .await?
}

impl types::Host for State {}

#[async_trait::async_trait]
impl client::Host for State {
    async fn call(
        &mut self, endpoint: String, request: Vec<u8>,
    ) -> wasmtime::Result<Result<Vec<u8>, Resource<Error>>> {
        tracing::span!(Level::INFO, "client::Host::call", endpoint = %endpoint).in_scope(|| {
            tracing::info!("client::Host::call for {}", endpoint);
        });


        // convert endpoint to safe NATS subject
        let subject = format!("rpc:{}", endpoint.replacen('/', ".", 1));

        // forward request to RPC server
        let client = CLIENT.get().ok_or_else(|| anyhow!("CLIENT not initialized"))?;
        let msg = client.request(subject, request.into()).await?;

        // check RPC server's reponse for error
        if let Some(headers) = &msg.headers
            && let Some(error) = headers.get("Error")
        {
            tracing::error!("client::Host::call Err: {error}");
            return Ok(Err(self.table().push(anyhow!("{error}"))?));
        }
       
       // simplify the logging output
        tracing::debug!("client::Host::call Ok: {endpoint}");
        tracing::trace!("client::Host::call Ok: {msg:?}");
        Ok(Ok(msg.payload.to_vec()))
    }
}

#[async_trait::async_trait]
impl HostError for State {
    async fn trace(&mut self, rep: Resource<Error>) -> wasmtime::Result<String> {
        tracing::trace!("HostError::trace");
        let error = self.table().get(&rep)?;
        Ok(error.to_string())
    }

    fn drop(&mut self, rep: Resource<Error>) -> wasmtime::Result<()> {
        tracing::trace!("HostError::drop");
        self.table().delete(rep)?;
        Ok(())
    }
}
