//! # Metrics

use std::sync::{Arc, Weak};
use std::time::Duration;

use opentelemetry::global;
// use opentelemetry_sdk as sdk;
use opentelemetry_sdk::error::OTelSdkResult;
use opentelemetry_sdk::metrics::data::ResourceMetrics;
use opentelemetry_sdk::metrics::exporter::PushMetricExporter;
use opentelemetry_sdk::metrics::{
    InstrumentKind, ManualReader, Pipeline, SdkMeterProvider, Temporality, reader,
};

// use wasi::clocks::monotonic_clock::subscribe_duration;
use crate::generated::wasi::otel::{metrics as wasi_metrics, types};

pub fn init() {
    let reader = MetricReader::new();
    let provider = SdkMeterProvider::builder().with_reader(reader).build();
    global::set_meter_provider(provider);
}

#[derive(Debug, Clone)]
pub struct MetricReader {
    inner: Arc<ManualReader>,
}

impl Drop for MetricReader {
    fn drop(&mut self) {
        self.export();
    }
}

impl MetricReader {
    /// Create a new `MetricReader`.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(ManualReader::default()),
        }
    }

    pub fn export(&self) {
        let mut rm = ResourceMetrics::default();
        reader::MetricReader::collect(self, &mut rm).unwrap();
        println!("Collected ResourceMetrics: {rm:?}");

        // wasi_metrics::collect(&rm).expect("should collect metrics");
    }
}

impl reader::MetricReader for MetricReader {
    fn register_pipeline(&self, pipeline: Weak<Pipeline>) {
        self.inner.register_pipeline(pipeline)
    }

    fn collect(&self, rm: &mut ResourceMetrics) -> OTelSdkResult {
        self.inner.collect(rm)
    }

    fn force_flush(&self) -> OTelSdkResult {
        self.inner.force_flush()
    }

    fn temporality(&self, kind: InstrumentKind) -> Temporality {
        self.inner.temporality(kind)
    }

    // Provided method
    fn shutdown(&self) -> OTelSdkResult {
        self.inner.shutdown()
    }

    fn shutdown_with_timeout(&self, timeout: Duration) -> OTelSdkResult {
        self.inner.shutdown_with_timeout(timeout)
    }
}
