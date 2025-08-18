//! # Metrics

use std::sync::{Arc, Weak};
use std::time::Duration;

use opentelemetry::global;
use opentelemetry_sdk::error::OTelSdkResult;
use opentelemetry_sdk::metrics::data::ResourceMetrics;
use opentelemetry_sdk::metrics::reader::MetricReader;
use opentelemetry_sdk::metrics::{
    InstrumentKind, ManualReader, Pipeline, SdkMeterProvider, Temporality,
};


use crate::generated::wasi::otel::metrics as wasi;

pub fn init() -> Reader {
    let reader = Reader::new();
    let provider = SdkMeterProvider::builder().with_reader(reader.clone()).build();
    global::set_meter_provider(provider);

    reader
}

#[derive(Debug, Clone)]
pub struct Reader(Arc<ManualReader>);

impl Reader {
    /// Create a new `MetricReader`.
    pub fn new() -> Self {
        Self(Arc::new(ManualReader::default()))
    }
}

impl MetricReader for Reader {
    fn register_pipeline(&self, pipeline: Weak<Pipeline>) {
        self.0.register_pipeline(pipeline)
    }

    fn collect(&self, rm: &mut ResourceMetrics) -> OTelSdkResult {
        self.0.collect(rm)
    }

    fn force_flush(&self) -> OTelSdkResult {
        self.0.force_flush()
    }

    fn temporality(&self, kind: InstrumentKind) -> Temporality {
        self.0.temporality(kind)
    }

    fn shutdown_with_timeout(&self, timeout: Duration) -> OTelSdkResult {
        self.0.shutdown_with_timeout(timeout)
    }
}

impl Drop for Reader {
    fn drop(&mut self) {
        let mut rm = ResourceMetrics::default();
        self.0.collect(&mut rm).expect("should collect");
        wasi::export(&rm.into()).expect("should export");
    }
}
