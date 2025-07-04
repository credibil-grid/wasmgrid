//! # Routing

use std::collections::{BTreeMap, HashMap};
use std::sync::LazyLock;

use anyhow::Result;
use regex::Regex;

use crate::handler::{Method, MethodHandler};
use crate::request::Request;
use crate::response::Response;

const PARAM_REGEX: &str = r"[-\w()@:%_+.~]+|https?://[-\w()@:%_+.~]+";
static ROUTE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"/(\{[-\w()@:%_+.~]+\})").expect("should compile"));

pub struct Router {
    pub routes: BTreeMap<String, HashMap<Method, Route>>,
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
        Self {
            routes: BTreeMap::new(),
        }
    }

    /// Add a new route to the router.
    #[must_use]
    pub fn route(mut self, route: impl Into<String>, handler: MethodHandler) -> Self {
        // create a regex to extract params from path
        let pattern = route.into();
        let mut matcher = pattern.clone();
        for (_, [param]) in ROUTE_REGEX.captures_iter(&pattern).map(|caps| caps.extract()) {
            let param_name = param.trim_start_matches('{').trim_end_matches('}');
            let param_regex = format!(r"(?<{param_name}>{PARAM_REGEX})",);
            matcher = matcher.replace(param, &param_regex);
        }
        let regex = Regex::new(&format!("^{matcher}$")).expect("should compile");

        self.routes
            .entry(pattern.clone())
            .or_default()
            .insert(handler.method.clone(), Route { regex, handler });
        self
    }

    pub(crate) fn find(&self, request: &Request) -> Option<(&Route, HashMap<String, String>)> {
        let uri = request.uri();
        let path = uri.path();
        let method = Method(request.method());

        for (pattern, routes) in self.routes.iter().rev() {
            tracing::trace!("{:?} {path}", request.method());
            tracing::trace!("{:?} {pattern}", routes.keys());

            let Some(route) = routes.get(&method) else {
                tracing::trace!("No matching method");
                continue;
            };

            if let Some(caps) = route.regex.captures(path) {
                tracing::debug!("`{pattern}` matched `{path}`");

                let mut params = HashMap::new();
                for n in route.regex.capture_names().filter_map(|n| n).collect::<Vec<&str>>() {
                    if let Some(c) = caps.name(n) {
                        params.insert(n.to_string(), c.as_str().to_string());
                    }
                }
                return Some((route, params));
            }
        }

        tracing::debug!("No matching route found for {path}");
        None
    }
}

pub struct Route {
    regex: Regex,
    pub handler: MethodHandler,
}

impl Route {
    pub fn handle(&self, request: &Request) -> Result<Response> {
        self.handler.handle(request)
    }
}

#[cfg(test)]
mod tests {
    use wasi::http::types::Method as HttpMethod;

    use super::*;

    #[test]
    fn match_route() {
        // create route
        let pattern = "/{greeting}/world/{id}/test/{again}";
        let router = Router::new().route(
            pattern,
            MethodHandler::new(HttpMethod::Get, |_| Ok(Response::from(vec![1, 2, 3]))),
        );
        let routes = router.routes.get(pattern).unwrap();

        // check path for match
        let method = Method(HttpMethod::Get);
        let will_match = "/hello/world/my-id/test/repeated";
        let Some(caps) = routes[&method].regex.captures(will_match) else {
            panic!("should match");
        };

        assert_eq!(caps.len(), 4);
        assert_eq!(&caps[1], "hello");
        assert_eq!(&caps[2], "my-id");
        assert_eq!(&caps[3], "repeated");

        // confirm regex does not match a different path
        let no_match = "/hello/auckland/my-id/test/repeated";
        assert!(routes[&method].regex.captures(no_match).is_none());
    }

    #[test]
    fn http_ids() {
        // create route
        let pattern = "/issuers/{issuer_id}/clients/{client_id}";
        let router = Router::new()
            .route(pattern, MethodHandler::new(HttpMethod::Get, |_| Ok(Response::from(vec![]))));
        let routes = router.routes.get(pattern).unwrap();

        // check path for match
        let method = Method(HttpMethod::Get);
        let will_match = "/issuers/http://issuer:8080/clients/http://wallet:8082";
        let Some(caps) = routes[&method].regex.captures(will_match) else {
            panic!("should match");
        };

        assert_eq!(caps.len(), 3);
        assert_eq!(&caps[1], "http://issuer:8080");
        assert_eq!(&caps[2], "http://wallet:8082");
    }
}
