//! # Metrics

use std::sync::{Arc, Weak};
use std::time::Duration;

use opentelemetry::global;
use opentelemetry_http::HttpClient;
use opentelemetry_otlp::{MetricExporter, WithHttpConfig};
use opentelemetry_sdk::error::OTelSdkResult;
use opentelemetry_sdk::metrics::data::ResourceMetrics;
use opentelemetry_sdk::metrics::exporter::PushMetricExporter;
use opentelemetry_sdk::metrics::reader::MetricReader;
use opentelemetry_sdk::metrics::{
    InstrumentKind, ManualReader, Pipeline, SdkMeterProvider, Temporality,
};

use crate::generated::wasi::otel::metrics as wasi;

pub fn init() -> Reader {
    let builder = MetricExporter::builder().with_http().with_http_client(MetricsClient);

    let exporter = builder.build().expect("should build exporter");
    let reader = Reader::new(exporter);

    let provider = SdkMeterProvider::builder().with_reader(reader.clone()).build();
    global::set_meter_provider(provider);

    reader
}

#[derive(Debug, Clone)]
pub struct Reader {
    reader: Arc<ManualReader>,
    exporter: Arc<MetricExporter>,
}

impl Reader {
    /// Create a new `MetricReader`.
    pub fn new(exporter: MetricExporter) -> Self {
        Self {
            reader: Arc::new(ManualReader::default()),
            exporter: Arc::new(exporter),
        }
    }
}

impl MetricReader for Reader {
    fn register_pipeline(&self, pipeline: Weak<Pipeline>) {
        self.reader.register_pipeline(pipeline)
    }

    fn collect(&self, rm: &mut ResourceMetrics) -> OTelSdkResult {
        self.reader.collect(rm)
    }

    fn force_flush(&self) -> OTelSdkResult {
        self.reader.force_flush()
    }

    fn temporality(&self, kind: InstrumentKind) -> Temporality {
        self.reader.temporality(kind)
    }

    fn shutdown_with_timeout(&self, timeout: Duration) -> OTelSdkResult {
        self.reader.shutdown_with_timeout(timeout)
    }
}

impl Drop for Reader {
    fn drop(&mut self) {
        let mut rm = ResourceMetrics::default();
        self.reader.collect(&mut rm).expect("should collect");
        block_on(async {
            self.exporter.export(&rm.into()).await.expect("should export metrics");
        });
        // wasi::export(&rm.into()).expect("should export");
    }
}

use std::mem;

use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;
use futures::executor::block_on;
use http::{Request, Response};
use opentelemetry_http::HttpError;
use sdk_http::Client;

#[derive(Debug)]
struct MetricsClient;

#[async_trait]
impl HttpClient for MetricsClient {
    async fn send_bytes(&self, request: Request<Bytes>) -> Result<Response<Bytes>, HttpError> {
        let mut response = Client::new()
            .post(request.uri())
            .headers(request.headers())
            .body(request.into_body().to_vec())
            .send()?;

        let headers = mem::take(response.headers_mut());
        let mut http_response =
            Response::builder().status(response.status()).body(response.body().clone().into())?;
        *http_response.headers_mut() = headers;

        Ok(http_response)
    }
}
