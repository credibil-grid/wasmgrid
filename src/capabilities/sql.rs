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
use wasi_sql::readwrite::ReadWriteView;
use wasi_sql::types::{ConnectionView, ErrorView, StatementView};
use wasi_sql::{self, RuntimeConnection, RuntimeStatement};
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

// Implement the [`wasi_sql::ReadWriteView`]` trait for State.
#[async_trait::async_trait]
impl ReadWriteView for State {
    async fn query(
        &mut self, c: Resource<types::Connection>, s: Resource<types::Statement>,
    ) -> anyhow::Result<Vec<Row>> {
        tracing::debug!("ReadWriteView::query");

        let rt = self.table();

        let c = rt.get(&c)?;
        let Some(db) = c.as_ref().as_any().downcast_ref::<Connection>() else {
            return Err(anyhow!("invalid connection"));
        };

        let s = rt.get(&s)?;
        let Some(stmt) = s.as_ref().as_any().downcast_ref::<Statement>() else {
            return Err(anyhow!("invalid connection"));
        };
        println!("{}", stmt.query);

        let filter = mongodb::bson::doc! {};
        let md = db.database.collection::<Metadata>("issuer").find_one(Some(filter), None).await;
        println!("md: {:?}", md);

        Ok(vec![])
    }

    // TODO: implement update_configuration
    async fn exec(
        &mut self, c: Resource<types::Connection>, s: Resource<types::Statement>,
    ) -> anyhow::Result<u32> {
        tracing::debug!("ReadWriteView::exec");
        let rt = self.table();

        let c = rt.get(&c)?;
        let Some(db) = c.as_ref().as_any().downcast_ref::<Connection>() else {
            return Err(anyhow!("invalid connection"));
        };

        let s = rt.get(&s)?;
        let Some(stmt) = s.as_ref().as_any().downcast_ref::<Statement>() else {
            return Err(anyhow!("invalid connection"));
        };
        println!("{}", stmt.query);

        let filter = mongodb::bson::doc! {};
        let md = db.database.collection::<Metadata>("issuer").find_one(Some(filter), None).await;
        println!("md: {:?}", md);

        Ok(0)
    }
}

// Implement the [`wasi_sql::ConnectionView`]` trait for State.
#[async_trait::async_trait]
impl ConnectionView for State {
    async fn open(&mut self, name: String) -> anyhow::Result<Resource<types::Connection>> {
        tracing::debug!("ConnectionView::open");
        let db: wasi_sql::Connection = Box::new(Connection::new(name.clone()).await?);
        Ok(self.table().push(db)?)
    }

    fn drop(&mut self, rep: Resource<types::Connection>) -> anyhow::Result<()> {
        tracing::debug!("ConnectionView::drop");
        self.table().delete(rep).map_or_else(|e| Err(anyhow!(e)), |_| Ok(()))
    }
}

// Implement the [`wasi_sql::StatementView`]` trait for State.
#[async_trait::async_trait]
impl StatementView for State {
    async fn prepare(
        &mut self, query: String, _params: Vec<String>,
    ) -> anyhow::Result<Resource<types::Statement>> {
        tracing::debug!("StatementView::prepare");
        let stmt: wasi_sql::Statement = Box::new(Statement::new(query));
        Ok(self.table().push(stmt)?)
    }

    fn drop(&mut self, rep: Resource<types::Statement>) -> anyhow::Result<()> {
        tracing::debug!("StatementView::drop");
        self.table().delete(rep).map_or_else(|e| Err(anyhow!(e)), |_| Ok(()))
    }
}

// Implement the [`wasi_sql::ErrorView`]` trait for State.
#[async_trait::async_trait]
impl ErrorView for State {
    async fn trace(&mut self, _self_: Resource<types::Error>) -> String {
        tracing::debug!("ErrorView::trace");
        todo!()
    }

    fn drop(&mut self, _rep: Resource<types::Error>) {
        tracing::debug!("ErrorView::drop");
        todo!()
    }
}

// Connection holds a reference to the the NATS client. It is used to implement the
// [`wasi_sql::RuntimeConnection`] trait used by the sql State.
#[derive(Debug)]
struct Connection {
    database: mongodb::Database,
}

impl Connection {
    // Create a new Connection for the specified NATS server.
    async fn new(name: String) -> anyhow::Result<Self> {
        tracing::trace!("Connection::new {name}");

        let client = MONGODB.get().ok_or(anyhow!("MongoDB not connected"))?;
        let database = client.database(&name);

        Ok(Self { database })
    }
}

impl RuntimeConnection for Connection {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// // Statement holds a reference to the the NATS client. It is used to implement the
// [`wasi_sql::RuntimeStatement`] trait used by the sql host.
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
