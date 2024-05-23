//! # WASI Peer-to-Peer Host
pub mod document;
pub mod types;

use std::sync::OnceLock;

use anyhow::{anyhow, Context};
use bindings::P2p;
use futures::TryStreamExt;
use iroh::docs::AuthorId;
use iroh::node::FsNode;
use wasmtime::component::Linker;

use crate::runtime::{self, Runtime, State};

pub mod bindings {
    #![allow(clippy::future_not_send)]

    pub use super::document::{Document, StreamObjectNames};
    pub use super::types::BlobValue;

    wasmtime::component::bindgen!({
        world: "p2p",
        path: "wit",
        async: true,
        tracing: true,
        trappable_imports: true,
        with: {
            "wasi:io": wasmtime_wasi::bindings::io,
            "wasi:blobstore/container/container": Document,
            "wasi:blobstore/container/stream-object-names": StreamObjectNames,
            "wasi:blobstore/types/incoming-value": BlobValue,
            "wasi:blobstore/types/outgoing-value": BlobValue,
        }
    });
}

static IROH_NODE: OnceLock<FsNode> = OnceLock::new();
pub const DEFAULT_IROH_PORT: u16 = 11204;

pub struct Capability;

pub const fn new() -> Capability {
    Capability
}

#[async_trait::async_trait]
impl runtime::Capability for Capability {
    fn namespace(&self) -> &str {
        "wasi:p2p"
    }

    fn add_to_linker(&self, linker: &mut Linker<State>) -> anyhow::Result<()> {
        P2p::add_to_linker(linker, |t| t)
    }

    #[allow(clippy::large_futures)]
    async fn run(&self, _runtime: Runtime) -> anyhow::Result<()> {
        start_node().await.context("failed to start Iroh")?;
        Ok(())
    }
}

async fn start_node() -> anyhow::Result<()> {
    let data_dir = std::env::var("IROH_DATA_DIR").context("IROH_DATA_DIR not set")?;
    let port = std::env::var("IROH_PORT")
        .unwrap_or_else(|_| DEFAULT_IROH_PORT.to_string())
        .parse::<u16>()
        .context("invalid IROH_PORT")?;

    tracing::info!("starting Iroh node on port {} with data dir {}", port, data_dir);

    // Start an Iroh node.
    let node = FsNode::persistent(data_dir)
        .await?
        .bind_port(port)
        .spawn()
        .await
        .context("failed to build and spawn Iroh node")?;
    IROH_NODE.get_or_init(|| node);

    Ok(())
}

fn iroh_node() -> anyhow::Result<&'static FsNode> {
    let Some(node) = IROH_NODE.get() else {
        return Err(anyhow!("Iroh node not started"));
    };
    Ok(node)
}

async fn find_author(author_short_id: &str) -> anyhow::Result<Option<AuthorId>> {
    let iroh = iroh_node().expect("failed to get Iroh node from once-lock");
    let mut authors = iroh.authors.list().await?;
    while let Some(author) = authors.try_next().await? {
        if author.fmt_short() == author_short_id {
            return Ok(Some(author));
        }
    }
    Ok(None)
}
