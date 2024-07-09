use anyhow::{anyhow, Result};
use http::header::AUTHORIZATION;
use http::Uri;
use serde::de::DeserializeOwned;
use wasi::http::types::{Fields, IncomingRequest, Method, Scheme};

#[derive(Clone)]
pub struct Request<'a> {
    inner: &'a IncomingRequest,
}

impl<'a> From<&'a IncomingRequest> for Request<'a> {
    fn from(inner: &'a IncomingRequest) -> Self {
        Self { inner }
    }
}

impl<'a> Request<'a> {
    #[must_use]
    pub fn uri(&self) -> Uri {
        let p_and_q = self.inner.path_with_query().unwrap_or_default();
        p_and_q.parse::<Uri>().unwrap_or_else(|_| Uri::default())
    }

    pub fn method(&self) -> String {
        method_string(self.inner.method())
    }

    /// Get the host the request was made to (using scheme and authority).
    ///
    /// # Errors
    pub fn host(&self) -> Result<String> {
        let authority = self.inner.authority().unwrap_or_default();
        let scheme = match self.inner.scheme() {
            Some(Scheme::Http) => String::from("http"),
            Some(Scheme::Https) => String::from("https"),
            Some(Scheme::Other(s)) => s,
            None => return Err(anyhow!("Scheme is missing")),
        };

        Ok(format!("{scheme}://{authority}"))
    }

    /// Get the access token from the Authorization header.
    ///
    /// # Errors
    #[allow(dead_code)]
    pub fn access_token(&self) -> Result<String> {
        let header = self.inner.headers().get(&AUTHORIZATION.to_string());
        if header.is_empty() {
            return Err(anyhow!("Authorization header is missing"));
        }
        let Ok(value) = String::from_utf8(header[0].clone()) else {
            return Err(anyhow!("Authorization header is not valid UTF-8"));
        };
        let Some(token) = value.split_whitespace().last() else {
            return Err(anyhow!("Authorization header is missing token"));
        };
        Ok(token.to_owned())
    }

    /// Get the request headers.
    #[must_use]
    pub fn headers(&self) -> Fields {
        self.inner.headers()
    }

    /// Request body.
    ///
    /// # Errors
    pub fn body(&self) -> Result<Vec<u8>> {
        let body = self.inner.consume().map_err(|()| anyhow!("error consuming request body"))?;
        let stream = body.stream().map_err(|()| anyhow!("error getting body stream"))?;

        // read body into a buffer.
        let mut buffer = Vec::new();
        while let Ok(bytes) = stream.blocking_read(4096)
            && !bytes.is_empty()
        {
            buffer.extend_from_slice(&bytes);
        }
        drop(stream);

        Ok(buffer)
    }

    /// Parse the request body from JSON.
    ///
    /// # Errors
    pub fn json<T: DeserializeOwned>(&self) -> Result<T> {
        Ok(serde_json::from_slice::<T>(&self.body()?)?)
    }

    /// Parse the request body from form-urlencoded.
    ///
    /// # Errors
    #[allow(dead_code)]
    pub fn form<T: DeserializeOwned>(&self) -> Result<T> {
        Ok(serde_urlencoded::from_bytes::<T>(&self.body()?)?)
    }
}

pub(crate) fn method_string(m: Method) -> String {
    match m {
        Method::Get => String::from("GET"),
        Method::Post => String::from("POST"),
        Method::Put => String::from("PUT"),
        Method::Delete => String::from("DELETE"),
        Method::Head => String::from("HEAD"),
        Method::Connect => String::from("CONNECT"),
        Method::Options => String::from("OPTIONS"),
        Method::Trace => String::from("TRACE"),
        Method::Patch => String::from("PATCH"),
        Method::Other(s) => s,
    }
}
