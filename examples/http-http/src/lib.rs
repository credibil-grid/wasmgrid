#![feature(let_chains)]

use anyhow::anyhow;
use http::header::{CONTENT_TYPE, USER_AGENT};
use wasex::{self, Request, Router};
// use serde::de::DeserializeOwned;
use serde_json::json;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::outgoing_handler;
use wasi::http::types::{
    Headers, IncomingRequest, Method, OutgoingBody, OutgoingRequest, ResponseOutparam, Scheme,
};
use wasi::io::streams;

struct HttpGuest;

impl Guest for HttpGuest {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        let subscriber =
            FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        let router = Router::new().route("/", call_downstream);

        let out = wasex::serve(&router, &request);
        ResponseOutparam::set(response, out);
    }
}

fn call_downstream(_: &Request) -> anyhow::Result<Vec<u8>> {
    // build outgoing request
    let headers = Headers::new();
    headers.append(&USER_AGENT.to_string(), &b"WASI-HTTP/0.0.1".to_vec())?;
    headers.append(&CONTENT_TYPE.to_string(), &b"application/json".to_vec())?;

    let out_req = OutgoingRequest::new(headers);
    out_req.set_method(&Method::Post).map_err(|()| anyhow!("failed to set method"))?;
    out_req.set_scheme(Some(&Scheme::Http)).map_err(|()| anyhow!("failed to set scheme"))?;
    out_req
        .set_authority(Some("localhost:8080"))
        .map_err(|()| anyhow!("failed to set authority"))?;
    out_req
        .set_path_with_query(Some("/"))
        .map_err(|()| anyhow!("failed to set path_with_query"))?;

    let out_body = out_req.body().map_err(|_| anyhow!("out_req request write failed"))?;

    let json_body = json!({
        "message": "Hello, World!"
    });
    let vec = serde_json::to_vec(&json_body)?;

    let out_stream = out_body.write().map_err(|_| anyhow!("out_req request write failed"))?;
    out_stream.blocking_write_and_flush(vec.as_slice()).unwrap();
    drop(out_stream);

    // make request
    let fut_resp = outgoing_handler::handle(out_req, None)?;
    if let Err(e) = OutgoingBody::finish(out_body, None) {
        anyhow::bail!("output stream error: {e}")
    }

    // Option<Result<Result<IncomingResponse, ErrorCode>, ()>>
    // process response
    let resp = match fut_resp.get() {
        Some(result) => result,
        None => {
            fut_resp.subscribe().block();
            let Some(result) = fut_resp.get() else {
                anyhow::bail!("response missing");
            };
            result
        }
    }
    .map_err(|()| anyhow!("ersponse taken"))??;
    drop(fut_resp);

    let status = resp.status();
    tracing::debug!("status: {:?}", status);

    let headers_handle = resp.headers();
    let headers = headers_handle.entries();
    tracing::debug!("headers: {:?}", headers);
    drop(headers_handle);

    let resp_body = resp.consume().map_err(|()| anyhow!("incoming response has no body stream"))?;

    drop(resp);

    let body_stream = resp_body.stream().unwrap();
    let body_stream_pollable = body_stream.subscribe();

    let mut body = Vec::new();
    loop {
        body_stream_pollable.block();
        let mut body_chunk = match body_stream.read(1024 * 1024) {
            Ok(c) => c,
            Err(streams::StreamError::Closed) => break,
            Err(e) => Err(anyhow!("input_stream read failed: {e:?}"))?,
        };
        if !body_chunk.is_empty() {
            body.append(&mut body_chunk);
        }
    }

    let json_res = json!({
        "message": "Hello, World!"
    });
    serde_json::to_vec(&json_res).map_err(Into::into)
}

wasi::http::proxy::export!(HttpGuest);
