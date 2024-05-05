//! # WASI SQL Host

use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;

use crate::bindings::wasi::sql::readwrite;
use crate::bindings::wasi::sql::types::{self, Error, Row};

/// Wrap generation of wit bindings to simplify exports
pub mod bindings {
    #![allow(clippy::future_not_send)]

    pub use super::{Connection, Statement}; //Error,

    wasmtime::component::bindgen!({
        world: "sql",
        path: "wit",
        tracing: true,
        async: true,
        with: {
            "wasi:sql/types/connection": Connection,
            "wasi:sql/types/statement": Statement,
            // "wasi:sql/types/error": Error,
        },
        // trappable_error_type: {
        //     "wasi:sql/sql-types/error" => Error,
        // },
    });
}

/// SqlView is implemented by the sql runtime to provide the host with
/// access to runtime-specific functionality.
#[allow(clippy::module_name_repetitions)]
#[async_trait::async_trait]
pub trait SqlView: WasiView + Send {
    async fn query(
        &mut self, c: Resource<Connection>, q: Resource<Statement>,
    ) -> anyhow::Result<Vec<Row>>;

    async fn exec(
        &mut self, c: Resource<Connection>, q: Resource<Statement>,
    ) -> anyhow::Result<Vec<u32>>;

    async fn open(&mut self, name: String) -> anyhow::Result<Resource<Connection>>;

    fn drop_connection(&mut self, rep: Resource<Connection>) -> anyhow::Result<()>;

    async fn prepare(
        &mut self, query: String, params: Vec<String>,
    ) -> anyhow::Result<Resource<Statement>>;

    fn drop_statement(&mut self, rep: Resource<Statement>) -> anyhow::Result<()>;

    async fn trace(&mut self, self_: Resource<Error>) -> String;

    fn drop_error(&mut self, rep: Resource<Error>);
}

// Implement the [`signer::Host`]` trait for SignatureView impls.
#[async_trait::async_trait]
impl<T: SqlView> readwrite::Host for T {
    async fn query(
        &mut self, c: Resource<Connection>, q: Resource<Statement>,
    ) -> wasmtime::Result<Result<Vec<Row>, Resource<types::Error>>> {
        tracing::debug!("Host::sign");
        Ok(Ok(T::query(self, c, q).await?))
    }

    async fn exec(
        &mut self, _: Resource<Connection>, _: Resource<Statement>,
    ) -> wasmtime::Result<Result<u32, Resource<types::Error>>> {
        tracing::debug!("Host::suite");
        // T::suite(self).await

        todo!()
    }
}

impl<T: SqlView> types::Host for T {}

// Implement the [`types::Host`] trait for SqlView impls.
#[async_trait::async_trait]
impl<T: SqlView> types::HostConnection for T {
    async fn open(
        &mut self, _name: String,
    ) -> wasmtime::Result<Result<Resource<Connection>, Resource<Error>>> {
        tracing::debug!("Host::open");
        T::open(self, _name).await.map(|r| Ok(r))
    }

    fn drop(&mut self, rep: Resource<Connection>) -> wasmtime::Result<()> {
        T::drop_connection(self, rep)
    }
}

#[async_trait::async_trait]
impl<T: SqlView> types::HostStatement for T {
    async fn prepare(
        &mut self, query: String, params: Vec<String>,
    ) -> wasmtime::Result<Result<Resource<Statement>, Resource<Error>>> {
        let stmt = T::prepare(self, query, params);
        Ok(Ok(stmt.await?))
    }

    fn drop(&mut self, rep: Resource<Statement>) -> wasmtime::Result<()> {
        T::drop_statement(self, rep)
    }
}

#[async_trait::async_trait]
impl<T: SqlView> types::HostError for T {
    async fn trace(&mut self, _self_: Resource<Error>) -> wasmtime::Result<String> {
        todo!()
    }

    fn drop(&mut self, _rep: wasmtime::component::Resource<Error>) -> wasmtime::Result<()> {
        todo!()
    }
}

use std::any::Any;

pub type Connection = Box<dyn RuntimeConnection>;
pub type Statement = Box<dyn RuntimeStatement>;
// pub type Error = Box<dyn RuntimeError>;

/// RuntimeConnection is implemented by the runtime to connect to a SQL database.
#[async_trait::async_trait]
pub trait RuntimeConnection: Sync + Send {
    fn as_any(&self) -> &dyn Any;
}

/// RuntimeStatement is implemented by the runtime to allow the runtime to prepare
/// implementation SQL statements.
#[async_trait::async_trait]
pub trait RuntimeStatement: Sync + Send {
    fn as_any(&self) -> &dyn Any;
}

// /// RuntimeError is implemented by the runtime to allow the runtime to capture
// /// and trace errors.
// #[async_trait::async_trait]
// pub trait RuntimeError: Sync + Send {
//     /// Trace error.
//     async fn trace(&self) -> String;
// }
