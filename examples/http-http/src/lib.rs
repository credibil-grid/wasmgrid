#![feature(let_chains)]

use anyhow::{anyhow, Result};
use http::header::CONTENT_TYPE; // AUTHORIZATION
use http::Uri;
// use serde::de::DeserializeOwned;
use serde_json::json;
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::outgoing_handler;
use wasi::http::types::{
    Fields, Headers, IncomingRequest, OutgoingBody, OutgoingRequest, OutgoingResponse,
    ResponseOutparam, Scheme,
}; // Scheme,

struct HttpGuest;

impl Guest for HttpGuest {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
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

        // write outgoing body
        let out_stream = out_body.write().unwrap();
        out_stream.blocking_write_and_flush(content.as_slice()).unwrap();
        drop(out_stream);
        OutgoingBody::finish(out_body, None).unwrap();
    }
}

fn outgoing(request: &Request) -> Result<Vec<u8>> {
    println!("request.uri: {}", request.uri());

    let json_req = serde_json::from_slice(&request.body()?)?;
    println!("{:?}", json_req);

    let headers = Headers::new();
    let out_req = OutgoingRequest::new(headers);

    let _ = out_req.set_scheme(Some(&Scheme::Http));
    let _ = out_req.set_authority(Some("localhost:8080"));
    let _ = out_req.set_path_with_query(Some("/"));

    let fut_resp = outgoing_handler::handle(out_req, None)?;
    let resp = fut_resp.get();
    println!("{:?}", resp);

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
