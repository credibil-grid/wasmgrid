//! # Handler

use anyhow::Result;
use wasi::http::types::Method as WasiMethod;

use crate::request::{Method, Request};
use crate::response::Response;

// pub type Handler<T: Into<Response>> = fn(&Request) -> Result<T>;

pub struct MethodHandler<T: Into<Response>> {
    pub method: Method,
    handler: fn(&Request) -> Result<T>,
}

impl<T: Into<Response>> MethodHandler<T> {
    /// Create a new method handler.
    pub fn new(method: WasiMethod, handler: fn(&Request) -> Result<T>) -> Self {
        MethodHandler {
            method: Method(method),
            handler,
        }
    }

    pub fn handle(&self, request: &Request) -> Result<T> {
        (self.handler)(request)
    }
}

pub fn get<T: Into<Response>>(handler: fn(&Request) -> Result<T>) -> MethodHandler<T> {
    MethodHandler {
        method: Method(WasiMethod::Get),
        handler,
    }
}

pub fn patch<T: Into<Response>>(handler: fn(&Request) -> Result<T>) -> MethodHandler<T> {
    MethodHandler {
        method: Method(WasiMethod::Patch),
        handler,
    }
}

pub fn post<T: Into<Response>>(handler: fn(&Request) -> Result<T>) -> MethodHandler<T> {
    MethodHandler {
        method: Method(WasiMethod::Post),
        handler,
    }
}

pub fn put<T: Into<Response>>(handler: fn(&Request) -> Result<T>) -> MethodHandler<T> {
    MethodHandler {
        method: Method(WasiMethod::Put),
        handler,
    }
}

pub fn delete<T: Into<Response>>(handler: fn(&Request) -> Result<T>) -> MethodHandler<T> {
    MethodHandler {
        method: Method(WasiMethod::Delete),
        handler,
    }
}
