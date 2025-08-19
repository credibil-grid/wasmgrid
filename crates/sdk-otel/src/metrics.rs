//! # Metrics

use std::sync::{Arc, Weak};
use std::time::Duration;

use futures::executor::block_on;
use opentelemetry::global;
use opentelemetry_otlp::{MetricExporter, WithHttpConfig};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::error::OTelSdkResult;
use opentelemetry_sdk::metrics::data::ResourceMetrics;
use opentelemetry_sdk::metrics::exporter::PushMetricExporter;
use opentelemetry_sdk::metrics::reader::MetricReader;
use opentelemetry_sdk::metrics::{
    InstrumentKind, ManualReader, Pipeline, SdkMeterProvider, Temporality,
};

use crate::ExportClient;
// use crate::generated::wasi::otel::metrics as wasi;

pub fn init(resource: Resource) -> Reader {
    let builder = MetricExporter::builder().with_http().with_http_client(ExportClient);

    let exporter = builder.build().expect("should build exporter");
    let reader = Reader::new(exporter);

    let provider =
        SdkMeterProvider::builder().with_resource(resource).with_reader(reader.clone()).build();
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
        // collect
        let mut rm = ResourceMetrics::default();
        self.reader.collect(&mut rm).expect("should collect");

        // export
        block_on(async {
            self.exporter.export(&rm.into()).await.expect("should export metrics");
        });
        // wasi::export(&rm.into()).expect("should export");
    }
}
