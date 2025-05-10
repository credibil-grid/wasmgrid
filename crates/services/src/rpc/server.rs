//! # RPC Server
//!
//! This module uses NATS to implement an RPC server.

use anyhow::{Result, anyhow};
use async_nats::{Client, HeaderMap, Message};
use bytes::Bytes;
use futures::stream::StreamExt;
use tracing::Level;
use wasmtime::Store;
use wasmtime::component::InstancePre;

use crate::Ctx;
use crate::rpc::generated::RpcPre;
use crate::rpc::generated::exports::wasi::rpc::server::ServerConfiguration;

// type Resources = <Service as runtime::Runnable>::Resources;

pub async fn run(pre: InstancePre<Ctx>, nats_client: Client) -> Result<()> {
    // bail if server is not required
    let component_type = pre.component().component_type();
    let mut exports = component_type.exports(pre.engine());
    if !exports.any(|e| e.0.starts_with("wasi:rpc")) {
        tracing::debug!("rpc server not required");
        return Ok(());
    }

    let client = nats_client.clone();

    // get 'server' component's name
    let pre = RpcPre::new(pre.clone())?;
    let mut store = Store::new(pre.engine(), Ctx::new(client.clone()));
    let rpc = pre.instantiate_async(&mut store).await?;
    let cfg = rpc.wasi_rpc_server().call_configure(&mut store).await??;

    // subscribe to rpc requests for 'server' endpoints
    listen(cfg, client, pre).await
}

async fn listen(cfg: ServerConfiguration, client: Client, pre: RpcPre<Ctx>) -> Result<()> {
    // subscribe to rpc requests for 'server' endpoints
    tracing::debug!("subscribing to rpc requests on rpc:{}.>", cfg.identifier);
    let mut subscriber = client.subscribe(format!("rpc:{}.>", cfg.identifier)).await?;

    // process requests
    while let Some(request) = subscriber.next().await {
        // ensure we have a reply subject
        let Some(subject) = request.clone().reply else {
            return Err(anyhow!("reply subject not found"));
        };

        match handle(pre.clone(), client.clone(), request).await {
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

// Forward request to the wasm Guest.
async fn handle(pre: RpcPre<Ctx>, client: Client, message: Message) -> Result<Vec<u8>> {
    tokio::spawn(async move {
        // convert subject to endpoint
        let endpoint = message.subject.trim_start_matches("rpc:").replace('.', "/");

        // forward request to 'server' component
        tracing::span!(Level::INFO, "forwarding request", endpoint = %endpoint).in_scope(|| {
            tracing::info!("forwarding request to {endpoint}");
        });

        let mut store = Store::new(pre.engine(), Ctx::new(client));
        store.limiter(|t| &mut t.limits);

        let rpc = pre.instantiate_async(&mut store).await?;
        rpc.wasi_rpc_server()
            .call_handle(&mut store, &endpoint, &message.payload.to_vec())
            .await?
            .map_err(|e| anyhow!(e))
    })
    .await?
}
