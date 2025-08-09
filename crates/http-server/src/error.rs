use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

// axum error handling.
pub struct AxumError {
    status: StatusCode,
    error: serde_json::Value,
}

impl From<anyhow::Error> for AxumError {
    fn from(e: anyhow::Error) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            error: json!({"error": e.to_string()}),
        }
    }
}

impl IntoResponse for AxumError {
    fn into_response(self) -> Response {
        (self.status, format!("{}", self.error)).into_response()
    }
}
