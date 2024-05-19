#![feature(let_chains)]

use anyhow::anyhow;
use http_shared::{self, Request, Router};
use serde::{Deserialize, Serialize};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};
use wasi_bindings::p2p::document;
use wasi_bindings::p2p::exports::{OutgoingValue, StreamObjectNames};
struct HttpGuest;

impl Guest for HttpGuest {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        init_tracing();

        let router = Router::new().route("/", handler);

        let out = http_shared::serve(&router, &request);
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
    let mut author = std::env::var("IROH_AUTHOR").unwrap_or_else(|_| "create".to_string());
    if author == "create" {
        author = create_author()?;
    }
    tracing::debug!("using author: {author}");
    log.entries.push(("author".to_string(), author.clone()));

    // Create the document.
    let (container, ticket) = document::create_container(&author).map_err(|e| anyhow!(e))?;
    tracing::debug!("created container");
    log.entries.push(("ticket".to_string(), ticket.clone()));
    tracing::debug!("ticket: {ticket}");
    let container_id = container.name().map_err(|e| anyhow!(e))?;
    log.entries.push(("container ID".to_string(), container_id.clone()));
    tracing::debug!("container ID: {container_id}");

    // Add entries.
    for entry in doc.entries.iter() {
        tracing::debug!("adding entry: {:?}", entry);
        let data = OutgoingValue::new_outgoing_value();
        let content = data
            .outgoing_value_write_body()
            .map_err(|_| anyhow!("unable to get outgoing value body"))?;
        let allowed_length = content.check_write().map_err(|e| anyhow!(e))?;
        if allowed_length < entry.data.len() as u64 {
            return Err(anyhow!("data too large"));
        }
        content.write(entry.data.as_bytes()).map_err(|e| anyhow!(e))?;
        container.write_data(&entry.key, &data).map_err(|e| anyhow!(e))?;
        tracing::debug!("entry written");
    }

    // Read the document back again.
    let container = document::get_container(&author, &ticket).map_err(|e| anyhow!(e))?;
    tracing::debug!("retrieved container");

    // List entry keys.
    let keys = container.list_objects().map_err(|e| anyhow!(e))?;
    let (names, _end) =
        StreamObjectNames::read_stream_object_names(&keys, 1024).map_err(|e| anyhow!(e))?;
    tracing::debug!("keys: {:?}", names);

    // Get entry metadata.

    // Get entry content.

    // Delete the entries.

    // Delete the document.

    tracing::debug!("log: {:?}", log);
    serde_json::to_vec(&log).map_err(Into::into)
}

fn create_author() -> anyhow::Result<String> {
    tracing::debug!("creating author");
    let author = match document::create_owner() {
        Ok(author) => author,
        Err(err) => {
            tracing::debug!("error creating author: {:?}", err);
            return Err(anyhow!(err));
        }
    };
    Ok(author)
}

wasi::http::proxy::export!(HttpGuest);

fn init_tracing() {
    let subscriber =
        FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
