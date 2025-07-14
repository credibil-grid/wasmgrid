use anyhow::{Result, anyhow};
use http::Uri;
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};
use serde::Serialize;
use wasi::http::outgoing_handler;
use wasi::http::types::{
    FutureIncomingResponse, Headers, Method, OutgoingBody, OutgoingRequest, Scheme,
};

const UNRESERVED: &AsciiSet =
    &NON_ALPHANUMERIC.remove(b'.').remove(b'_').remove(b'-').remove(b'~').remove(b'/');
use crate::response::Response;

pub struct Client {}

impl Client {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, url: impl Into<String>) -> RequestBuilder<NoBody, NoJson, NoForm> {
        RequestBuilder::new(url)
    }

    pub fn post(&self, url: impl Into<String>) -> RequestBuilder<NoBody, NoJson, NoForm> {
        RequestBuilder::new(url).method(Method::Post)
    }
}

#[derive(Debug)]
pub struct RequestBuilder<B, J, F> {
    method: Method,
    url: String,
    headers: Vec<(String, String)>,
    query: Option<String>,
    body: B,
    json: J,
    form: F,
}

/// Builder has no body.
#[doc(hidden)]
pub struct NoBody;
/// Builder has a body.
#[doc(hidden)]
pub struct HasBody(Vec<u8>);

/// Builder has no json.
#[doc(hidden)]
pub struct NoJson;
/// Builder has a body.
#[doc(hidden)]
pub struct HasJson<T: Serialize>(T);

/// Builder has no json.
#[doc(hidden)]
pub struct NoForm;
/// Builder has a body.
#[doc(hidden)]
pub struct HasForm<T: Serialize>(T);

impl RequestBuilder<NoBody, NoJson, NoForm> {
    fn new(url: impl Into<String>) -> Self {
        Self {
            method: Method::Get,
            url: url.into(),
            headers: vec![],
            query: None,
            body: NoBody,
            json: NoJson,
            form: NoForm,
        }
    }

    pub fn body(self, body: &[u8]) -> RequestBuilder<HasBody, NoJson, NoForm> {
        RequestBuilder {
            method: self.method.clone(),
            url: self.url.clone(),
            headers: self.headers.clone(),
            query: self.query.clone(),
            body: HasBody(body.to_vec()),
            json: NoJson,
            form: NoForm,
        }
    }

    pub fn json<T: Serialize>(self, json: T) -> RequestBuilder<NoBody, HasJson<T>, NoForm> {
        RequestBuilder {
            method: self.method,
            url: self.url,
            headers: self.headers,
            query: self.query,
            body: NoBody,
            json: HasJson(json),
            form: NoForm,
        }
    }

    pub fn form<T: Serialize>(self, form: T) -> RequestBuilder<NoBody, NoJson, HasForm<T>> {
        RequestBuilder {
            method: self.method,
            url: self.url,
            headers: self.headers,
            query: self.query,
            body: NoBody,
            json: NoJson,
            form: HasForm(form),
        }
    }
}

impl<B, J, F> RequestBuilder<B, J, F> {
    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    pub fn query(&mut self, query: impl Into<String>) -> &mut Self {
        self.query = Some(query.into());
        self
    }

    pub fn bearer_auth(mut self, token: &str) -> Self {
        self.headers.push((AUTHORIZATION.to_string(), format!("Bearer {token}")));
        self
    }
}

impl RequestBuilder<NoBody, NoJson, NoForm> {
    pub fn send(&self) -> Result<Response> {
        self.send_any(None)
    }
}

impl RequestBuilder<HasBody, NoJson, NoForm> {
    pub fn send(&self) -> Result<Response> {
        self.send_any(Some(&self.body.0))
    }
}

impl<T: Serialize> RequestBuilder<NoBody, HasJson<T>, NoForm> {
    pub fn send(&mut self) -> Result<Response> {
        let body =
            serde_json::to_vec(&self.json.0).map_err(|e| anyhow!("issue serializing json: {e}"))?;
        self.headers.push((CONTENT_TYPE.to_string(), "application/json".to_string()));
        self.send_any(Some(&body))
    }
}

impl<T: Serialize> RequestBuilder<NoBody, NoJson, HasForm<T>> {
    pub fn send(&mut self) -> Result<Response> {
        let body = credibil_core::html::form_encode(&self.form.0)
            .map_err(|e| anyhow!("issue serializing form: {e}"))?;
        let bytes =
            serde_json::to_vec(&body).map_err(|e| anyhow!("issue serializing form: {e}"))?;
        self.headers
            .push((CONTENT_TYPE.to_string(), "application/x-www-form-urlencoded".to_string()));
        self.send_any(Some(&bytes))
    }
}

impl<B, J, F> RequestBuilder<B, J, F> {
    pub fn send_any(&self, body: Option<&[u8]>) -> Result<Response> {
        let request = self.prepare_request(body)?;

        tracing::trace!(
            "sending request: {:?}://{}{}",
            request.scheme().unwrap_or(Scheme::Http),
            request.authority().unwrap_or_default(),
            request.path_with_query().unwrap_or_default()
        );

        let fut_resp = outgoing_handler::handle(request, None)
            .map_err(|e| anyhow!("issue making request: {e}"))?;
        Self::process_response(fut_resp)
    }

    fn prepare_request(&self, body: Option<&[u8]>) -> Result<OutgoingRequest> {
        let headers = Headers::new();
        for (key, value) in self.headers.iter() {
            headers
                .append(&key, value.as_bytes())
                .map_err(|e| anyhow!("issue setting header: {e}"))?;
        }
        let request = OutgoingRequest::new(headers);
        request.set_method(&self.method).map_err(|_| anyhow!("issue setting method"))?;

        // url
        let url = &self.url.parse::<Uri>().map_err(|e| anyhow!("issue parsing url: {e}"))?;
        let Some(scheme) = url.scheme() else {
            return Err(anyhow!("missing scheme"));
        };
        let scheme = match scheme.as_str() {
            "http" => Scheme::Http,
            "https" => Scheme::Https,
            _ => return Err(anyhow!("unsupported scheme: {}", scheme.as_str())),
        };
        request.set_scheme(Some(&scheme)).map_err(|()| anyhow!("issue setting scheme"))?;
        request
            .set_authority(url.authority().map(|a| a.as_str()))
            .map_err(|()| anyhow!("issue setting authority"))?;

        // path + query
        let path = url.path().to_string();
        let mut path_with_query = utf8_percent_encode(&path, UNRESERVED).to_string();
        if let Some(query) = url.query() {
            let query = utf8_percent_encode(query, UNRESERVED).to_string();
            path_with_query = format!("{path_with_query}?{query}");
        }
        tracing::trace!("encoded path_with_query: {path_with_query}");

        request
            .set_path_with_query(Some(&path_with_query))
            .map_err(|()| anyhow!("issue setting path_with_query"))?;

        let out_body = request.body().map_err(|_| anyhow!("issue getting outgoing body"))?;
        if let Some(mut buf) = body {
            let out_stream =
                out_body.write().map_err(|_| anyhow!("issue getting output stream"))?;

            let pollable = out_stream.subscribe();
            while !buf.is_empty() {
                pollable.block();
                let Ok(permit) = out_stream.check_write() else {
                    return Err(anyhow!("output stream is not writable"));
                };
                let len = buf.len().min(permit as usize);
                let (chunk, rest) = buf.split_at(len);
                if out_stream.write(chunk).is_err() {
                    return Err(anyhow!("issue writing to output stream"));
                }
                buf = rest;
            }

            if out_stream.flush().is_err() {
                return Err(anyhow!("issue flushing output stream"));
            }

            pollable.block();
            if out_stream.check_write().is_err() {
                return Err(anyhow!("output stream error"));
            }
        }

        OutgoingBody::finish(out_body, None)?;
        Ok(request)
    }

    fn process_response(fut_resp: FutureIncomingResponse) -> Result<Response> {
        fut_resp.subscribe().block();
        let Some(result) = fut_resp.get() else {
            return Err(anyhow!("missing response"));
        };
        let response = result
            .map_err(|()| anyhow!("issue getting response"))?
            .map_err(|e| anyhow!("response error: {e}"))?;

        // turn unsuccessful requests into an error
        if response.status() < 200 || response.status() >= 300 {
            return Err(anyhow!("unexpected status: {}", response.status()));
        }

        // process body
        let mut resp = Response::default();
        let body = response.consume().map_err(|()| anyhow!("issue getting body"))?;
        let stream = body.stream().map_err(|()| anyhow!("issue getting body's stream"))?;
        while let Ok(chunk) = stream.blocking_read(1024 * 1024) {
            resp.body.extend_from_slice(&chunk);
        }

        drop(stream);
        drop(response);

        Ok(resp)
    }
}
