//! # WASI SQL Capability
//!
//! This module implements a runtime capability for `wasi:sql`
//! (<https://github.com/WebAssembly/wasi-sql>).

use std::sync::{LazyLock, OnceLock};

use anyhow::anyhow;
use bindings::wasi::sql::readwrite;
use bindings::wasi::sql::types::{self, DataType, Error, Row};
use bindings::Sql;
use bson::Document;
use mongodb::options::ClientOptions;
pub use mongodb::{Client, Database as Connection};
use regex::Regex;
use wasmtime::component::{Linker, Resource};
use wasmtime_wasi::WasiView;

use crate::runtime::{self, Runtime, State};

static MONGODB: OnceLock<mongodb::Client> = OnceLock::new();

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

pub struct Capability {
    pub addr: String,
}

pub const fn new(addr: String) -> Capability {
    Capability { addr }
}

#[async_trait::async_trait]
impl runtime::Capability for Capability {
    fn namespace(&self) -> &str {
        "wasi:sql"
    }

    fn add_to_linker(&self, linker: &mut Linker<State>) -> anyhow::Result<()> {
        Sql::add_to_linker(linker, |t| t)
    }

    /// Provide sql capability for the specified wasm component.
    async fn run(&self, _: Runtime) -> anyhow::Result<()> {
        // Connect to MongoDB
        let mut client_options = ClientOptions::parse(&self.addr).await?;
        client_options.app_name = Some("Credibil Grid".to_string());
        let client = Client::with_options(client_options)?;
        MONGODB.get_or_init(|| client);

        tracing::info!("connected to MongoDB");
        Ok(())
    }
}

// Implement the [`wasi_sql::ReadWriteView`]` trait for State.
#[async_trait::async_trait]
impl readwrite::Host for State {
    async fn query(
        &mut self, c: Resource<bindings::Connection>, s: Resource<bindings::Statement>,
    ) -> wasmtime::Result<Result<Vec<Row>, Resource<Error>>> {
        tracing::debug!("readwrite::Host::query");

        let table = self.table();
        let cnn = table.get(&c)?;
        let stmt = table.get(&s)?;

        let Some(md) = cnn
            .collection::<Document>(&stmt.collection)
            .find_one(stmt.filter.clone(), None)
            .await?
        else {
            return Err(anyhow!("not found"));
        };

        let row = Row {
            field_name: String::from("issuer"),
            value: DataType::Binary(serde_json::to_vec(&md)?),
        };

        Ok(Ok(vec![row]))
    }

    // TODO: implement update_configuration
    async fn exec(
        &mut self, c: Resource<bindings::Connection>, s: Resource<bindings::Statement>,
    ) -> wasmtime::Result<Result<u32, Resource<Error>>> {
        tracing::debug!("readwrite::Host::exec");

        let rt = self.table();
        let cnn = rt.get(&c)?;
        let _stmt = rt.get(&s)?;

        let filter = mongodb::bson::doc! {};
        let md = cnn.collection::<Document>("issuer").find_one(Some(filter), None).await?;
        tracing::debug!("md: {:?}", md);

        Ok(Ok(0))
    }
}

impl types::Host for State {}

// Implement the [`wasi::sql::HostConnection`]` trait for State.
#[async_trait::async_trait]
impl types::HostConnection for State {
    async fn open(
        &mut self, name: String,
    ) -> wasmtime::Result<Result<Resource<Connection>, Resource<Error>>> {
        tracing::debug!("HostConnection::open");

        let client = MONGODB.get().ok_or_else(|| anyhow!("MongoDB not connected"))?;
        let cnn = client.database(&name);
        Ok(Ok(self.table().push(cnn)?))
    }

    fn drop(&mut self, rep: Resource<bindings::Connection>) -> wasmtime::Result<()> {
        tracing::debug!("HostConnection::drop");
        self.table().delete(rep).map_or_else(|e| Err(anyhow!(e)), |_| Ok(()))
    }
}

// Implement the [`wasi::sql::HostStatement`]` trait for State.
#[async_trait::async_trait]
impl types::HostStatement for State {
    async fn prepare(
        &mut self, query: String, params: Vec<String>,
    ) -> wasmtime::Result<Result<Resource<bindings::Statement>, Resource<Error>>> {
        tracing::debug!("HostStatement::prepare");
        let stmt = Statement::parse(&query, &params)?;
        Ok(Ok(self.table().push(stmt)?))
    }

    fn drop(&mut self, rep: Resource<bindings::Statement>) -> wasmtime::Result<()> {
        tracing::debug!("HostStatement::drop");
        self.table().delete(rep).map_or_else(|e| Err(anyhow!(e)), |_| Ok(()))
    }
}

// Implement the [`wasi::sql::HostError`]` trait for State.
#[async_trait::async_trait]
impl types::HostError for State {
    async fn trace(&mut self, _self_: Resource<Error>) -> wasmtime::Result<String> {
        tracing::debug!("HostError::trace");
        todo!()
    }

    fn drop(&mut self, _rep: Resource<Error>) -> wasmtime::Result<()> {
        tracing::debug!("HostError::drop");
        todo!()
    }
}

// // Statement holds a reference to the the NATS client. It is used to implement the
// [`wasi_sql::RuntimeStatement`] trait used by the sql host.
#[derive(Debug)]
pub struct Statement {
    collection: String,
    filter: Option<Document>,
}

static SQL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"SELECT \* FROM (?<table>\w+) WHERE (?<field>\w+) = '?'")
        .expect("regex should parse")
});

impl Statement {
    // Parse the SQL query and return a Statement.
    fn parse(sql: &str, params: &[String]) -> anyhow::Result<Self> {
        tracing::trace!("Statement::parse");

        let Some(caps) = SQL_REGEX.captures(sql) else {
            return Err(anyhow!("invalid query: cannot parse {sql}"));
        };

        if params.is_empty() {
            return Err(anyhow!("invalid query: expected a parameter"));
        }

        // build simple filter
        let filter = Some(mongodb::bson::doc! {&caps["field"]: &params[0]});

        Ok(Self {
            collection: String::from(&caps["table"]),
            filter,
        })
    }
}
