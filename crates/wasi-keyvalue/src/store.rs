use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;

use crate::bindings::wasi::keyvalue::store::{self, Bucket, KeyResponse};

/// KeyValueView is implemented by the keyvalue runtime to provide the host with
/// access to runtime-specific functionality.
#[allow(clippy::module_name_repetitions)]
#[async_trait::async_trait]
pub trait StoreView: WasiView + Send {
    async fn open(&mut self, identifier: String) -> anyhow::Result<Resource<Bucket>>;
}

#[async_trait::async_trait]
impl<T: StoreView + BucketView> store::Host for T {
    async fn open(
        &mut self, identifier: String,
    ) -> wasmtime::Result<Result<Resource<Bucket>, store::Error>> {
        tracing::debug!("Host::open {identifier}");
        Ok(Ok(T::open(self, identifier).await?))
    }
}

#[async_trait::async_trait]
pub trait BucketView: WasiView + Send {
    async fn get(
        &mut self, bucket: Resource<Bucket>, key: String,
    ) -> anyhow::Result<Option<Vec<u8>>>;

    async fn set(
        &mut self, bucket: Resource<Bucket>, key: String, value: Vec<u8>,
    ) -> anyhow::Result<()>;

    async fn delete(&mut self, bucket: Resource<Bucket>, key: String) -> anyhow::Result<()>;

    async fn exists(&mut self, bucket: Resource<Bucket>, key: String) -> anyhow::Result<bool>;

    /// List keys in the bucket.
    async fn list_keys(
        &mut self, bucket: Resource<Bucket>, cursor: Option<u64>,
    ) -> anyhow::Result<KeyResponse>;

    /// Drop the bucket and release any resources associated with it.
    ///
    /// # Errors
    fn drop(&mut self, bucket: Resource<Bucket>) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl<T: BucketView> store::HostBucket for T {
    async fn get(
        &mut self, bucket: Resource<Bucket>, key: String,
    ) -> wasmtime::Result<Result<Option<Vec<u8>>, store::Error>> {
        tracing::debug!("HostBucket::get {key}");
        Ok(Ok(T::get(self, bucket, key).await?))
    }

    async fn set(
        &mut self, bucket: Resource<Bucket>, key: String, value: Vec<u8>,
    ) -> wasmtime::Result<Result<(), store::Error>, wasmtime::Error> {
        tracing::debug!("HostBucket::set {key}");
        Ok(Ok(T::set(self, bucket, key, value).await?))
    }

    async fn delete(
        &mut self, bucket: Resource<Bucket>, key: String,
    ) -> Result<Result<(), store::Error>, wasmtime::Error> {
        tracing::debug!("HostBucket::delete {key}");
        Ok(Ok(T::delete(self, bucket, key).await?))
    }

    async fn exists(
        &mut self, bucket: Resource<Bucket>, key: String,
    ) -> wasmtime::Result<Result<bool, store::Error>> {
        tracing::debug!("HostBucket::exists {key}");
        Ok(Ok(T::exists(self, bucket, key).await?))
    }

    async fn list_keys(
        &mut self, bucket: Resource<Bucket>, cursor: Option<u64>,
    ) -> Result<Result<KeyResponse, store::Error>, wasmtime::Error> {
        tracing::debug!("HostBucket::list_keys {cursor:?}");
        Ok(Ok(T::list_keys(self, bucket, cursor).await?))
    }

    fn drop(&mut self, bucket: Resource<Bucket>) -> Result<(), wasmtime::Error> {
        tracing::debug!("HostBucket::close");
        T::drop(self, bucket)
    }
}
