//! # WASI Blobstore Service for MongoDB
//!
//! This module implements a runtime service for `wasi:sql`
//! (<https://github.com/WebAssembly/wasi-sql>).

/// Wrap generation of wit bindings to simplify exports.
/// See <https://docs.rs/wasmtime/latest/wasmtime/component/macro.bindgen.html>
mod generated {
    #![allow(clippy::trait_duplication_in_bounds)]

    pub use super::{Container, IncomingValue, OutgoingValue, StreamObjectNames};

    wasmtime::component::bindgen!({
        world: "blobstore",
        path: "../../wit",
        tracing: true,
        async: true,
        trappable_imports: true,
        with: {
            "wasi:io": wasmtime_wasi::p2::bindings::io,

            "wasi:blobstore/types/incoming-value": IncomingValue,
            "wasi:blobstore/types/outgoing-value": OutgoingValue,
            "wasi:blobstore/container/container": Container,
            "wasi:blobstore/container/stream-object-names": StreamObjectNames,
        },
        trappable_error_type: {
            "wasi:blobstore/types/error" => anyhow::Error,
        },
    });
}

use anyhow::{Result, anyhow};
use bson::{Document, doc};
use bytes::Bytes;
use chrono::Utc;
use futures::StreamExt;
use mongodb::{Collection, bson};
use runtime::Linkable;
use serde::{Deserialize, Serialize};
use wasmtime::component::{HasData, Linker, Resource, ResourceTable};
use wasmtime_wasi::p2::bindings::io::streams::{InputStream, OutputStream};
use wasmtime_wasi::p2::pipe::{MemoryInputPipe, MemoryOutputPipe};

use self::generated::wasi::blobstore::blobstore::{self, ObjectId};
use self::generated::wasi::blobstore::container::{self, ContainerMetadata, ObjectMetadata};
use self::generated::wasi::blobstore::types::{self, IncomingValueSyncBody};
use crate::{Ctx, Resources};

pub type Container = Collection<Blob>;
pub type IncomingValue = Bytes;
pub type OutgoingValue = MemoryOutputPipe;
pub type StreamObjectNames = Vec<String>;

pub struct Blobstore<'a> {
    resources: &'a Resources,
    table: &'a mut ResourceTable,
}

impl Blobstore<'_> {
    const fn new(c: &mut Ctx) -> Blobstore<'_> {
        Blobstore {
            resources: &c.resources,
            table: &mut c.table,
        }
    }
}

struct Data;
impl HasData for Data {
    type Data<'a> = Blobstore<'a>;
}

pub struct Service;

impl Linkable for Service {
    type Ctx = Ctx;

    // Add all the `wasi-keyvalue` world's interfaces to a [`Linker`], and
    // instantiate the `Blobstore` for the component.
    fn add_to_linker(&self, l: &mut Linker<Self::Ctx>) -> anyhow::Result<()> {
        blobstore::add_to_linker::<_, Data>(l, Blobstore::new)?;
        container::add_to_linker::<_, Data>(l, Blobstore::new)?;
        types::add_to_linker::<_, Data>(l, Blobstore::new)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blob {
    name: String,
    doc: Document,
    size: u64,
    created_at: u64,
}

// Implement the [`wasi_sql::ReadWriteView`]` trait for Blobstore<'_>.
impl blobstore::Host for Blobstore<'_> {
    async fn create_container(&mut self, name: String) -> Result<Resource<Container>> {
        let Some(db) = self.resources.mongo()?.default_database() else {
            return Err(anyhow!("No default database found"));
        };
        let collection = db.collection::<Blob>(&name);
        Ok(self.table.push(collection)?)
    }

    async fn get_container(&mut self, name: String) -> Result<Resource<Container>> {
        let Some(db) = self.resources.mongo()?.default_database() else {
            return Err(anyhow!("No default database found"));
        };
        let collection = db.collection::<Blob>(&name);
        Ok(self.table.push(collection)?)
    }

    async fn delete_container(&mut self, name: String) -> Result<()> {
        let Some(db) = self.resources.mongo()?.default_database() else {
            return Err(anyhow!("No default database found"));
        };
        db.collection::<Blob>(&name)
            .drop()
            .await
            .map_err(|e| anyhow!("issue deleting container: {e}"))
    }

    async fn container_exists(&mut self, _name: String) -> Result<bool> {
        Ok(true)
    }

    async fn copy_object(&mut self, _src: ObjectId, _dest: ObjectId) -> Result<()> {
        todo!()
    }

    async fn move_object(&mut self, _src: ObjectId, _dest: ObjectId) -> Result<()> {
        todo!()
    }
}

impl container::Host for Blobstore<'_> {}

impl container::HostContainer for Blobstore<'_> {
    async fn name(&mut self, coll_ref: Resource<Container>) -> Result<String> {
        let Ok(collection) = self.table.get(&coll_ref) else {
            return Err(anyhow!("Container not found"));
        };
        Ok(collection.name().to_string())
    }

    async fn info(&mut self, _coll_ref: Resource<Container>) -> Result<ContainerMetadata> {
        todo!()
    }

    async fn get_data(
        &mut self, coll_ref: Resource<Container>, name: String, _start: u64, _end: u64,
    ) -> Result<Resource<IncomingValue>> {
        let Ok(collection) = self.table.get(&coll_ref) else {
            return Err(anyhow!("Container not found"));
        };
        let Some(blob) = collection.find_one(doc! { "name": name }).await? else {
            return Err(anyhow!("Object not found"));
        };
        let data =
            serde_json::to_vec(&blob.doc).map_err(|e| anyhow!("failed to serialize BSON: {e}"))?;

        Ok(self.table.push(Bytes::from(data))?)
    }

    async fn write_data(
        &mut self, coll_ref: Resource<Container>, name: String, value_ref: Resource<OutgoingValue>,
    ) -> Result<()> {
        let Ok(collection) = self.table.get(&coll_ref) else {
            return Err(anyhow!("Container not found"));
        };
        let Ok(value) = self.table.get(&value_ref) else {
            return Err(anyhow!("OutgoingValue not found"));
        };

        // `put` should update any previous value, so delete first
        collection.delete_one(doc! { "name": &name }).await?;

        let doc = serde_json::from_slice::<Document>(&value.contents())
            .map_err(|e| anyhow!("failed to deserialize BSON: {e}"))?;
        let blob = Blob {
            name,
            doc,
            size: value.contents().len() as u64,
            #[allow(clippy::cast_sign_loss)]
            created_at: Utc::now().timestamp_millis() as u64,
        };
        collection.insert_one(blob).await?;

        Ok(())
    }

    async fn list_objects(
        &mut self, coll_ref: Resource<Container>,
    ) -> Result<Resource<StreamObjectNames>> {
        let Ok(collection) = self.table.get(&coll_ref) else {
            return Err(anyhow!("Container not found"));
        };
        let mut list = collection.find(doc! {}).await?;

        let mut names = vec![];
        while let Some(n) = list.next().await {
            match n {
                Ok(blob) => names.push(blob.name),
                Err(e) => tracing::warn!("issue listing object: {e}"),
            }
        }

        Ok(self.table.push(names)?)
    }

    async fn delete_object(&mut self, coll_ref: Resource<Container>, name: String) -> Result<()> {
        let Ok(collection) = self.table.get_mut(&coll_ref) else {
            return Err(anyhow!("Container not found"));
        };
        collection.delete_one(doc! { "name": name }).await?;
        Ok(())
    }

    async fn delete_objects(
        &mut self, coll_ref: Resource<Container>, names: Vec<String>,
    ) -> Result<()> {
        let Ok(collection) = self.table.get_mut(&coll_ref) else {
            return Err(anyhow!("Container not found"));
        };
        collection.delete_many(doc! { "name": { "$in": names } }).await?;
        Ok(())
    }

    async fn has_object(&mut self, coll_ref: Resource<Container>, name: String) -> Result<bool> {
        let Ok(collection) = self.table.get(&coll_ref) else {
            return Err(anyhow!("Container not found"));
        };
        Ok(collection.find_one(doc! { "name": name }).await?.is_some())
    }

    async fn object_info(
        &mut self, coll_ref: Resource<Container>, name: String,
    ) -> Result<ObjectMetadata> {
        let Ok(collection) = self.table.get(&coll_ref) else {
            return Err(anyhow!("Container not found"));
        };
        let Some(blob) = collection.find_one(doc! { "name": name }).await? else {
            return Err(anyhow!("Object not found"));
        };

        Ok(ObjectMetadata {
            name: blob.name,
            container: collection.name().to_string(),
            size: blob.size,
            created_at: blob.created_at,
        })
    }

    async fn clear(&mut self, coll_ref: Resource<Container>) -> Result<()> {
        let Ok(collection) = self.table.get(&coll_ref) else {
            return Err(anyhow!("Container not found"));
        };
        Ok(collection.drop().await?)
    }

    async fn drop(&mut self, coll_ref: Resource<Container>) -> Result<()> {
        self.table.delete(coll_ref)?;
        Ok(())
    }
}

impl container::HostStreamObjectNames for Blobstore<'_> {
    async fn read_stream_object_names(
        &mut self, _names_ref: Resource<StreamObjectNames>, _len: u64,
    ) -> Result<(Vec<String>, bool)> {
        todo!()
    }

    async fn skip_stream_object_names(
        &mut self, _names_ref: Resource<StreamObjectNames>, _num: u64,
    ) -> Result<(u64, bool)> {
        todo!()
    }

    async fn drop(&mut self, names_ref: Resource<StreamObjectNames>) -> Result<()> {
        Ok(self.table.delete(names_ref).map(|_| ())?)
    }
}

impl types::Host for Blobstore<'_> {
    fn convert_error(&mut self, err: anyhow::Error) -> Result<String> {
        tracing::error!("{err}");
        Ok(err.to_string())
    }
}

impl types::HostIncomingValue for Blobstore<'_> {
    async fn incoming_value_consume_sync(
        &mut self, value_ref: Resource<IncomingValue>,
    ) -> Result<IncomingValueSyncBody> {
        let value = self.table.get(&value_ref)?;
        Ok(value.to_vec())
    }

    async fn incoming_value_consume_async(
        &mut self, value_ref: Resource<IncomingValue>,
    ) -> Result<Resource<InputStream>> {
        let value = self.table.get(&value_ref)?;
        let rs = MemoryInputPipe::new(value.clone());
        let stream: InputStream = Box::new(rs);

        Ok(self.table.push(stream)?)
    }

    async fn size(&mut self, value_ref: Resource<IncomingValue>) -> Result<u64> {
        let value = self.table.get(&value_ref)?;
        Ok(value.len() as u64)
    }

    async fn drop(&mut self, value_ref: Resource<IncomingValue>) -> Result<()> {
        Ok(self.table.delete(value_ref).map(|_| ())?)
    }
}

impl types::HostOutgoingValue for Blobstore<'_> {
    async fn new_outgoing_value(&mut self) -> Result<Resource<OutgoingValue>> {
        // HACK: 1 MiB is the maximum capacity for in-mem outgoing values.
        Ok(self.table.push(OutgoingValue::new(1_048_576))?)
    }

    async fn outgoing_value_write_body(
        &mut self, value_ref: Resource<OutgoingValue>,
    ) -> Result<Resource<OutputStream>> {
        let value = self.table.get(&value_ref)?;
        let stream: OutputStream = Box::new(value.clone());
        Ok(self.table.push(stream)?)
    }

    async fn finish(&mut self, _: Resource<OutgoingValue>) -> Result<()> {
        // self.table.delete(value_ref)?;
        Ok(())
    }

    async fn drop(&mut self, value_ref: Resource<OutgoingValue>) -> Result<()> {
        Ok(self.table.delete(value_ref).map(|_| ())?)
    }
}
