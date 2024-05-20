//! # WASI Document Capability
//!
//! This module implements a runtime capability for `wasi:sql`
//! (<https://github.com/WebAssembly/wasi-sql>).

use std::sync::OnceLock;

use anyhow::anyhow;
use bindings::wasi::docdb::readwrite;
use bindings::wasi::docdb::types::{self, HostDatabase, HostError, HostStatement};
use bindings::Docdb;
use mongodb::options::{ClientOptions, ReplaceOptions};
use mongodb::Client;
use wasmtime::component::{Linker, Resource};
use wasmtime_wasi::WasiView;

use crate::runtime::{self, Runtime, State};

static MONGODB: OnceLock<mongodb::Client> = OnceLock::new();

mod bindings {
    #![allow(clippy::future_not_send)]

    pub use super::{Database, Error, Statement};

    wasmtime::component::bindgen!({
        world: "docdb",
        path: "wit",
        tracing: true,
        async: true,
        with: {
            "wasi:docdb/types/database": Database,
            "wasi:docdb/types/statement": Statement,
            "wasi:docdb/types/error": Error,
        }
    });
}

pub type Database = mongodb::Database;
pub type Error = anyhow::Error;

pub struct Statement {
    collection: String,
    conditions: Option<bson::Document>,
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
        "wasi:docdb"
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
    async fn insert(
        &mut self, db: Resource<Database>, s: Resource<Statement>, d: Vec<u8>,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::debug!("readwrite::Host::insert");

        let table = self.table();
        let database = table.get(&db)?;
        let stmt = table.get(&s)?;

        let doc: bson::Document = serde_json::from_slice(&d)?;
        let _ = database.collection(&stmt.collection).insert_one(doc, None).await?;

        Ok(Ok(()))
    }

    async fn find(
        &mut self, db: Resource<Database>, s: Resource<Statement>,
    ) -> wasmtime::Result<Result<Vec<Vec<u8>>, Resource<Error>>> {
        tracing::debug!("readwrite::Host::find");

        let table = self.table();
        let database = table.get(&db)?;
        let stmt = table.get(&s)?;

        let Some(doc) = database
            .collection::<bson::Document>(&stmt.collection)
            .find_one(stmt.conditions.clone(), None)
            .await?
        else {
            return Err(anyhow!("document not found"));
        };

        let ser = serde_json::to_vec(&doc)?;
        Ok(Ok(vec![ser]))
    }

    async fn update(
        &mut self, db: Resource<Database>, s: Resource<Statement>, d: Vec<u8>,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::debug!("readwrite::Host::update");

        let table = self.table();
        let database = table.get(&db)?;
        let stmt = table.get(&s)?;

        let doc: bson::Document = serde_json::from_slice(&d)?;
        let Some(query) = stmt.conditions.clone() else {
            return Err(anyhow!("filter not found"));
        };
        let options = ReplaceOptions::builder().upsert(true).build();
        let _ = database.collection(&stmt.collection).replace_one(query, doc, options).await?;

        Ok(Ok(()))
    }

    async fn delete(
        &mut self, db: Resource<Database>, s: Resource<Statement>,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::debug!("readwrite::Host::delete");

        let table = self.table();
        let database = table.get(&db)?;
        let stmt = table.get(&s)?;

        let Some(query) = stmt.conditions.clone() else {
            return Err(anyhow!("filter not found"));
        };
        let _ =
            database.collection::<bson::Document>(&stmt.collection).delete_one(query, None).await?;

        Ok(Ok(()))
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

// Implement the [`HostStatement`]` trait for State.
#[async_trait::async_trait]
impl HostStatement for State {
    async fn prepare(
        &mut self, collection: String, conditions: Vec<(String, String)>,
    ) -> wasmtime::Result<Result<Resource<Statement>, Resource<Error>>> {
        tracing::debug!("HostFilter::prepare");

        let mut doc = bson::Document::new();
        for (k, v) in conditions {
            doc.insert(k, v);
        }
        let query = Statement {
            collection,
            conditions: Some(doc),
        };

        Ok(Ok(self.table().push(query)?))
    }

    fn drop(&mut self, rep: Resource<Statement>) -> wasmtime::Result<()> {
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
