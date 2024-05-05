//! # WASI SQL Host

pub mod readwrite;
pub mod types;
use std::any::Any;

pub type Connection = Box<dyn RuntimeConnection>;
pub type Statement = Box<dyn RuntimeStatement>;
// pub type Error = Box<dyn RuntimeError>;

/// Wrap generation of wit bindings to simplify exports
pub mod bindings {
    #![allow(clippy::future_not_send)]

    pub use super::{Connection, Statement};

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

/// RuntimeConnection is implemented by the runtime to connect to a SQL database.
#[async_trait::async_trait]
pub trait RuntimeConnection: Sync + Send {
    fn as_any(&self) -> &dyn Any;
}

/// RuntimeStatement is implemented by the runtime to allow the runtime to prepare
/// implementation SQL statements.
#[async_trait::async_trait]
pub trait RuntimeStatement: Any + Sync + Send {
    fn as_any(&self) -> &dyn Any;
}

// /// RuntimeError is implemented by the runtime to allow the runtime to capture
// /// and trace errors.
// #[async_trait::async_trait]
// pub trait RuntimeError: Sync + Send {
//     fn as_any(&self) -> &dyn Any;
// }
