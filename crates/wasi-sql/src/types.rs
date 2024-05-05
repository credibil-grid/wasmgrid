//! # WASI SQL Host

use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;

use super::{Connection, Statement};
use crate::bindings::wasi::sql::types::{self, Error};

/// Implemented by the runtime to support aggregating linking of all types.
impl<T: ConnectionView + StatementView + ErrorView> types::Host for T {}

/// ConnectionView is implemented by the runtime to support connecting to the
/// runtime database.
#[async_trait::async_trait]
pub trait ConnectionView: WasiView + Send {
    async fn open(&mut self, name: String) -> anyhow::Result<Resource<Connection>>;

    /// Drop the connection.
    /// 
    /// # Errors
    fn drop(&mut self, rep: Resource<Connection>) -> anyhow::Result<()>;
}

/// Implement the [`types::HostConnection`]` trait for T (any concrete type that
/// implements ConnectionView). Typically T is the runtime State object.
#[async_trait::async_trait]
impl<T: ConnectionView> types::HostConnection for T {
    async fn open(
        &mut self, _name: String,
    ) -> wasmtime::Result<Result<Resource<Connection>, Resource<Error>>> {
        tracing::debug!("Host::open");
        T::open(self, _name).await.map(Ok)
    }

    fn drop(&mut self, rep: Resource<Connection>) -> wasmtime::Result<()> {
        T::drop(self, rep)
    }
}

/// StatementView is implemented by the runtime to support preparing statements for
/// the runtime database.
#[async_trait::async_trait]
pub trait StatementView: WasiView + Send {
    async fn prepare(
        &mut self, query: String, params: Vec<String>,
    ) -> anyhow::Result<Resource<Statement>>;

    /// Drop the statement.
    ///
    /// # Errors
    fn drop(&mut self, rep: Resource<Statement>) -> anyhow::Result<()>;
}

/// Implement the [`types::HostStatement`]` trait for T (any concrete type that
/// implements StatementView). Typically T is the runtime State object.
#[async_trait::async_trait]
impl<T: StatementView> types::HostStatement for T {
    async fn prepare(
        &mut self, query: String, params: Vec<String>,
    ) -> wasmtime::Result<Result<Resource<Statement>, Resource<Error>>> {
        let stmt = T::prepare(self, query, params);
        Ok(Ok(stmt.await?))
    }

    fn drop(&mut self, rep: Resource<Statement>) -> wasmtime::Result<()> {
        T::drop(self, rep)
    }
}

/// ErrorView is implemented by the runtime to support processing errors for
/// the runtime database.
#[async_trait::async_trait]
pub trait ErrorView: WasiView + Send {
    async fn trace(&mut self, self_: Resource<Error>) -> String;

    fn drop(&mut self, rep: Resource<Error>);
}

/// Implement the [`types::HostError`]` trait for T (any concrete type that
/// implements ErrorView). Typically T is the runtime State object.
#[async_trait::async_trait]
impl<T: ErrorView> types::HostError for T {
    async fn trace(&mut self, _self_: Resource<Error>) -> wasmtime::Result<String> {
        todo!()
    }

    fn drop(&mut self, _rep: wasmtime::component::Resource<Error>) -> wasmtime::Result<()> {
        todo!()
    }
}
