//! # RPC Service
//!
//! This module implements a runtime service for `wasi:messaging`
//! (<https://github.com/WebAssembly/wasi-messaging>).

/// Wrap generation of wit bindings to simplify exports.
/// See <https://docs.rs/wasmtime/latest/wasmtime/component/macro.bindgen.html>
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

use anyhow::{Error, Result, anyhow};
use async_nats::{HeaderMap, Message};
use bytes::Bytes;
use futures::stream::StreamExt;
use tracing::Level;
use wasmtime::Store;
use wasmtime::component::{InstancePre, Linker, Resource};
use wasmtime_wasi::ResourceTable;

use self::generated::RpcPre;
use self::generated::wasi::rpc;
use self::generated::wasi::rpc::client::{self, HostError};
use self::generated::wasi::rpc::types;
use crate::Ctx;

type Resources = <Service as runtime::Runnable>::Resources;

pub struct RpcHost<'a> {
    client: &'a async_nats::Client,
    table: &'a mut ResourceTable,
}

impl<'a> RpcHost<'a> {
    pub const fn new(client: &'a async_nats::Client, table: &'a mut ResourceTable) -> Self {
        Self { client, table }
    }
}

pub struct Service;

impl runtime::Linkable for Service {
    type Ctx = Ctx;

    fn add_to_linker(&self, linker: &mut Linker<Self::Ctx>) -> Result<()> {
        add_to_linker(linker, |c: &mut Ctx| RpcHost::new(&c.nats_client, &mut c.table))?;
        tracing::trace!("added to linker");
        Ok(())
    }
}

/// Add all the `wasi-keyvalue` world's interfaces to a [`Linker`].
fn add_to_linker<T: Send>(
    l: &mut Linker<T>, f: impl Fn(&mut T) -> RpcHost<'_> + Send + Sync + Copy + 'static,
) -> Result<()> {
    rpc::client::add_to_linker_get_host(l, f)?;
    rpc::types::add_to_linker_get_host(l, f)
}

impl runtime::Runnable for Service {
    type Resources = async_nats::Client;

    async fn run(&self, pre: InstancePre<Self::Ctx>, resources: Self::Resources) -> Result<()> {
        // bail if server is not required
        let component_type = pre.component().component_type();
        let mut exports = component_type.exports(pre.engine());
        if !exports.any(|e| e.0.starts_with("wasi:rpc")) {
            tracing::debug!("rpc server not required");
            return Ok(());
        }

        let client = resources.clone();

        // get 'server' component's name
        let pre = RpcPre::new(pre.clone())?;
        let mut store = Store::new(pre.engine(), Ctx::new(client.clone()));
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
            let res = resources.clone();

            match handle(pre.clone(), res, request).await {
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
async fn handle(pre: RpcPre<Ctx>, resources: Resources, message: Message) -> Result<Vec<u8>> {
    tokio::spawn(async move {
        // convert subject to endpoint
        let endpoint = message.subject.trim_start_matches("rpc:").replace('.', "/");

        // forward request to 'server' component
        tracing::span!(Level::INFO, "forwarding request", endpoint = %endpoint).in_scope(|| {
            tracing::info!("forwarding request to {endpoint}");
        });

        let mut store = Store::new(pre.engine(), Ctx::new(resources));
        store.limiter(|t| &mut t.limits);

        let rpc = pre.instantiate_async(&mut store).await?;
        rpc.wasi_rpc_server()
            .call_handle(&mut store, &endpoint, &message.payload.to_vec())
            .await?
            .map_err(|e| anyhow!(e))
    })
    .await?
}

impl types::Host for RpcHost<'_> {}

impl client::Host for RpcHost<'_> {
    #[allow(clippy::cognitive_complexity)]
    async fn call(
        &mut self, endpoint: String, request: Vec<u8>,
    ) -> wasmtime::Result<Result<Vec<u8>, Resource<Error>>> {
        tracing::span!(Level::INFO, "client::Host::call", endpoint = %endpoint).in_scope(|| {
            tracing::info!("client::Host::call for {}", endpoint);
        });

        // convert endpoint to safe NATS subject
        let subject = format!("rpc:{}", endpoint.replacen('/', ".", 1));

        // forward request to RPC server
        let client = self.client.clone();
        let msg = client.request(subject, request.into()).await?;

        // check RPC server's reponse for error
        if let Some(headers) = &msg.headers
            && let Some(error) = headers.get("Error")
        {
            tracing::error!("client::Host::call Err: {error}");
            return Ok(Err(self.table.push(anyhow!("{error}"))?));
        }

        // simplify the logging output
        tracing::debug!("client::Host::call Ok: {endpoint}");
        tracing::trace!("client::Host::call Ok: {msg:?}");
        Ok(Ok(msg.payload.to_vec()))
    }
}

impl HostError for RpcHost<'_> {
    async fn trace(&mut self, rep: Resource<Error>) -> wasmtime::Result<String> {
        tracing::trace!("HostError::trace");
        let error = self.table.get(&rep)?;
        Ok(error.to_string())
    }

    async fn drop(&mut self, rep: Resource<Error>) -> wasmtime::Result<()> {
        tracing::trace!("HostError::drop");
        self.table.delete(rep)?;
        Ok(())
    }
}
