//! # WASI JSON Database Service
//!
//! This module implements a runtime service for `wasi:sql`
//! (<https://github.com/WebAssembly/wasi-sql>).

/// Wrap generation of wit bindings to simplify exports.
/// See <https://docs.rs/wasmtime/latest/wasmtime/component/macro.bindgen.html>
mod generated {
    pub use async_nats::jetstream::object_store::ObjectStore;

    pub use super::{IncomingValue, OutgoingValue, StreamObjectNames};

    wasmtime::component::bindgen!({
        world: "blobstore",
        path: "../../wit",
        tracing: true,
        async: true,
        trappable_imports: true,
        with: {
            "wasi:blobstore/types/incoming-value": IncomingValue,
            "wasi:blobstore/types/outgoing-value": OutgoingValue,
            "wasi:blobstore/container/container": ObjectStore,
            "wasi:blobstore/container/stream-object-names": StreamObjectNames,
        },
        trappable_error_type: {
            "wasi:blobstore/types/error" => anyhow::Error,
        },
    });
}

use anyhow::{Context, Result, anyhow};
use async_nats::jetstream;
use async_nats::jetstream::object_store::{Config, ObjectStore};
use futures::StreamExt;
use runtime::Linkable;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use wasmtime::component::{Linker, Resource, ResourceTable};

use self::generated::wasi::blobstore::blobstore::{self, ObjectId};
use self::generated::wasi::blobstore::container::{self, ContainerMetadata, ObjectMetadata};
use self::generated::wasi::blobstore::types::{
    self, IncomingValueAsyncBody, IncomingValueSyncBody, OutputStream,
};
use crate::{Ctx, Resources};

pub type IncomingValue = Vec<u8>;
pub type OutgoingValue = Vec<u8>;
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

pub struct Service;

impl Linkable for Service {
    type Ctx = Ctx;

    // Add all the `wasi-keyvalue` world's interfaces to a [`Linker`], and
    // instantiate the `Blobstore` for the component.
    fn add_to_linker(&self, linker: &mut Linker<Self::Ctx>) -> anyhow::Result<()> {
        blobstore::add_to_linker_get_host(linker, Blobstore::new)?;
        container::add_to_linker_get_host(linker, Blobstore::new)?;
        types::add_to_linker_get_host(linker, Blobstore::new)?;
        tracing::trace!("added to linker");
        Ok(())
    }
}

// Implement the [`wasi_sql::ReadWriteView`]` trait for Blobstore<'_>.
impl blobstore::Host for Blobstore<'_> {
    async fn create_container(&mut self, name: String) -> Result<Resource<ObjectStore>> {
        tracing::trace!("blobstore::Host::create_container");

        let jetstream = jetstream::new(self.resources.nats()?.clone());
        let bucket = jetstream
            .create_object_store(Config {
                bucket: name,
                ..Config::default()
            })
            .await?;

        Ok(self.table.push(bucket)?)
    }

    async fn get_container(&mut self, name: String) -> Result<Resource<ObjectStore>> {
        tracing::trace!("blobstore::Host::get_container");

        let jetstream = jetstream::new(self.resources.nats()?.clone());
        let bucket = jetstream
            .get_object_store(&name)
            .await
            .map_err(|e| anyhow!("issue getting object store: {e}"))?;

        Ok(self.table.push(bucket)?)
    }

    async fn delete_container(&mut self, name: String) -> Result<()> {
        tracing::trace!("blobstore::Host::delete_container");

        let jetstream = jetstream::new(self.resources.nats()?.clone());
        jetstream
            .delete_object_store(&name)
            .await
            .map_err(|e| anyhow!("issue deleting object store: {e}"))?;

        Ok(())
    }

    async fn container_exists(&mut self, name: String) -> Result<bool> {
        tracing::trace!("blobstore::Host::container_exists");

        let jetstream = jetstream::new(self.resources.nats()?.clone());
        let exists = jetstream.get_object_store(&name).await.is_ok();

        Ok(exists)
    }

    async fn copy_object(&mut self, src: ObjectId, dest: ObjectId) -> Result<()> {
        tracing::trace!("blobstore::Host::copy_object");
        todo!()
    }

    async fn move_object(&mut self, src: ObjectId, dest: ObjectId) -> Result<()> {
        tracing::trace!("blobstore::Host::move_object");
        todo!()
    }
}

impl container::Host for Blobstore<'_> {}

impl container::HostContainer for Blobstore<'_> {
    async fn name(&mut self, store_ref: Resource<ObjectStore>) -> Result<String> {
        tracing::trace!("container::HostContainer::name");
        todo!()
    }

    async fn info(&mut self, store_ref: Resource<ObjectStore>) -> Result<ContainerMetadata> {
        tracing::trace!("container::HostContainer::info");
        todo!()
    }

    async fn get_data(
        &mut self, store_ref: Resource<ObjectStore>, name: String, _start: u64, _end: u64,
    ) -> Result<Resource<IncomingValue>> {
        tracing::trace!("container::HostContainer::get_data");

        let Ok(bucket) = self.table.get(&store_ref) else {
            return Err(anyhow!("ObjectStore not found"));
        };

        // read the object data from the bucket
        let mut data = bucket.get(&name).await.map_err(|e| anyhow!("issue getting object: {e}"))?;
        let mut buf = vec![];
        let bytes = data.read_to_end(&mut buf).await?;

        tracing::trace!("read {bytes} bytes from object '{name}'");

        // return a reference to the data
        Ok(self.table.push(buf)?)
    }

    async fn write_data(
        &mut self, store_ref: Resource<ObjectStore>, name: String,
        data_ref: Resource<OutgoingValue>,
    ) -> Result<()> {
        tracing::trace!("container::HostContainer::write_data");

        let Ok(data) = self.table.get(&data_ref) else {
            return Err(anyhow!("OutgoingValue not found"));
        };
        // HACK: clone the data to get around issues with accessing self.table 2x
        let data = data.clone();

        let Ok(bucket) = self.table.get_mut(&store_ref) else {
            return Err(anyhow!("ObjectStore not found"));
        };

        // write the data to the bucket
        bucket
            .put(name.as_str(), &mut data.as_slice())
            .await
            .map_err(|e| anyhow!("issue writing object: {e}"))?;

        Ok(())
    }

    async fn list_objects(
        &mut self, store_ref: Resource<ObjectStore>,
    ) -> Result<Resource<StreamObjectNames>> {
        tracing::trace!("container::HostContainer::list_objects");

        let Ok(bucket) = self.table.get(&store_ref) else {
            return Err(anyhow!("ObjectStore not found"));
        };
        let mut list = bucket.list().await.map_err(|e| anyhow!("issue listing objects: {e}"))?;

        let mut names = vec![];
        while let Some(n) = list.next().await {
            match n {
                Ok(obj_info) => names.push(obj_info.name),
                Err(e) => tracing::warn!("issue listing object: {e}"),
            }
        }

        Ok(self.table.push(names)?)
    }

    async fn delete_object(
        &mut self, store_ref: Resource<ObjectStore>, name: String,
    ) -> Result<()> {
        tracing::trace!("container::HostContainer::delete_object");
        todo!()
    }

    async fn delete_objects(
        &mut self, store_ref: Resource<ObjectStore>, names: Vec<String>,
    ) -> Result<()> {
        tracing::trace!("container::HostContainer::delete_objects");
        todo!()
    }

    async fn has_object(&mut self, store_ref: Resource<ObjectStore>, name: String) -> Result<bool> {
        tracing::trace!("container::HostContainer::has_object");
        todo!()
    }

    async fn object_info(
        &mut self, store_ref: Resource<ObjectStore>, name: String,
    ) -> Result<ObjectMetadata> {
        tracing::trace!("container::HostContainer::object_info");
        todo!()
    }

    async fn clear(&mut self, store_ref: Resource<ObjectStore>) -> Result<()> {
        tracing::trace!("container::HostContainer::clear");
        todo!()
    }

    async fn drop(&mut self, store_ref: Resource<ObjectStore>) -> Result<()> {
        tracing::trace!("container::HostContainer::drop");
        // self.table.delete(rep)?;
        // Ok(())
        todo!()
    }
}

impl container::HostStreamObjectNames for Blobstore<'_> {
    async fn read_stream_object_names(
        &mut self, this: Resource<StreamObjectNames>, len: u64,
    ) -> Result<(Vec<String>, bool)> {
        tracing::trace!("container::HostStreamObjectNames::read_stream_object_names");
        todo!()
    }

    async fn skip_stream_object_names(
        &mut self, this: Resource<StreamObjectNames>, num: u64,
    ) -> Result<(u64, bool)> {
        tracing::trace!("container::HostStreamObjectNames::skip_stream_object_names");
        todo!()
    }

    async fn drop(&mut self, this: Resource<StreamObjectNames>) -> Result<()> {
        tracing::trace!("container::HostStreamObjectNames::drop");
        // self.table.delete(this)?;
        // Ok(())
        todo!()
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
        &mut self, this: Resource<IncomingValue>,
    ) -> Result<IncomingValueSyncBody> {
        tracing::trace!("types::HostIncomingValue::incoming_value_consume_sync");

        // let incoming_value = self.table.get(&this)?;
        // let body = types::IncomingValueSyncBody {
        //     value: incoming_value.value.clone(),
        // }

        todo!()
    }

    async fn incoming_value_consume_async(
        &mut self, this: Resource<IncomingValue>,
    ) -> Result<Resource<IncomingValueAsyncBody>> {
        tracing::trace!("types::HostIncomingValue::incoming_value_consume_async");

        // let incoming_value = self.table.get(&this)?;
        // let body = types::IncomingValueAsyncBody {
        //     value: incoming_value.value.clone(),
        // }

        todo!()
    }

    async fn size(&mut self, res: Resource<IncomingValue>) -> Result<u64> {
        tracing::trace!("types::HostIncomingValue::size");

        // let incoming_value = self.table.get(&res)?;
        // Ok(incoming_value.value.len() as u64)
        todo!()
    }

    async fn drop(&mut self, res: Resource<IncomingValue>) -> Result<()> {
        tracing::trace!("types::HostIncomingValue::drop");

        // self.table.delete(res)?;
        todo!()
    }
}

impl types::HostOutgoingValue for Blobstore<'_> {
    async fn new_outgoing_value(&mut self) -> Result<Resource<OutgoingValue>> {
        tracing::trace!("types::HostOutgoingValue::new_outgoing_value");
        todo!()
    }

    async fn outgoing_value_write_body(
        &mut self, this: Resource<OutgoingValue>,
    ) -> Result<Result<Resource<OutputStream>, ()>> {
        tracing::trace!("types::HostOutgoingValue::outgoing_value_write_body");
        todo!()
    }

    async fn finish(&mut self, this: Resource<OutgoingValue>) -> Result<()> {
        tracing::trace!("types::HostOutgoingValue::finish");

        // let outgoing_value = self.table.get(&this)?;
        // Ok(Ok(())) // No specific finish logic needed for this example

        todo!()
    }

    async fn drop(&mut self, rep: Resource<OutgoingValue>) -> Result<()> {
        tracing::trace!("types::HostOutgoingValue::drop");

        // self.table.delete(rep)?;
        // Ok(())

        todo!()
    }
}
