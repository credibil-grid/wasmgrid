//! # Handler

use anyhow::Result;
use wasi::http::types::Method;

use crate::request::Request;

pub trait Handler = Fn(&Request) -> Result<Vec<u8>>;

pub struct MethodHandler {
    pub method: Method,
    handler: Box<dyn Handler>,
}

impl MethodHandler {
    pub fn handle(&self, request: &Request) -> Result<Vec<u8>> {
        (self.handler)(request)
    }
}

pub fn get(handler: impl Handler + 'static) -> MethodHandler {
    MethodHandler {
        method: Method::Get,
        handler: Box::new(handler),
    }
}

pub fn patch(handler: impl Handler + 'static) -> MethodHandler {
    MethodHandler {
        method: Method::Patch,
        handler: Box::new(handler),
    }
}

pub fn post(handler: impl Handler + 'static) -> MethodHandler {
    MethodHandler {
        method: Method::Post,
        handler: Box::new(handler),
    }
}

pub fn put(handler: impl Handler + 'static) -> MethodHandler {
    MethodHandler {
        method: Method::Put,
        handler: Box::new(handler),
    }
}

pub fn delete(handler: impl Handler + 'static) -> MethodHandler {
    MethodHandler {
        method: Method::Delete,
        handler: Box::new(handler),
    }
}
