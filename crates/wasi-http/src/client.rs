use anyhow::anyhow;
use http::header::CONTENT_TYPE; // AUTHORIZATION, CONTENT_LENGTH,
use http::StatusCode;
use serde::de::DeserializeOwned;
use serde::Serialize;
use url::Url;
use wasi::http::outgoing_handler;
use wasi::http::types::{Headers, Method, OutgoingBody, OutgoingRequest, Scheme};

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
    query: Option<String>,
    body: Option<Vec<u8>>,
    errors: Vec<String>,
}

impl RequestBuilder {
    fn new(method: Method, url: String) -> Self {
        Self {
            method,
            url,
            headers: Headers::new(),
            query: None,
            body: None,
            errors: Vec::new(),
        }
    }

    pub fn header(&mut self, name: &str, value: &str) -> &mut Self {
        let _ = self.headers.append(&name.to_string(), &value.as_bytes().to_vec()).map_err(|e| {
            self.errors.push(format!("issue setting header: {e}"));
        });
        self
    }

    pub fn json(&mut self, json: impl Serialize) -> &mut Self {
        self.body = match serde_json::to_vec(&json) {
            Ok(bytes) => Some(bytes),
            Err(e) => {
                self.errors.push(format!("issue serializing body: {e}"));
                return self;
            }
        };
        self.header(CONTENT_TYPE.as_str(), "application/json");
        self
    }

    pub fn query(&mut self, query: &str) -> &mut Self {
        self.query = match Url::parse(query) {
            Ok(url) => url.query().map(|s| s.to_string()),
            Err(e) => {
                self.errors.push(format!("issue serializing body: {e}"));
                return self;
            }
        };
        self
    }

    // pub fn body(&mut self, body: impl Into<Body>) -> &mut Self {
    //     self
    // }
}

impl RequestBuilder {
    pub fn send(&self) -> anyhow::Result<Response> {
        // builder errors
        if !self.errors.is_empty() {
            return Err(anyhow!("issue(s) building request: {}", self.errors.join("\n")));
        }

        // --------------------------------------------------------------------
        // Create request
        // --------------------------------------------------------------------
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

        // set body, if provided
        if let Some(bytes) = &self.body {
            let body = request.body().map_err(|()| anyhow!("issue getting body"))?;
            let stream = body.write().map_err(|()| anyhow!("issue getting stream"))?;
            stream
                .blocking_write_and_flush(bytes)
                .map_err(|e| anyhow!("issue writing body: {e}"))?;

            drop(stream);
            OutgoingBody::finish(body, None).map_err(|e| anyhow!("issue finishing body: {e}"))?;
        };

        // send
        let fut_resp = outgoing_handler::handle(request, None)
            .map_err(|e| anyhow!("issue making request: {e}"))?;

        // --------------------------------------------------------------------
        // Process response
        // --------------------------------------------------------------------
        fut_resp.subscribe().block();
        let Some(result) = fut_resp.get() else {
            return Err(anyhow!("missing response"));
        };

        let response = result
            .map_err(|()| anyhow!("issue getting response"))?
            .map_err(|e| anyhow!("response error: {e}"))?;

        if response.status() != StatusCode::OK {
            return Err(anyhow!("unexpected status: {}", response.status()));
        }

        let mut resp = Response { body: vec![] };

        // process body
        let body = response.consume().map_err(|()| anyhow!("issue getting body"))?;
        let stream = body.stream().map_err(|()| anyhow!("issue getting body's stream"))?;
        while let Ok(chunk) = stream.blocking_read(1024 * 1024) {
            resp.body.extend_from_slice(&chunk);
        }

        drop(stream);
        drop(response);

        return Ok(resp);
    }
}

pub struct Response {
    body: Vec<u8>,
}

impl Response {
    pub fn as_bytes(&self) -> &[u8] {
        &self.body
    }

    /// Parse the request payload as JSON.
    ///
    /// # Errors
    pub fn json<T: DeserializeOwned>(&self) -> serde_json::Result<T> {
        serde_json::from_slice::<T>(&self.body)
    }
}
