use std::ops::Deref;

use http::StatusCode;
use serde::de::DeserializeOwned;

/// Top-level response data structure common to all handler.
#[derive(Clone, Debug, Default)]
pub struct Response {
    /// Response HTTP status code.
    pub status: StatusCode,

    /// The endpoint-specific response.
    pub body: Vec<u8>,
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

impl From<Vec<u8>> for Response {
    fn from(body: Vec<u8>) -> Self {
        Self {
            status: StatusCode::OK,
            body,
        }
    }
}

impl Deref for Response {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.body
    }
}
