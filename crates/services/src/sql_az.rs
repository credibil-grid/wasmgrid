//! # WASI SQL Service for Azure Data Tables
//!
//! This module implements a runtime service for `wasi:sql`
//! (<https://github.com/WebAssembly/wasi-sql>).

/// Wrap generation of wit bindings to simplify exports.
/// See <https://docs.rs/wasmtime/latest/wasmtime/component/macro.bindgen.html>
mod generated {
    #![allow(clippy::trait_duplication_in_bounds)]

    pub use super::{Connection, Statement};

    wasmtime::component::bindgen!({
        world: "sql",
        path: "../../wit",
        tracing: true,
        async: true,
        trappable_imports: true,
        with: {
            "wasi:sql/types/connection": Connection,
            "wasi:sql/types/statement": Statement,
            "wasi:sql/types/error": anyhow::Error,
        },
        trappable_error_type: {
            "wasi:sql/types/error" => anyhow::Error,
        },
    });
}

use anyhow::{Result, anyhow};
use bson::Document;
use mongodb::{Collection, bson};
use runtime::Linkable;
use wasmtime::component::{HasData, Linker, Resource, ResourceTable};

use self::generated::wasi::sql::types::{Error, Row};
use self::generated::wasi::sql::{readwrite, types};
use crate::{Ctx, Resources};

pub type Connection = Collection<Document>;
pub type Statement = Document;

pub struct Sql<'a> {
    resources: &'a Resources,
    table: &'a mut ResourceTable,
}

impl Sql<'_> {
    const fn new(c: &mut Ctx) -> Sql<'_> {
        Sql {
            resources: &c.resources,
            table: &mut c.table,
        }
    }
}

struct Data;
impl HasData for Data {
    type Data<'a> = Sql<'a>;
}

pub struct Service;

impl Linkable for Service {
    type Ctx = Ctx;

    // Add all the `wasi-sql` world's interfaces to a [`Linker`], and
    // instantiate the `Sql` for the component.
    fn add_to_linker(&self, l: &mut Linker<Self::Ctx>) -> anyhow::Result<()> {
        readwrite::add_to_linker::<_, Data>(l, Sql::new)?;
        types::add_to_linker::<_, Data>(l, Sql::new)
    }
}

impl readwrite::Host for Sql<'_> {
    async fn query(
        &mut self, _c: Resource<Connection>, _s: Resource<Statement>,
    ) -> Result<Result<Vec<Row>, Resource<Error>>> {
        // // let table = self.table;
        // let cnn = self.table.get(&c)?;
        // let stmt = self.table.get(&s)?;

        // let Some(doc) = cnn
        //     .collection::<Document>(&stmt.collection)
        //     .find_one(stmt.filter.clone(), None)
        //     .await?
        // else {
        //     return Err(anyhow!("not found"));
        // };
        // let row = Row {
        //     field_name: String::from("document"),
        //     value: DataType::Binary(serde_json::to_vec(&doc)?),
        // };
        // Ok(Ok(vec![row]))

        todo!()
    }

    async fn exec(
        &mut self, _c: Resource<Connection>, _s: Resource<Statement>,
    ) -> Result<Result<u32, Resource<Error>>> {
        // let cnn = self.table.get(&c)?;
        // let stmt = self.table.get(&s)?;

        // let query = stmt.filter.clone().unwrap();
        // let replacement = stmt.document.clone().unwrap();
        // let options = Some(ReplaceOptions::builder().upsert(true).build());

        // let coll = cnn.collection::<Document>(&stmt.collection);
        // let _doc = coll.replace_one(query, &replacement, options);

        // Ok(Ok(0))

        todo!()
    }
}

impl types::Host for Sql<'_> {
    fn convert_error(&mut self, err: anyhow::Error) -> Result<Error> {
        tracing::error!("{err}");
        Ok(err)
    }
}

impl types::HostConnection for Sql<'_> {
    async fn open(
        &mut self, name: String,
    ) -> Result<Result<Resource<Connection>, Resource<Error>>> {
        let Some(db) = self.resources.mongo()?.default_database() else {
            return Err(anyhow!("No default database found"));
        };
        let collection = db.collection::<Document>(&name);
        Ok(Ok(self.table.push(collection)?))
    }

    async fn drop(&mut self, rep: Resource<Connection>) -> Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}

impl types::HostStatement for Sql<'_> {
    async fn prepare(
        &mut self, _query: String, _params: Vec<String>,
    ) -> Result<Result<Resource<Statement>, Resource<Error>>> {
        // let stmt = Statement::parse(&query, &params)?;
        // Ok(Ok(self.table.push(stmt)?))
        todo!()
    }

    async fn drop(&mut self, rep: Resource<Statement>) -> Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}

impl types::HostError for Sql<'_> {
    async fn trace(&mut self, rep: Resource<Error>) -> Result<String> {
        let error = self.table.get(&rep)?;
        Ok(error.to_string())
    }

    async fn drop(&mut self, rep: Resource<Error>) -> Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
