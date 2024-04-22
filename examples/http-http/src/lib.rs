#![feature(let_chains)]

use anyhow::{anyhow, Result};
use http::header::{CONTENT_TYPE, USER_AGENT}; // AUTHORIZATION
use http::Uri;
// use serde::de::DeserializeOwned;
use serde_json::json;
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::outgoing_handler;
use wasi::http::types::{
    Headers, IncomingRequest, Method, OutgoingBody, OutgoingRequest, OutgoingResponse,
    ResponseOutparam, Scheme,
};
use wasi::io::streams;

struct HttpGuest;

impl Guest for HttpGuest {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        // set up response in case of early failure
        let headers = Headers::new();
        let _ = headers.set(&CONTENT_TYPE.to_string(), &[b"application/json".to_vec()]);
        let out_resp = OutgoingResponse::new(headers);

        let Ok(out_body) = out_resp.body() else {
            return;
        };
        ResponseOutparam::set(response, Ok(out_resp));

        let req = Request::from(&request);

        // invoke handler based on path
        let result = match req.uri().path() {
            "/" => outgoing(&req),
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

        // write out_req body
        let out_stream = out_body.write().unwrap();
        out_stream.blocking_write_and_flush(content.as_slice()).unwrap();
        drop(out_stream);
        OutgoingBody::finish(out_body, None).unwrap();
    }
}

fn outgoing(request: &Request) -> Result<Vec<u8>> {
    println!("request.uri: {}", request.uri());

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

    let fut_resp = outgoing_handler::handle(out_req, None)?;
    if let Err(e) = OutgoingBody::finish(out_body, None) {
        println!("error finishing out_req body: {e}");
        anyhow::bail!("output stream error: {e}")
    }

    let in_resp = match fut_resp.get() {
        Some(Ok(result)) => result,
        Some(Err(())) => anyhow::bail!("response taken"),
        None => {
            println!("here 5.2");
            fut_resp.subscribe().block();
            let Some(result) = fut_resp.get() else {
                anyhow::bail!("response missing");
            };
            result.map_err(|()| anyhow!("response taken"))?
        }
    }?;
    drop(fut_resp);

    let status = in_resp.status();
    println!("status: {:?}", status);

    let headers_handle = in_resp.headers();
    let headers = headers_handle.entries();
    println!("headers: {:?}", headers);
    drop(headers_handle);

    let in_body = in_resp
        .consume()
        .map_err(|()| anyhow!("incoming response has no body stream"))?;

    drop(in_resp);

    let input_stream = in_body.stream().unwrap();
    let input_stream_pollable = input_stream.subscribe();

    let mut body = Vec::new();
    loop {
        input_stream_pollable.block();

        let mut body_chunk = match input_stream.read(1024 * 1024) {
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

    // // Get the host the request was made to (using scheme and authority).
    // fn host(&self) -> Result<String> {
    //     let Some(authority) = self.inner.authority() else {
    //         return Err(anyhow!("Authority is missing"));
    //     };

    //     let scheme = match self.inner.scheme() {
    //         Some(Scheme::Http) => String::from("http"),
    //         Some(Scheme::Https) => String::from("https"),
    //         Some(Scheme::Other(s)) => s,
    //         None => return Err(anyhow!("Scheme is missing")),
    //     };

    //     Ok(format!("{scheme}://{authority}"))
    // }

    // // Get the access token from the Authorization header.
    // fn auth_token(&self) -> Result<String> {
    //     let header = self.inner.headers().get(&AUTHORIZATION.to_string());
    //     if header.is_empty() {
    //         return Err(anyhow!("Authorization header is missing"));
    //     }
    //     let Ok(value) = String::from_utf8(header[0].clone()) else {
    //         return Err(anyhow!("Authorization header is not valid UTF-8"));
    //     };
    //     let Some(token) = value.split_whitespace().last() else {
    //         return Err(anyhow!("Authorization header is missing token"));
    //     };
    //     Ok(token.to_owned())
    // }

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

    // fn from_json<T: DeserializeOwned>(&self) -> Result<T> {
    //     Ok(serde_json::from_slice::<T>(&self.body()?)?)
    // }

    // fn from_form<T: DeserializeOwned>(&self) -> Result<T> {
    //     Ok(serde_urlencoded::from_bytes::<T>(&self.body()?)?)
    // }
}
