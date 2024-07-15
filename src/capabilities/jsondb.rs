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
use mongodb::options::ClientOptions;
use mongodb::{bson, Client};
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
    conditions: bson::Document,
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

    /// Provide jsondb capability for the specified wasm component.
    async fn run(&self, _: Runtime) -> anyhow::Result<()> {
        let mut opts = ClientOptions::parse(&self.addr).await?;
        opts.app_name = Some("Credibil Grid".into());
        let client = Client::with_options(opts)?;

        // redact password from connection string
        let mut redacted = url::Url::parse(&self.addr).unwrap();
        redacted.set_password(Some("*****")).map_err(|()| anyhow!("issue redacting password"))?;
        tracing::info!("connected to: {redacted}");

        MONGODB.set(client).map_err(|_| anyhow!("MongoDB already initialized"))
    }
}

// Implement the [`wasi_sql::ReadWriteView`]` trait for State.
#[allow(dependency_on_unit_never_type_fallback)]
#[async_trait::async_trait]
impl readwrite::Host for State {
    async fn insert(
        &mut self, db: Resource<Database>, s: Resource<Statement>, d: Vec<u8>,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::debug!("readwrite::Host::insert");

        let table = self.table();
        let database = table.get(&db)?;
        let stmt = table.get(&s)?;

        let doc = match serde_json::from_slice::<bson::Document>(&d) {
            Ok(doc) => doc,
            Err(e) => {
                tracing::debug!("issue deserializing document for insert: {e}");
                return Ok(Err(self.table().push(anyhow!("issue deserializing document: {e}"))?));
            }
        };

        if let Err(e) = database.collection(&stmt.collection).insert_one(doc).await {
            tracing::debug!("issue inserting document: {e}");
            return Ok(Err(self.table().push(anyhow!("issue inserting document: {e}"))?));
        }

        Ok(Ok(()))
    }

    async fn find(
        &mut self, db: Resource<Database>, s: Resource<Statement>,
    ) -> wasmtime::Result<Result<Vec<Vec<u8>>, Resource<Error>>> {
        tracing::debug!("readwrite::Host::find");

        let table = self.table();
        let database = table.get(&db)?;
        let stmt = table.get(&s)?;

        tracing::debug!("readwrite::Host::find: {}, {:?}", stmt.collection, stmt.conditions);

        let ser = if let Some::<bson::Document>(doc) =
            database.collection(&stmt.collection).find_one(stmt.conditions.clone()).await?
        {
            tracing::debug!("readwrite::Host::find: document found");
            match serde_json::to_vec(&doc) {
                Ok(ser) => ser,
                Err(e) => {
                    tracing::debug!("issue serializing result: {e}");
                    return Ok(Err(self.table().push(anyhow!("issue serializing result: {e}"))?));
                }
            }
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

        let doc = match serde_json::from_slice::<bson::Document>(&d) {
            Ok(doc) => doc,
            Err(e) => {
                tracing::debug!("issue deserializing replacement document: {e}");
                return Ok(Err(self
                    .table()
                    .push(anyhow!("issue deserializing replacement document: {e}"))?));
            }
        };

        if let Err(e) =
            database.collection(&stmt.collection).replace_one(stmt.conditions.clone(), doc).await
        {
            tracing::debug!("issue replacing document: {e}");
            return Ok(Err(self.table().push(anyhow!("issue replacing document: {e}"))?));
        }

        Ok(Ok(()))
    }

    async fn delete(
        &mut self, db: Resource<Database>, s: Resource<Statement>,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::debug!("readwrite::Host::delete");

        let table = self.table();
        let database = table.get(&db)?;
        let stmt = table.get(&s)?;

        if let Err(e) = database
            .collection::<bson::Document>(&stmt.collection)
            .delete_one(stmt.conditions.clone())
            .await
        {
            tracing::debug!("issue deleting document: {e}");
            return Ok(Err(self.table().push(anyhow!("issue deleting document: {e}"))?));
        }

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

        let Some(client) = MONGODB.get() else {
            return Ok(Err(self.table().push(anyhow!("MongoDB not connected"))?));
        };
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
                return Ok(Err(self.table().push(anyhow!("invalid JMESPath projection"))?));
            };
            let Ast::Condition { predicate, .. } = rhs.as_ref() else {
                return Ok(Err(self.table().push(anyhow!("invalid JMESPath condition"))?));
            };
            let Ast::Comparison { lhs, rhs, .. } = predicate.as_ref() else {
                return Ok(Err(self.table().push(anyhow!("invalid JMESPath comparison"))?));
            };
            let Ast::Field { name, .. } = lhs.as_ref() else {
                return Ok(Err(self.table().push(anyhow!("invalid JMESPath LHS"))?));
            };
            let Ast::Literal { value, .. } = rhs.as_ref() else {
                return Ok(Err(self.table().push(anyhow!("invalid JMESPath RHS"))?));
            };
            let Some(value) = value.as_string() else {
                return Ok(Err(self.table().push(anyhow!("invalid JMESPath value"))?));
            };

            tracing::debug!("HostFilter::prepare: key {name}: value {value}");
            bson::doc! {name: value}
        } else {
            bson::doc! {}
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
                "test".into(),
                Some("[?credential_issuer=='https://issuance.demo.credibil.io']".into()),
            )
            .await
            .unwrap();
    }

    // #[test]
    // fn redact() {
    //     let addr = "mongodb+srv://wasmgrid:A.Passw0rd!@cluster0.uqnlxl8.mongodb.net/";
    //     // let re = Regex::new(r#"^mongodb(?:\+srv)?:\/\/(?:.+):(?<password>.+)@(?:.+)$"#).unwrap();
    //     // let Some(caps) = re.captures(addr) else {
    //     //     println!("no match!");
    //     //     return;
    //     // };
    //     // let redacted = addr.replace(&caps["password"], "*****");
    //     let mut u = url::Url::parse(addr).unwrap();
    //     u.set_password(Some("*****"));
    //     println!("The name is: {}", u.to_string());
    // }
}
