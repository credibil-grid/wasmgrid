use anyhow::anyhow;
use http::header::CONTENT_TYPE; // AUTHORIZATION, CONTENT_LENGTH,
use http::StatusCode;
use serde::de::DeserializeOwned;
use serde::Serialize; // Deserialize
use url::Url;
use wasi::http::outgoing_handler::{self}; // ErrorCode
use wasi::http::types::{Fields, Headers, Method, OutgoingBody, OutgoingRequest, Scheme};
use wasi::io::streams;

pub struct Client {}

impl Client {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, url: impl Into<String>) -> RequestBuilder {
        RequestBuilder::new(Method::Get, url.into())
    }

    pub fn post(&self, url: impl Into<String>) -> RequestBuilder {
        RequestBuilder::new(Method::Post, url.into())
    }
}

#[derive(Debug)]
pub struct RequestBuilder {
    method: Method,
    url: String,
    headers: Headers,
    body: Option<Vec<u8>>,
    errors: Vec<String>,
}

impl RequestBuilder {
    fn new(method: Method, url: String) -> Self {
        Self {
            method,
            url,
            headers: Headers::new(),
            body: None,
            errors: Vec::new(),
        }
    }

    pub fn header(&mut self, _name: &str, _value: &str) -> &mut Self {
        self
    }

    pub fn json(&mut self, json: impl Serialize) -> &mut Self {
        let bytes = match serde_json::to_vec(&json) {
            Ok(bytes) => bytes,
            Err(e) => {
                self.errors.push(e.to_string());
                return self;
            }
        };

        if let Err(e) =
            self.headers.append(&CONTENT_TYPE.to_string(), &b"application/json".to_vec())
        {
            self.errors.push(e.to_string());
            return self;
        };

        self.body = Some(bytes);
        self
    }

    pub fn query(&mut self, _: impl Serialize) -> &mut Self {
        self
    }

    // pub fn body(&mut self, body: impl Into<Body>) -> &mut Self {
    //     self
    // }
}

impl RequestBuilder {
    pub fn send(&self) -> anyhow::Result<Response> {
        if !self.errors.is_empty() {
            return Err(anyhow!("issue(s) building request: {}", self.errors.join("\n")));
        }

        let url = Url::parse(&self.url).map_err(|e| anyhow!("issue parsing url: {e}"))?;

        let request = OutgoingRequest::new(self.headers.clone());
        request.set_method(&self.method).map_err(|()| anyhow!("issue setting method"))?;
        request
            .set_authority(Some(url.authority()))
            .map_err(|()| anyhow!("issue setting authority"))?;

        let scheme = match url.scheme() {
            "http" => Scheme::Http,
            "https" => Scheme::Https,
            _ => return Err(anyhow!("unsupported scheme: {}", url.scheme())),
        };
        request.set_scheme(Some(&scheme)).map_err(|()| anyhow!("issue setting scheme"))?;

        let mut path_and_query = url.path().to_string();
        if let Some(query) = url.query() {
            path_and_query = format!("{}?{}", path_and_query, query);
        }
        request
            .set_path_with_query(Some(&path_and_query))
            .map_err(|()| anyhow!("Failed to set path_with_query"))?;

        if let Some(b) = &self.body {
            let body = request.body().map_err(|()| anyhow!("issue getting body"))?;
            let stream = body.write().map_err(|()| anyhow!("issue getting stream"))?;
            stream.blocking_write_and_flush(b).map_err(|e| anyhow!("issue writing body: {e}"))?;
            // stream
            //     .write_all(&some_payload)
            //     .map_err(|e| new_internal_string(format!("write_all failed {e}")))?;
            drop(stream);

            OutgoingBody::finish(body, None).map_err(|e| anyhow!("output stream error: {e}"))?;
        };

        // send request
        let result = outgoing_handler::handle(request, None)
            .map_err(|e| anyhow!("issue making request: {e}"))?;

        // process response
        let resp = match result.get() {
            Some(result) => result,
            None => {
                result.subscribe().block();
                let Some(result) = result.get() else {
                    return Err(anyhow!("Response missing"));
                };
                result
            }
        }
        .map_err(|()| anyhow!("Response taken"))??;
        drop(result);

        let status = resp.status();

        let headers_handle = resp.headers();
        let cloned_headers = headers_handle.clone();

        drop(headers_handle);

        let response_headers = MyHeaders {
            inner: cloned_headers,
        };

        let resp_body =
            resp.consume().map_err(|()| anyhow!("incoming response has no body stream"))?;

        drop(resp);

        let body_stream =
            resp_body.stream().map_err(|()| anyhow!("failed to create response stream"))?;
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

        println!("http: response bytes len={}", body.len());

        if body.len() < 1000 {
            let utf8_string = unsafe { String::from_utf8_unchecked(body.clone()) };
            println!("http: response as string: {}", utf8_string);
        }

        let response = Response {
            body: Body { payload: body },
            status_code: StatusCode::from_u16(status).unwrap_or_default(),
            headers: response_headers,
        };

        if status >= 500 {
            //return Err(HttpResponseError::Response { response });
        }

        return Ok(response);
    }
}

pub struct MyHeaders {
    pub(crate) inner: Fields,
}

pub struct Response {
    pub status_code: StatusCode,
    pub body: Body,
    pub headers: MyHeaders,
}

impl Response {
    // pub fn as_bytes(&self) -> &[u8] {
    //     &self.payload
    // }

    /// Parse the request payload as JSON.
    ///
    /// # Errors
    pub fn json<T: DeserializeOwned>(&self) -> serde_json::Result<T> {
        // serde_json::from_slice::<T>(&self.payload)
        todo!()
    }
}

pub struct Body {
    pub(crate) payload: Vec<u8>,
}

impl Body {
    pub fn as_bytes(&self) -> &[u8] {
        &self.payload
    }
}
