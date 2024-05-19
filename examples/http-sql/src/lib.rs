#![feature(let_chains)]

use anyhow::anyhow;
use http_shared::{self, Request, Router};
use serde_json::json;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use vercre_core::metadata::Issuer as IssuerMetadata;
use wasi::exports::http::incoming_handler::Guest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};
use wasi_bindings::docdb::readwrite;
use wasi_bindings::docdb::types::{Database, Statement};

struct HttpGuest;

impl Guest for HttpGuest {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        let subscriber =
            FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
        tracing::subscriber::set_global_default(subscriber).expect("should set subscriber");

        let router = Router::new().route("/", handler);

        let out = http_shared::serve(&router, &request);
        ResponseOutparam::set(response, out);
    }
}

fn handler(request: &Request) -> anyhow::Result<Vec<u8>> {
    tracing::debug!("request.uri: {}", request.uri());

    let body = request.body()?;
    let req: serde_json::Value = serde_json::from_slice(&body)?;
    tracing::debug!("json: {:?}", req);

    let db = Database::connect("issuance").unwrap();
    let query = Statement::prepare(
        "issuer",
        &[(String::from("credential_issuer"), "https://issuance.demo.credibil.io".to_string())],
    )
    .map_err(|e| anyhow!(e.trace()))?;

    let results = readwrite::find(&db, &query).map_err(|e| anyhow!(e.trace()))?;
    let doc = results.first().ok_or_else(|| anyhow!("No issuer metadata found"))?;

    let md: IssuerMetadata = serde_json::from_slice(&doc)?;
    tracing::debug!("md: {:?}", md);

    let resp = json!({
        "message": "Hello, World!"
    });
    serde_json::to_vec(&resp).map_err(Into::into)
}

wasi::http::proxy::export!(HttpGuest);
