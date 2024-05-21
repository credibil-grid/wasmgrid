use anyhow::anyhow;
use mongodb;
use mongodb::options::ReplaceOptions;
use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;

use super::bindings::wasi::docdb::readwrite;
use super::{Database, Error, Statement};
use crate::runtime::State;

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

    async fn insert_v2(
        &mut self, db: Resource<Database>, collection: String, d: Vec<u8>,
    ) -> wasmtime::Result<Result<(), Resource<Error>>> {
        tracing::debug!("readwrite::Host::insert_v2");

        tracing::debug!("readwrite::Host::insert_v2::collection:{}", collection);
        tracing::debug!("readwrite::Host::insert_v2::data:{:?}", d);

        let table = self.table();
        let database = table.get(&db)?;

        tracing::debug!("readwrite::Host::insert_v2::here?");

        let doc: bson::Document = bson::from_slice(&d)?;

        tracing::debug!("readwrite::Host::insert_v2::doc: {}", doc);

        database.collection(collection.as_str()).insert_one(doc, None).await?;

        Ok(Ok(()))
    }

    async fn find_one_v2(
        &mut self, db: Resource<Database>, collection: String, query: Vec<u8>,
    ) -> wasmtime::Result<Result<Vec<u8>, Resource<Error>>> {
        tracing::debug!("readwrite::Host::find_v2");

        let table = self.table();
        let database = table.get(&db)?;

        let query: bson::Document = bson::from_slice(&query)?;

        let Some(doc) = database
            .collection::<bson::Document>(collection.as_str())
            .find_one(query, None)
            .await?
        else {
            return Err(anyhow!("document not found"));
        };

        let mut result: Vec<u8> = Vec::new();
        doc.to_writer(&mut result)?;

        Ok(Ok(result))
    }
}
