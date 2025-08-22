// use axum::extract::{Path, State};
// use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
// use axum::response::{Html, IntoResponse, Redirect, Response};

use anyhow::Context;
use axum::routing::{get, post};
use axum::{Json, Router};
use http::Method;
use sdk_http::{Client, Decode, Result};
use serde_json::{Value, json};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};

struct HttpGuest;

impl Guest for HttpGuest {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        let subscriber =
            FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        let router = Router::new()
            .route("/", get(get_handler))
            .layer(
                CorsLayer::new()
                    .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                    .allow_headers(Any)
                    .allow_origin(Any),
            )
            .route("/", post(post_handler));

        let out = sdk_http::serve(router, request);
        ResponseOutparam::set(response, out);
    }
}

// Forward request to external service and return the response
async fn get_handler() -> Result<Json<Value>> {
    let body = Client::new()
        .get("https://jsonplaceholder.cypress.io/posts/1")
        .send()?
        .json::<Value>()
        .context("issue sending request")?;

    Ok(Json(json!({
        "response": body
    })))
}

// Forward request to external service and return the response
async fn post_handler(Json(body): Json<Value>) -> Result<Json<Value>> {
    let body = Client::new()
        .post("https://jsonplaceholder.cypress.io/posts")
        .bearer_auth("some token") // not required, but shown for example
        .json(&body)
        .send()?
        .json::<Value>()
        .context("issue sending request")?;

    Ok(Json(json!({
        "response": body
    })))
}

wasi::http::proxy::export!(HttpGuest);
