use wasmtime::component::Resource;

use crate::bindings::wasi::keyvalue::batch;
use crate::bindings::wasi::keyvalue::store::{self, Bucket};
use crate::KeyValueView;

#[async_trait::async_trait]
impl<T: KeyValueView> batch::Host for T {
    async fn get_many(
        &mut self, bucket: Resource<Bucket>, keys: Vec<String>,
    ) -> wasmtime::Result<Result<Vec<Option<(String, Vec<u8>)>>, store::Error>> {
        tracing::debug!("Host::get_many {keys:?}");
        let bucket = self.table().get_mut(&bucket)?;

        let res = bucket.get_many(keys).await?;
        let resp = res.into_iter().map(|(k, v)| Some((k, v))).collect::<Vec<_>>();

        Ok(Ok(resp))
    }

    async fn set_many(
        &mut self, bucket: Resource<Bucket>, key_values: Vec<(String, Vec<u8>)>,
    ) -> wasmtime::Result<Result<(), store::Error>> {
        tracing::debug!("Host::set_many {key_values:?}");
        let bucket = self.table().get_mut(&bucket)?;
        Ok(Ok(bucket.set_many(key_values).await?))
    }

    async fn delete_many(
        &mut self, bucket: Resource<Bucket>, keys: Vec<String>,
    ) -> wasmtime::Result<Result<(), store::Error>> {
        tracing::debug!("Host::delete_many {keys:?}");
        let bucket = self.table().get_mut(&bucket)?;
        Ok(Ok(bucket.delete_many(keys).await?))
    }
}
