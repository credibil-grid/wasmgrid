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
use tower::ServiceExt;
use wasi::http::types::{
    ErrorCode, Fields, Headers, IncomingRequest, Method as WasiMethod, OutgoingBody,
    OutgoingResponse,
};

/// Serve an incoming request using the provided router.
///
/// # Errors
///
/// Returns a [`wasi::http::types::ErrorCode`] if the request could not be served.
pub fn serve(
    router: axum::Router, request: IncomingRequest,
) -> Result<OutgoingResponse, ErrorCode> {
    // forward request to axum `Router` to handle
    let http_req =
        Request(request).try_into().map_err(|e| error!("issue converting request: {e}"))?;
    let http_resp = block_on(async { router.oneshot(http_req).await })
        .map_err(|e| error!("issue processing request: {e}"))?;

    println!("create response: {http_resp:?}");

    // transform `http::Response` into `OutgoingResponse`
    let headers = Headers::new();
    headers
        .set(CONTENT_TYPE.as_str(), &[b"application/json".to_vec()])
        .map_err(|e| error!("issue setting header: {e}"))?;
    let response = OutgoingResponse::new(headers);
    response
        .set_status_code(http_resp.status().as_u16())
        .map_err(|_| error!("issue setting status code"))?;

    // write `OutgoingBody`
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

struct Request(IncomingRequest);

impl Request {
    pub fn method(&self) -> Method {
        Method(self.0.method())
    }

    pub fn headers(&self) -> Fields {
        self.0.headers()
    }

    fn uri(&self) -> Uri {
        let p_and_q = self.0.path_with_query().unwrap_or_default();
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

impl TryFrom<Request> for HttpRequest<Body> {
    type Error = ErrorCode;

    fn try_from(request: Request) -> Result<Self, Self::Error> {
        let method = request.method().to_string();
        let headers = request.headers();
        let uri = request.uri();
        let bytes = request.body().map_err(|e| error!("issue getting request body: {e}"))?;

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
