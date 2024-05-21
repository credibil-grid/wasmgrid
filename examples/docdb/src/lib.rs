#![feature(let_chains)]

use anyhow::anyhow;
use bson::{self, Bson};
use serde_json::json;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use vercre_core::metadata::Issuer as IssuerMetadata;
use wasex::{self, Request, Router};
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

        let router = Router::new().route("/v2", handler_v2).route("/", handler);

        let out = wasex::serve(&router, &request);
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

fn handler_v2(request: &Request) -> anyhow::Result<Vec<u8>> {
    let db = Database::connect("test_db").unwrap();
    let payload = bson::doc! {
        "some": "payload",
        "value": 5
    };

    readwrite::insert_v2(&db, "test_collection", &bson::to_vec(&payload)?).unwrap();

    let find_one_result = readwrite::find_one_v2(
        &db,
        "test_collection",
        &bson::to_vec(&bson::doc! {
            "$and": [
                {
                    "some": "payload",
                },
                {
                    "$or": [
                        {
                            "value": {
                                "$gt": 2
                            }
                        },
                        {
                            "some_field": {
                                "$exists": true
                            }
                        }
                    ]
                }
            ]
        })?,
    )
    .unwrap();

    let deserialized_result: Bson = bson::from_slice(&find_one_result).unwrap();
    let deser_canon_json = deserialized_result.into_canonical_extjson();
    let json_bytes = serde_json::to_vec(&deser_canon_json).unwrap();

    Ok(json_bytes)
}

wasi::http::proxy::export!(HttpGuest);
