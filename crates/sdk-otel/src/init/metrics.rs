//! # Metrics

use std::sync::{Arc, Weak};
use std::time::Duration;

use anyhow::Result;
use futures::executor::block_on;
use opentelemetry::global;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::error::OTelSdkResult;
use opentelemetry_sdk::metrics::data::ResourceMetrics;
use opentelemetry_sdk::metrics::exporter::PushMetricExporter;
use opentelemetry_sdk::metrics::reader::MetricReader;
use opentelemetry_sdk::metrics::{
    InstrumentKind, ManualReader, Pipeline, SdkMeterProvider, Temporality,
};

use crate::export::metrics::Exporter;

pub fn init(resource: Resource) -> Result<SdkMeterProvider> {
    let exporter = Exporter::new()?;
    let reader = Reader::new(exporter);
    let provider = SdkMeterProvider::builder().with_resource(resource).with_reader(reader).build();
    global::set_meter_provider(provider.clone());
    Ok(provider)
}

#[derive(Debug, Clone)]
struct Reader {
    reader: Arc<ManualReader>,
    exporter: Arc<Exporter>,
}

impl Reader {
    #[must_use]
    fn new(exporter: Exporter) -> Self {
        Self {
            reader: Arc::new(ManualReader::default()),
            exporter: Arc::new(exporter),
        }
    }
}

impl MetricReader for Reader {
    fn register_pipeline(&self, pipeline: Weak<Pipeline>) {
        self.reader.register_pipeline(pipeline);
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

    fn shutdown_with_timeout(&self, _: Duration) -> OTelSdkResult {
        let mut rm = ResourceMetrics::default();
        self.reader.collect(&mut rm)?;
        block_on(async { self.exporter.export(&rm).await })
    }
}
