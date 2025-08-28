//! # Metrics
//!
//! Convert OpenTelemetry metrics types in `wasi-otel` types.

use std::time::Duration;

use anyhow::Result;
use cfg_if::cfg_if;
#[cfg(feature = "guest-export")]
use opentelemetry_otlp::{MetricExporter, WithHttpConfig};
use opentelemetry_sdk::error::{OTelSdkError, OTelSdkResult};
use opentelemetry_sdk::metrics::Temporality;
use opentelemetry_sdk::metrics::data::ResourceMetrics;
use opentelemetry_sdk::metrics::exporter::PushMetricExporter;

#[cfg(not(feature = "guest-export"))]
use crate::generated::wasi::otel::metrics as wasi;

#[derive(Debug)]
pub struct Exporter {
    #[cfg(feature = "guest-export")]
    inner: MetricExporter,
}

impl Exporter {
    #[cfg(feature = "guest-export")]
    pub fn new() -> Result<Self> {
        use std::env;

        use opentelemetry_otlp::WithExportConfig;

        use crate::export::ExportClient;

        let mut builder = MetricExporter::builder().with_http().with_http_client(ExportClient);
        if let Ok(endpoint) = env::var("OTEL_HTTP_ADDR") {
            builder = builder.with_endpoint(format!("{endpoint}/v1/metrics"));
        }
        let inner = builder.build()?;

        Ok(Self { inner })
    }

    #[allow(clippy::unnecessary_wraps)]
    #[cfg(not(feature = "guest-export"))]
    pub const fn new() -> Result<Self> {
        Ok(Self {})
    }
}

impl PushMetricExporter for Exporter {
    #[cfg(feature = "guest-export")]
    async fn export(&self, rm: &ResourceMetrics) -> Result<(), OTelSdkError> {
        self.inner.export(rm).await
    }

    #[cfg(not(feature = "guest-export"))]
    async fn export(&self, rm: &ResourceMetrics) -> Result<(), OTelSdkError> {
        wasi::export(&rm.into())
            .map_err(|e| OTelSdkError::InternalFailure(format!("failed to export metrics: {e}")))?;
        Ok(())
    }

    fn force_flush(&self) -> OTelSdkResult {
        cfg_if! {
            if #[cfg(feature = "guest-export")] {
                self.inner.force_flush()
            } else {
                Ok(())
            }
        }
    }

    fn temporality(&self) -> Temporality {
        cfg_if! {
            if #[cfg(feature = "guest-export")] {
                self.inner.temporality()
            } else {
                Temporality::Cumulative
            }
        }
    }

    #[cfg(feature = "guest-export")]
    fn shutdown_with_timeout(&self, duration: Duration) -> OTelSdkResult {
        self.inner.shutdown_with_timeout(duration)
    }

    #[cfg(not(feature = "guest-export"))]
    fn shutdown_with_timeout(&self, _: Duration) -> OTelSdkResult {
        Ok(())
    }
}

cfg_if! {
    if #[cfg(not(feature = "guest-export"))] {
        use num_traits::ToPrimitive;
        use opentelemetry_sdk::Resource;
        use opentelemetry_sdk::metrics::data::{
            AggregatedMetrics, Exemplar, ExponentialBucket, ExponentialHistogram,
            ExponentialHistogramDataPoint, Gauge, GaugeDataPoint, Histogram, HistogramDataPoint, Metric,
            MetricData, ScopeMetrics, Sum, SumDataPoint,
        };

        use crate::generated::wasi::otel::metrics as wm;

        impl From<&ResourceMetrics> for wm::ResourceMetrics {
            fn from(rm: &ResourceMetrics) -> Self {
                Self {
                    resource: rm.resource().into(),
                    scope_metrics: rm.scope_metrics().map(Into::into).collect(),
                }
            }
        }

        impl From<&Resource> for wm::Resource {
            fn from(resource: &Resource) -> Self {
                Self {
                    attributes: resource.iter().map(Into::into).collect(),
                    schema_url: resource.schema_url().map(ToString::to_string),
                }
            }
        }

        impl From<&ScopeMetrics> for wm::ScopeMetrics {
            fn from(scope_metrics: &ScopeMetrics) -> Self {
                Self {
                    scope: scope_metrics.scope().into(),
                    metrics: scope_metrics.metrics().map(Into::into).collect(),
                }
            }
        }

        impl From<&Metric> for wm::Metric {
            fn from(metric: &Metric) -> Self {
                Self {
                    name: metric.name().to_string(),
                    description: metric.description().to_string(),
                    unit: metric.unit().to_string(),
                    data: metric.data().into(),
                }
            }
        }

        impl From<&AggregatedMetrics> for wm::AggregatedMetrics {
            fn from(am: &AggregatedMetrics) -> Self {
                match am {
                    AggregatedMetrics::F64(v) => Self::F64(v.into()),
                    AggregatedMetrics::I64(v) => Self::S64(v.into()),
                    AggregatedMetrics::U64(v) => Self::U64(v.into()),
                }
            }
        }

        impl<T: ToPrimitive + Copy> From<&MetricData<T>> for wm::MetricData {
            fn from(md: &MetricData<T>) -> Self {
                match md {
                    MetricData::Gauge(v) => Self::Gauge(v.into()),
                    MetricData::Sum(v) => Self::Sum(v.into()),
                    MetricData::Histogram(v) => Self::Histogram(v.into()),
                    MetricData::ExponentialHistogram(v) => Self::ExponentialHistogram(v.into()),
                }
            }
        }

        impl<T: ToPrimitive + Copy> From<&Gauge<T>> for wm::Gauge {
            fn from(gauge: &Gauge<T>) -> Self {
                Self {
                    data_points: gauge.data_points().map(Into::into).collect(),
                    start_time: gauge.start_time().map(Into::into),
                    time: gauge.time().into(),
                }
            }
        }

        impl<T: ToPrimitive + Copy> From<&GaugeDataPoint<T>> for wm::GaugeDataPoint {
            fn from(data_point: &GaugeDataPoint<T>) -> Self {
                Self {
                    attributes: data_point.attributes().map(Into::into).collect(),
                    value: data_point.value().into(),
                    exemplars: data_point.exemplars().map(Into::into).collect(),
                }
            }
        }

        impl<T: ToPrimitive + Copy> From<&Exemplar<T>> for wm::Exemplar {
            fn from(exemplar: &Exemplar<T>) -> Self {
                Self {
                    filtered_attributes: exemplar
                        .filtered_attributes()
                        .map(Into::into)
                        .collect(),
                    time: exemplar.time().into(),
                    value: exemplar.value.into(),
                    span_id: String::from_utf8_lossy(exemplar.span_id()).into(),
                    trace_id: String::from_utf8_lossy(exemplar.trace_id()).into(),
                }
            }
        }

        impl<T: ToPrimitive + Copy> From<&Sum<T>> for wm::Sum {
            fn from(sum: &Sum<T>) -> Self {
                Self {
                    data_points: sum.data_points().map(Into::into).collect(),
                    start_time: sum.start_time().into(),
                    time: sum.time().into(),
                    temporality: sum.temporality().into(),
                    is_monotonic: sum.is_monotonic(),
                }
            }
        }

        impl<T: ToPrimitive + Copy> From<&SumDataPoint<T>> for wm::SumDataPoint {
            fn from(data_point: &SumDataPoint<T>) -> Self {
                Self {
                    attributes: data_point.attributes().map(Into::into).collect(),
                    value: data_point.value().into(),
                    exemplars: data_point.exemplars().map(Into::into).collect(),
                }
            }
        }

        impl<T: ToPrimitive + Copy> From<&Histogram<T>> for wm::Histogram {
            fn from(histogram: &Histogram<T>) -> Self {
                Self {
                    data_points: histogram.data_points().map(Into::into).collect(),
                    start_time: histogram.start_time().into(),
                    time: histogram.time().into(),
                    temporality: histogram.temporality().into(),
                }
            }
        }

        impl<T: ToPrimitive + Copy> From<&HistogramDataPoint<T>> for wm::HistogramDataPoint {
            fn from(data_point: &HistogramDataPoint<T>) -> Self {
                Self {
                    attributes: data_point.attributes().map(Into::into).collect(),
                    count: data_point.count(),
                    bounds: data_point.bounds().collect(),
                    bucket_counts: data_point.bucket_counts().collect(),
                    min: data_point.min().map(Into::into),
                    max: data_point.max().map(Into::into),
                    sum: data_point.sum().into(),
                    exemplars: data_point.exemplars().map(Into::into).collect(),
                }
            }
        }

        impl<T: ToPrimitive + Copy> From<&ExponentialHistogram<T>> for wm::ExponentialHistogram {
            fn from(histogram: &ExponentialHistogram<T>) -> Self {
                Self {
                    data_points: histogram.data_points().map(Into::into).collect(),
                    start_time: histogram.start_time().into(),
                    time: histogram.time().into(),
                    temporality: histogram.temporality().into(),
                }
            }
        }

        impl<T: ToPrimitive + Copy> From<&ExponentialHistogramDataPoint<T>>
            for wm::ExponentialHistogramDataPoint
        {
            fn from(data_point: &ExponentialHistogramDataPoint<T>) -> Self {
                Self {
                    attributes: data_point.attributes().map(Into::into).collect(),
                    scale: data_point.scale(),
                    zero_count: data_point.zero_count(),
                    positive_bucket: data_point.positive_bucket().into(),
                    negative_bucket: data_point.negative_bucket().into(),
                    zero_threshold: data_point.zero_threshold(),
                    min: data_point.min().map(Into::into),
                    max: data_point.max().map(Into::into),
                    sum: data_point.sum().into(),
                    count: data_point.count() as u64,
                    exemplars: data_point.exemplars().map(Into::into).collect(),
                }
            }
        }

        impl<T: ToPrimitive> From<T> for wm::DataValue {
            fn from(value: T) -> Self {
                value.to_u64().map_or_else(|| value.to_i64().map_or_else(|| Self::F64(value.to_f64().unwrap_or_default()), Self::S64), Self::U64)
            }
        }

        impl From<&ExponentialBucket> for wm::ExponentialBucket {
            fn from(bucket: &ExponentialBucket) -> Self {
                Self {
                    offset: bucket.offset(),
                    counts: bucket.counts().collect(),
                }
            }
        }

        impl From<Temporality> for wm::Temporality {
            fn from(temporality: Temporality) -> Self {
                match temporality {
                    Temporality::Delta => Self::Delta,
                    Temporality::Cumulative => Self::Cumulative,
                    Temporality::LowMemory => Self::LowMemory,
                    _ => unimplemented!(),
                }
            }
        }
    }
}
