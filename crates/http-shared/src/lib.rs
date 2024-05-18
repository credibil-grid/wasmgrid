#![feature(let_chains)]
#![feature(fn_traits)]
#![feature(lazy_cell)]
#![feature(trait_alias)]

pub mod request;

use http::header::CONTENT_TYPE;
pub use request::Request;
use wasi::http::types::{ErrorCode, Headers, IncomingRequest, OutgoingBody, OutgoingResponse};

pub trait Handler = Fn(&Request) -> anyhow::Result<Vec<u8>>;

pub struct Router {
    routes: Vec<Route>,
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
    pub fn route(self, path: impl Into<String>, handler: impl Handler + 'static) -> Self {
        let route = Route {
            path: path.into(),
            handler: Box::new(handler),
        };

        let mut routes = self.routes;
        routes.push(route);

        Self { routes }
    }
}

pub struct Route {
    path: String,
    handler: Box<dyn Handler>,
}

/// Serve an incoming request using the provided router.
///
/// # Errors
///
/// Returns a [`wasi::http::types::ErrorCode`] if the request could not be served.
pub fn serve(router: &Router, request: &IncomingRequest) -> Result<OutgoingResponse, ErrorCode> {
    let req = request::Request::from(request);

    let Some(route) = router.routes.iter().find(|r| req.uri().path().starts_with(&r.path)) else {
        return Err(ErrorCode::DestinationNotFound);
    };

    // serialize result
    let result = (route.handler)(&req);
    let content = match result {
        Ok(resp) => resp,
        Err(err) => {
            let mapped = err.downcast_ref::<vercre_vci::Error>().map_or_else(
                || serde_json::json!({"error": "server_error", "error_description": err.to_string()}),
                vercre_vci::Error::to_json,
            );

            tracing::error!("{}", mapped);

            let Ok(ser) = serde_json::to_vec(&mapped) else {
                return Err(ErrorCode::InternalError(Some(
                    "failed to serialize error".to_string(),
                )));
            };
            ser
        }
    };

    // create outgoing response
    let headers = Headers::new();
    headers
        .set(&CONTENT_TYPE.to_string(), &[b"application/json".to_vec()])
        .map_err(|e| ErrorCode::InternalError(Some(e.to_string())))?;

    let resp = OutgoingResponse::new(headers);

    // write outgoing body
    let body = resp
        .body()
        .map_err(|()| ErrorCode::InternalError(Some("outgoing Body unavailable".to_string())))?;
    let stream = body
        .write()
        .map_err(|()| ErrorCode::InternalError(Some("output-stream unavailable".to_string())))?;
    stream
        .blocking_write_and_flush(&content)
        .map_err(|e| ErrorCode::InternalError(Some(e.to_string())))?;
    drop(stream);

    OutgoingBody::finish(body, None)?;

    Ok(resp)
}
