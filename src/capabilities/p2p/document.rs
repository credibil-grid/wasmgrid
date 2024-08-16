use std::pin::Pin;
use std::str::FromStr;

use anyhow::{anyhow, Context};
use futures::stream::Stream;
use futures::TryStreamExt;
use iroh::base::node_addr::AddrInfoOptions;
use iroh::client::docs::{Doc, Entry, ShareMode};
use iroh::docs::store::Query;
use iroh::docs::{AuthorId, DocTicket};
use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;

use super::bindings::wasi::blobstore::container::{
    self, ContainerMetadata, Error, HostContainer, HostStreamObjectNames, ObjectMetadata,
    ObjectName,
};
use super::bindings::wasi::p2p::document::Host;
use super::bindings::wasi::p2p::types::{ContainerToken, Owner};
use super::iroh_node;
use super::types::{Blob, BlobValue};
use crate::capabilities::p2p::find_author;
use crate::runtime::State;

impl container::Host for State {}

/// Document is a wrapper for a container that implements Iroh operations.
pub struct Document {
    author: AuthorId,
    doc: Doc,
}

#[async_trait::async_trait]
impl Host for State {
    /// Create a new author.
    async fn create_owner(&mut self) -> wasmtime::Result<Result<Owner, Error>> {
        tracing::debug!("Host::create_owner");
        let iroh = iroh_node()?;
        let author = iroh.authors().create().await?;
        Ok(Ok(author.fmt_short()))
    }

    /// Create a new document.
    async fn create_container(
        &mut self, owner: Owner,
    ) -> wasmtime::Result<Result<(Resource<Document>, ContainerToken), Error>> {
        // Do not log owner - it is sensitive.
        tracing::debug!("Host::create_container");

        let Some(author) = find_author(&owner).await? else {
            return Ok(Err("Author not found".into()));
        };
        let iroh = iroh_node()?;
        let doc = iroh.docs().create().await?;
        tracing::debug!(
            "Host::create_container: created document with id: {}",
            doc.id().to_string()
        );
        let ticket = doc.share(ShareMode::Write, AddrInfoOptions::default()).await?;
        tracing::debug!("Host::create_container: shared document with ticket: {ticket}");

        let container = Document { author, doc };
        let stashed = self.table().push(container)?;
        Ok(Ok((stashed, ticket.to_string())))
    }

    /// Get an existing container.
    async fn get_container(
        &mut self, owner: Owner, token: ContainerToken,
    ) -> wasmtime::Result<Result<Resource<Document>, Error>> {
        // Do not log ticket - it is sensitive.
        tracing::debug!("Host::get_container");

        let Some(author) = find_author(&owner).await? else {
            return Ok(Err("Author not found".into()));
        };
        let ticket = DocTicket::from_str(&token).context("invalid token")?;
        let iroh = iroh_node()?;
        let doc = iroh.docs().import(ticket.clone()).await?;
        let container = Document { author, doc };
        let stashed = self.table().push(container)?;
        Ok(Ok(stashed))
    }

    /// Delete the container
    async fn delete_container(
        &mut self, container: Resource<Document>,
    ) -> wasmtime::Result<Result<(), Error>> {
        // Do not log ticket - it is sensitive.
        tracing::debug!("Host::delete_container");

        let iroh = iroh_node()?;
        let container = self.table().get_mut(&container)?;
        container.doc.close().await?;
        iroh.docs().drop_doc(container.doc.id()).await?;
        Ok(Ok(()))
    }
}

#[async_trait::async_trait]
impl HostContainer for State {
    /// Get the ID of the Iroh document.
    async fn name(
        &mut self, container: Resource<Document>,
    ) -> wasmtime::Result<Result<String, Error>> {
        tracing::debug!("HostContainer::name");
        let container = self.table().get_mut(&container)?;
        Ok(Ok(container.doc.id().to_string()))
    }

    /// Iroh does not support document metadata so this function will always return an error.
    async fn info(
        &mut self, _container: Resource<Document>,
    ) -> wasmtime::Result<Result<ContainerMetadata, Error>> {
        Err(anyhow!("Unsupported"))
    }

    /// Retrieve a portion or all of the data from an entry. Use [`object_info`] to get the size of
    /// the entry.
    async fn get_data(
        &mut self, container: Resource<Document>, name: ObjectName, start: u64, end: u64,
    ) -> wasmtime::Result<Result<Resource<BlobValue>, Error>> {
        tracing::debug!("HostContainer::get_data {name} {start} {end}");
        let document = self.table().get_mut(&container)?;
        let Some(entry) = document.doc.get_exact(document.author, name, false).await? else {
            return Ok(Err("Entry not found".into()));
        };
        let mut test_end = end;
        if test_end >= entry.content_len() {
            test_end = entry.content_len() - 1;
        }
        let len = test_end - start + 1;
        let hash = entry.content_hash();
        let iroh = iroh_node()?;
        let data = iroh.blobs().read_at_to_bytes(hash, start, Some(usize::try_from(len)?)).await?;
        let blob = Blob::from(data);
        Ok(Ok(self.table().push(BlobValue::new(blob))?))
    }

    /// Create or replace an entry with the given data.
    async fn write_data(
        &mut self, container: Resource<Document>, name: ObjectName, data: Resource<BlobValue>,
    ) -> wasmtime::Result<Result<(), Error>> {
        tracing::debug!("HostContainer::write_data {name}");
        let table = self.table();
        let document = table.get(&container)?;
        tracing::debug!("HostContainer::write_data: writing to document {}", document.doc.id());
        let value = table.get(&data)?;
        let blob = value.blob.clone();
        // TODO: This await never resolves.
        document.doc.set_bytes(document.author, name, blob).await?;
        tracing::debug!("HostContainer::write_data: complete");
        Ok(Ok(()))
    }

    /// Get a list of all entries in the document. (Returns their references, not all of the data.)
    async fn list_objects(
        &mut self, container: Resource<Document>,
    ) -> wasmtime::Result<Result<Resource<StreamObjectNames>, Error>> {
        tracing::debug!("HostContainer::list_objects");
        let document = self.table().get_mut(&container)?;
        let entries = document.doc.get_many(Query::single_latest_per_key()).await?;
        let objects: Pin<Box<dyn Stream<Item = anyhow::Result<Entry>> + Send>> = Box::pin(entries);
        Ok(Ok(self.table().push(objects)?))
    }

    /// Remove an entry from the document on the Iroh network. Will not remove the resource from the
    /// runtime state.
    async fn delete_object(
        &mut self, container: Resource<Document>, name: ObjectName,
    ) -> wasmtime::Result<Result<(), Error>> {
        tracing::debug!("HostContainer::delete_object {name}");
        let document = self.table().get_mut(&container)?;
        document.doc.del(document.author, name).await?;
        Ok(Ok(()))
    }

    /// Remove multiple objects from the container on the Iroh network. Will not remove the
    /// resources from the runtime state.
    async fn delete_objects(
        &mut self, container: Resource<Document>, names: Vec<ObjectName>,
    ) -> wasmtime::Result<Result<(), Error>> {
        tracing::debug!("HostContainer::delete_objects");
        let document = self.table().get_mut(&container)?;
        for name in names {
            document.doc.del(document.author, name).await?;
        }
        Ok(Ok(()))
    }

    /// Check if an object with the given key exists in the Iroh document.
    async fn has_object(
        &mut self, container: Resource<Document>, name: ObjectName,
    ) -> wasmtime::Result<Result<bool, Error>> {
        tracing::debug!("HostContainer::has_object {name}");
        let document = self.table().get_mut(&container)?;
        match document.doc.get_exact(document.author, name, false).await? {
            None => Ok(Ok(false)),
            Some(_) => Ok(Ok(true)),
        }
    }

    /// Get metadata about an entry in the Iroh document, including the length of content to aid
    /// reading in chunks.
    async fn object_info(
        &mut self, container: Resource<Document>, name: ObjectName,
    ) -> wasmtime::Result<Result<ObjectMetadata, Error>> {
        tracing::debug!("HostContainer::object_info {name}");
        let document = self.table().get_mut(&container)?;
        let Some(entry) = document.doc.get_exact(document.author, name, false).await? else {
            return Ok(Err("Entry not found".into()));
        };
        let md = ObjectMetadata {
            name: String::from_utf8(entry.key().to_vec())?,
            container: document.doc.id().to_string(),
            created_at: entry.timestamp(),
            size: entry.content_len(),
        };
        Ok(Ok(md))
    }

    /// Clear all entries from the document on the Iroh network. Will not remove any resources
    /// from the runtime state.
    async fn clear(
        &mut self, container: Resource<Document>,
    ) -> wasmtime::Result<Result<(), Error>> {
        tracing::debug!("HostContainer::clear");
        let document = self.table().get_mut(&container)?;
        let mut keys = Vec::new();
        let mut entries = document.doc.get_many(Query::all()).await?;
        while let Some(entry) = entries.try_next().await? {
            keys.push(entry.key().to_vec());
        }
        for key in keys {
            document.doc.del(document.author, key).await?;
        }
        Ok(Ok(()))
    }

    /// Remove the document from the runtime state.
    fn drop(&mut self, container: Resource<Document>) -> wasmtime::Result<()> {
        tracing::debug!("HostContainer::drop");
        self.table().delete(container)?;
        Ok(())
    }
}

pub type StreamObjectNames = Pin<Box<dyn Stream<Item = anyhow::Result<Entry>> + Send>>;

#[async_trait::async_trait]
impl HostStreamObjectNames for State {
    /// Read the next number of entries from the stream up to len. Returns the keys for the document
    /// entries and a boolean indicating if the end of the stream was reached.
    async fn read_stream_object_names(
        &mut self, stream: Resource<StreamObjectNames>, len: u64,
    ) -> wasmtime::Result<Result<(Vec<ObjectName>, bool), Error>> {
        let entries = self.table().get_mut(&stream)?;
        let mut keys: Vec<ObjectName> = Vec::new();
        let mut end = true;
        while let Some(entry) = entries.try_next().await? {
            let key_bytes = entry.key();
            let key = std::str::from_utf8(key_bytes).context("invalid utf8 key")?;
            keys.push(key.into());
            if keys.len() == usize::try_from(len)? {
                end = false;
                break;
            }
        }
        Ok(Ok((keys, end)))
    }

    /// Skip the next number of entries in the stream. Returns the number of document entries
    /// skipped and a boolean indicating if the end of the stream was reached.
    async fn skip_stream_object_names(
        &mut self, stream: Resource<StreamObjectNames>, num: u64,
    ) -> wasmtime::Result<Result<(u64, bool), Error>> {
        let entries = self.table().get_mut(&stream)?;
        let mut skipped: u64 = 0;
        let mut end = true;
        while let Some(_entry) = entries.try_next().await? {
            skipped += 1;
            if skipped == num {
                end = false;
                break;
            }
        }
        Ok(Ok((skipped, end)))
    }

    /// Remove the entries stream from runtime state.
    #[allow(unused_must_use)]
    fn drop(&mut self, stream: Resource<StreamObjectNames>) -> wasmtime::Result<()> {
        tracing::debug!("HostStreamObjectNames::drop");
        self.table().delete(stream)?;
        Ok(())
    }
}
