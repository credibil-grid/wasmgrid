//! # Handler

use std::fmt::Display;
use std::hash::{Hash, Hasher};

use anyhow::Result;
use wasi::http::types::Method as HttpMethod;

use crate::request::Request;
use crate::response::Response;

pub type Handler = fn(&Request) -> Result<Response>;

pub struct MethodHandler {
    pub method: Method,
    handler: Handler,
}

impl MethodHandler {
    /// Create a new method handler.
    pub fn new(method: HttpMethod, handler: Handler) -> Self {
        MethodHandler {
            method: Method(method),
            handler,
        }
    }

    pub fn handle(&self, request: &Request) -> Result<Response> {
        (self.handler)(request)
    }
}

pub fn get(handler: Handler) -> MethodHandler {
    MethodHandler {
        method: Method(HttpMethod::Get),
        handler,
    }
}

pub fn patch(handler: Handler) -> MethodHandler {
    MethodHandler {
        method: Method(HttpMethod::Patch),
        handler,
    }
}

pub fn post(handler: Handler) -> MethodHandler {
    MethodHandler {
        method: Method(HttpMethod::Post),
        handler,
    }
}

pub fn put(handler: Handler) -> MethodHandler {
    MethodHandler {
        method: Method(HttpMethod::Put),
        handler,
    }
}

pub fn delete(handler: Handler) -> MethodHandler {
    MethodHandler {
        method: Method(HttpMethod::Delete),
        handler,
    }
}

#[derive(Debug, Clone)]
pub struct Method(pub HttpMethod);

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl PartialEq for Method {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for Method {}

impl Hash for Method {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

impl From<HttpMethod> for Method {
    fn from(method: HttpMethod) -> Self {
        Method(method)
    }
}
