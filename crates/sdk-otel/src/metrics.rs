//! # Metrics

use std::sync::atomic::AtomicBool;
use std::sync::{Arc, OnceLock, Weak};
use std::time::Duration;

use opentelemetry::global;
// use opentelemetry_sdk as sdk;
use opentelemetry_sdk::error::OTelSdkResult;
use opentelemetry_sdk::metrics::data::ResourceMetrics;
use opentelemetry_sdk::metrics::exporter::PushMetricExporter;
use opentelemetry_sdk::metrics::{
    Instrument, InstrumentKind, ManualReader, PeriodicReader, Pipeline, SdkMeterProvider, Stream,
    Temporality, reader,
};

use crate::generated::wasi::otel::{metrics as wasi_metrics, types};

pub fn init() -> MetricReader {
    // let view = |i: &Instrument| {
    //     println!("view: {i:?}");
    //     Some(
    //         Stream::builder()
    //             .with_name("my_counter_renamed")
    //             .build()
    //             .expect("Stream should be valid"),
    //     )
    // };

    let reader = MetricReader::new();
    let provider = SdkMeterProvider::builder().with_reader(reader.clone()).build();
    global::set_meter_provider(provider.clone());

    reader
}

#[derive(Debug, Clone)]
pub struct MetricReader {
    // is_shutdown: AtomicBool,
    inner: Arc<ManualReader>,
    // inner: PeriodicReader<Exporter>,
}

impl MetricReader {
    /// Create a new `MetricReader`.
    pub fn new() -> Self {
        let reader = ManualReader::default();

        Self {
            inner: Arc::new(reader),
        }
    }

    pub fn collect_and_export(&self) {
        let mut rm = ResourceMetrics::default();
        reader::MetricReader::collect(self, &mut rm).unwrap();
        println!("Collected ResourceMetrics: {:?}", rm);
    }
}

impl reader::MetricReader for MetricReader {
    fn register_pipeline(&self, pipeline: Weak<Pipeline>) {
        println!("register_pipeline: {pipeline:?}");
        self.inner.register_pipeline(pipeline);
    }

    fn collect(&self, rm: &mut ResourceMetrics) -> OTelSdkResult {
        println!("collect: {rm:?}");
        self.inner.collect(rm)?;
        println!("{:?}", self.inner);
        println!("{rm:?}");
        Ok(())
    }

    fn force_flush(&self) -> OTelSdkResult {
        println!("force_flush");
        self.inner.force_flush()
    }

    fn temporality(&self, kind: InstrumentKind) -> Temporality {
        println!("temporality: {kind:?}");
        self.inner.temporality(kind)
    }

    // Provided method
    fn shutdown(&self) -> OTelSdkResult {
        println!("shutdown");
        self.inner.shutdown()
    }

    fn shutdown_with_timeout(&self, timeout: Duration) -> OTelSdkResult {
        println!("shutdown_with_timeout: {timeout:?}");
        self.inner.shutdown_with_timeout(timeout)
    }
}

#[derive(Debug)]
pub struct Exporter;

impl PushMetricExporter for Exporter {
    async fn export(&self, metrics: &ResourceMetrics) -> OTelSdkResult {
        println!("export: {metrics:?}");
        Ok(())
    }

    /// Flushes any metric data held by an exporter.
    fn force_flush(&self) -> OTelSdkResult {
        println!("force_flush");
        Ok(())
    }

    fn shutdown_with_timeout(&self, timeout: Duration) -> OTelSdkResult {
        println!("shutdown_with_timeout: {timeout:?}");
        Ok(())
    }

    fn shutdown(&self) -> OTelSdkResult {
        self.shutdown_with_timeout(Duration::from_secs(5))
    }

    fn temporality(&self) -> Temporality {
        println!("temporality");
        Temporality::Cumulative
    }
}
