#![feature(let_chains)]

use anyhow::{anyhow, Result};
use http::header::CONTENT_TYPE; // AUTHORIZATION
use http::Uri;
use serde_json::json;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use vercre_core::metadata::Issuer as IssuerMetadata;
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{
    Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};
use wasi_bindings::sql::readwrite::{self, Connection, Statement};
use wasi_bindings::sql::types::DataType;

struct HttpGuest;

impl Guest for HttpGuest {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        let subscriber =
            FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        // set up response in case of early failure
        let headers = Fields::new();
        let _ = headers.set(&CONTENT_TYPE.to_string(), &[b"application/json".to_vec()]);
        let out_resp = OutgoingResponse::new(headers);

        let Ok(out_body) = out_resp.body() else {
            return;
        };
        ResponseOutparam::set(response, Ok(out_resp));

        let req = Request::from(&request);

        // invoke handler based on path
        let result = match req.uri().path() {
            "/" => hello(&req),
            path => Err(anyhow!("path {path} not found")),
        };

        // serialize response
        let content = match result {
            Ok(resp) => resp,
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

fn hello(request: &Request) -> Result<Vec<u8>> {
    tracing::debug!("request.uri: {}", request.uri());

    let body = request.body()?;
    let req: serde_json::Value = serde_json::from_slice(&body)?;
    tracing::debug!("json: {:?}", req);

    let cnn = Connection::open("metadata").unwrap();
    let query = Statement::prepare(
        "SELECT * FROM issuer WHERE credential_issuer = '?'",
        &["issuer".to_string()],
    )
    .unwrap();
    let res = readwrite::query(&cnn, &query).unwrap();
    let row = res[0].clone();

    let DataType::Binary(md) = row.value else {
        return Err(anyhow!("invalid row value"));
    };
    let md: IssuerMetadata = serde_json::from_slice(&md)?;
    tracing::debug!("md: {:?}", md);

    let resp = json!({
        "message": "Hello, World!"
    });
    serde_json::to_vec(&resp).map_err(Into::into)
}

wasi::http::proxy::export!(HttpGuest);

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
