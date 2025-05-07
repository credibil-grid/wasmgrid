//! # WASI JSON Database Capability
//!
//! This module implements a runtime capability for `wasi:sql`
//! (<https://github.com/WebAssembly/wasi-sql>).

/// Wrap generation of wit bindings to simplify exports.
/// See <https://docs.rs/wasmtime/latest/wasmtime/component/macro.bindgen.html>
mod generated {
    #![allow(clippy::future_not_send)]
    #![allow(clippy::trait_duplication_in_bounds)]
    use super::bindgen;
    pub use super::{Database, Error, Statement};

    bindgen!({
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

use std::sync::OnceLock;

use anyhow::anyhow;
use futures::TryStreamExt;
use jmespath::ast::{Ast, Comparator};
use mongodb::bson::{self, Document};
use mongodb::options::ClientOptions;
use mongodb::{Client, Cursor};
use wasmtime::component::{InstancePre, Linker, Resource, bindgen};
use wasmtime_wasi::IoView;

use self::generated::Jsondb;
use self::generated::wasi::jsondb::readwrite;
use self::generated::wasi::jsondb::types::{self, HostDatabase, HostError, HostStatement};
use crate::runtime::{self, Ctx};

static MONGODB: OnceLock<mongodb::Client> = OnceLock::new();

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
    fn namespace(&self) -> &'static str {
        "wasi:jsondb"
    }

    fn add_to_linker(&self, linker: &mut Linker<Ctx>) -> anyhow::Result<()> {
        Jsondb::add_to_linker(linker, |t| t)
    }

    /// Provide jsondb capability for the specified wasm component.
    async fn start(&self, _: InstancePre<Ctx>) -> anyhow::Result<()> {
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

// Implement the [`wasi_sql::ReadWriteView`]` trait for Ctx.
impl readwrite::Host for Ctx {
    async fn insert(
        &mut self, db: Resource<Database>, s: Resource<Statement>, d: Vec<u8>,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::trace!("readwrite::Host::insert");

        let table = self.table();
        let database = table.get(&db)?;
        let stmt = table.get(&s)?;

        let doc = match serde_json::from_slice::<bson::Document>(&d) {
            Ok(doc) => doc,
            Err(e) => {
                tracing::error!("issue deserializing document for insert: {e}");
                return Ok(Err(self.table().push(anyhow!("issue deserializing document: {e}"))?));
            }
        };

        if let Err(e) = database.collection(&stmt.collection).insert_one(doc).await {
            tracing::error!("issue inserting document: {e}");
            return Ok(Err(self.table().push(anyhow!("issue inserting document: {e}"))?));
        }

        Ok(Ok(()))
    }

    #[allow(clippy::cognitive_complexity)]
    async fn find(
        &mut self, db: Resource<Database>, s: Resource<Statement>,
    ) -> wasmtime::Result<Result<Vec<Vec<u8>>, Resource<Error>>> {
        tracing::trace!("readwrite::Host::find");

        let table = self.table();
        let database = table.get(&db)?;
        let stmt = table.get(&s)?;

        tracing::trace!("readwrite::Host::find: {}, {:?}", stmt.collection, stmt.conditions);

        let mut results: Vec<Vec<u8>> = Vec::new();
        let mut cursor: Cursor<bson::Document> =
            match database.collection(&stmt.collection).find(stmt.conditions.clone()).await {
                Ok(cursor) => cursor,
                Err(e) => {
                    tracing::error!("issue finding documents: {e}");
                    return Ok(Err(self.table().push(anyhow!("issue finding documents: {e}"))?));
                }
            };
        while let Some(doc) = cursor.try_next().await? {
            let ser = match serde_json::to_vec(&doc) {
                Ok(ser) => ser,
                Err(e) => {
                    tracing::error!("issue serializing result: {e}");
                    return Ok(Err(self.table().push(anyhow!("issue serializing result: {e}"))?));
                }
            };
            results.push(ser);
        }

        // let ser = if let Some::<bson::Document>(doc) =
        //     database.collection(&stmt.collection).find_one(stmt.conditions.clone()).await?
        // {
        //     tracing::debug!("readwrite::Host::find: document found");
        //     match serde_json::to_vec(&doc) {
        //         Ok(ser) => ser,
        //         Err(e) => {
        //             tracing::debug!("issue serializing result: {e}");
        //             return Ok(Err(self.table().push(anyhow!("issue serializing result: {e}"))?));
        //         }
        //     }
        // } else {
        //     tracing::debug!("readwrite::Host::find: no document found");
        //     vec![]
        // };

        Ok(Ok(results))
    }

    async fn update(
        &mut self, db: Resource<Database>, s: Resource<Statement>, d: Vec<u8>,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::trace!("readwrite::Host::update");

        let table = self.table();
        let database = table.get(&db)?;
        let stmt = table.get(&s)?;

        let doc = match serde_json::from_slice::<bson::Document>(&d) {
            Ok(doc) => doc,
            Err(e) => {
                tracing::error!("issue deserializing replacement document: {e}");
                return Ok(Err(self
                    .table()
                    .push(anyhow!("issue deserializing replacement document: {e}"))?));
            }
        };

        if let Err(e) =
            database.collection(&stmt.collection).replace_one(stmt.conditions.clone(), doc).await
        {
            tracing::error!("issue replacing document: {e}");
            return Ok(Err(self.table().push(anyhow!("issue replacing document: {e}"))?));
        }

        Ok(Ok(()))
    }

    async fn delete(
        &mut self, db: Resource<Database>, s: Resource<Statement>,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::trace!("readwrite::Host::delete");

        let table = self.table();
        let database = table.get(&db)?;
        let stmt = table.get(&s)?;

        if let Err(e) = database
            .collection::<bson::Document>(&stmt.collection)
            .delete_one(stmt.conditions.clone())
            .await
        {
            tracing::error!("issue deleting document: {e}");
            return Ok(Err(self.table().push(anyhow!("issue deleting document: {e}"))?));
        }

        Ok(Ok(()))
    }
}

impl types::Host for Ctx {}

// Implement the [`HostDatabase`]` trait for Ctx.
impl HostDatabase for Ctx {
    async fn connect(
        &mut self, name: String,
    ) -> wasmtime::Result<Result<Resource<Database>, Resource<Error>>> {
        tracing::trace!("HostDatabase::open");

        let Some(client) = MONGODB.get() else {
            return Ok(Err(self.table().push(anyhow!("MongoDB not connected"))?));
        };
        let db = client.database(&name);
        Ok(Ok(self.table().push(db)?))
    }

    async fn drop(&mut self, rep: Resource<Database>) -> wasmtime::Result<()> {
        tracing::trace!("HostDatabase::drop");
        self.table().delete(rep)?;
        Ok(())
    }
}

// Implement the [`HostStatement`]` trait for Ctx.
impl HostStatement for Ctx {
    // Prepare a bson query for the specified collection, translating the JMESPath
    // to a bson query. For example,
    // [?credential_issuer=='https://issuance.demo.credibil.io'] will translate
    // to { "credential_issuer": "https://issuance.demo.credibil.io" }.

    async fn prepare(
        &mut self, collection: String, jmes_path: Option<String>,
    ) -> wasmtime::Result<Result<Resource<Statement>, Resource<Error>>> {
        tracing::trace!("HostFilter::prepare {collection} {jmes_path:?}");

        let doc = if let Some(jmes_path) = jmes_path {
            // create Mongo filter from JMESPath expression
            let expr = jmespath::compile(&jmes_path)?;

            // let Ast::Projection { rhs, .. } = expr.as_ast() else {
            //     return Ok(Err(self.table().push(anyhow!("invalid JMESPath projection"))?));
            // };
            // let Ast::Condition { predicate, .. } = rhs.as_ref() else {
            //     return Ok(Err(self.table().push(anyhow!("invalid JMESPath condition"))?));
            // };
            // let Ast::Comparison { lhs, rhs, .. } = predicate.as_ref() else {
            //     return Ok(Err(self.table().push(anyhow!("invalid JMESPath comparison"))?));
            // };
            // let Ast::Field { name, .. } = lhs.as_ref() else {
            //     return Ok(Err(self.table().push(anyhow!("invalid JMESPath LHS"))?));
            // };
            // let Ast::Literal { value, .. } = rhs.as_ref() else {
            //     return Ok(Err(self.table().push(anyhow!("invalid JMESPath RHS"))?));
            // };
            // let Some(value) = value.as_string() else {
            //     return Ok(Err(self.table().push(anyhow!("invalid JMESPath value"))?));
            // };

            // tracing::debug!("HostFilter::prepare: key {name}: value {value}");
            // bson::doc! {name: value}

            let ast = expr.as_ast();
            process(ast)?
        } else {
            bson::doc! {}
        };

        let query = Statement {
            collection,
            conditions: doc,
        };

        Ok(Ok(self.table().push(query)?))
    }

    async fn drop(&mut self, rep: Resource<Statement>) -> wasmtime::Result<()> {
        tracing::trace!("HostFilter::drop");
        self.table().delete(rep)?;
        Ok(())
    }
}

// Parse part of a JMESPath abstract syntax tree into a bson query.
// TODO: this is incomplete. It only supports a subset of JMESPath and assumes
// every literal is a string.
fn process(ast: &Ast) -> anyhow::Result<Document> {
    match ast {
        Ast::Projection { rhs, .. } => process(rhs),
        Ast::Condition { predicate, .. } => process(predicate),
        Ast::And { lhs, rhs, .. } => {
            let lhs_doc = process(lhs)?;
            let rhs_doc = process(rhs)?;
            Ok(bson::doc! { "$and": [lhs_doc, rhs_doc] })
        }
        Ast::Comparison {
            lhs, rhs, comparator, ..
        } => {
            let lhs_str = process_string(lhs)?;
            let rhs_str = process_string(rhs)?;
            let doc = match comparator {
                Comparator::Equal => bson::doc! { lhs_str: { "$eq": rhs_str } },
                Comparator::NotEqual => bson::doc! { lhs_str: { "$ne": rhs_str } },
                Comparator::LessThan => bson::doc! { lhs_str: { "$lt": rhs_str } },
                Comparator::LessThanEqual => bson::doc! { lhs_str: { "$lte": rhs_str } },
                Comparator::GreaterThan => bson::doc! { lhs_str: { "$gt": rhs_str } },
                Comparator::GreaterThanEqual => bson::doc! { lhs_str: { "$gte": rhs_str } },
            };
            Ok(doc)
        }
        _ => Err(anyhow!("unsupported JMESPath node: {ast}")),
    }
}

// Parse part of a JMESPath abstract syntax tree into a bson query where the
// node is expected to be translatable to a string literal.
fn process_string(ast: &Ast) -> anyhow::Result<String> {
    match ast {
        Ast::Field { name, .. } => Ok(name.into()),
        Ast::Literal { value, .. } => {
            let value = value
                .as_string()
                .ok_or_else(|| anyhow!("JMESPath literal not convertable to string"))?;
            Ok(value.into())
        }
        _ => Err(anyhow!("unsupported JMESPath string node: {ast}")),
    }
}

// Implement the [`wasi::sql::HostError`]` trait for Ctx.
impl HostError for Ctx {
    async fn trace(&mut self, rep: Resource<Error>) -> wasmtime::Result<String> {
        tracing::trace!("HostError::trace");
        let error = self.table().get(&rep)?;
        Ok(error.to_string())
    }

    async fn drop(&mut self, rep: Resource<Error>) -> wasmtime::Result<()> {
        tracing::trace!("HostError::drop");
        self.table().delete(rep)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn prepare() {
        let mut state = Ctx::default();
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
