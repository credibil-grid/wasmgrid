// use anyhow::Error;
use wasmtime::component::Resource;

use crate::bindings::wasi::keyvalue::store::{self, Bucket, KeyResponse};
use crate::KeyValueView;

#[async_trait::async_trait]
impl<T: KeyValueView> store::Host for T {
    async fn open(
        &mut self, identifier: String,
    ) -> wasmtime::Result<Result<Resource<Bucket>, store::Error>> {
        todo!("implement open")
    }
}

#[async_trait::async_trait]
impl<T: KeyValueView> store::HostBucket for T {
    async fn get(
        &mut self, bucket: Resource<Bucket>, key: String,
    ) -> wasmtime::Result<Result<Option<Vec<u8>>, store::Error>> {
        todo!("implement open")
    }

    async fn set(
        &mut self, bucket: Resource<Bucket>, key: String, value: Vec<u8>,
    ) -> wasmtime::Result<Result<(), store::Error>, wasmtime::Error> {
        todo!()
    }

    async fn delete(
        &mut self, bucket: Resource<Bucket>, key: String,
    ) -> Result<Result<(), store::Error>, wasmtime::Error> {
        todo!()
    }

    async fn exists(
        &mut self, bucket: Resource<Bucket>, key: String,
    ) -> wasmtime::Result<Result<bool, store::Error>> {
        todo!("implement open")
    }

    async fn list_keys(
        &mut self, _: Resource<Bucket>, _: Option<u64>,
    ) -> Result<Result<KeyResponse, store::Error>, wasmtime::Error> {
        todo!()
    }

    fn drop(&mut self, bucket: Resource<Bucket>) -> Result<(), wasmtime::Error> {
        todo!()
    }
}
