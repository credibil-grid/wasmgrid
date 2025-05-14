//! # WASI JSON Database Service
//!
//! This module implements a runtime service for `wasi:sql`
//! (<https://github.com/WebAssembly/wasi-sql>).

/// Wrap generation of wit bindings to simplify exports.
/// See <https://docs.rs/wasmtime/latest/wasmtime/component/macro.bindgen.html>
mod generated {
    #![allow(clippy::trait_duplication_in_bounds)]
    pub use anyhow::Error;
    pub use mongodb::Database;

    pub use super::Statement;

    wasmtime::component::bindgen!({
        world: "jsondb",
        path: "../../wit",
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

use anyhow::{Error, Result, anyhow};
use futures::TryStreamExt;
use jmespath::ast::{Ast, Comparator};
use mongodb::bson::{self, Document};
use mongodb::{Client, Cursor, Database};
use runtime::Linkable;
use wasmtime::component::{Linker, Resource, ResourceTable};

use self::generated::wasi::jsondb;
use crate::Ctx;

pub struct Statement {
    collection: String,
    conditions: bson::Document,
}

pub struct JsonDbHost<'a> {
    client: &'a Client,
    table: &'a mut ResourceTable,
}

impl JsonDbHost<'_> {
    fn new(c: &mut Ctx) -> JsonDbHost<'_> {
        JsonDbHost {
            client: c.resources.mongo(),
            table: &mut c.table,
        }
    }
}

pub struct Service;

impl Linkable for Service {
    type Ctx = Ctx;

    // Add all the `wasi-keyvalue` world's interfaces to a [`Linker`], and
    // instantiate the `JsonDbHost` for the component.
    fn add_to_linker(&self, linker: &mut Linker<Self::Ctx>) -> anyhow::Result<()> {
        jsondb::readwrite::add_to_linker_get_host(linker, JsonDbHost::new)?;
        jsondb::types::add_to_linker_get_host(linker, JsonDbHost::new)?;
        tracing::trace!("added to linker");
        Ok(())
    }
}

// Implement the [`wasi_sql::ReadWriteView`]` trait for JsonDbHost<'_>.
impl jsondb::readwrite::Host for JsonDbHost<'_> {
    async fn insert(
        &mut self, db: Resource<Database>, s: Resource<Statement>, d: Vec<u8>,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::trace!("readwrite::Host::insert");

        let database = self.table.get(&db)?;
        let stmt = self.table.get(&s)?;

        let doc = match serde_json::from_slice::<bson::Document>(&d) {
            Ok(doc) => doc,
            Err(e) => {
                tracing::error!("issue deserializing document for insert: {e}");
                return Ok(Err(self.table.push(anyhow!("issue deserializing document: {e}"))?));
            }
        };

        if let Err(e) = database.collection(&stmt.collection).insert_one(doc).await {
            tracing::error!("issue inserting document: {e}");
            return Ok(Err(self.table.push(anyhow!("issue inserting document: {e}"))?));
        }

        Ok(Ok(()))
    }

    #[allow(clippy::cognitive_complexity)]
    async fn find(
        &mut self, db: Resource<Database>, s: Resource<Statement>,
    ) -> wasmtime::Result<Result<Vec<Vec<u8>>, Resource<Error>>> {
        tracing::trace!("readwrite::Host::find");

        let database = self.table.get(&db)?;
        let stmt = self.table.get(&s)?;
        tracing::trace!("readwrite::Host::find: {}, {:?}", stmt.collection, stmt.conditions);

        let mut results: Vec<Vec<u8>> = Vec::new();
        let mut cursor: Cursor<bson::Document> =
            match database.collection(&stmt.collection).find(stmt.conditions.clone()).await {
                Ok(cursor) => cursor,
                Err(e) => {
                    tracing::error!("issue finding documents: {e}");
                    return Ok(Err(self.table.push(anyhow!("issue finding documents: {e}"))?));
                }
            };
        while let Some(doc) = cursor.try_next().await? {
            let ser = match serde_json::to_vec(&doc) {
                Ok(ser) => ser,
                Err(e) => {
                    tracing::error!("issue serializing result: {e}");
                    return Ok(Err(self.table.push(anyhow!("issue serializing result: {e}"))?));
                }
            };
            results.push(ser);
        }

        Ok(Ok(results))
    }

    async fn update(
        &mut self, db: Resource<Database>, s: Resource<Statement>, d: Vec<u8>,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::trace!("readwrite::Host::update");

        let database = self.table.get(&db)?;
        let stmt = self.table.get(&s)?;

        let doc = match serde_json::from_slice::<bson::Document>(&d) {
            Ok(doc) => doc,
            Err(e) => {
                tracing::error!("issue deserializing replacement document: {e}");
                return Ok(Err(self
                    .table
                    .push(anyhow!("issue deserializing replacement document: {e}"))?));
            }
        };

        if let Err(e) =
            database.collection(&stmt.collection).replace_one(stmt.conditions.clone(), doc).await
        {
            tracing::error!("issue replacing document: {e}");
            return Ok(Err(self.table.push(anyhow!("issue replacing document: {e}"))?));
        }

        Ok(Ok(()))
    }

    async fn delete(
        &mut self, db: Resource<Database>, s: Resource<Statement>,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::trace!("readwrite::Host::delete");

        let database = self.table.get(&db)?;
        let stmt = self.table.get(&s)?;

        if let Err(e) = database
            .collection::<bson::Document>(&stmt.collection)
            .delete_one(stmt.conditions.clone())
            .await
        {
            tracing::error!("issue deleting document: {e}");
            return Ok(Err(self.table.push(anyhow!("issue deleting document: {e}"))?));
        }

        Ok(Ok(()))
    }
}

impl jsondb::types::Host for JsonDbHost<'_> {}

impl jsondb::types::HostDatabase for JsonDbHost<'_> {
    async fn connect(
        &mut self, name: String,
    ) -> wasmtime::Result<Result<Resource<Database>, Resource<Error>>> {
        tracing::trace!("HostDatabase::open");

        let db = self.client.database(&name);
        Ok(Ok(self.table.push(db)?))
    }

    async fn drop(&mut self, rep: Resource<Database>) -> wasmtime::Result<()> {
        tracing::trace!("HostDatabase::drop");
        self.table.delete(rep)?;
        Ok(())
    }
}

impl jsondb::types::HostStatement for JsonDbHost<'_> {
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
            //     return Ok(Err(self.table.push(anyhow!("invalid JMESPath projection"))?));
            // };
            // let Ast::Condition { predicate, .. } = rhs.as_ref() else {
            //     return Ok(Err(self.table.push(anyhow!("invalid JMESPath condition"))?));
            // };
            // let Ast::Comparison { lhs, rhs, .. } = predicate.as_ref() else {
            //     return Ok(Err(self.table.push(anyhow!("invalid JMESPath comparison"))?));
            // };
            // let Ast::Field { name, .. } = lhs.as_ref() else {
            //     return Ok(Err(self.table.push(anyhow!("invalid JMESPath LHS"))?));
            // };
            // let Ast::Literal { value, .. } = rhs.as_ref() else {
            //     return Ok(Err(self.table.push(anyhow!("invalid JMESPath RHS"))?));
            // };
            // let Some(value) = value.as_string() else {
            //     return Ok(Err(self.table.push(anyhow!("invalid JMESPath value"))?));
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

        Ok(Ok(self.table.push(query)?))
    }

    async fn drop(&mut self, rep: Resource<Statement>) -> wasmtime::Result<()> {
        tracing::trace!("HostFilter::drop");
        self.table.delete(rep)?;
        Ok(())
    }
}

impl jsondb::types::HostError for JsonDbHost<'_> {
    async fn trace(&mut self, rep: Resource<Error>) -> wasmtime::Result<String> {
        tracing::trace!("HostError::trace");
        let error = self.table.get(&rep)?;
        Ok(error.to_string())
    }

    async fn drop(&mut self, rep: Resource<Error>) -> wasmtime::Result<()> {
        tracing::trace!("HostError::drop");
        self.table.delete(rep)?;
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn prepare() {
//         let mut state = Ctx::default();
//         let _ = state
//             .prepare(
//                 "test".into(),
//                 Some("[?credential_issuer=='https://issuance.demo.credibil.io']".into()),
//             )
//             .await
//             .unwrap();
//     }
// }
