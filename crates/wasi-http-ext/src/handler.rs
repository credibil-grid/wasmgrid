//! # Handler

use std::fmt::Display;
use std::hash::{Hash, Hasher};

use anyhow::Result;
use wasi::http::types::Method as HttpMethod;

use crate::request::Request;

pub trait Handler = Fn(&Request) -> Result<Vec<u8>>;

pub struct MethodHandler {
    pub method: Method,
    handler: Box<dyn Handler>,
}

impl MethodHandler {
    /// Create a new method handler.
    pub fn new(method: HttpMethod, handler: impl Handler + 'static) -> Self {
        MethodHandler {
            method: Method(method),
            handler: Box::new(handler),
        }
    }

    pub fn handle(&self, request: &Request) -> Result<Vec<u8>> {
        (self.handler)(request)
    }
}

pub fn get(handler: impl Handler + 'static) -> MethodHandler {
    MethodHandler {
        method: Method(HttpMethod::Get),
        handler: Box::new(handler),
    }
}

pub fn patch(handler: impl Handler + 'static) -> MethodHandler {
    MethodHandler {
        method: Method(HttpMethod::Patch),
        handler: Box::new(handler),
    }
}

pub fn post(handler: impl Handler + 'static) -> MethodHandler {
    MethodHandler {
        method: Method(HttpMethod::Post),
        handler: Box::new(handler),
    }
}

pub fn put(handler: impl Handler + 'static) -> MethodHandler {
    MethodHandler {
        method: Method(HttpMethod::Put),
        handler: Box::new(handler),
    }
}

pub fn delete(handler: impl Handler + 'static) -> MethodHandler {
    MethodHandler {
        method: Method(HttpMethod::Delete),
        handler: Box::new(handler),
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
