//! # Metrics

use std::sync::{Arc, Weak};
use std::time::Duration;

use anyhow::Result;
use opentelemetry::global;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::error::OTelSdkResult;
use opentelemetry_sdk::metrics::data::ResourceMetrics;
use opentelemetry_sdk::metrics::reader::MetricReader;
use opentelemetry_sdk::metrics::{
    InstrumentKind, ManualReader, Pipeline, SdkMeterProvider, Temporality,
};

use crate::export::metrics;

pub(crate) fn init(resource: Resource) -> Result<Reader> {
    let reader = Reader::new();
    let provider =
        SdkMeterProvider::builder().with_resource(resource).with_reader(reader.clone()).build();
    global::set_meter_provider(provider);

    Ok(reader)
}

pub struct Guard;

#[derive(Debug, Clone)]
pub struct Reader {
    reader: Arc<ManualReader>,
}

impl Reader {
    /// Create a new `MetricReader`.
    pub fn new() -> Self {
        Self {
            reader: Arc::new(ManualReader::default()),
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
        metrics::export(rm).expect("should export metrics");
    }
}
