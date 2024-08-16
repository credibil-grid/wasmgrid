//! # WASI Peer-to-Peer Host

use std::fmt::Debug;
use std::sync::OnceLock;

use anyhow::{anyhow, Context};
use bindings::wasi::p2p::container;
use bindings::wasi::p2p::types::{self, ContainerId, Entry, EntryMetadata, Error, Owner, Token};
use bindings::P2p;
// use bytes::{Buf, BufMut, Bytes, BytesMut};
use bytes::BytesMut;
use futures::TryStreamExt;
use iroh::client::Doc;
use iroh::docs::{AuthorId, DocTicket};
use iroh::node::FsNode;
use wasmtime::component::{Linker, Resource};
use wasmtime_wasi::WasiView;

use crate::runtime::{self, Runtime, State};

/// Wrap generation of wit bindings to simplify exports.
/// See <https://docs.rs/wasmtime/latest/wasmtime/component/macro.bindgen.html>
mod bindings {
    #![allow(clippy::future_not_send)]

    pub use super::{Blob, Document};

    wasmtime::component::bindgen!({
        world: "p2p",
        path: "wit",
        async: true,
        tracing: true,
        trappable_imports: true,
        with: {
            "wasi:p2p/container/container": Document,
            "wasi:p2p/types/entry": Blob,
        }
    });
}

// Handle to the local Iroh node.
static IROH_NODE: OnceLock<FsNode> = OnceLock::new();
/// Port to use for Iroh if none specified in environment variables.
pub const DEFAULT_IROH_PORT: u16 = 11204;

pub struct Document {
    pub author: AuthorId,
    pub ticket: DocTicket,
    pub doc: Doc,
}

impl Debug for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Document")
            .field("author", &self.author.to_string())
            .field("ticket", &self.ticket.to_string())
            .field("doc", &self.doc.id().to_string())
            .finish()
    }
}

/// Implementation of the `wasi:p2p/container` interface.
#[async_trait::async_trait]
impl container::Host for State {
    /// Create a new author.
    async fn create_owner(&mut self) -> wasmtime::Result<Result<Owner, Error>> {
        todo!()
    }

    /// Create a new document.
    async fn create_container(
        &mut self, _owner: Owner,
    ) -> wasmtime::Result<Result<(Resource<Document>, Token), Error>> {
        todo!()
    }

    /// Retrieve a handle to an existing document.
    async fn get_container(
        &mut self, _owner: Owner, _token: Token,
    ) -> wasmtime::Result<Result<Resource<Document>, Error>> {
        todo!()
    }

    /// Delete a document.
    async fn delete_container(
        &mut self, _container: Resource<Document>,
    ) -> wasmtime::Result<Result<(), Error>> {
        todo!()
    }
}

/// Implementation of the `wasi:p2p/container/container` interface.
#[async_trait::async_trait]
impl container::HostContainer for State {
    /// Get the ID of the document as a string.
    async fn id(
        &mut self, _container: Resource<Document>,
    ) -> wasmtime::Result<Result<ContainerId, Error>> {
        todo!()
    }

    /// Write an entry to the document.
    async fn write_entry(
        &mut self, _container: Resource<Document>, _key: String, _data: Resource<Entry>,
    ) -> wasmtime::Result<Result<(), Error>> {
        todo!()
    }

    /// List the keys of the entries in the document.
    async fn list_entries(
        &mut self, _container: Resource<Document>,
    ) -> wasmtime::Result<Result<Vec<String>, Error>> {
        todo!()
    }

    /// Get information about an entry.
    async fn get_entry_metadata(
        &mut self, _container: Resource<Document>, _key: String,
    ) -> wasmtime::Result<Result<EntryMetadata, Error>> {
        todo!()
    }

    /// Read an entry
    async fn read_entry(
        &mut self, _container: Resource<Document>, _key: String,
    ) -> wasmtime::Result<Result<Resource<Entry>, Error>> {
        todo!()
    }

    /// Delete an entry.
    async fn delete_entry(
        &mut self, _container: Resource<Document>, _key: String,
    ) -> wasmtime::Result<Result<(), Error>> {
        todo!()
    }

    /// Delete all entries in the document.
    async fn clear_entries(
        &mut self, _container: Resource<Document>,
    ) -> wasmtime::Result<Result<(), Error>> {
        todo!()
    }

    /// Remove the document from runtime state.
    fn drop(&mut self, container: Resource<Document>) -> wasmtime::Result<()> {
        tracing::trace!("container::HostContainer::drop");
        self.table().delete(container).map_or_else(|e| Err(anyhow!(e)), |_| Ok(()))
    }
}

/// Implementation of the `wasi:p2p/types` interface.
impl types::Host for State {}

/// Implementation of the `wasi:p2p/types/entry` interface.
#[async_trait::async_trait]
impl types::HostEntry for State {
    /// Get the entry identifier within the container.
    async fn key(&mut self, _entry: Resource<Blob>) -> wasmtime::Result<Result<String, Error>> {
        todo!()
    }

    /// Read the data from the entry.
    async fn read(
        &mut self, _entry: Resource<Blob>, _len: u64,
    ) -> wasmtime::Result<Result<Vec<u8>, Error>> {
        todo!()
    }

    /// Check capacity left in the stream to see if it can be written to.
    async fn check_write(
        &mut self, _entry: Resource<Blob>,
    ) -> wasmtime::Result<Result<u64, Error>> {
        todo!()
    }

    /// Write data to the entry.
    async fn write(
        &mut self, _entry: Resource<Blob>, _data: Vec<u8>,
    ) -> wasmtime::Result<Result<(), Error>> {
        todo!()
    }

    /// Remove the entry from runtime state.
    fn drop(&mut self, entry: Resource<Blob>) -> wasmtime::Result<()> {
        tracing::trace!("types::HostEntry::drop");
        self.table().delete(entry).map_or_else(|e| Err(anyhow!(e)), |_| Ok(()))
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct Blob {
    data: BytesMut,
}

/// Capability configuration.
pub struct Capability;

/// Create a new capability.
pub const fn new() -> Capability {
    Capability
}

/// Implentation required by the `wasmgrid` runtime.
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

// Start the Iroh node.
async fn start_node() -> anyhow::Result<()> {
    let data_dir = std::env::var("IROH_DATA_DIR").context("IROH_DATA_DIR not set")?;
    let port = std::env::var("IROH_PORT")
        .unwrap_or_else(|_| DEFAULT_IROH_PORT.to_string())
        .parse::<u16>()
        .context("invalid IROH_PORT")?;

    tracing::info!("starting Iroh node on port {} with data dir {}", port, data_dir);

    // Start an Iroh node.
    #[allow(clippy::large_futures)]
    let node = FsNode::persistent(data_dir)
        .await?
        .bind_port(port)
        .spawn()
        .await
        .context("failed to build and spawn Iroh node")?;
    IROH_NODE.get_or_init(|| node);

    Ok(())
}

// Return a handle to the Iroh node.
#[allow(dead_code)]
fn iroh_node() -> anyhow::Result<&'static FsNode> {
    let Some(node) = IROH_NODE.get() else {
        return Err(anyhow!("Iroh node not started"));
    };
    Ok(node)
}

// Given the short ID of an author, return the full author public key.
#[allow(dead_code)]
async fn find_author(author_short_id: &str) -> anyhow::Result<Option<AuthorId>> {
    let iroh = iroh_node().expect("failed to get Iroh node from once-lock");
    let mut authors = iroh.authors().list().await?;
    while let Some(author) = authors.try_next().await? {
        if author.fmt_short() == author_short_id {
            return Ok(Some(author));
        }
    }
    Ok(None)
}
