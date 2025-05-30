//! # RPC Host

use anyhow::{Error, Result, anyhow};
use tracing::Level;
use wasmtime::component::{Linker, Resource};
use wasmtime_wasi::ResourceTable;

use crate::rpc_nats::generated::wasi::rpc::client::HostError;
use crate::rpc_nats::generated::wasi::rpc::{self, client, types};
use crate::{Ctx, Resources};

pub struct RpcHost<'a> {
    resources: &'a Resources,
    table: &'a mut ResourceTable,
}

impl RpcHost<'_> {
    const fn new(c: &mut Ctx) -> RpcHost<'_> {
        RpcHost {
            resources: &c.resources,
            table: &mut c.table,
        }
    }
}

/// Add all the `rpc` world's interfaces to a [`Linker`].
pub fn add_to_linker(l: &mut Linker<Ctx>) -> Result<()> {
    rpc::client::add_to_linker_get_host(l, RpcHost::new)?;
    rpc::types::add_to_linker_get_host(l, RpcHost::new)
}

impl types::Host for RpcHost<'_> {}

impl client::Host for RpcHost<'_> {
    async fn call(
        &mut self, endpoint: String, request: Vec<u8>,
    ) -> wasmtime::Result<Result<Vec<u8>, Resource<Error>>> {
        tracing::span!(Level::INFO, "client::Host::call", endpoint = %endpoint).in_scope(|| {
            tracing::info!("client::Host::call for {}", endpoint);
        });

        // convert endpoint to safe NATS subject
        let subject = format!("rpc:{}", endpoint.replacen('/', ".", 1));

        // forward request to RPC server
        let msg = self.resources.nats()?.request(subject, request.into()).await?;

        // check RPC server's reponse for error
        if let Some(headers) = &msg.headers
            && let Some(error) = headers.get("Error")
        {
            tracing::error!("client::Host::call Err: {error}");
            return Ok(Err(self.table.push(anyhow!("{error}"))?));
        }

        // simplify the logging output
        tracing::debug!("client::Host::call Ok: {endpoint}");
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
