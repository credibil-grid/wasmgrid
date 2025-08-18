//! # Metrics
//!
//! Convert OpenTelemetry metrics types in `wasi-otel` types.

use num_traits::ToPrimitive;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::metrics::Temporality;
use opentelemetry_sdk::metrics::data::{
    AggregatedMetrics, Exemplar, ExponentialBucket, ExponentialHistogram,
    ExponentialHistogramDataPoint, Gauge, GaugeDataPoint, Histogram, HistogramDataPoint, Metric,
    MetricData, ResourceMetrics, ScopeMetrics, Sum, SumDataPoint,
};

use crate::generated::wasi::otel::metrics as wasi;

impl From<ResourceMetrics> for wasi::ResourceMetrics {
    fn from(rm: ResourceMetrics) -> Self {
        Self {
            resource: rm.resource().into(),
            scope_metrics: rm.scope_metrics().into_iter().map(Into::into).collect(),
        }
    }
}

impl From<&Resource> for wasi::Resource {
    fn from(resource: &Resource) -> Self {
        Self {
            attributes: resource.iter().map(Into::into).collect(),
            schema_url: resource.schema_url().map(|s| s.to_string()),
        }
    }
}

impl From<&ScopeMetrics> for wasi::ScopeMetrics {
    fn from(scope_metrics: &ScopeMetrics) -> Self {
        Self {
            scope: scope_metrics.scope().into(),
            metrics: scope_metrics.metrics().into_iter().map(Into::into).collect(),
        }
    }
}

impl From<&Metric> for wasi::Metric {
    fn from(metric: &Metric) -> Self {
        Self {
            name: metric.name().to_string(),
            description: metric.description().to_string(),
            unit: metric.unit().to_string(),
            data: metric.data().into(),
        }
    }
}

impl From<&AggregatedMetrics> for wasi::AggregatedMetrics {
    fn from(am: &AggregatedMetrics) -> Self {
        match am {
            AggregatedMetrics::F64(v) => Self::F64(v.into()),
            AggregatedMetrics::I64(v) => Self::S64(v.into()),
            AggregatedMetrics::U64(v) => Self::U64(v.into()),
        }
    }
}

impl<T: ToPrimitive + Copy> From<&MetricData<T>> for wasi::MetricData {
    fn from(md: &MetricData<T>) -> Self {
        match md {
            MetricData::Gauge(v) => Self::Gauge(v.into()),
            MetricData::Sum(v) => Self::Sum(v.into()),
            MetricData::Histogram(v) => Self::Histogram(v.into()),
            MetricData::ExponentialHistogram(v) => Self::ExponentialHistogram(v.into()),
        }
    }
}

impl<T: ToPrimitive + Copy> From<&Gauge<T>> for wasi::Gauge {
    fn from(gauge: &Gauge<T>) -> Self {
        Self {
            data_points: gauge.data_points().into_iter().map(Into::into).collect(),
            start_time: gauge.start_time().map(Into::into),
            time: gauge.time().into(),
        }
    }
}

impl<T: ToPrimitive + Copy> From<&GaugeDataPoint<T>> for wasi::GaugeDataPoint {
    fn from(data_point: &GaugeDataPoint<T>) -> Self {
        Self {
            attributes: data_point.attributes().into_iter().map(Into::into).collect(),
            value: data_point.value().into(),
            exemplars: data_point.exemplars().into_iter().map(Into::into).collect(),
        }
    }
}

impl<T: ToPrimitive + Copy> From<&Exemplar<T>> for wasi::Exemplar {
    fn from(exemplar: &Exemplar<T>) -> Self {
        Self {
            filtered_attributes: exemplar
                .filtered_attributes()
                .into_iter()
                .map(Into::into)
                .collect(),
            time: exemplar.time().into(),
            value: exemplar.value.into(),
            span_id: String::from_utf8_lossy(exemplar.span_id()).into(),
            trace_id: String::from_utf8_lossy(exemplar.trace_id()).into(),
        }
    }
}

impl<T: ToPrimitive + Copy> From<&Sum<T>> for wasi::Sum {
    fn from(sum: &Sum<T>) -> Self {
        Self {
            data_points: sum.data_points().into_iter().map(Into::into).collect(),
            start_time: sum.start_time().into(),
            time: sum.time().into(),
            temporality: sum.temporality().into(),
            is_monotonic: sum.is_monotonic(),
        }
    }
}

impl<T: ToPrimitive + Copy> From<&SumDataPoint<T>> for wasi::SumDataPoint {
    fn from(data_point: &SumDataPoint<T>) -> Self {
        Self {
            attributes: data_point.attributes().into_iter().map(Into::into).collect(),
            value: data_point.value().into(),
            exemplars: data_point.exemplars().into_iter().map(Into::into).collect(),
        }
    }
}

impl<T: ToPrimitive + Copy> From<&Histogram<T>> for wasi::Histogram {
    fn from(histogram: &Histogram<T>) -> Self {
        Self {
            data_points: histogram.data_points().into_iter().map(Into::into).collect(),
            start_time: histogram.start_time().into(),
            time: histogram.time().into(),
            temporality: histogram.temporality().into(),
        }
    }
}

impl<T: ToPrimitive + Copy> From<&HistogramDataPoint<T>> for wasi::HistogramDataPoint {
    fn from(data_point: &HistogramDataPoint<T>) -> Self {
        Self {
            attributes: data_point.attributes().into_iter().map(Into::into).collect(),
            count: data_point.count(),
            bounds: data_point.bounds().collect(),
            bucket_counts: data_point.bucket_counts().collect(),
            min: data_point.min().map(Into::into),
            max: data_point.max().map(Into::into),
            sum: data_point.sum().into(),
            exemplars: data_point.exemplars().into_iter().map(Into::into).collect(),
        }
    }
}

impl<T: ToPrimitive + Copy> From<&ExponentialHistogram<T>> for wasi::ExponentialHistogram {
    fn from(histogram: &ExponentialHistogram<T>) -> Self {
        Self {
            data_points: histogram.data_points().into_iter().map(Into::into).collect(),
            start_time: histogram.start_time().into(),
            time: histogram.time().into(),
            temporality: histogram.temporality().into(),
        }
    }
}

impl<T: ToPrimitive + Copy> From<&ExponentialHistogramDataPoint<T>>
    for wasi::ExponentialHistogramDataPoint
{
    fn from(data_point: &ExponentialHistogramDataPoint<T>) -> Self {
        Self {
            attributes: data_point.attributes().into_iter().map(Into::into).collect(),
            scale: data_point.scale().into(),
            zero_count: data_point.zero_count().into(),
            positive_bucket: data_point.positive_bucket().into(),
            negative_bucket: data_point.negative_bucket().into(),
            zero_threshold: data_point.zero_threshold().into(),
            min: data_point.min().map(Into::into),
            max: data_point.max().map(Into::into),
            sum: data_point.sum().into(),
            count: data_point.count() as u64,
            exemplars: data_point.exemplars().into_iter().map(Into::into).collect(),
        }
    }
}

impl<T: ToPrimitive> From<T> for wasi::DataValue {
    fn from(value: T) -> Self {
        if let Some(val) = value.to_u64() {
            Self::U64(val)
        } else if let Some(val) = value.to_i64() {
            Self::S64(val)
        } else {
            Self::F64(value.to_f64().unwrap_or_default())
        }
    }
}

// // ----------------------------------------------------------------------------
// // i64 types
// // ----------------------------------------------------------------------------
// impl From<&MetricData<i64>> for wasi::MetricData {
//     fn from(md: &MetricData<i64>) -> Self {
//         match md {
//             MetricData::Gauge(v) => Self::Gauge(v.into()),
//             MetricData::Sum(v) => Self::Sum(v.into()),
//             MetricData::Histogram(v) => Self::Histogram(v.into()),
//             MetricData::ExponentialHistogram(v) => Self::ExponentialHistogram(v.into()),
//         }
//     }
// }

// impl From<&Gauge<i64>> for wasi::Gauge {
//     fn from(gauge: &Gauge<i64>) -> Self {
//         Self {
//             data_points: gauge.data_points().into_iter().map(Into::into).collect(),
//             start_time: gauge.start_time().map(Into::into),
//             time: gauge.time().into(),
//         }
//     }
// }

// impl From<&GaugeDataPoint<i64>> for wasi::GaugeDataPoint {
//     fn from(data_point: &GaugeDataPoint<i64>) -> Self {
//         Self {
//             attributes: data_point.attributes().into_iter().map(Into::into).collect(),
//             value: data_point.value().into(),
//             exemplars: data_point.exemplars().into_iter().map(Into::into).collect(),
//         }
//     }
// }

// impl From<&Exemplar<i64>> for wasi::Exemplar {
//     fn from(exemplar: &Exemplar<i64>) -> Self {
//         Self {
//             filtered_attributes: exemplar
//                 .filtered_attributes()
//                 .into_iter()
//                 .map(Into::into)
//                 .collect(),
//             time: exemplar.time().into(),
//             value: exemplar.value.into(),
//             span_id: String::from_utf8_lossy(exemplar.span_id()).into(),
//             trace_id: String::from_utf8_lossy(exemplar.trace_id()).into(),
//         }
//     }
// }

// impl From<&Sum<i64>> for wasi::Sum {
//     fn from(sum: &Sum<i64>) -> Self {
//         Self {
//             data_points: sum.data_points().into_iter().map(Into::into).collect(),
//             start_time: sum.start_time().into(),
//             time: sum.time().into(),
//             temporality: sum.temporality().into(),
//             is_monotonic: sum.is_monotonic(),
//         }
//     }
// }

// impl From<&SumDataPoint<i64>> for wasi::SumDataPoint {
//     fn from(data_point: &SumDataPoint<i64>) -> Self {
//         Self {
//             attributes: data_point.attributes().into_iter().map(Into::into).collect(),
//             value: data_point.value().into(),
//             exemplars: data_point.exemplars().into_iter().map(Into::into).collect(),
//         }
//     }
// }

// impl From<&Histogram<i64>> for wasi::Histogram {
//     fn from(histogram: &Histogram<i64>) -> Self {
//         Self {
//             data_points: histogram.data_points().into_iter().map(Into::into).collect(),
//             start_time: histogram.start_time().into(),
//             time: histogram.time().into(),
//             temporality: histogram.temporality().into(),
//         }
//     }
// }

// impl From<&HistogramDataPoint<i64>> for wasi::HistogramDataPoint {
//     fn from(data_point: &HistogramDataPoint<i64>) -> Self {
//         Self {
//             attributes: data_point.attributes().into_iter().map(Into::into).collect(),
//             count: data_point.count(),
//             bounds: data_point.bounds().collect(),
//             bucket_counts: data_point.bucket_counts().collect(),
//             min: data_point.min().map(Into::into),
//             max: data_point.max().map(Into::into),
//             sum: data_point.sum().into(),
//             exemplars: data_point.exemplars().into_iter().map(Into::into).collect(),
//         }
//     }
// }

// impl From<&ExponentialHistogram<i64>> for wasi::ExponentialHistogram {
//     fn from(histogram: &ExponentialHistogram<i64>) -> Self {
//         Self {
//             data_points: histogram.data_points().into_iter().map(Into::into).collect(),
//             start_time: histogram.start_time().into(),
//             time: histogram.time().into(),
//             temporality: histogram.temporality().into(),
//         }
//     }
// }

// impl From<&ExponentialHistogramDataPoint<i64>> for wasi::ExponentialHistogramDataPoint {
//     fn from(data_point: &ExponentialHistogramDataPoint<i64>) -> Self {
//         Self {
//             attributes: data_point.attributes().into_iter().map(Into::into).collect(),
//             scale: data_point.scale().into(),
//             zero_count: data_point.zero_count().into(),
//             positive_bucket: data_point.positive_bucket().into(),
//             negative_bucket: data_point.negative_bucket().into(),
//             zero_threshold: data_point.zero_threshold().into(),
//             min: data_point.min().map(Into::into),
//             max: data_point.max().map(Into::into),
//             sum: data_point.sum().into(),
//             count: data_point.count() as u64,
//             exemplars: data_point.exemplars().into_iter().map(Into::into).collect(),
//         }
//     }
// }

// impl From<i64> for wasi::DataValue {
//     fn from(value: i64) -> Self {
//         Self::S64(value)
//     }
// }

// // ----------------------------------------------------------------------------
// // u64 types
// // ----------------------------------------------------------------------------
// impl From<&MetricData<u64>> for wasi::MetricData {
//     fn from(md: &MetricData<u64>) -> Self {
//         match md {
//             MetricData::Gauge(v) => Self::Gauge(v.into()),
//             MetricData::Sum(v) => Self::Sum(v.into()),
//             MetricData::Histogram(v) => Self::Histogram(v.into()),
//             MetricData::ExponentialHistogram(v) => Self::ExponentialHistogram(v.into()),
//         }
//     }
// }

// impl From<&Gauge<u64>> for wasi::Gauge {
//     fn from(gauge: &Gauge<u64>) -> Self {
//         Self {
//             data_points: gauge.data_points().into_iter().map(Into::into).collect(),
//             start_time: gauge.start_time().map(Into::into),
//             time: gauge.time().into(),
//         }
//     }
// }

// impl From<&GaugeDataPoint<u64>> for wasi::GaugeDataPoint {
//     fn from(data_point: &GaugeDataPoint<u64>) -> Self {
//         Self {
//             attributes: data_point.attributes().into_iter().map(Into::into).collect(),
//             value: data_point.value().into(),
//             exemplars: data_point.exemplars().into_iter().map(Into::into).collect(),
//         }
//     }
// }

// impl From<&Exemplar<u64>> for wasi::Exemplar {
//     fn from(exemplar: &Exemplar<u64>) -> Self {
//         Self {
//             filtered_attributes: exemplar
//                 .filtered_attributes()
//                 .into_iter()
//                 .map(Into::into)
//                 .collect(),
//             time: exemplar.time().into(),
//             value: exemplar.value.into(),
//             span_id: String::from_utf8_lossy(exemplar.span_id()).into(),
//             trace_id: String::from_utf8_lossy(exemplar.trace_id()).into(),
//         }
//     }
// }

// impl From<&Sum<u64>> for wasi::Sum {
//     fn from(sum: &Sum<u64>) -> Self {
//         Self {
//             data_points: sum.data_points().into_iter().map(Into::into).collect(),
//             start_time: sum.start_time().into(),
//             time: sum.time().into(),
//             temporality: sum.temporality().into(),
//             is_monotonic: sum.is_monotonic(),
//         }
//     }
// }

// impl From<&SumDataPoint<u64>> for wasi::SumDataPoint {
//     fn from(data_point: &SumDataPoint<u64>) -> Self {
//         Self {
//             attributes: data_point.attributes().into_iter().map(Into::into).collect(),
//             value: data_point.value().into(),
//             exemplars: data_point.exemplars().into_iter().map(Into::into).collect(),
//         }
//     }
// }

// impl From<&Histogram<u64>> for wasi::Histogram {
//     fn from(histogram: &Histogram<u64>) -> Self {
//         Self {
//             data_points: histogram.data_points().into_iter().map(Into::into).collect(),
//             start_time: histogram.start_time().into(),
//             time: histogram.time().into(),
//             temporality: histogram.temporality().into(),
//         }
//     }
// }

// impl From<&HistogramDataPoint<u64>> for wasi::HistogramDataPoint {
//     fn from(data_point: &HistogramDataPoint<u64>) -> Self {
//         Self {
//             attributes: data_point.attributes().into_iter().map(Into::into).collect(),
//             count: data_point.count(),
//             bounds: data_point.bounds().collect(),
//             bucket_counts: data_point.bucket_counts().collect(),
//             min: data_point.min().map(Into::into),
//             max: data_point.max().map(Into::into),
//             sum: data_point.sum().into(),
//             exemplars: data_point.exemplars().into_iter().map(Into::into).collect(),
//         }
//     }
// }

// impl From<&ExponentialHistogram<u64>> for wasi::ExponentialHistogram {
//     fn from(histogram: &ExponentialHistogram<u64>) -> Self {
//         Self {
//             data_points: histogram.data_points().into_iter().map(Into::into).collect(),
//             start_time: histogram.start_time().into(),
//             time: histogram.time().into(),
//             temporality: histogram.temporality().into(),
//         }
//     }
// }

// impl From<&ExponentialHistogramDataPoint<u64>> for wasi::ExponentialHistogramDataPoint {
//     fn from(data_point: &ExponentialHistogramDataPoint<u64>) -> Self {
//         Self {
//             attributes: data_point.attributes().into_iter().map(Into::into).collect(),
//             scale: data_point.scale().into(),
//             zero_count: data_point.zero_count().into(),
//             positive_bucket: data_point.positive_bucket().into(),
//             negative_bucket: data_point.negative_bucket().into(),
//             zero_threshold: data_point.zero_threshold().into(),
//             min: data_point.min().map(Into::into),
//             max: data_point.max().map(Into::into),
//             sum: data_point.sum().into(),
//             count: data_point.count() as u64,
//             exemplars: data_point.exemplars().into_iter().map(Into::into).collect(),
//         }
//     }
// }

// impl From<u64> for wasi::DataValue {
//     fn from(value: u64) -> Self {
//         Self::U64(value)
//     }
// }

impl From<&ExponentialBucket> for wasi::ExponentialBucket {
    fn from(bucket: &ExponentialBucket) -> Self {
        Self {
            offset: bucket.offset(),
            counts: bucket.counts().collect(),
        }
    }
}

impl From<Temporality> for wasi::Temporality {
    fn from(temporality: Temporality) -> Self {
        match temporality {
            Temporality::Delta => Self::Delta,
            Temporality::Cumulative => Self::Cumulative,
            Temporality::LowMemory => Self::LowMemory,
            _ => unimplemented!(),
        }
    }
}
