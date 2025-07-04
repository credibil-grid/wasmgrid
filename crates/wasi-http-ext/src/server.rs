use std::cmp;

use http::header::CONTENT_TYPE;
use serde_json::json;
use wasi::http::types::{ErrorCode, Headers, OutgoingBody, OutgoingResponse};

use crate::request::Request;
use crate::routing::Router;

/// Serve an incoming request using the provided router.
///
/// # Errors
///
/// Returns a [`wasi::http::types::ErrorCode`] if the request could not be served.
pub fn serve<'a>(
    router: &Router, request: impl Into<Request<'a>>,
) -> Result<OutgoingResponse, ErrorCode> {
    let mut request = request.into();

    // create outgoing response
    let headers = Headers::new();
    headers
        .set(&CONTENT_TYPE.to_string(), &[b"application/json".to_vec()])
        .map_err(|e| ErrorCode::InternalError(Some(format!("issue setting header: {e}"))))?;
    let response = OutgoingResponse::new(headers);

    let Some((route, params)) = router.find(&request) else {
        return Err(ErrorCode::DestinationNotFound);
    };
    request.params = Some(params);

    // call the route's handler to process the request
    let mut inner_bytes = match route.handle(&request) {
        Ok(resp) => {
            response.set_status_code(resp.status.as_u16()).map_err(|_| {
                ErrorCode::InternalError(Some("issue setting status code".to_string()))
            })?;
            resp.body
        }
        Err(err) => {
            tracing::error!("{err}");
            response.set_status_code(http::StatusCode::INTERNAL_SERVER_ERROR.as_u16()).map_err(
                |_| ErrorCode::InternalError(Some("issue setting status code".to_string())),
            )?;
            let err_json = json!({"error": "server_error", "error_description": err.to_string()});
            serde_json::to_vec(&err_json).map_err(|_| {
                ErrorCode::InternalError(Some("failed to serialize error".to_string()))
            })?
        }
    };

    // write outgoing body
    let body = response
        .body()
        .map_err(|()| ErrorCode::InternalError(Some("issue getting outgoing body".into())))?;
    let stream = body
        .write()
        .map_err(|()| ErrorCode::InternalError(Some("issue getting body stream".into())))?;

    // write to stream in chunks as max bytes for `blocking_write_and_flush` is 4096
    let pollable = stream.subscribe();
    while !inner_bytes.is_empty() {
        pollable.block();
        let n = stream
            .check_write()
            .map_err(|e| ErrorCode::InternalError(Some(format!("issue checking write: {e}"))))?;
        let mid = cmp::min(n as usize, inner_bytes.len());
        let (chunk, remaining) = inner_bytes.split_at(mid);
        if let Err(e) = stream.write(chunk) {
            return Err(ErrorCode::InternalError(Some(format!("issue writing to stream: {e}"))));
        };

        inner_bytes = remaining.to_vec();
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

    Ok(response)
}
