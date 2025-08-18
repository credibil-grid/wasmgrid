//! # WASI Tracing

use std::sync::{Arc, Weak};
use std::time::Duration;

use anyhow::Result;
use opentelemetry::metrics::{Counter, Gauge, Meter, MeterProvider};
use opentelemetry_otlp::MetricExporter;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::error::{OTelSdkError, OTelSdkResult};
use opentelemetry_sdk::metrics::data::ResourceMetrics;
use opentelemetry_sdk::metrics::exporter::PushMetricExporter;
use opentelemetry_sdk::metrics::reader::MetricReader;
use opentelemetry_sdk::metrics::{
    InstrumentKind, ManualReader, Pipeline, SdkMeterProvider, Temporality,
};

use crate::Otel;
use crate::generated::wasi::otel::metrics::{self as wm};
use crate::generated::wasi::otel::types;

impl wm::Host for Otel<'_> {
    async fn export(&mut self, rm: wm::ResourceMetrics) -> Result<(), types::Error> {
        // convert to opentelemetry metrics
        let resource = Resource::from(&rm.resource);
        let reader = Reader::new();
        let provider =
            SdkMeterProvider::builder().with_resource(resource).with_reader(reader.clone()).build();
        //

        for m in rm.scope_metrics {
            let meter = provider.meter_with_scope(m.scope.into());
            let writer = MeterWriter::new(meter.clone());
            for metric in m.metrics {
                writer.write(metric);
            }
        }

        // export metrics
        let mut rm = ResourceMetrics::default();
        reader.inner.collect(&mut rm).unwrap();
        let exporter =
            MetricExporter::builder().with_tonic().build().expect("should build exporter");
        exporter.export(&rm).await?;

        println!("exported: {rm:?}");

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Reader {
    inner: Arc<ManualReader>,
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
        self.inner.register_pipeline(pipeline);
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

impl types::Host for Otel<'_> {
    fn convert_error(&mut self, err: types::Error) -> anyhow::Result<types::Error> {
        tracing::error!("{err}");
        Ok(err)
    }
}

struct MeterWriter {
    meter: Meter,
}

impl MeterWriter {
    const fn new(meter: Meter) -> Self {
        Self { meter }
    }

    fn write(&self, metric: wm::Metric) {
        match metric.data {
            wm::AggregatedMetrics::U64(data) => match data {
                wm::MetricData::Sum(sum) => {
                    let counter = self
                        .meter
                        .u64_counter(metric.name)
                        .with_description(metric.description)
                        .with_unit(metric.unit)
                        .build();
                    SumWriter::new(CounterType::U64(counter)).write(sum);
                }
                wm::MetricData::Gauge(gauge) => {
                    let gauge64 = self
                        .meter
                        .u64_gauge(metric.name)
                        .with_description(metric.description)
                        .with_unit(metric.unit)
                        .build();
                    GaugeWriter::new(gauge64).write(gauge);
                }
                wm::MetricData::Histogram(_) => {
                    unimplemented!("Histogram is not supported");
                }
                wm::MetricData::ExponentialHistogram(_) => {
                    unimplemented!("ExponentialHistogram is not supported");
                }
            },
            _ => {}
        }
    }
}

enum CounterType {
    U64(Counter<u64>),
    S64(Counter<i64>),
    F64(Counter<f64>),
}

struct SumWriter {
    counter: CounterType,
}

impl SumWriter {
    const fn new(counter: CounterType) -> Self {
        Self { counter }
    }

    fn write(&self, sum: wm::Sum) {
        for dp in sum.data_points.into_iter().rev() {
            let attributes = dp.attributes.iter().map(Into::into).collect::<Vec<_>>();

            match dp.value {
                wm::DataValue::U64(value) => {
                    if let CounterType::U64(counter) = &self.counter {
                        counter.add(value, &attributes);
                    }
                }
                wm::DataValue::S64(value) => {
                    if let CounterType::S64(counter) = &self.counter {
                        counter.add(value, &attributes);
                    }
                }
                wm::DataValue::F64(value) => {
                    if let CounterType::F64(counter) = &self.counter {
                        counter.add(value, &attributes);
                    }
                }
            }
        }
    }
}

struct GaugeWriter {
    gauge: Gauge<u64>,
}

impl GaugeWriter {
    const fn new(gauge: Gauge<u64>) -> Self {
        Self { gauge }
    }

    fn write(&self, gauge: wm::Gauge) {
        for dp in gauge.data_points.into_iter().rev() {
            let attributes = dp.attributes.iter().map(Into::into).collect::<Vec<_>>();

            match dp.value {
                wm::DataValue::U64(value) => {
                    self.gauge.record(value, &attributes);
                }
                _ => {}
            }
        }
    }
}

impl From<&wm::Resource> for Resource {
    fn from(resource: &wm::Resource) -> Self {
        let attributes = resource.attributes.iter().map(Into::into);
        let schema_url = resource.schema_url.clone().unwrap_or_default();
        Self::builder().with_schema_url(attributes, schema_url).build()
    }
}

impl From<OTelSdkError> for types::Error {
    fn from(err: OTelSdkError) -> Self {
        match err {
            OTelSdkError::AlreadyShutdown => Self::AlreadyShutdown,
            OTelSdkError::Timeout(duration) => Self::Timeout(duration.as_secs()),
            OTelSdkError::InternalFailure(msg) => Self::InternalFailure(msg),
        }
    }
}

impl From<anyhow::Error> for types::Error {
    fn from(err: anyhow::Error) -> Self {
        Self::InternalFailure(err.to_string())
    }
}
