#![feature(let_chains)]

use anyhow::{anyhow, Result};
use http::header::CONTENT_TYPE;
use http::Uri;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{
    Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};
//use wasi_bindings::p2p::document;

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

fn create_document(req: &Request) -> Result<Vec<u8>> {
    let log: Log = Vec::new();
    let body = req.body()?;
    tracing::debug!("request body: {:?}", body);
    let doc: Doc = serde_json::from_slice(&body)?;
    tracing::debug!("processing document: {:?}", doc);

    serde_json::to_vec(&log).map_err(Into::into)
}

wasi::http::proxy::export!(HttpP2pGuest);

fn init_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
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

    fn body(&self) -> Result<Vec<u8>> {
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

