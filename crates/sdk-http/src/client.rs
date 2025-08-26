use anyhow::{Result, anyhow};
use bytes::Bytes;
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use http::uri::Authority;
use http::{HeaderMap, HeaderName, Response};
use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};
use serde::Serialize;
use serde::de::DeserializeOwned;
use wasi::http::outgoing_handler;
use wasi::http::types::{
    FutureIncomingResponse, Headers, Method, OutgoingBody, OutgoingRequest, Scheme,
};

use crate::uri::UriLike;

const UNRESERVED: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'=')
    .remove(b'&')
    .remove(b'.')
    .remove(b'_')
    .remove(b'-')
    .remove(b'~')
    .remove(b'/');

#[derive(Default)]
pub struct Client;

impl Client {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    pub fn get<U: Into<UriLike>>(&self, uri: U) -> RequestBuilder<NoBody, NoJson, NoForm> {
        RequestBuilder::new(uri)
    }

    pub fn post<U: Into<UriLike>>(&self, uri: U) -> RequestBuilder<NoBody, NoJson, NoForm> {
        RequestBuilder::new(uri).method(Method::Post)
    }
}

#[derive(Debug)]
pub struct RequestBuilder<B, J, F> {
    method: Method,
    uri: UriLike,
    headers: HeaderMap<String>,
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
    fn new<U: Into<UriLike>>(uri: U) -> Self {
        Self {
            method: Method::Get,
            uri: uri.into(),
            headers: HeaderMap::default(),
            query: None,
            body: NoBody,
            json: NoJson,
            form: NoForm,
        }
    }

    pub fn body(self, body: Vec<u8>) -> RequestBuilder<HasBody, NoJson, NoForm> {
        RequestBuilder {
            method: self.method,
            uri: self.uri,
            headers: self.headers,
            query: self.query,
            body: HasBody(body),
            json: NoJson,
            form: NoForm,
        }
    }

    pub fn json<T: Serialize>(self, json: T) -> RequestBuilder<NoBody, HasJson<T>, NoForm> {
        RequestBuilder {
            method: self.method,
            uri: self.uri,
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
            uri: self.uri,
            headers: self.headers,
            query: self.query,
            body: NoBody,
            json: NoJson,
            form: HasForm(form),
        }
    }
}

impl<B, J, F> RequestBuilder<B, J, F> {
    #[must_use]
    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    #[must_use]
    pub fn header(mut self, name: impl Into<HeaderName>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    #[must_use]
    pub fn headers(mut self, headers: &HeaderMap) -> Self {
        self.headers = headers
            .iter()
            .map(|(k, v)| (k.clone(), v.to_str().unwrap_or_default().to_string()))
            .collect();
        self
    }

    pub fn query(&mut self, query: impl Into<String>) -> &mut Self {
        self.query = Some(query.into());
        self
    }

    #[must_use]
    pub fn bearer_auth(mut self, token: &str) -> Self {
        self.headers.insert(AUTHORIZATION, format!("Bearer {token}"));
        self
    }
}

impl RequestBuilder<NoBody, NoJson, NoForm> {
    /// Send the request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send.
    pub fn send(&self) -> Result<Response<Bytes>> {
        self.send_bytes(None)
    }
}

impl RequestBuilder<HasBody, NoJson, NoForm> {
    /// Send the request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send.
    pub fn send(&self) -> Result<Response<Bytes>> {
        self.send_bytes(Some(&self.body.0))
    }
}

impl<B: Serialize> RequestBuilder<NoBody, HasJson<B>, NoForm> {
    /// Send the request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send.
    pub fn send(&mut self) -> Result<Response<Bytes>> {
        let body =
            serde_json::to_vec(&self.json.0).map_err(|e| anyhow!("issue serializing json: {e}"))?;
        self.headers.insert(CONTENT_TYPE, "application/json".into());
        self.send_bytes(Some(&body))
    }
}

impl<B: Serialize> RequestBuilder<NoBody, NoJson, HasForm<B>> {
    /// Send the request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to send.
    pub fn send(&mut self) -> Result<Response<Bytes>> {
        let body = credibil_encoding::form_encode(&self.form.0)
            .map_err(|e| anyhow!("issue serializing form: {e}"))?;
        let bytes =
            serde_json::to_vec(&body).map_err(|e| anyhow!("issue serializing form: {e}"))?;
        self.headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".into());
        self.send_bytes(Some(&bytes))
    }
}

impl<B, J, F> RequestBuilder<B, J, F> {
    fn send_bytes(&self, body: Option<&[u8]>) -> Result<Response<Bytes>> {
        let request = self.prepare_request(body)?;

        tracing::trace!(
            "sending request: {:?}://{}{}",
            request.scheme().unwrap_or(Scheme::Http),
            request.authority().unwrap_or_default(),
            request.path_with_query().unwrap_or_default()
        );

        let fut_resp = outgoing_handler::handle(request, None)
            .map_err(|e| anyhow!("issue making request: {e}"))?;
        Self::process_response(&fut_resp)
    }

    fn prepare_request(&self, body: Option<&[u8]>) -> Result<OutgoingRequest> {
        let headers = Headers::new();
        for (key, value) in &self.headers {
            headers
                .append(key.as_str(), value.as_bytes())
                .map_err(|e| anyhow!("issue setting header: {e}"))?;
        }
        let request = OutgoingRequest::new(headers);
        request.set_method(&self.method).map_err(|()| anyhow!("issue setting method"))?;

        // url
        let uri = self.uri.into_uri()?;
        let Some(scheme) = uri.scheme() else {
            return Err(anyhow!("missing scheme"));
        };
        let scheme = match scheme.as_str() {
            "http" => Scheme::Http,
            "https" => Scheme::Https,
            _ => return Err(anyhow!("unsupported scheme: {}", scheme.as_str())),
        };
        request.set_scheme(Some(&scheme)).map_err(|()| anyhow!("issue setting scheme"))?;
        request
            .set_authority(uri.authority().map(Authority::as_str))
            .map_err(|()| anyhow!("issue setting authority"))?;

        // path + query
        let path = uri.path().to_string();
        let mut path_with_query = utf8_percent_encode(&path, UNRESERVED).to_string();
        if let Some(query) = uri.query() {
            let query = utf8_percent_encode(query, UNRESERVED).to_string();
            path_with_query = format!("{path_with_query}?{query}");
        }
        tracing::trace!("encoded path_with_query: {path_with_query}");

        request
            .set_path_with_query(Some(&path_with_query))
            .map_err(|()| anyhow!("issue setting path_with_query"))?;

        let out_body = request.body().map_err(|()| anyhow!("issue getting outgoing body"))?;
        if let Some(mut buf) = body {
            let out_stream =
                out_body.write().map_err(|()| anyhow!("issue getting output stream"))?;

            let pollable = out_stream.subscribe();
            while !buf.is_empty() {
                pollable.block();
                let Ok(permit) = out_stream.check_write() else {
                    return Err(anyhow!("output stream is not writable"));
                };

                #[allow(clippy::cast_possible_truncation)]
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

    fn process_response(fut_resp: &FutureIncomingResponse) -> Result<Response<Bytes>> {
        fut_resp.subscribe().block();
        let Some(result) = fut_resp.get() else {
            return Err(anyhow!("missing response"));
        };
        let response = result
            .map_err(|()| anyhow!("issue getting response"))?
            .map_err(|e| anyhow!("response error: {e}"))?;

        // process body
        let body = response.consume().map_err(|()| anyhow!("issue getting body"))?;
        let stream = body.stream().map_err(|()| anyhow!("issue getting body's stream"))?;

        let mut body = vec![];
        while let Ok(chunk) = stream.blocking_read(1024 * 1024) {
            body.extend_from_slice(&chunk);
        }

        // transform unsuccessful requests into an error
        let status = response.status();

        if !(200..300).contains(&status) {
            let body = String::from_utf8_lossy(&body);
            return Err(anyhow!("request unsuccessful {status}, {body}"));
        }

        drop(stream);
        drop(response);

        Ok(Response::new(Bytes::from(body)))
    }
}

pub trait Decode {
    /// Decode the response body as JSON.
    ///
    /// # Errors
    ///
    /// Returns an error if the response body is not valid JSON.
    fn json<T: DeserializeOwned>(self) -> Result<T>;
}

impl Decode for Response<Bytes> {
    fn json<T: DeserializeOwned>(self) -> Result<T> {
        let body = self.into_body();
        let data = serde_json::from_slice::<T>(&body)?;
        Ok(data)
    }
}
