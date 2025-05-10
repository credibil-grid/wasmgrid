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

pub async fn run(pre: InstancePre<Ctx>, client: Client) -> Result<()> {
    // bail if server is not required
    let component_type = pre.component().component_type();
    let mut exports = component_type.exports(pre.engine());
    if !exports.any(|e| e.0.starts_with("wasi:rpc")) {
        tracing::debug!("rpc server not required");
        return Ok(());
    }

    let client = client.clone();

    // get 'server' component's name
    let rpc_pre = RpcPre::new(pre.clone())?;
    let mut store = Store::new(pre.engine(), Ctx::new(client.clone(), pre.clone()));
    let rpc = rpc_pre.instantiate_async(&mut store).await?;
    let sc = rpc.wasi_rpc_server().call_configure(&mut store).await??;

    // process_requests rpc requests
    subscribe(sc, client, rpc_pre).await
}

async fn subscribe(sc: ServerConfiguration, client: Client, pre: RpcPre<Ctx>) -> Result<()> {
    // subscribe to rpc requests for 'server' endpoints
    tracing::debug!("subscribing to rpc requests on rpc:{}.>", sc.identifier);
    let mut subscriber = client.subscribe(format!("rpc:{}.>", sc.identifier)).await?;

    // process requests
    while let Some(msg) = subscriber.next().await {
        // ensure we have a reply subject
        let Some(subject) = msg.clone().reply else {
            return Err(anyhow!("reply subject not found"));
        };

        let pre = pre.clone();
        let cli = client.clone();
        tokio::spawn(async move {
            match call_guest(pre, cli.clone(), msg).await {
                Ok(resp) => cli.publish(subject, resp.into()).await,
                Err(e) => {
                    tracing::error!("rpc server error: {e:?}");
                    let mut headers = HeaderMap::new();
                    headers.insert("Error", format!("rpc server error: {e}"));
                    cli.publish_with_headers(subject, headers, Bytes::new()).await
                }
            }
        });
    }

    Ok(())
}

// Forward request to the wasm Guest.
async fn call_guest(pre: RpcPre<Ctx>, client: Client, message: Message) -> Result<Vec<u8>> {
    // convert subject to endpoint
    let endpoint = message.subject.trim_start_matches("rpc:").replace('.', "/");

    // forward request to 'server' component
    tracing::span!(Level::INFO, "forwarding request", endpoint = %endpoint).in_scope(|| {
        tracing::info!("forwarding request to {endpoint}");
    });

    let mut store = Store::new(pre.engine(), Ctx::new(client, pre.instance_pre().clone()));
    store.limiter(|t| &mut t.limits);

    let rpc = pre.instantiate_async(&mut store).await?;
    rpc.wasi_rpc_server()
        .call_handle(&mut store, &endpoint, &message.payload.to_vec())
        .await?
        .map_err(|e| anyhow!(e))
}
