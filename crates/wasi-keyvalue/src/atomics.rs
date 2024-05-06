use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;

use crate::bindings::wasi::keyvalue::atomics::{self, Bucket};
use crate::bindings::wasi::keyvalue::store::Error;

/// AtomicsView is implemented by the runtime to support atomics in the runtime
/// keyvalue implementation.
#[allow(clippy::module_name_repetitions)]
#[async_trait::async_trait]
pub trait AtomicsView: WasiView + Send {
    async fn increment(
        &mut self, bucket: Resource<Bucket>, key: String, delta: u64,
    ) -> anyhow::Result<u64>;
}

#[async_trait::async_trait]
impl<T: AtomicsView> atomics::Host for T {
    async fn increment(
        &mut self, bucket: Resource<Bucket>, key: String, delta: u64,
    ) -> wasmtime::Result<Result<u64, Error>> {
        tracing::debug!("Host::increment {key}, {delta}");
        Ok(Ok(T::increment(self, bucket, key, delta).await?))
    }
}
