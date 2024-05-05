//! # WASI SQL Capability
//!
//! This module implements a runtime capability for `wasi:sql`
//! (<https://github.com/WebAssembly/wasi-sql>).

use std::any::Any;
use std::sync::OnceLock;

use anyhow::anyhow;
use mongodb::options::ClientOptions;
use mongodb::Client;
use wasi_sql::bindings::wasi::sql::types::{self, Row};
use wasi_sql::bindings::Sql;
use wasi_sql::{self, RuntimeConnection, RuntimeStatement, SqlView};
use wasmtime::component::{Linker, Resource};
use wasmtime_wasi::WasiView;

use crate::runtime::{self, Runtime, State};

const CNN_STRING:&str="mongodb+srv://oidc-user:fCNpCf-PJNuum9A_7CkFGa-wqsnUUH@cluster0.uqnlxl8.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0";
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
        let mut client_options = ClientOptions::parse(CNN_STRING).await?;
        client_options.app_name = Some("Credibil Grid".to_string());
        let client = Client::with_options(client_options)?;
        MONGODB.get_or_init(|| client);

        tracing::info!("connected to MongoDB on {}", self.addr);

        Ok(())
    }
}

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Metadata {
    issuer: String,
}

// Implement the [`wasi_sql::SqlView`]` trait for State.
#[async_trait::async_trait]
impl SqlView for State {
    async fn query(
        &mut self, cnn: Resource<types::Connection>, stmt: Resource<types::Statement>,
    ) -> anyhow::Result<Vec<Row>> {
        tracing::debug!("SqlView::query");

        let rt = self.table();

        let cnn = rt.get(&cnn)?;
        let Some(db) = cnn.as_ref().as_any().downcast_ref::<Connection>() else {
            return Err(anyhow!("invalid connection"));
        };

        let stmt = rt.get(&stmt)?;
        let Some(query) = stmt.as_ref().as_any().downcast_ref::<Statement>() else {
            return Err(anyhow!("invalid connection"));
        };
        println!("{query:?}");

        let filter = mongodb::bson::doc! {};
        let md = db.database.collection::<Metadata>("issuer").find_one(Some(filter), None).await;
        println!("md: {:?}", md);

        Ok(vec![])
    }

    // TODO: implement update_configuration
    async fn exec(
        &mut self, _: Resource<types::Connection>, _: Resource<types::Statement>,
    ) -> anyhow::Result<Vec<u32>> {
        tracing::debug!("SqlView::exec");
        todo!()
    }

    async fn open(&mut self, name: String) -> anyhow::Result<Resource<types::Connection>> {
        tracing::debug!("SqlView::open");
        let db: wasi_sql::Connection = Box::new(Connection::new(name.clone()).await?);
        Ok(self.table().push(db)?)
    }

    fn drop_connection(&mut self, rep: Resource<types::Connection>) -> anyhow::Result<()> {
        tracing::debug!("SqlView::drop_connection");
        self.table().delete(rep).map_or_else(|e| Err(anyhow!(e)), |_| Ok(()))
    }

    async fn prepare(
        &mut self, query: String, _params: Vec<String>,
    ) -> anyhow::Result<Resource<types::Statement>> {
        tracing::debug!("SqlView::prepare");
        let stmt: wasi_sql::Statement = Box::new(Statement::new(query));
        Ok(self.table().push(stmt)?)
    }

    fn drop_statement(&mut self, rep: Resource<types::Statement>) -> anyhow::Result<()> {
        tracing::debug!("SqlView::drop_statement");
        self.table().delete(rep).map_or_else(|e| Err(anyhow!(e)), |_| Ok(()))
    }

    async fn trace(&mut self, _self_: Resource<types::Error>) -> String {
        tracing::debug!("SqlView::trace");
        todo!()
    }

    fn drop_error(&mut self, _rep: Resource<types::Error>) {
        tracing::debug!("SqlView::drop_error");
        todo!()
    }
}

// Connection holds a reference to the the NATS client. It is used to implement the
// [`wasi_sql::RuntimeConnection`] trait used by the sql State.
#[derive(Debug)]
struct Connection {
    name: String,
    database: mongodb::Database,
}

impl Connection {
    // Create a new Connection for the specified NATS server.
    async fn new(name: String) -> anyhow::Result<Self> {
        tracing::trace!("Connection::new {name}");

        let client = MONGODB.get().ok_or(anyhow!("MongoDB not connected"))?;
        let database = client.database(&name);

        Ok(Self { name, database })
    }
}

impl RuntimeConnection for Connection {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// // Statement holds a reference to the the NATS client. It is used to implement the
// [`wasi_sql::RuntimeConnection`] trait used by the sql host.
#[derive(Debug)]
struct Statement {
    query: String,
}

impl Statement {
    // Create a new Statement for the specified NATS server.
    fn new(query: String) -> Self {
        tracing::trace!("Statement::new");
        Self { query }
    }
}
impl RuntimeStatement for Statement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
