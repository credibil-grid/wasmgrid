use std::ops::Deref;

use http::StatusCode;

/// Top-level response data structure common to all handler.
#[derive(Clone, Debug)]
pub struct Response {
    /// Response HTTP status code.
    pub status: StatusCode,

    /// The endpoint-specific response.
    pub body: Vec<u8>,
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
