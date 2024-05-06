use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;

use crate::bindings::wasi::keyvalue::batch;
use crate::bindings::wasi::keyvalue::store::{self, Bucket};

/// AtomicsView is implemented by the runtime to support atomics in the runtime
/// keyvalue implementation.
#[async_trait::async_trait]
pub trait BatchView: WasiView + Send {
    async fn get_many(
        &mut self, bucket: Resource<Bucket>, keys: Vec<String>,
    ) -> anyhow::Result<Vec<Option<(String, Vec<u8>)>>>;

    async fn set_many(
        &mut self, bucket: Resource<Bucket>, key_values: Vec<(String, Vec<u8>)>,
    ) -> anyhow::Result<()>;

    async fn delete_many(
        &mut self, bucket: Resource<Bucket>, keys: Vec<String>,
    ) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl<T: BatchView> batch::Host for T {
    async fn get_many(
        &mut self, bucket: Resource<Bucket>, keys: Vec<String>,
    ) -> wasmtime::Result<Result<Vec<Option<(String, Vec<u8>)>>, store::Error>> {
        tracing::debug!("Host::get_many {keys:?}");
        Ok(Ok(T::get_many(self, bucket, keys).await?))
    }

    async fn set_many(
        &mut self, bucket: Resource<Bucket>, key_values: Vec<(String, Vec<u8>)>,
    ) -> wasmtime::Result<Result<(), store::Error>> {
        tracing::debug!("Host::set_many {key_values:?}");
        Ok(Ok(T::set_many(self, bucket, key_values).await?))
    }

    async fn delete_many(
        &mut self, bucket: Resource<Bucket>, keys: Vec<String>,
    ) -> wasmtime::Result<Result<(), store::Error>> {
        tracing::debug!("Host::delete_many {keys:?}");
        Ok(Ok(T::delete_many(self, bucket, keys).await?))
    }
}
