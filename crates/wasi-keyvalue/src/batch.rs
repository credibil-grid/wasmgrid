// use anyhow::Error;
use wasmtime::component::Resource;

use crate::bindings::wasi::keyvalue::batch;
use crate::bindings::wasi::keyvalue::store::{self, Bucket};
use crate::KeyValueView;

#[async_trait::async_trait]
impl<T: KeyValueView> batch::Host for T {
    async fn get_many(
        &mut self, bucket: Resource<Bucket>, keys: Vec<String>,
    ) -> Result<Result<Vec<Option<(String, Vec<u8>)>>, store::Error>, wasmtime::Error> {
        todo!()
    }

    async fn set_many(
        &mut self, _: Resource<Bucket>, _: Vec<(String, Vec<u8>)>,
    ) -> Result<Result<(), store::Error>, wasmtime::Error> {
        todo!()
    }

    async fn delete_many(
        &mut self, bucket: Resource<Bucket>, keys: Vec<String>,
    ) -> Result<Result<(), store::Error>, wasmtime::Error> {
        todo!()
    }
}
