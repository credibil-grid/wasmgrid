//! # WASI Blobstore Service for NATS
//!
//! This module implements a runtime service for `wasi:blobstore`
//! (<https://github.com/WebAssembly/wasi-blobstore>).

mod generated {
    #![allow(clippy::trait_duplication_in_bounds)]

    pub use super::{Container, IncomingValue, OutgoingValue, StreamObjectNames};

    wasmtime::component::bindgen!({
        world: "blobstore",
        path: "../../wit",
        imports: {
            default: async | tracing | trappable,
        },
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

use std::sync::OnceLock;

use anyhow::{Result, anyhow};
use async_nats::jetstream;
use async_nats::jetstream::object_store::{Config, ObjectStore};
use bytes::{Bytes, BytesMut};
use futures::StreamExt;
use runtime::{AddResource, RunState};
use time::OffsetDateTime;
use tokio::io::AsyncReadExt;
use wasmtime::component::{HasData, Linker, Resource, ResourceTable};
use wasmtime_wasi::p2::bindings::io::streams::{InputStream, OutputStream};
use wasmtime_wasi::p2::pipe::{MemoryInputPipe, MemoryOutputPipe};

use self::generated::wasi::blobstore::blobstore::{self, ObjectId};
use self::generated::wasi::blobstore::container::{self, ContainerMetadata, ObjectMetadata};
use self::generated::wasi::blobstore::types::{self, IncomingValueSyncBody};

pub type Container = ObjectStore;
pub type IncomingValue = Bytes;
pub type OutgoingValue = MemoryOutputPipe;
pub type StreamObjectNames = Vec<String>;

static NATS_CLIENT: OnceLock<async_nats::Client> = OnceLock::new();

#[derive(Debug)]
pub struct Service;

impl runtime::Service for Service {
    fn add_to_linker(&self, l: &mut Linker<RunState>) -> Result<()> {
        blobstore::add_to_linker::<_, Data>(l, Blobstore::new)?;
        container::add_to_linker::<_, Data>(l, Blobstore::new)?;
        types::add_to_linker::<_, Data>(l, Blobstore::new)?;
        Ok(())
    }
}

impl AddResource<async_nats::Client> for Service {
    fn resource(self, resource: async_nats::Client) -> Result<Self> {
        NATS_CLIENT.set(resource).map_err(|_| anyhow!("client already set"))?;
        Ok(self)
    }
}

struct Data;
impl HasData for Data {
    type Data<'a> = Blobstore<'a>;
}

pub struct Blobstore<'a> {
    table: &'a mut ResourceTable,
}

impl Blobstore<'_> {
    const fn new(c: &mut RunState) -> Blobstore<'_> {
        Blobstore { table: &mut c.table }
    }
}

fn nats() -> Result<&'static async_nats::Client> {
    NATS_CLIENT.get().ok_or_else(|| anyhow!("NATS client not initialized."))
}

// Implement the [`wasi_sql::ReadWriteView`]` trait for Blobstore<'_>.
impl blobstore::Host for Blobstore<'_> {
    async fn create_container(&mut self, name: String) -> Result<Resource<Container>> {
        let jetstream = jetstream::new(nats()?.clone());
        let store = jetstream
            .create_object_store(Config {
                bucket: name,
                ..Config::default()
            })
            .await?;

        Ok(self.table.push(store)?)
    }

    async fn get_container(&mut self, name: String) -> Result<Resource<Container>> {
        let jetstream = jetstream::new(nats()?.clone());
        let store = jetstream
            .get_object_store(&name)
            .await
            .map_err(|e| anyhow!("issue getting object store: {e}"))?;

        Ok(self.table.push(store)?)
    }

    async fn delete_container(&mut self, name: String) -> Result<()> {
        let jetstream = jetstream::new(nats()?.clone());
        jetstream
            .delete_object_store(&name)
            .await
            .map_err(|e| anyhow!("issue deleting object store: {e}"))?;

        Ok(())
    }

    async fn container_exists(&mut self, name: String) -> Result<bool> {
        let jetstream = jetstream::new(nats()?.clone());
        let exists = jetstream.get_object_store(&name).await.is_ok();

        Ok(exists)
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
    async fn name(&mut self, store_ref: Resource<Container>) -> Result<String> {
        let Ok(store) = self.table.get(&store_ref) else {
            return Err(anyhow!("Container not found"));
        };

        // HACK: get the store name from the first object in the store
        // TODO: wrap NATS ObjectStore with a custom type
        let mut list = store.list().await.map_err(|e| anyhow!("issue listing objects: {e}"))?;
        let Some(Ok(n)) = list.next().await else {
            return Err(anyhow!("No objects found in the store"));
        };

        Ok(n.bucket)
    }

    async fn info(&mut self, _store_ref: Resource<Container>) -> Result<ContainerMetadata> {
        todo!()
    }

    async fn get_data(
        &mut self, store_ref: Resource<Container>, name: String, _start: u64, _end: u64,
    ) -> Result<Resource<IncomingValue>> {
        let Ok(store) = self.table.get(&store_ref) else {
            return Err(anyhow!("Container not found"));
        };

        // read the object data from the store
        let mut data = store.get(&name).await.map_err(|e| anyhow!("issue getting object: {e}"))?;
        let mut buf = BytesMut::new();
        data.read_buf(&mut buf).await?;

        Ok(self.table.push(buf.into())?)
    }

    async fn write_data(
        &mut self, store_ref: Resource<Container>, name: String, value_ref: Resource<OutgoingValue>,
    ) -> Result<()> {
        let Ok(value) = self.table.get(&value_ref) else {
            return Err(anyhow!("OutgoingValue not found"));
        };
        let bytes = value.contents();

        let Ok(store) = self.table.get_mut(&store_ref) else {
            return Err(anyhow!("Container not found"));
        };

        // write the data to the store
        store
            .put(name.as_str(), &mut bytes.to_vec().as_slice())
            .await
            .map_err(|e| anyhow!("issue writing object: {e}"))?;

        Ok(())
    }

    async fn list_objects(
        &mut self, store_ref: Resource<Container>,
    ) -> Result<Resource<StreamObjectNames>> {
        let Ok(store) = self.table.get(&store_ref) else {
            return Err(anyhow!("Container not found"));
        };
        let mut list = store.list().await.map_err(|e| anyhow!("issue listing objects: {e}"))?;

        let mut names = vec![];
        while let Some(n) = list.next().await {
            match n {
                Ok(obj_info) => names.push(obj_info.name),
                Err(e) => tracing::warn!("issue listing object: {e}"),
            }
        }

        Ok(self.table.push(names)?)
    }

    async fn delete_object(&mut self, store_ref: Resource<Container>, name: String) -> Result<()> {
        let Ok(store) = self.table.get_mut(&store_ref) else {
            return Err(anyhow!("Container not found"));
        };
        store.delete(&name).await.map_err(|e| anyhow!("issue deleting: {e}"))?;

        Ok(())
    }

    async fn delete_objects(
        &mut self, store_ref: Resource<Container>, names: Vec<String>,
    ) -> Result<()> {
        let Ok(store) = self.table.get_mut(&store_ref) else {
            return Err(anyhow!("Container not found"));
        };
        for name in names {
            store.delete(&name).await.map_err(|e| anyhow!("issue deleting '{name}': {e}"))?;
        }

        Ok(())
    }

    async fn has_object(&mut self, store_ref: Resource<Container>, name: String) -> Result<bool> {
        let Ok(store) = self.table.get(&store_ref) else {
            return Err(anyhow!("Container not found"));
        };
        Ok(store.info(&name).await.is_ok())
    }

    async fn object_info(
        &mut self, store_ref: Resource<Container>, name: String,
    ) -> Result<ObjectMetadata> {
        let Ok(store) = self.table.get(&store_ref) else {
            return Err(anyhow!("Container not found"));
        };
        let info = store.info(&name).await?;

        #[allow(clippy::cast_sign_loss)]
        let metadata = ObjectMetadata {
            name: info.name,
            container: name,
            size: info.size as u64,
            created_at: info.modified.unwrap_or(OffsetDateTime::UNIX_EPOCH).unix_timestamp() as u64,
        };
        Ok(metadata)
    }

    async fn clear(&mut self, store_ref: Resource<Container>) -> Result<()> {
        let Ok(store) = self.table.get(&store_ref) else {
            return Err(anyhow!("Container not found"));
        };
        let mut list = store.list().await.map_err(|e| anyhow!("issue listing objects: {e}"))?;

        while let Some(n) = list.next().await {
            match n {
                Ok(obj_info) => store.delete(obj_info.name).await?,
                Err(e) => tracing::warn!("issue listing object: {e}"),
            }
        }

        Ok(())
    }

    async fn drop(&mut self, store_ref: Resource<Container>) -> Result<()> {
        self.table.delete(store_ref)?;
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
        Ok(self.table.push(OutgoingValue::new(1024))?)
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
