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

// use wasi::clocks::monotonic_clock::subscribe_duration;
use crate::generated::wasi::otel::metrics as wasi;

pub fn init() {
    let reader = Reader::new();
    let provider = SdkMeterProvider::builder().with_reader(reader).build();
    global::set_meter_provider(provider);
}

#[derive(Debug, Clone)]
pub struct Reader {
    inner: Arc<ManualReader>,
}

impl Drop for Reader {
    fn drop(&mut self) {
        let mut rm = ResourceMetrics::default();
        self.inner.collect(&mut rm).unwrap();
        println!("Collected ResourceMetrics: {rm:?}");

        wasi::export(&rm.into()).expect("should collect metrics");
    }
}

impl Reader {
    /// Create a new `MetricReader`.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(ManualReader::default()),
        }
    }
}

impl MetricReader for Reader {
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

    fn shutdown_with_timeout(&self, timeout: Duration) -> OTelSdkResult {
        self.inner.shutdown_with_timeout(timeout)
    }
}
