// use anyhow::Error;
use wasmtime::component::Resource;

use crate::bindings::wasi::keyvalue::atomics;
use crate::bindings::wasi::keyvalue::atomics::Bucket;
use crate::bindings::wasi::keyvalue::store::Error;
use crate::KeyValueView;

#[async_trait::async_trait]
impl<T: KeyValueView> atomics::Host for T {
    async fn increment(
        &mut self, bucket: Resource<Bucket>, key: String, delta: u64,
    ) -> wasmtime::Result<Result<u64, Error>> {
        todo!("implement increment")
    }
}
