use anyhow::anyhow;
use http::header::CONTENT_TYPE; // AUTHORIZATION, CONTENT_LENGTH,
use http::StatusCode;
use serde::de::DeserializeOwned;
use serde::Serialize; // Deserialize
use url::Url;
use wasi::http::outgoing_handler::{self}; // ErrorCode
use wasi::http::types::{Fields, Headers, Method, OutgoingBody, OutgoingRequest, Scheme};

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

        // set body
        if let Some(bytes) = &self.body {
            let body = request.body().map_err(|()| anyhow!("issue getting body"))?;
            let stream = body.write().map_err(|()| anyhow!("issue getting stream"))?;
            stream
                .blocking_write_and_flush(bytes)
                .map_err(|e| anyhow!("issue writing body: {e}"))?;
            drop(stream);

            OutgoingBody::finish(body, None).map_err(|e| anyhow!("issue finishing body: {e}"))?;
        };

        // send request
        let fut_resp = outgoing_handler::handle(request, None)
            .map_err(|e| anyhow!("issue making request: {e}"))?;

        // get response
        fut_resp.subscribe().block();
        let Some(result) = fut_resp.get() else {
            return Err(anyhow!("missing response"));
        };
        let response = result.map_err(|()| anyhow!("Response taken"))??;

        // --------------------------------------------------------------------
        // Process response
        // --------------------------------------------------------------------
        let mut resp = Response {
            body: vec![],
            status: StatusCode::from_u16(response.status()).unwrap_or_default(),
            headers: MyHeaders {
                inner: response.headers().clone(),
            },
        };

        // body
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

pub struct MyHeaders {
    pub(crate) inner: Fields,
}

pub struct Response {
    pub status: StatusCode,
    pub body: Vec<u8>,
    pub headers: MyHeaders,
}

impl Response {
    // pub fn as_bytes(&self) -> &[u8] {
    //     &self.body
    // }

    /// Parse the request payload as JSON.
    ///
    /// # Errors
    pub fn json<T: DeserializeOwned>(&self) -> serde_json::Result<T> {
        serde_json::from_slice::<T>(&self.body)
        // todo!()
    }
}
