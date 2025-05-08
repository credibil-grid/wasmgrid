//! # WASI Peer-to-Peer Host

/// Wrap generation of wit bindings to simplify exports.
/// See <https://docs.rs/wasmtime/latest/wasmtime/component/macro.bindgen.html>
mod generated {
    #![allow(clippy::future_not_send)]

    pub use super::Document;

    wasmtime::component::bindgen!({
        world: "p2p",
        path: "wit",
        async: true,
        tracing: true,
        trappable_imports: true,
        with: {
            "wasi:p2p/container/container": Document,
        },
        additional_derives: [
            serde::Deserialize,
            serde::Serialize,
        ]
    });
}

use std::fmt::Debug;
use std::str::FromStr;
use std::sync::OnceLock;

use anyhow::{Context, anyhow};
use futures::{StreamExt, TryStreamExt};
use iroh::base::node_addr::AddrInfoOptions;
use iroh::client::Doc;
use iroh::client::docs::{Entry, ShareMode};
use iroh::docs::store::Query;
use iroh::docs::{AuthorId, DocTicket};
use iroh::node::FsNode;
use wasmtime::component::{InstancePre, Linker, Resource};
use wasmtime_wasi::WasiView;

use self::generated::P2p;
use self::generated::wasi::p2p::container;
use self::generated::wasi::p2p::types::{
    self, Author, ContainerId, EntryMetadata, Error, Permission, Token,
};
use crate::runtime::{self, Ctx};

// Handle to the local Iroh node.
static IROH_NODE: OnceLock<FsNode> = OnceLock::new();
/// Port to use for Iroh if none specified in environment variables.
pub const DEFAULT_IROH_PORT: u16 = 11204;

pub struct Document {
    pub doc: Doc,
}

impl Debug for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Document").field("doc", &self.doc.id().to_string()).finish()
    }
}

/// Implementation of the `wasi:p2p/container` interface.
#[async_trait::async_trait]
impl container::Host for Ctx {
    /// Create a new author.
    async fn create_author(&mut self) -> wasmtime::Result<Result<Author, Error>> {
        tracing::trace!("container::Host::create_owner");
        let Ok(iroh) = iroh_node() else {
            return Ok(Err(Error::CapabilityUnavailable));
        };
        let author = match iroh.authors().create().await {
            Ok(author) => author,
            Err(e) => return Ok(Err(Error::CapabilityError(e.to_string()))),
        };
        Ok(Ok(author.fmt_short()))
    }

    /// Create a new document.
    async fn create_container(&mut self) -> wasmtime::Result<Result<Resource<Document>, Error>> {
        tracing::trace!("container::Host::create_container");
        let Ok(iroh) = iroh_node() else {
            return Ok(Err(Error::CapabilityUnavailable));
        };
        let doc = match iroh.docs().create().await {
            Ok(doc) => doc,
            Err(e) => {
                tracing::error!("failed to create document: {}", e);
                return Ok(Err(Error::CapabilityError(e.to_string())));
            }
        };
        let container = Document { doc };
        let stashed = match self.table().push(container) {
            Ok(stashed) => stashed,
            Err(e) => {
                tracing::error!("failed add document to resource table: {}", e);
                return Ok(Err(Error::Other(e.to_string())));
            }
        };
        Ok(Ok(stashed))
    }

    /// Retrieve a handle to an existing document.
    async fn get_container(
        &mut self, token: Token,
    ) -> wasmtime::Result<Result<Resource<Document>, Error>> {
        // Do not log token, it is sensitive information.
        tracing::trace!("container::Host::get_container");
        let Ok(iroh) = iroh_node() else {
            return Ok(Err(Error::CapabilityUnavailable));
        };
        let ticket = match DocTicket::from_str(&token) {
            Ok(ticket) => ticket,
            Err(e) => {
                tracing::error!("failed to parse ticket: {}", e);
                return Ok(Err(Error::CapabilityError(e.to_string())));
            }
        };
        let doc = match iroh.docs().import(ticket).await {
            Ok(doc) => doc,
            Err(e) => {
                tracing::error!("failed to import document: {}", e);
                return Ok(Err(Error::CapabilityError(e.to_string())));
            }
        };
        let container = Document { doc };
        let stashed = match self.table().push(container) {
            Ok(stashed) => stashed,
            Err(e) => {
                tracing::error!("failed add document to resource table: {}", e);
                return Ok(Err(Error::Other(e.to_string())));
            }
        };
        Ok(Ok(stashed))
    }

    /// Delete a document.
    async fn delete_container(
        &mut self, container: Resource<Document>,
    ) -> wasmtime::Result<Result<(), Error>> {
        tracing::trace!("container::Host::delete_container");
        let Ok(iroh) = iroh_node() else {
            return Ok(Err(Error::CapabilityUnavailable));
        };
        let container = match self.table().get_mut(&container) {
            Ok(container) => container,
            Err(e) => {
                tracing::error!("failed to get document from resource table: {}", e);
                return Ok(Err(Error::Other(e.to_string())));
            }
        };
        tracing::trace!("deleting document {}", container.doc.id());
        match container.doc.close().await {
            Ok(()) => (),
            Err(e) => {
                tracing::error!("failed to close document: {}", e);
                return Ok(Err(Error::CapabilityError(e.to_string())));
            }
        };
        match iroh.docs().drop_doc(container.doc.id()).await {
            Ok(()) => (),
            Err(e) => {
                tracing::error!("failed to drop document: {}", e);
                return Ok(Err(Error::CapabilityError(e.to_string())));
            }
        };
        Ok(Ok(()))
    }
}

/// Implementation of the `wasi:p2p/container/container` interface.
#[async_trait::async_trait]
impl container::HostContainer for Ctx {
    /// Get the ID of the document as a string.
    async fn id(
        &mut self, container: Resource<Document>,
    ) -> wasmtime::Result<Result<ContainerId, Error>> {
        tracing::trace!("container::HostContainer::id");
        let container = match self.table().get_mut(&container) {
            Ok(container) => container,
            Err(e) => {
                tracing::error!("failed to get document from resource table: {}", e);
                return Ok(Err(Error::Other(e.to_string())));
            }
        };
        Ok(Ok(container.doc.id().to_string()))
    }

    /// Write an entry to the document.
    async fn write_entry(
        &mut self, container: Resource<Document>, key: String, author: Author, data: Vec<u8>,
    ) -> wasmtime::Result<Result<(), Error>> {
        tracing::trace!("container::HostContainer::write_entry {key} by author {author}");
        let Ok(iroh) = iroh_node() else {
            return Ok(Err(Error::CapabilityUnavailable));
        };
        let container = match self.table().get_mut(&container) {
            Ok(container) => container,
            Err(e) => {
                tracing::error!("failed to get document from resource table: {}", e);
                return Ok(Err(Error::Other(e.to_string())));
            }
        };
        let author = match find_author(iroh, &author).await {
            Ok(Some(author)) => author,
            Ok(None) => {
                tracing::error!("author not found");
                return Ok(Err(Error::Other("author not found".to_string())));
            }
            Err(e) => {
                tracing::error!("failed to find author: {}", e);
                return Ok(Err(Error::CapabilityError(e.to_string())));
            }
        };
        match container.doc.set_bytes(author, key, data).await {
            Ok(_) => Ok(Ok(())),
            Err(e) => {
                tracing::error!("failed to write entry: {}", e);
                Ok(Err(Error::CapabilityError(e.to_string())))
            }
        }
    }

    /// List the keys of the entries in the document.
    async fn list_entries(
        &mut self, container: Resource<Document>,
    ) -> wasmtime::Result<Result<Vec<String>, Error>> {
        tracing::trace!("container::HostContainer::list_entries");
        let container = match self.table().get_mut(&container) {
            Ok(container) => container,
            Err(e) => {
                tracing::error!("failed to get document from resource table: {}", e);
                return Ok(Err(Error::Other(e.to_string())));
            }
        };
        let mut entries = match container.doc.get_many(Query::single_latest_per_key()).await {
            Ok(entries) => entries,
            Err(e) => {
                tracing::error!("failed to query entries: {}", e);
                return Ok(Err(Error::CapabilityError(e.to_string())));
            }
        };
        let mut keys = Vec::new();
        while let Some(entry) = entries.try_next().await? {
            let key_bytes = entry.key();
            let key = match String::from_utf8(key_bytes.to_vec()) {
                Ok(key) => key,
                Err(e) => {
                    tracing::error!("failed to convert key to string: {}", e);
                    return Ok(Err(Error::Other(e.to_string())));
                }
            };
            keys.push(key);
        }
        Ok(Ok(keys))
    }

    /// Get information about an entry.
    async fn get_entry_metadata(
        &mut self, container: Resource<Document>, key: String,
    ) -> wasmtime::Result<Result<EntryMetadata, Error>> {
        tracing::trace!("container::HostContainer::get_entry_metadata {key}");
        let container = match self.table().get_mut(&container) {
            Ok(container) => container,
            Err(e) => {
                tracing::error!("failed to get document from resource table: {}", e);
                return Ok(Err(Error::Other(e.to_string())));
            }
        };
        let entry = match get_container_entry(container, &key).await {
            Ok(Some(entry)) => entry,
            Ok(None) => {
                tracing::error!("entry {key} not found");
                return Ok(Err(Error::NoSuchEntry));
            }
            Err(e) => {
                tracing::error!("failed to query entry: {}", e);
                return Ok(Err(Error::CapabilityError(e.to_string())));
            }
        };
        let metadata = EntryMetadata {
            name: key.clone(),
            container: container.doc.id().to_string(),
            size: entry.content_len(),
            created_at: entry.timestamp(),
            author: entry.author().fmt_short(),
        };
        Ok(Ok(metadata))
    }

    /// Read an entry
    async fn read_entry(
        &mut self, container: Resource<Document>, key: String, start: u64, len: u64,
    ) -> wasmtime::Result<Result<Vec<u8>, Error>> {
        tracing::trace!(
            "container::HostContainer::read_entry {key} from {start} up to {len} bytes"
        );
        let Ok(iroh) = iroh_node() else {
            return Ok(Err(Error::CapabilityUnavailable));
        };
        let container = match self.table().get_mut(&container) {
            Ok(container) => container,
            Err(e) => {
                tracing::error!("failed to get document from resource table: {}", e);
                return Ok(Err(Error::Other(e.to_string())));
            }
        };
        let entry = match get_container_entry(container, &key).await {
            Ok(Some(entry)) => entry,
            Ok(None) => {
                tracing::error!("entry {key} not found");
                return Ok(Err(Error::NoSuchEntry));
            }
            Err(e) => {
                tracing::error!("failed to query entry: {}", e);
                return Ok(Err(Error::CapabilityError(e.to_string())));
            }
        };
        let test_len = std::cmp::min(len, entry.content_len() - start);
        let hash = entry.content_hash();
        let data = match iroh
            .blobs()
            .read_at_to_bytes(hash, start, Some(usize::try_from(test_len)?))
            .await
        {
            Ok(data) => data,
            Err(e) => {
                tracing::error!("failed to read entry: {}", e);
                return Ok(Err(Error::CapabilityError(e.to_string())));
            }
        };
        Ok(Ok(data.to_vec()))
    }

    /// Delete an entry from the document.
    async fn delete_entry(
        &mut self, container: Resource<Document>, key: String,
    ) -> wasmtime::Result<Result<(), Error>> {
        tracing::trace!("container::HostContainer::delete_entry {key}");
        let container = match self.table().get_mut(&container) {
            Ok(container) => container,
            Err(e) => {
                tracing::error!("failed to get document from resource table: {}", e);
                return Ok(Err(Error::Other(e.to_string())));
            }
        };
        let entry = match get_container_entry(container, &key).await {
            Ok(Some(entry)) => entry,
            Ok(None) => {
                tracing::error!("entry {key} not found");
                return Ok(Err(Error::NoSuchEntry));
            }
            Err(e) => {
                tracing::error!("failed to query entry: {}", e);
                return Ok(Err(Error::CapabilityError(e.to_string())));
            }
        };
        match container.doc.del(entry.author(), key.clone()).await {
            Ok(_) => Ok(Ok(())),
            Err(e) => {
                tracing::error!("failed to delete entry: {}", e);
                Ok(Err(Error::CapabilityError(e.to_string())))
            }
        }
    }

    /// Delete all entries in the document.
    async fn clear_entries(
        &mut self, container: Resource<Document>,
    ) -> wasmtime::Result<Result<(), Error>> {
        tracing::trace!("container::HostContainer::clear_entries");
        let container = match self.table().get_mut(&container) {
            Ok(container) => container,
            Err(e) => {
                tracing::error!("failed to get document from resource table: {}", e);
                return Ok(Err(Error::Other(e.to_string())));
            }
        };
        let mut entries = match container.doc.get_many(Query::all()).await {
            Ok(entries) => entries,
            Err(e) => {
                tracing::error!("failed to query entries: {}", e);
                return Ok(Err(Error::CapabilityError(e.to_string())));
            }
        };
        let mut keys = Vec::new();
        let mut authors = Vec::new();
        while let Some(entry) = entries.try_next().await? {
            keys.push(entry.key().to_vec());
            authors.push(entry.author());
        }
        for (key, author) in keys.iter().zip(authors.iter()) {
            match container.doc.del(*author, key.clone()).await {
                Ok(_) => (),
                Err(e) => {
                    tracing::error!("failed to delete entry: {}", e);
                    return Ok(Err(Error::CapabilityError(e.to_string())));
                }
            }
        }
        Ok(Ok(()))
    }

    /// Get a token to work with the document
    async fn get_token(
        &mut self, container: Resource<Document>, mode: Permission,
    ) -> wasmtime::Result<Result<Token, Error>> {
        tracing::trace!("container::HostContainer::get_token");
        let container = match self.table().get_mut(&container) {
            Ok(container) => container,
            Err(e) => {
                tracing::error!("failed to get document from resource table: {}", e);
                return Ok(Err(Error::Other(e.to_string())));
            }
        };
        let share_mode = match mode {
            Permission::Read => ShareMode::Read,
            Permission::Write => ShareMode::Write,
        };
        let ticket = match container.doc.share(share_mode, AddrInfoOptions::default()).await {
            Ok(ticket) => ticket,
            Err(e) => {
                tracing::error!("failed to get document ticket: {}", e);
                return Ok(Err(Error::CapabilityError(e.to_string())));
            }
        };
        Ok(Ok(ticket.to_string()))
    }

    /// Remove the document from runtime state.
    fn drop(&mut self, container: Resource<Document>) -> wasmtime::Result<()> {
        tracing::trace!("container::HostContainer::drop");
        self.table().delete(container).map_or_else(|e| Err(anyhow!(e)), |_| Ok(()))
    }
}

/// Get the entry with the given key from the container.
async fn get_container_entry(container: &Document, key: &str) -> Result<Option<Entry>, Error> {
    let mut entry_stream =
        match container.doc.get_many(Query::single_latest_per_key().key_exact(key)).await {
            Ok(entries) => entries,
            Err(e) => {
                tracing::error!("failed to query entries: {}", e);
                return Err(Error::CapabilityError(e.to_string()));
            }
        };
    match entry_stream.next().await {
        Some(Ok(entry)) => Ok(Some(entry)),
        Some(Err(e)) => {
            tracing::error!("failed to get entry: {}", e);
            Err(Error::CapabilityError(e.to_string()))
        }
        None => Ok(None),
    }
}

/// Implementation of the `wasi:p2p/types` interface.
impl types::Host for Ctx {}

/// Service configuration.
pub struct Service;

/// Create a new service.
pub const fn new() -> Service {
    Service
}

/// Implentation required by the `wasmgrid` runtime.
#[async_trait::async_trait]
impl runtime::Service for Service {
    fn namespace(&self) -> &'static str {
        "wasi:p2p"
    }

    fn add_to_linker(&self, linker: &mut Linker<Ctx>) -> anyhow::Result<()> {
        P2p::add_to_linker(linker, |t| t)
    }

    #[allow(clippy::large_futures)]
    async fn start(&self, _runtime: Runtime) -> anyhow::Result<()> {
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
fn iroh_node() -> anyhow::Result<&'static FsNode> {
    let Some(node) = IROH_NODE.get() else {
        tracing::error!("Iroh node not started");
        return Err(anyhow!("Iroh node not started"));
    };
    Ok(node)
}

// Given the short ID of an author, return the full author public key.
async fn find_author(iroh: &FsNode, author_short_id: &str) -> anyhow::Result<Option<AuthorId>> {
    let mut authors = match iroh.authors().list().await {
        Ok(authors) => authors,
        Err(e) => {
            tracing::error!("failed to get authors list: {}", e);
            return Err(e);
        }
    };
    while let Some(author) = authors.try_next().await? {
        if author.fmt_short() == author_short_id {
            return Ok(Some(author));
        }
    }
    tracing::warn!("requested author not found");
    Ok(None)
}
