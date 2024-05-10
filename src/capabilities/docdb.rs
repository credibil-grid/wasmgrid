//! # WASI Document Capability
//!
//! This module implements a runtime capability for `wasi:sql`
//! (<https://github.com/WebAssembly/wasi-sql>).

use std::sync::OnceLock;

use anyhow::anyhow;
use bindings::wasi::docdb::readwrite;
use bindings::wasi::docdb::types::{self, HostDatabase, HostError, HostQuery};
use bindings::Docdb;
// use bson::Document;
use mongodb::options::ClientOptions;
use mongodb::Client;
use wasmtime::component::{Linker, Resource};
use wasmtime_wasi::WasiView;

use crate::runtime::{self, Runtime, State};

mod bindings {
    #![allow(clippy::future_not_send)]

    pub use super::{Database, Error, Query};

    wasmtime::component::bindgen!({
        world: "docdb",
        path: "wit",
        tracing: true,
        async: true,
        with: {
            "wasi:docdb/types/database": Database,
            "wasi:docdb/types/query": Query,
            "wasi:docdb/types/error": Error,
        }
    });
}

pub type Database = mongodb::Database;
pub type Error = anyhow::Error;

static MONGODB: OnceLock<mongodb::Client> = OnceLock::new();

pub struct Query {
    collection: String,
    filter: Option<bson::Document>,
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
        "wasi:doc"
    }

    fn add_to_linker(&self, linker: &mut Linker<State>) -> anyhow::Result<()> {
        Docdb::add_to_linker(linker, |t| t)
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
    async fn find(
        &mut self, db: Resource<Database>, q: Resource<Query>,
    ) -> wasmtime::Result<Result<Vec<Vec<u8>>, Resource<Error>>> {
        tracing::debug!("readwrite::Host::query");

        let table = self.table();
        let database = table.get(&db)?;
        let query = table.get(&q)?;

        let Some(doc) = database
            .collection::<bson::Document>(&query.collection)
            .find_one(query.filter.clone(), None)
            .await?
        else {
            return Err(anyhow!("document not found"));
        };

        let ser = serde_json::to_vec(&doc)?;
        Ok(Ok(vec![ser]))
    }
}

impl types::Host for State {}

// Implement the [`HostDatabase`]` trait for State.
#[async_trait::async_trait]
impl HostDatabase for State {
    async fn connect(
        &mut self, name: String,
    ) -> wasmtime::Result<Result<Resource<Database>, Resource<Error>>> {
        tracing::debug!("HostDatabase::open");

        let client = MONGODB.get().ok_or_else(|| anyhow!("MongoDB not connected"))?;
        let db = client.database(&name);
        Ok(Ok(self.table().push(db)?))
    }

    fn drop(&mut self, rep: Resource<Database>) -> wasmtime::Result<()> {
        tracing::debug!("HostDatabase::drop");
        self.table().delete(rep)?;
        Ok(())
    }
}

// Implement the [`HostQuery`]` trait for State.
#[async_trait::async_trait]
impl HostQuery for State {
    async fn prepare(
        &mut self, collection: String, keyvalues: Vec<(String, String)>,
    ) -> wasmtime::Result<Result<Resource<Query>, Resource<Error>>> {
        tracing::debug!("HostFilter::prepare");

        let mut filter = bson::Document::new();
        for (k, v) in keyvalues {
            filter.insert(k, v);
        }
        let query = Query {
            collection,
            filter: Some(filter),
        };

        Ok(Ok(self.table().push(query)?))
    }

    fn drop(&mut self, rep: Resource<Query>) -> wasmtime::Result<()> {
        tracing::debug!("HostFilter::drop");
        self.table().delete(rep)?;
        Ok(())
    }
}

// Implement the [`wasi::sql::HostError`]` trait for State.
#[async_trait::async_trait]
impl HostError for State {
    async fn trace(&mut self, rep: Resource<Error>) -> wasmtime::Result<String> {
        tracing::debug!("HostError::trace");
        let error = self.table().get(&rep)?;
        Ok(error.to_string())
    }

    fn drop(&mut self, rep: Resource<Error>) -> wasmtime::Result<()> {
        tracing::debug!("HostError::drop");
        self.table().delete(rep)?;
        Ok(())
    }
}
