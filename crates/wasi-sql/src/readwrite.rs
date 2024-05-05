use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;

use super::{Connection, Statement};
use crate::bindings::wasi::sql::readwrite;
use crate::bindings::wasi::sql::types::{Error, Row};

/// ReadWriteView is implemented by the runtime to support querying the runtime
/// database implementation.
#[async_trait::async_trait]
pub trait ReadWriteView: WasiView + Send {
    async fn query(
        &mut self, c: Resource<Connection>, q: Resource<Statement>,
    ) -> anyhow::Result<Vec<Row>>;

    async fn exec(
        &mut self, c: Resource<Connection>, q: Resource<Statement>,
    ) -> anyhow::Result<u32>;
}

/// Implement the [`readwrite::Host`]` trait for T (any concrete type that implements
/// ReadWriteView). Typically T is the runtime State object.
#[async_trait::async_trait]
impl<T: ReadWriteView> readwrite::Host for T {
    async fn query(
        &mut self, c: Resource<Connection>, q: Resource<Statement>,
    ) -> wasmtime::Result<Result<Vec<Row>, Resource<Error>>> {
        tracing::debug!("Host::sign");
        Ok(Ok(T::query(self, c, q).await?))
    }

    async fn exec(
        &mut self, c: Resource<Connection>, q: Resource<Statement>,
    ) -> wasmtime::Result<Result<u32, Resource<Error>>> {
        tracing::debug!("Host::suite");
        Ok(Ok(T::exec(self, c, q).await?))
    }
}
