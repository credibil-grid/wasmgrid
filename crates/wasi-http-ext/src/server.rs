use std::cmp;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use anyhow::{Result, anyhow};
use axum::body::Body;
use axum::http::Request as HttpRequest;
use futures::StreamExt;
use futures::executor::block_on;
use http::header::CONTENT_TYPE;
use http::{HeaderName, HeaderValue, Uri};
use percent_encoding::percent_decode_str;
use serde_json::json;
use tower::ServiceExt;
use wasi::http::types::{
    ErrorCode, Fields, Headers, IncomingRequest, Method as WasiMethod, OutgoingBody,
    OutgoingResponse,
};

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
    // call the route's handler to process the request
    let http_req = Request2(request)
        .try_into()
        .map_err(|e| ErrorCode::InternalError(Some(format!("issue converting request: {e}"))))?;
    let http_resp = block_on(async { router.oneshot(http_req).await })
        .map_err(|e| error!("issue processing request: {e}"))?;

    // create outgoing response
    let headers = Headers::new();
    headers
        .set(&CONTENT_TYPE.to_string(), &[b"application/json".to_vec()])
        .map_err(|e| error!("issue setting header: {e}"))?;
    let response = OutgoingResponse::new(headers);
    response
        .set_status_code(http_resp.status().as_u16())
        .map_err(|_| error!("issue setting status code"))?;

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

    // check for errors
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

struct Request2(IncomingRequest);

impl Request2 {
    pub fn method(&self) -> Method {
        Method(self.0.method())
    }

    pub fn headers(&self) -> Fields {
        self.0.headers()
    }

    fn uri(&self) -> Uri {
        let p_and_q = self.0.path_with_query().unwrap_or_default();
        // FIXME: potentially repeated when decoding query parameters
        let decoded = percent_decode_str(p_and_q.as_str()).decode_utf8_lossy();
        decoded.parse::<Uri>().unwrap_or_else(|_| Uri::default())
    }

    fn body(&self) -> Result<Vec<u8>> {
        let body = self.0.consume().map_err(|_| anyhow!("issue consuming request body"))?;
        let stream = body.stream().map_err(|_| anyhow!("issue getting body stream"))?;

        let mut buffer = Vec::new();
        while let Ok(bytes) = stream.blocking_read(4096)
            && bytes.len() > 0
        {
            buffer.extend_from_slice(&bytes);
        }
        drop(stream);

        Ok(buffer)
    }
}

impl TryFrom<Request2> for HttpRequest<Body> {
    type Error = ErrorCode;

    fn try_from(inner: Request2) -> Result<Self, Self::Error> {
        let method = inner.method().to_string();
        let headers = inner.headers();
        let uri = inner.uri();
        let bytes = inner.body().map_err(|e| error!("issue getting request body: {e}"))?;

        let mut req = HttpRequest::builder()
            .method(method.as_str())
            .uri(uri)
            .body(Body::from(bytes))
            .map_err(|e| error!("issue building request: {e}"))?;

        for (key, value) in headers.entries() {
            req.headers_mut().insert(
                HeaderName::from_str(&key).unwrap(),
                HeaderValue::from_bytes(&value).unwrap(),
            );
        }

        Ok(req)
    }
}

#[derive(Debug, Clone)]
struct Method(WasiMethod);

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let method = match &self.0 {
            WasiMethod::Get => "GET",
            WasiMethod::Post => "POST",
            WasiMethod::Put => "PUT",
            WasiMethod::Delete => "DELETE",
            WasiMethod::Patch => "PATCH",
            WasiMethod::Head => "HEAD",
            WasiMethod::Options => "OPTIONS",
            WasiMethod::Trace => "TRACE",
            WasiMethod::Connect => "CONNECT",
            WasiMethod::Other(s) => s,
        };
        write!(f, "{method}")
    }
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
