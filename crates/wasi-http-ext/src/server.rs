use std::cmp;

use axum::body::Body;
use axum::http::Request as HttpRequest;
use futures::StreamExt;
use futures::executor::block_on;
use http::header::CONTENT_TYPE;
use serde_json::json;
use tower::ServiceExt;
use wasi::http::types::{ErrorCode, Headers, IncomingRequest, OutgoingBody, OutgoingResponse};

use crate::request::Request;
use crate::response::Response;
use crate::routing::Router;

/// Serve an incoming request using the provided router.
///
/// # Errors
///
/// Returns a [`wasi::http::types::ErrorCode`] if the request could not be served.
pub fn serve<'a, T: Into<Response>>(
    router: &Router<T>, request: impl Into<Request<'a>>,
) -> Result<OutgoingResponse, ErrorCode> {
    let mut request = request.into();

    // create outgoing response
    let headers = Headers::new();
    headers
        .set(&CONTENT_TYPE.to_string(), &[b"application/json".to_vec()])
        .map_err(|e| ErrorCode::InternalError(Some(format!("issue setting header: {e}"))))?;
    let response = OutgoingResponse::new(headers);

    let Some((route, captures)) = router.find(&request) else {
        return Err(ErrorCode::DestinationNotFound);
    };
    request.captures = Some(captures);

    // call the route's handler to process the request
    let mut inner_bytes = match route.handle(&request) {
        Ok(resp) => {
            let resp = resp.into();
            response.set_status_code(resp.status.as_u16()).map_err(|_| {
                ErrorCode::InternalError(Some("issue setting status code".to_string()))
            })?;
            resp.body
        }
        Err(e) => {
            let stack = e.chain().map(|cause| format!(" -> {cause}")).collect::<String>();
            tracing::error!("error serving '{}'{stack}", request.uri(),);

            response.set_status_code(http::StatusCode::INTERNAL_SERVER_ERROR.as_u16()).map_err(
                |_| ErrorCode::InternalError(Some("issue setting status code".to_string())),
            )?;
            let err_json = json!({"error": "server_error", "error_description": e.to_string()});
            serde_json::to_vec(&err_json).map_err(|_| {
                ErrorCode::InternalError(Some("failed to serialize error".to_string()))
            })?
        }
    };

    // write outgoing body
    let body = response
        .body()
        .map_err(|_| ErrorCode::InternalError(Some("issue getting outgoing body".to_string())))?;
    let stream = body
        .write()
        .map_err(|_| ErrorCode::InternalError(Some("issue getting body stream".to_string())))?;

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

/// Serve an incoming request using the provided router.
///
/// # Errors
///
/// Returns a [`wasi::http::types::ErrorCode`] if the request could not be served.
pub fn serve2(
    router: axum::Router, request: IncomingRequest,
) -> Result<OutgoingResponse, ErrorCode> {
    let request: Request = Request::from(&request);

    // convert to `http::Request`
    let method = request.method().to_string();
    // let headers = request.headers();
    let uri = request.uri();
    let bytes = request.body().map_err(|e| error!("issue getting request body: {e}"))?;

    let req = HttpRequest::builder()
        .method(method.as_str())
        // .header(headers)
        .uri(uri)
        .body(Body::from(bytes))
        .map_err(|e| error!("issue building request: {e}"))?;

    // call the route's handler to process the request
    let http_resp = block_on(async { router.oneshot(req).await })
        .map_err(|e| error!("issue processing request: {e}"))?;

    // create outgoing response
    let headers = Headers::new();
    headers
        .set(&CONTENT_TYPE.to_string(), &[b"application/json".to_vec()])
        .map_err(|e| error!("issue setting header: {e}"))?;
    let response = OutgoingResponse::new(headers);

    // write outgoing body
    let http_body = http_resp.into_body();
    let mut http_stream = http_body.into_data_stream();
    let out_body = response.body().map_err(|_| error!("issue getting outgoing body"))?;
    let out_stream = out_body.write().map_err(|_| error!("issue getting body stream"))?;

    let pollable = out_stream.subscribe();
    while let Some(Ok(chunk)) = block_on(async { http_stream.next().await }) {
        pollable.block();
        out_stream.check_write().map_err(|e| error!("issue checking write: {e}"))?;

        if let Err(e) = out_stream.write(&chunk) {
            return Err(error!("issue writing to stream: {e}"));
        };
    }
    if let Err(e) = out_stream.flush() {
        return Err(error!("issue flushing stream: {e}"));
    };
    pollable.block();

    // check for any errors
    if let Err(e) = out_stream.check_write() {
        return Err(error!("issue writing to stream: {e}"));
    };
    drop(pollable);
    drop(out_stream);

    if let Err(e) = OutgoingBody::finish(out_body, None) {
        return Err(error!("issue finishing body: {e}"));
    };

    Ok(response)
}

macro_rules! error {
    ($fmt:expr, $($arg:tt)*) => {
        ErrorCode::InternalError(Some(format!($fmt, $($arg)*)))
    };
     ($err:expr $(,)?) => {
        ErrorCode::InternalError(Some(format!($err)))
    };
}
pub(crate) use error;

// let mut http_body = match http_resp {
//     Ok(resp) => {
//         // let resp = resp.into();
//         response.set_status_code(resp.status().as_u16()).map_err(|_| {
//             ErrorCode::InternalError(Some("issue setting status code".to_string()))
//         })?;
//         resp.into_body()
//     }
//     Err(e) => {
//         let stack = e.chain().map(|cause| format!(" -> {cause}")).collect::<String>();
//         tracing::error!("error serving '{}'{stack}", request.uri(),);

//         response.set_status_code(http::StatusCode::INTERNAL_SERVER_ERROR.as_u16()).map_err(
//             |_| ErrorCode::InternalError(Some("issue setting status code".to_string())),
//         )?;
//         let err_json = json!({"error": "server_error", "error_description": e.to_string()});
//         let data = serde_json::to_vec(&err_json).map_err(|_| {
//             ErrorCode::InternalError(Some("failed to serialize error".to_string()))
//         })?;
//         Body::from(data)
//     }
// };
