use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

/// Type alias for axum-compatible Result.
pub type Result<T, E = Error> = anyhow::Result<T, E>;

// axum error handling.
pub struct Error {
    status: StatusCode,
    error: serde_json::Value,
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            error: json!({"error": e.to_string()}),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (self.status, format!("{}", self.error)).into_response()
    }
}
