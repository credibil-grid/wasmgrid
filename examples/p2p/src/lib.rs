#![feature(let_chains)]

use std::u64;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};
use wasi_bindings::p2p::container;
use wasi_bindings::p2p::types::Permission;
use wasi_http::{self, post, Request, Router};
struct HttpGuest;

impl Guest for HttpGuest {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        init_tracing();

        let router = Router::new().route("/", post(handler));

        let out = wasi_http::serve(&router, &request);
        ResponseOutparam::set(response, out);
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct DocEntry {
    key: String,
    data: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Doc {
    entries: Vec<DocEntry>,
}

#[derive(Debug, Serialize)]
struct Log {
    entries: Vec<(String, String)>,
}

fn handler(req: &Request) -> anyhow::Result<Vec<u8>> {
    let mut log = Log { entries: Vec::new() };
    let body = req.body()?;
    let doc: Doc = serde_json::from_slice(&body)?;
    tracing::debug!("processing document: {:?}", doc);

    // Establish an author - either by using an environment variable or creating a new one.
    let mut author = std::env::var("IROH_AUTHOR").unwrap_or_else(|_| "create".into());
    if author == "create" {
        author = container::create_author().map_err(|e| anyhow!(e))?;
    }
    tracing::debug!("using author: {author}");
    log.entries.push(("author".into(), author.clone()));

    // Create the document.
    let document = container::create_container().map_err(|e| anyhow!(e))?;
    tracing::debug!("created container");
    // Get a ticket with write permission.
    let ticket = document.get_token(Permission::Write).map_err(|e| anyhow!(e))?;
    log.entries.push(("ticket".into(), ticket.clone()));
    tracing::debug!("ticket: {ticket}");
    let container_id = document.id().map_err(|e| anyhow!(e))?;
    log.entries.push(("container ID".into(), container_id.clone()));
    tracing::debug!("container ID: {container_id}");

    // Add entries.
    for entry in doc.entries.iter() {
        tracing::debug!("adding entry: {entry:?}");
        let data = entry.data.as_bytes();
        document.write_entry(&entry.key, &author, &data).map_err(|e| anyhow!(e))?;
        tracing::debug!("entry written");
    }

    // Read the document back again.
    let document = container::get_container(&ticket).map_err(|e| anyhow!(e))?;
    tracing::debug!("retrieved container");

    // List entry keys.
    let keys = document.list_entries().map_err(|e| anyhow!(e))?;
    tracing::debug!("keys: {:?}", keys);
    log.entries.push(("keys".into(), keys.join(", ")));

    // Get entry metadata.
    for key in keys.iter() {
        let metadata = document.get_entry_metadata(&key).map_err(|e| anyhow!(e))?;
        tracing::debug!("metadata for {key}: {metadata:?}");
        let metadata_str = serde_json::to_string(&metadata).map_err(|e| anyhow!(e))?;
        log.entries.push((format!("metadata for {key}", key = key), metadata_str));
    }

    // Get entry content.
    for key in keys.iter() {
        let content = document.read_entry(&key, 0, u64::MAX).map_err(|e| anyhow!(e))?;
        let content_str = String::from_utf8(content).map_err(|e| anyhow!(e))?;
        tracing::debug!("content for {key}: {content_str}");
        log.entries.push((key.into(), content_str));
    }

    // Delete the entries.
    for key in keys.iter() {
        document.delete_entry(&key).map_err(|e| anyhow!(e))?;
        tracing::debug!("deleted entry: {key}");
    }

    // Delete the document.
    container::delete_container(document).map_err(|e| anyhow!(e))?;
    tracing::debug!("deleted document");

    tracing::debug!("log: {:?}", log);
    serde_json::to_vec(&log).map_err(Into::into)
}

wasi::http::proxy::export!(HttpGuest);

fn init_tracing() {
    let subscriber =
        FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
