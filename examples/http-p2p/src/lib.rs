#![feature(let_chains)]

use anyhow::anyhow;
use http::header::CONTENT_TYPE;
use http::Uri;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{
    Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};
use wasi_bindings::p2p::document;
use wasi_bindings::p2p::exports::OutgoingValue;
struct HttpP2pGuest;

impl Guest for HttpP2pGuest {
    fn handle(req: IncomingRequest, res: ResponseOutparam) {
        init_tracing();

        // set up response in case of errors
        let headers = Fields::new();
        let _ = headers.set(&CONTENT_TYPE.to_string(), &[b"application/json".to_vec()]);
        let out_res = OutgoingResponse::new(headers);

        let Ok(out_body) = out_res.body() else {
            return;
        };
        ResponseOutparam::set(res, Ok(out_res));

        let req = Request::from(&req);

        // invoke handler based on path
        tracing::debug!("request.uri: {}", req.uri());
        let result = match req.uri().path() {
            "/" => create_document(&req),
            path => Err(anyhow!("path {path} not found")),
        };

        // serialize response
        let content = match result {
            Ok(res) => res,
            Err(err) => {
                let json = json!({"error": "server_error", "error_description": err.to_string()});
                serde_json::to_vec(&json).unwrap()
            }
        };

        // write outgoing body
        let out_stream = out_body.write().unwrap();
        out_stream.blocking_write_and_flush(content.as_slice()).unwrap();
        drop(out_stream);
        OutgoingBody::finish(out_body, None).unwrap();
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

type Log = Vec<(String, String)>;

fn create_document(req: &Request) -> anyhow::Result<Vec<u8>> {
    let mut log: Log = Vec::new();
    let body = req.body()?;
    let doc: Doc = serde_json::from_slice(&body)?;
    tracing::debug!("processing document: {:?}", doc);

    // Establish an author - either by using an environment variable or creating a new one.
    let mut author = std::env::var("IROH_AUTHOR").unwrap_or_else(|_| "create".to_string());
    if author == "create" {
        author = create_author()?;
    }
    tracing::debug!("using author: {author}");
    log.push(("author".to_string(), author.clone()));

    // Create the document.
    let (container, ticket) = document::create_container(&author).map_err(|e| anyhow!(e))?;
    tracing::debug!("created container");
    log.push(("ticket".to_string(), ticket.clone()));
    tracing::debug!("ticket: {ticket}");
    let container_id = container.name().map_err(|e| anyhow!(e))?;
    log.push(("container ID".to_string(), container_id.clone()));
    tracing::debug!("container ID: {container_id}");

    // Add entries.
    for entry in doc.entries {
        let data = OutgoingValue::new_outgoing_value();
        let content = data
            .outgoing_value_write_body()
            .map_err(|_| anyhow!("unable to get outgoing value body"))?;
        let allowed_length = content.check_write()?;
        if allowed_length < entry.data.len() as u64 {
            return Err(anyhow!("data too large"));
        }
        content.write(entry.data.as_bytes())?;
        container.write_data(&entry.key, &data).map_err(|e| anyhow!(e))?;
    }

    // Read the document back again.

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

wasi::http::proxy::export!(HttpP2pGuest);

fn init_tracing() {
    let subscriber =
        FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

#[derive(Debug)]
pub struct Request<'a> {
    inner: &'a IncomingRequest,
}

impl<'a> From<&'a IncomingRequest> for Request<'a> {
    fn from(inner: &'a IncomingRequest) -> Self {
        Self { inner }
    }
}

impl<'a> Request<'a> {
    pub fn uri(&self) -> Uri {
        let p_and_q = self.inner.path_with_query().unwrap_or_default();
        p_and_q.parse::<Uri>().unwrap_or_else(|_| Uri::default())
    }

    fn body(&self) -> anyhow::Result<Vec<u8>> {
        let body = self.inner.consume().map_err(|()| anyhow!("error consuming request body"))?;
        let stream = body.stream().map_err(|()| anyhow!("error getting body stream"))?;

        // Read the entire body into a buffer.
        let mut buffer = Vec::new();
        while let Ok(bytes) = stream.read(1000)
            && !bytes.is_empty()
        {
            buffer.extend_from_slice(&bytes);
        }
        Ok(buffer)
    }
}
