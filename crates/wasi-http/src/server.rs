use std::cmp;

use http::header::CONTENT_TYPE;
use wasi::http::types::{ErrorCode, Headers, OutgoingBody, OutgoingResponse};

use crate::request::Request;

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
pub fn serve<'a>(
    router: &Router, request: impl Into<Request<'a>>,
) -> Result<OutgoingResponse, ErrorCode> {
    let req: Request = request.into();

    let Some(route) = router.routes.iter().find(|r| req.uri().path().starts_with(&r.path)) else {
        return Err(ErrorCode::DestinationNotFound);
    };

    // serialize result
    let mut content = match (route.handler)(&req) {
        Ok(resp) => resp,
        Err(err) => {
            tracing::error!("{}", err);
            let err_json =
                serde_json::json!({"error": "server_error", "error_description": err.to_string()});
            let Ok(ser) = serde_json::to_vec(&err_json) else {
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
        .map_err(|e| ErrorCode::InternalError(Some(format!("issue setting header: {e}"))))?;

    

    let resp = OutgoingResponse::new(headers);

    // write outgoing body
    let body = resp
        .body()
        .map_err(|()| ErrorCode::InternalError(Some("issue getting outgoing body".into())))?;
    let stream = body
        .write()
        .map_err(|()| ErrorCode::InternalError(Some("issue getting body stream".into())))?;

    // write to stream in chunks as max bytes for `blocking_write_and_flush` is 4096
    let pollable = stream.subscribe();
    while !content.is_empty() {
        // wait for the stream to become writable
        pollable.block();

        // get number of bytes that can be written
        let n = stream
            .check_write()
            .map_err(|e| ErrorCode::InternalError(Some(format!("issue checking write: {e}"))))?;

        // write a chunk of data
        let mid = cmp::min(n as usize, content.len());
        let (chunk, remaining) = content.split_at(mid);
        if let Err(e) = stream.write(chunk) {
            return Err(ErrorCode::InternalError(Some(format!("issue writing to stream: {e}"))));
        };

        content = remaining.to_vec();
    }

    if let Err(e) = stream.flush() {
        return Err(ErrorCode::InternalError(Some(format!("issue flushing stream: {e}"))));
    };
    pollable.block();

    // check for any errors
    if let Err(e) = stream.check_write() {
        return Err(ErrorCode::InternalError(Some(format!("issue writing to stream: {e}"))));
    };

    drop(pollable);
    drop(stream);

    if let Err(e) = OutgoingBody::finish(body, None) {
        return Err(ErrorCode::InternalError(Some(format!("issue finishing body: {e}"))));
    };

    Ok(resp)
}
