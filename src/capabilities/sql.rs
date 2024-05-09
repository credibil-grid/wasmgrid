//! # WASI SQL Capability
//!
//! This module implements a runtime capability for `wasi:sql`
//! (<https://github.com/WebAssembly/wasi-sql>).

use std::sync::OnceLock;

use anyhow::anyhow;
use bindings::wasi::sql::readwrite;
use bindings::wasi::sql::types::{self, DataType, Row};
use bindings::Sql;
use bson::Document;
use mongodb::options::{ClientOptions, ReplaceOptions};
use mongodb::{bson, Client};
use regex::Regex;
use wasmtime::component::{Linker, Resource};
use wasmtime_wasi::WasiView;

use crate::runtime::{self, Runtime, State};

mod bindings {
    #![allow(clippy::future_not_send)]

    pub use super::{Connection, Error, Statement};

    wasmtime::component::bindgen!({
        world: "sql",
        path: "wit",
        tracing: true,
        async: true,
        with: {
            "wasi:sql/types/connection": Connection,
            "wasi:sql/types/statement": Statement,
            "wasi:sql/types/error": Error,
        },
        // trappable_error_type: {
        //     "wasi:sql/sql-types/error" => Error,
        // },
    });
}

pub type Connection = mongodb::Database;
pub type Error = anyhow::Error;

static MONGODB: OnceLock<mongodb::Client> = OnceLock::new();

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
        &mut self, c: Resource<Connection>, s: Resource<Statement>,
    ) -> wasmtime::Result<Result<Vec<Row>, Resource<Error>>> {
        tracing::debug!("readwrite::Host::query");

        let table = self.table();
        let cnn = table.get(&c)?;
        let stmt = table.get(&s)?;

        let Some(doc) = cnn
            .collection::<Document>(&stmt.collection)
            .find_one(stmt.filter.clone(), None)
            .await?
        else {
            return Err(anyhow!("not found"));
        };

        let row = Row {
            field_name: String::from("document"),
            value: DataType::Binary(serde_json::to_vec(&doc)?),
        };

        Ok(Ok(vec![row]))
    }

    // TODO: implement update_configuration
    async fn exec(
        &mut self, c: Resource<Connection>, s: Resource<Statement>,
    ) -> wasmtime::Result<Result<u32, Resource<Error>>> {
        tracing::debug!("readwrite::Host::exec");

        let rt = self.table();
        let cnn = rt.get(&c)?;
        let stmt = rt.get(&s)?;

        let query = stmt.filter.clone().unwrap();
        let replacement = stmt.document.clone().unwrap();
        let options = Some(ReplaceOptions::builder().upsert(true).build());

        let coll = cnn.collection::<Document>(&stmt.collection);
        let _doc = coll.replace_one(query, &replacement, options);

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
        tracing::debug!("types::HostConnection::open");

        let client = MONGODB.get().ok_or_else(|| anyhow!("MongoDB not connected"))?;
        let cnn = client.database(&name);
        Ok(Ok(self.table().push(cnn)?))
    }

    fn drop(&mut self, rep: Resource<Connection>) -> wasmtime::Result<()> {
        tracing::debug!("types::HostConnection::drop");
        self.table().delete(rep)?;
        Ok(())
    }
}

// Implement the [`wasi::sql::HostStatement`]` trait for State.
#[async_trait::async_trait]
impl types::HostStatement for State {
    async fn prepare(
        &mut self, query: String, params: Vec<String>,
    ) -> wasmtime::Result<Result<Resource<Statement>, Resource<Error>>> {
        tracing::debug!("types::HostStatement::prepare");
        let stmt = Statement::parse(&query, &params)?;
        Ok(Ok(self.table().push(stmt)?))
    }

    fn drop(&mut self, rep: Resource<Statement>) -> wasmtime::Result<()> {
        tracing::debug!("types::HostStatement::drop");
        self.table().delete(rep)?;
        Ok(())
    }
}

// Implement the [`wasi::sql::HostError`]` trait for State.
#[async_trait::async_trait]
impl types::HostError for State {
    async fn trace(&mut self, rep: Resource<Error>) -> wasmtime::Result<String> {
        tracing::debug!("types::HostError::trace");
        let error = self.table().get(&rep)?;
        Ok(error.to_string())
    }

    fn drop(&mut self, rep: Resource<Error>) -> wasmtime::Result<()> {
        tracing::debug!("types::HostError::drop");
        self.table().delete(rep)?;
        Ok(())
    }
}

// // Statement holds a reference to the the NATS client. It is used to implement the
// [`wasi_sql::RuntimeStatement`] trait used by the sql host.
#[derive(Debug)]
pub struct Statement {
    collection: String,
    filter: Option<Document>,
    document: Option<Document>,
}

static QUERY_REGEX: OnceLock<Regex> = OnceLock::new();

impl Statement {
    // Parse the SQL query and return a Statement.
    fn parse(sql: &str, params: &[String]) -> anyhow::Result<Self> {
        tracing::trace!("Statement::parse");

        if params.is_empty() {
            return Err(anyhow!("invalid query: expected a parameter"));
        }

        // query or exec?
        if sql.starts_with("SELECT") {
            let re = QUERY_REGEX.get_or_init(|| {
                Regex::new(r"SELECT \* FROM (?<collection>\w+) WHERE (?<filter_col>\w+) = '?'")
                    .expect("regex should parse")
            });
            let Some(caps) = re.captures(sql) else {
                return Err(anyhow!("invalid query: query format should be: SELECT * FROM <collection> WHERE <filter_col> = '?'"));
            };

            // simple filter
            let filter = bson::doc! {&caps["filter_col"]: &params[0]};

            return Ok(Self {
                collection: String::from(&caps["collection"]),
                filter: Some(filter),
                document: None,
            });
        } else {
            let re = QUERY_REGEX.get_or_init(|| {
                Regex::new(r"UPDATE (?<collection>\w+) SET (?<update_col>\w+) = '\?' WHERE (?<filter_col>\w+) = '\?'")
                    .expect("regex should parse")
            });
            let Some(caps) = re.captures(sql) else {
                return Err(anyhow!("invalid query: query format should be: UPDATE <collection> SET <update_col> WHERE <filter_col> = '?'"));
            };

            let filter = bson::doc! {&caps["filter_col"]: &params[1]};
            let replacement: Document = serde_json::from_str(&params[0])?;

            return Ok(Self {
                collection: String::from(&caps["collection"]),
                filter: Some(filter),
                document: Some(replacement),
            });
        }
    }
}
