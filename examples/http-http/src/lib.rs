// use axum::extract::{Path, State};
// use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
// use axum::response::{Html, IntoResponse, Redirect, Response};
use anyhow::Context;
use axum::routing::get;
use axum::{Json, Router};
use http::Method;
// use axum_extra::TypedHeader;
use serde_json::json;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};
use wasi_http_ext::{self, AxumError, Client};

struct HttpGuest;

impl Guest for HttpGuest {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        let subscriber =
            FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        let router = Router::new().route("/", get(get_handler)).layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                .allow_headers(Any)
                .allow_origin(Any),
        ); //.route("/", post(post_handler));

        let out = wasi_http_ext::serve2(router, request);
        ResponseOutparam::set(response, out);
    }
}

// Forward request to external service and return the response
async fn get_handler() -> Result<Json<serde_json::Value>, AxumError> {
    let resp = Client::new()
        .get("http://jsonplaceholder.cypress.io/posts/1")
        .send()
        .context("issue sending request")?;

    Ok(Json(json!({
        "response": resp.json::<serde_json::Value>().context("some error occurred")?
    })))

    // return (
    //     StatusCode::OK,
    //     Json(json!({
    //         "response": resp.json::<serde_json::Value>().unwrap()
    //     })),
    // )
    //     .into_response();
}

// // Forward request to external service and return the response
// fn post_handler(request: &Request) -> anyhow::Result<Response> {
//     let body: serde_json::Value = serde_json::from_slice(&request.body()?)?;

//     let resp = Client::new()
//         .post("https://jsonplaceholder.cypress.io/posts")
//         .bearer_auth("some token") // not required, but shown for example
//         .json(&body)
//         .send()?;

//     Ok(serde_json::to_vec(&json!({
//         "response": resp.json::<serde_json::Value>()?
//     }))?
//     .into())
// }

wasi::http::proxy::export!(HttpGuest);

// // Wrap anyhow::Error.
// struct AxError {
//     status: StatusCode,
//     error: serde_json::Value,
// }

// impl From<anyhow::Error> for AxError {
//     fn from(e: anyhow::Error) -> Self {
//         Self {
//             status: StatusCode::INTERNAL_SERVER_ERROR,
//             error: json!({"error": e.to_string()}),
//         }
//     }
// }

// impl IntoResponse for AxError {
//     fn into_response(self) -> Response {
//         (self.status, format!("{}", self.error)).into_response()
//     }
// }
