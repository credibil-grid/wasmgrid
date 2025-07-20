//! # Handler

use anyhow::Result;
use wasi::http::types::Method as WasiMethod;

use crate::request::{Method, Request};
use crate::response::Response;

pub type Handler = fn(&Request) -> Result<Response>;

pub struct MethodHandler {
    pub method: Method,
    handler: Handler,
}

impl MethodHandler {
    /// Create a new method handler.
    pub fn new(method: WasiMethod, handler: Handler) -> Self {
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
        method: Method(WasiMethod::Get),
        handler,
    }
}

pub fn patch(handler: Handler) -> MethodHandler {
    MethodHandler {
        method: Method(WasiMethod::Patch),
        handler,
    }
}

pub fn post(handler: Handler) -> MethodHandler {
    MethodHandler {
        method: Method(WasiMethod::Post),
        handler,
    }
}

pub fn put(handler: Handler) -> MethodHandler {
    MethodHandler {
        method: Method(WasiMethod::Put),
        handler,
    }
}

pub fn delete(handler: Handler) -> MethodHandler {
    MethodHandler {
        method: Method(WasiMethod::Delete),
        handler,
    }
}
