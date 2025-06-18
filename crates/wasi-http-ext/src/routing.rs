//! # Routing

use wasi::http::types::Method;

use crate::handler::MethodHandler;
use crate::request::Request;

pub struct Router {
    pub routes: Vec<Route>,
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

impl Router {
    /// Create a new router.
    #[must_use]
    pub const fn new() -> Self {
        Self { routes: Vec::new() }
    }

    /// Add a new route to the router.
    #[must_use]
    pub fn route(mut self, path: impl Into<String>, handler: MethodHandler) -> Self {
        self.routes.push(Route {
            path: path.into(),
            handler,
        });
        self
    }

    pub(crate) fn find(&self, request: &Request) -> Option<&Route> {
        self.routes.iter().find(|r| {
            request.uri().path().starts_with(&r.path)
                && is_match(&request.method(), &r.handler.method)
        })
    }
}

pub struct Route {
    pub path: String,
    pub handler: MethodHandler,
}

fn is_match(m1: &Method, m2: &Method) -> bool {
    match m1 {
        &Method::Get => matches!(m2, &Method::Get),
        &Method::Patch => matches!(m2, &Method::Patch),
        &Method::Post => matches!(m2, &Method::Post),
        &Method::Put => matches!(m2, &Method::Put),
        &Method::Delete => matches!(m2, &Method::Delete),
        _ => false,
    }
}
