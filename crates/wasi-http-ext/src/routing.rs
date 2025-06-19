//! # Routing

use std::collections::HashMap;
use std::sync::LazyLock;

use anyhow::Result;
use regex::Regex;
use wasi::http::types::Method;

use crate::handler::MethodHandler;
use crate::request::Request;

const PARAM_REGEX: &str = r"[-\w()@:%_+.~]+";
static ROUTE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"/(\{[-\w()@:%_+.~]+\})").expect("should compile"));

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
    pub fn route(mut self, route: impl Into<String>, handler: MethodHandler) -> Self {
        // create a regex to extract params from path
        let pattern: String = route.into();
        let mut matcher = pattern.clone();
        for (_, [param]) in ROUTE_REGEX.captures_iter(&pattern).map(|caps| caps.extract()) {
            let param_name = param.trim_start_matches('{').trim_end_matches('}');
            let param_regex = format!(r"(?<{param_name}>{PARAM_REGEX})",);
            matcher = matcher.replace(param, &param_regex);
        }

        let regex = Regex::new(&matcher).expect("should compile");
        self.routes.push(Route { regex, handler });

        self
    }

    pub(crate) fn find(&self, request: &Request) -> Option<(&Route, HashMap<String, String>)> {
        for r in &self.routes {
            if !is_match(&request.method(), &r.handler.method) {
                continue;
            }

            if let Some(caps) = r.regex.captures(request.uri().path()) {
                let mut params = HashMap::new();
                for n in r.regex.capture_names().filter_map(|n| n).collect::<Vec<&str>>() {
                    if let Some(c) = caps.name(n) {
                        params.insert(n.to_string(), c.as_str().to_string());
                    }
                }
                return Some((r, params));
            }
        }

        None
    }
}

pub struct Route {
    regex: Regex,
    pub handler: MethodHandler,
}

impl Route {
    pub fn handle(&self, request: &Request) -> Result<Vec<u8>> {
        self.handler.handle(request)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_route() {
        let pattern = "/{greeting}/world/{id}/test/{again}";

        // create a regex to extract params from path
        let mut matcher = pattern.to_string();
        for (_, [param]) in ROUTE_REGEX.captures_iter(pattern).map(|caps| caps.extract()) {
            let param_name = param.trim_start_matches('{').trim_end_matches('}');
            let param_regex = format!(r"(?<{param_name}>{PARAM_REGEX})",);
            matcher = matcher.replace(param, &param_regex);
        }

        // test url path against regex
        let route_regex = Regex::new(&format!("{matcher}")).unwrap();

        // check path for match
        let will_match = "/hello/world/my-id/test/repeated";
        let Some(caps) = route_regex.captures(will_match) else {
            panic!("should match");
        };

        let names: Vec<&str> = route_regex.capture_names().filter_map(|n| n).collect();
        assert_eq!(caps.len(), 4);
        assert_eq!(&caps[names[0]], "hello");
        assert_eq!(&caps[names[1]], "my-id");
        assert_eq!(&caps[names[2]], "repeated");

        // confirm regex does not match a different path
        let no_match = "/hello/auckland/my-id/test/repeated";
        assert!(route_regex.captures(no_match).is_none());
    }
}
