use wasmtime::component::Resource;

use crate::bindings::wasi::keyvalue::store::{self, Bucket, KeyResponse};
use crate::KeyValueView;

#[async_trait::async_trait]
impl<T: KeyValueView> store::Host for T {
    async fn open(
        &mut self, identifier: String,
    ) -> wasmtime::Result<Result<Resource<Bucket>, store::Error>> {
        tracing::debug!("Host::open {identifier}");
        Ok(Ok(T::open(self, identifier).await?))
    }
}

#[async_trait::async_trait]
impl<T: KeyValueView> store::HostBucket for T {
    async fn get(
        &mut self, bucket: Resource<Bucket>, key: String,
    ) -> wasmtime::Result<Result<Option<Vec<u8>>, store::Error>> {
        tracing::debug!("HostBucket::get {key}");
        let bucket = self.table().get_mut(&bucket)?;
        Ok(Ok(bucket.get(key).await?))
    }

    async fn set(
        &mut self, bucket: Resource<Bucket>, key: String, value: Vec<u8>,
    ) -> wasmtime::Result<Result<(), store::Error>, wasmtime::Error> {
        tracing::debug!("HostBucket::set {key}");
        let bucket = self.table().get_mut(&bucket)?;
        Ok(Ok(bucket.set(key, value).await?))
    }

    async fn delete(
        &mut self, bucket: Resource<Bucket>, key: String,
    ) -> Result<Result<(), store::Error>, wasmtime::Error> {
        tracing::debug!("HostBucket::delete {key}");
        let bucket = self.table().get_mut(&bucket)?;
        Ok(Ok(bucket.delete(key).await?))
    }

    async fn exists(
        &mut self, bucket: Resource<Bucket>, key: String,
    ) -> wasmtime::Result<Result<bool, store::Error>> {
        tracing::debug!("HostBucket::exists {key}");
        let bucket = self.table().get_mut(&bucket)?;
        Ok(Ok(bucket.exists(key).await?))
    }

    async fn list_keys(
        &mut self, bucket: Resource<Bucket>, cursor: Option<u64>,
    ) -> Result<Result<KeyResponse, store::Error>, wasmtime::Error> {
        tracing::debug!("HostBucket::list_keys {cursor:?}");
        let bucket = self.table().get_mut(&bucket)?;
        Ok(Ok(bucket.list_keys(cursor).await?))
    }

    fn drop(&mut self, bucket: Resource<Bucket>) -> Result<(), wasmtime::Error> {
        tracing::debug!("HostBucket::close");
        let b = self.table().get_mut(&bucket)?;
        b.close()?;
        Ok(self.table().delete(bucket).map(|_| ())?)
    }
}
