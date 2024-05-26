//! # WASI JSON Database Capability
//!
//! This module implements a runtime capability for `wasi:sql`
//! (<https://github.com/WebAssembly/wasi-sql>).

use std::sync::OnceLock;

use anyhow::anyhow;
use bindings::wasi::jsondb::readwrite;
use bindings::wasi::jsondb::types::{self, HostDatabase, HostError, HostStatement};
use bindings::Jsondb;
use jmespath::ast::Ast;
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
        world: "jsondb",
        path: "wit",
        tracing: true,
        async: true,
        trappable_imports: true,
        with: {
            "wasi:jsondb/types/database": Database,
            "wasi:jsondb/types/statement": Statement,
            "wasi:jsondb/types/error": Error,
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
        "wasi:jsondb"
    }

    fn add_to_linker(&self, linker: &mut Linker<State>) -> anyhow::Result<()> {
        Jsondb::add_to_linker(linker, |t| t)
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

        tracing::debug!("readwrite::Host::find: collection {:?}", stmt.conditions);

        let ser = if let Some(doc) = database
            .collection::<bson::Document>(&stmt.collection)
            .find_one(stmt.conditions.clone(), None)
            .await?
        {
            tracing::debug!("readwrite::Host::find: found document");
            serde_json::to_vec(&doc)?
        } else {
            tracing::debug!("readwrite::Host::find: no document found");
            vec![]
        };

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
    // Prepare a bson query for the specified collection, translating the JMESPath
    // to a bson query. For example,
    // [?credential_issuer=='https://issuance.demo.credibil.io'] will translate
    // to { "credential_issuer": "https://issuance.demo.credibil.io" }.

    async fn prepare(
        &mut self, collection: String, jmes_path: Option<String>,
    ) -> wasmtime::Result<Result<Resource<Statement>, Resource<Error>>> {
        tracing::debug!("HostFilter::prepare {collection} {jmes_path:?}");

        let doc = if let Some(jmes_path) = jmes_path {
            // create Mongo filter from JMESPath expression
            let expr = jmespath::compile(&jmes_path)?;
            let Ast::Projection { rhs, .. } = expr.as_ast() else {
                return Err(anyhow!("invalid JMESPath projecttion"));
            };
            let Ast::Condition { predicate, .. } = rhs.as_ref() else {
                return Err(anyhow!("invalid JMESPath condition"));
            };
            let Ast::Comparison { lhs, rhs, .. } = predicate.as_ref() else {
                return Err(anyhow!("invalid JMESPath comparison"));
            };
            let Ast::Field { name, .. } = lhs.as_ref() else {
                return Err(anyhow!("invalid JMESPath LHS"));
            };
            let Ast::Literal { value, .. } = rhs.as_ref() else {
                return Err(anyhow!("invalid JMESPath RHS"));
            };

            let value = value.as_string().ok_or_else(|| anyhow!("invalid JMESPath value"))?;

            tracing::debug!("HostFilter::prepare: key {name}: value {value}");

            Some(bson::doc! {
                name: value,
            })
        } else {
            None
        };

        let query = Statement {
            collection,
            conditions: doc,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn prepare() {
        let mut state = State::default();
        let _ = state
            .prepare(
                "test".to_string(),
                Some("[?credential_issuer=='https://issuance.demo.credibil.io']".to_string()),
            )
            .await
            .unwrap();
    }
}
