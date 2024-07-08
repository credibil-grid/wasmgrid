use std::io::{Read, Write};

use http::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT};
use http::{StatusCode, Uri};
use multipart::client::lazy::Multipart;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{self};
use wasi::http::outgoing_handler::{self, ErrorCode};
use wasi::http::types::{Fields, Headers, Method, OutgoingBody, OutgoingRequest, Scheme};
use wasi::io::streams;

#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: StatusCode,
    pub body: Body,
    pub headers: MyHeaders,
}

pub struct HttpRequest {
    headers: Headers,
    method: Method,
    uri: Uri,
    payload: Option<Vec<u8>>,
}

#[derive(Debug)]
pub enum HttpResponseError {
    Internal { message: String },
    Response { response: HttpResponse },
}

#[derive(Debug)]
pub enum BuildMultipartError {
    Prepare,
    ReadToEnd,
    AppendHeaders,
}

#[derive(Debug)]
pub enum BuildJsonError {
    Parse,
    AppendHeaders,
}

impl From<ErrorCode> for HttpResponseError {
    fn from(value: ErrorCode) -> Self {
        HttpResponseError::Internal {
            message: value.to_string(),
        }
    }
}

impl HttpRequest {
    fn new(method: Method, uri: Uri) -> Self {
        let headers = Headers::new();
        let _ = headers.append(&USER_AGENT.to_string(), &b"WASI-HTTP/0.0.1".to_vec());

        HttpRequest {
            headers,
            method,
            uri,
            payload: None,
        }
    }

    pub fn get(uri: Uri) -> Self {
        Self::new(Method::Get, uri)
    }

    pub fn post(uri: Uri) -> Self {
        Self::new(Method::Post, uri)
    }

    pub fn bytes(mut self, data: Vec<u8>) -> Self {
        self.payload = Some(data);
        self
    }

    pub fn multipart(mut self, mut data: Multipart) -> Result<Self, BuildMultipartError> {
        //Somewhy code will not compile in case of ```data.prepare()?```
        let prepared_result = data.prepare();

        if prepared_result.is_err() {
            return Err(BuildMultipartError::Prepare);
        }

        let mut prepared = prepared_result.unwrap();

        let boundary = prepared.boundary();
        let content_type = format!("multipart/form-data; boundary={}", &boundary);

        let mut buffer = Vec::new();
        prepared.read_to_end(&mut buffer).map_err(|_| BuildMultipartError::ReadToEnd)?;

        self.payload = Some(buffer);

        self.headers
            .append(&CONTENT_TYPE.to_string(), &content_type.as_bytes().to_vec())
            .map_err(|_| BuildMultipartError::AppendHeaders)?;

        Ok(self)
    }

    pub fn json(mut self, data: impl Serialize) -> Result<Self, BuildJsonError> {
        let bytes = serde_json::to_vec(&data).map_err(|_| BuildJsonError::Parse)?;
        self.payload = Some(bytes);
        self.headers
            .append(&CONTENT_TYPE.to_string(), &b"application/json".to_vec())
            .map_err(|_| BuildJsonError::AppendHeaders)?;

        Ok(self)
    }

    pub fn send(self) -> Result<HttpResponse, HttpResponseError> {
        let out_req = OutgoingRequest::new(self.headers);
        out_req.set_method(&self.method).map_err(|()| new_internal_str("Failed to set method"))?;
        out_req
            .set_scheme(Some(&Scheme::Http))
            .map_err(|()| new_internal_str("Failed to set scheme"))?;
        out_req
            .set_authority(self.uri.authority().and_then(|v| Some(v.as_str())))
            .map_err(|()| new_internal_str("Failed to set authority"))?;
        out_req
            .set_path_with_query(self.uri.path_and_query().and_then(|v| Some(v.as_str())))
            .map_err(|()| new_internal_str("Failed to set path_with_query"))?;

        let out_body =
            out_req.body().map_err(|_| new_internal_str("out_req request write failed"))?;

        let mut out_stream =
            out_body.write().map_err(|_| new_internal_str("out_req request write failed"))?;

        let some_payload = self.payload.unwrap_or_default();

        out_stream
            .write_all(&some_payload)
            .map_err(|e| new_internal_string(format!("write_all failed {e}")))?;

        drop(out_stream);

        // make request
        let fut_resp = outgoing_handler::handle(out_req, None)
            .map_err(|e| new_internal_string(format!("handle error: {e}")))?;

        OutgoingBody::finish(out_body, None)
            .map_err(|e| new_internal_string(format!("output stream error: {e}")))?;

        // Option<Result<Result<IncomingResponse, ErrorCode>, ()>>
        // process response
        let resp = match fut_resp.get() {
            Some(result) => result,
            None => {
                fut_resp.subscribe().block();
                let Some(result) = fut_resp.get() else {
                    return Err(new_internal_str("Response missing"));
                };
                result
            }
        }
        .map_err(|()| new_internal_str("Response taken"))??;
        drop(fut_resp);

        let status = resp.status();

        let headers_handle = resp.headers();
        let cloned_headers = headers_handle.clone();

        drop(headers_handle);

        let response_headers = MyHeaders {
            inner: cloned_headers,
        };

        let resp_body = resp
            .consume()
            .map_err(|()| new_internal_str("incoming response has no body stream"))?;

        drop(resp);

        let body_stream = resp_body
            .stream()
            .map_err(|()| new_internal_str("failed to create response stream"))?;
        let body_stream_pollable = body_stream.subscribe();

        let mut body = Vec::new();
        loop {
            body_stream_pollable.block();
            let mut body_chunk = match body_stream.read(1024 * 1024) {
                Ok(c) => c,
                Err(streams::StreamError::Closed) => break,
                Err(e) => Err(new_internal_string(format!("input_stream read failed: {e:?}")))?,
            };
            if !body_chunk.is_empty() {
                body.append(&mut body_chunk);
            }
        }

        println!("http: response bytes len={}", body.len());

        if body.len() < 1000 {
            let utf8_string = unsafe { String::from_utf8_unchecked(body.clone()) };
            println!("http: response as string: {}", utf8_string);
        }

        let response = HttpResponse {
            body: Body::from(body),
            status_code: StatusCode::from_u16(status).unwrap_or_default(),
            headers: response_headers,
        };

        if status >= 500 {
            return Err(HttpResponseError::Response { response });
        }

        return Ok(response);
    }
}

fn new_internal_str(reason: &str) -> HttpResponseError {
    HttpResponseError::Internal {
        message: reason.into(),
    }
}

fn new_internal_string(reason: String) -> HttpResponseError {
    HttpResponseError::Internal { message: reason }
}

#[derive(Debug)]
pub struct Body {
    pub(crate) payload: Vec<u8>,
}

impl From<Vec<u8>> for Body {
    fn from(value: Vec<u8>) -> Self {
        Body { payload: value }
    }
}

impl Body {
    pub fn as_bytes(&self) -> &[u8] {
        &self.payload
    }

    pub fn get_bytes(self) -> Vec<u8> {
        self.payload
    }

    /// Parse the request payload as JSON.
    ///
    /// # Errors
    pub fn as_json<T: DeserializeOwned>(&self) -> serde_json::Result<T> {
        serde_json::from_slice::<T>(&self.payload)
    }

    /// Parse the request body from form-urlencoded.
    ///
    /// # Errors
    #[allow(dead_code)]
    pub fn as_form<T: DeserializeOwned>(&self) -> Result<T, serde_urlencoded::de::Error> {
        serde_urlencoded::from_bytes::<T>(&self.payload)
    }
}

#[derive(Debug)]
pub struct MyHeaders {
    pub(crate) inner: Fields,
}

impl From<Fields> for MyHeaders {
    fn from(value: Fields) -> Self {
        Self { inner: value }
    }
}

impl MyHeaders {
    pub fn get(&self, name: &str) -> Option<String> {
        let header = self.inner.get(&name.to_string());
        if header.is_empty() {
            return None;
        }

        let parsed = String::from_utf8(header[0].clone());

        match parsed {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    }

    pub fn authorization(&self) -> Option<String> {
        self.get(AUTHORIZATION.as_str())
    }

    pub fn content_type(&self) -> Option<String> {
        self.get(CONTENT_TYPE.as_str())
    }

    pub fn content_length(&self) -> Option<String> {
        self.get(CONTENT_LENGTH.as_str())
    }

    /// Get the access token from the Authorization header. Return Err if not found.
    ///
    /// # Errors
    #[allow(dead_code)]
    pub fn ensure_access_token(&self) -> Result<String, EnsureTokenError> {
        let header = self.authorization();

        match header {
            None => Err(EnsureTokenError::NoToken),

            Some(value) => {
                let Some(token) = value.split_whitespace().last() else {
                    return Err(EnsureTokenError::NotParsed);
                };

                Ok(token.to_owned())
            }
        }
    }
}

#[derive(Debug)]
pub enum EnsureTokenError {
    NoToken,
    NotParsed,
}
