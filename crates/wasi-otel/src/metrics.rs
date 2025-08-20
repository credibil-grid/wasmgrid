//! # WASI Tracing

use anyhow::{Result, anyhow};
use http::header::CONTENT_TYPE;
use opentelemetry_proto::tonic::collector::metrics::v1::ExportMetricsServiceRequest;
use opentelemetry_proto::tonic::common::v1::any_value::Value;
use opentelemetry_proto::tonic::common::v1::{
    AnyValue, ArrayValue, InstrumentationScope, KeyValue,
};
use opentelemetry_proto::tonic::metrics::v1::exemplar::Value as ExemplarValue;
use opentelemetry_proto::tonic::metrics::v1::exponential_histogram_data_point::Buckets;
use opentelemetry_proto::tonic::metrics::v1::metric::Data as MetricData;
use opentelemetry_proto::tonic::metrics::v1::number_data_point::Value as NumberValue;
use opentelemetry_proto::tonic::metrics::v1::{
    AggregationTemporality, Exemplar, ExponentialHistogram, ExponentialHistogramDataPoint, Gauge,
    Histogram, HistogramDataPoint, Metric, NumberDataPoint, ResourceMetrics, ScopeMetrics, Sum,
};
use opentelemetry_proto::tonic::resource::v1::Resource;
use opentelemetry_sdk::error::OTelSdkError;
use prost::Message;

use crate::Otel;
use crate::generated::wasi::otel::metrics::{self as wm};
use crate::generated::wasi::otel::types;

impl wm::Host for Otel<'_> {
    async fn export(&mut self, rm: wm::ResourceMetrics) -> Result<(), types::Error> {
        // convert to opentelemetry metrics
        let request = ExportMetricsServiceRequest::from(rm);
        let body = Message::encode_to_vec(&request);

        reqwest::Client::new()
            .post("http://localhost:4318/v1/metrics")
            .header(CONTENT_TYPE, "application/x-protobuf")
            .body(body)
            .send()
            .await
            .map_err(|e| anyhow!("{e:?}"))?;

        Ok(())
    }
}

impl types::Host for Otel<'_> {
    fn convert_error(&mut self, err: types::Error) -> anyhow::Result<types::Error> {
        tracing::error!("{err}");
        Ok(err)
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

impl From<wm::ResourceMetrics> for ExportMetricsServiceRequest {
    fn from(rm: wm::ResourceMetrics) -> Self {
        let schema_url = rm.resource.schema_url.clone().unwrap_or_default();

        Self {
            resource_metrics: vec![ResourceMetrics {
                resource: Some(rm.resource.into()),
                scope_metrics: rm.scope_metrics.into_iter().map(Into::into).collect(),
                schema_url,
            }],
        }
    }
}

impl From<wm::Resource> for Resource {
    fn from(resource: wm::Resource) -> Self {
        let mut attributes = resource.attributes.into_iter().map(Into::into).collect::<Vec<_>>();
        attributes.push(KeyValue {
            key: "schema_url".to_string(),
            value: resource.schema_url.as_ref().map(|s| AnyValue {
                value: Some(Value::StringValue(s.clone())),
            }),
        });

        Self {
            attributes,
            dropped_attributes_count: 0,
            entity_refs: vec![],
        }
    }
}

impl From<wm::KeyValue> for KeyValue {
    fn from(value: wm::KeyValue) -> Self {
        Self {
            key: value.key,
            value: Some(value.value.into()),
        }
    }
}

impl From<wm::Value> for AnyValue {
    fn from(value: wm::Value) -> Self {
        let v: Value = match value {
            wm::Value::Bool(v) => Value::BoolValue(v),
            wm::Value::S64(v) => Value::IntValue(v),
            wm::Value::F64(v) => Value::DoubleValue(v),
            wm::Value::String(v) => Value::StringValue(v),
            wm::Value::BoolArray(items) => Value::ArrayValue(ArrayValue {
                values: items
                    .into_iter()
                    .map(|v| Self {
                        value: Some(Value::BoolValue(v)),
                    })
                    .collect(),
            }),
            wm::Value::S64Array(items) => Value::ArrayValue(ArrayValue {
                values: items
                    .into_iter()
                    .map(|v| Self {
                        value: Some(Value::IntValue(v)),
                    })
                    .collect(),
            }),
            wm::Value::F64Array(items) => Value::ArrayValue(ArrayValue {
                values: items
                    .into_iter()
                    .map(|v| Self {
                        value: Some(Value::DoubleValue(v)),
                    })
                    .collect(),
            }),
            wm::Value::StringArray(items) => Value::ArrayValue(ArrayValue {
                values: items
                    .into_iter()
                    .map(|v| Self {
                        value: Some(Value::StringValue(v)),
                    })
                    .collect(),
            }),
        };

        Self { value: Some(v) }
    }
}

impl From<wm::ScopeMetrics> for ScopeMetrics {
    fn from(sm: wm::ScopeMetrics) -> Self {
        let schema_url = sm.scope.clone().schema_url.unwrap_or_default();

        Self {
            scope: Some(sm.scope.into()),
            metrics: sm.metrics.into_iter().map(Into::into).collect(),
            schema_url,
        }
    }
}

impl From<wm::InstrumentationScope> for InstrumentationScope {
    fn from(data: wm::InstrumentationScope) -> Self {
        Self {
            name: data.name,
            version: data.version.unwrap_or_default(),
            attributes: data.attributes.into_iter().map(Into::into).collect(),
            dropped_attributes_count: 0,
        }
    }
}

impl From<wm::Metric> for Metric {
    fn from(metric: wm::Metric) -> Self {
        Self {
            name: metric.name,
            description: metric.description,
            unit: metric.unit,
            metadata: vec![],
            data: Some(match metric.data {
                wm::AggregatedMetrics::F64(data)
                | wm::AggregatedMetrics::U64(data)
                | wm::AggregatedMetrics::S64(data) => data.into(),
            }),
        }
    }
}

impl From<wm::MetricData> for MetricData {
    fn from(data: wm::MetricData) -> Self {
        match data {
            wm::MetricData::Gauge(gauge) => Self::Gauge(gauge.into()),
            wm::MetricData::Sum(sum) => Self::Sum(sum.into()),
            wm::MetricData::Histogram(hist) => Self::Histogram(hist.into()),
            wm::MetricData::ExponentialHistogram(hist) => Self::ExponentialHistogram(hist.into()),
        }
    }
}

impl From<wm::Gauge> for Gauge {
    fn from(gauge: wm::Gauge) -> Self {
        Self {
            data_points: gauge
                .data_points
                .into_iter()
                .map(|dp| NumberDataPoint {
                    attributes: dp.attributes.into_iter().map(Into::into).collect(),
                    start_time_unix_nano: gauge.start_time.map(Into::into).unwrap_or_default(),
                    time_unix_nano: gauge.time.into(),
                    exemplars: dp.exemplars.into_iter().map(Into::into).collect(),
                    flags: DataPointFlags::default() as u32,
                    value: Some(dp.value.into()),
                })
                .collect(),
        }
    }
}

impl From<wm::Sum> for Sum {
    fn from(sum: wm::Sum) -> Self {
        Self {
            data_points: sum
                .data_points
                .into_iter()
                .map(|dp| NumberDataPoint {
                    attributes: dp.attributes.into_iter().map(Into::into).collect(),
                    start_time_unix_nano: sum.start_time.into(),
                    time_unix_nano: sum.time.into(),
                    exemplars: dp.exemplars.into_iter().map(Into::into).collect(),
                    flags: DataPointFlags::default() as u32,
                    value: Some(dp.value.into()),
                })
                .collect(),
            aggregation_temporality: AggregationTemporality::from(sum.temporality).into(),
            is_monotonic: sum.is_monotonic,
        }
    }
}

impl From<wm::Histogram> for Histogram {
    fn from(hist: wm::Histogram) -> Self {
        Self {
            data_points: hist
                .data_points
                .into_iter()
                .map(|dp| HistogramDataPoint {
                    attributes: dp.attributes.into_iter().map(Into::into).collect(),
                    start_time_unix_nano: hist.start_time.into(),
                    time_unix_nano: hist.time.into(),
                    count: dp.count,
                    sum: Some(dp.sum.into()),
                    bucket_counts: dp.bucket_counts,
                    explicit_bounds: dp.bounds,
                    exemplars: dp.exemplars.into_iter().map(Into::into).collect(),
                    flags: DataPointFlags::default() as u32,
                    min: dp.min.map(Into::into),
                    max: dp.max.map(Into::into),
                })
                .collect(),
            aggregation_temporality: hist.temporality.into(),
        }
    }
}

impl From<wm::ExponentialHistogram> for ExponentialHistogram {
    fn from(hist: wm::ExponentialHistogram) -> Self {
        Self {
            data_points: hist
                .data_points
                .into_iter()
                .map(|dp| ExponentialHistogramDataPoint {
                    attributes: dp.attributes.into_iter().map(Into::into).collect(),
                    start_time_unix_nano: hist.start_time.into(),
                    time_unix_nano: hist.time.into(),
                    count: dp.count,
                    sum: Some(dp.sum.into()),
                    scale: dp.scale.into(),
                    zero_count: dp.zero_count,
                    positive: Some(Buckets {
                        offset: dp.positive_bucket.offset,
                        bucket_counts: dp.positive_bucket.counts,
                    }),
                    negative: Some(Buckets {
                        offset: dp.negative_bucket.offset,
                        bucket_counts: dp.negative_bucket.counts,
                    }),
                    flags: DataPointFlags::default() as u32,
                    exemplars: dp.exemplars.into_iter().map(Into::into).collect(),
                    min: dp.min.map(Into::into),
                    max: dp.max.map(Into::into),
                    zero_threshold: dp.zero_threshold,
                })
                .collect(),
            aggregation_temporality: hist.temporality.into(),
        }
    }
}

impl From<wm::Exemplar> for Exemplar {
    fn from(ex: wm::Exemplar) -> Self {
        Self {
            filtered_attributes: ex.filtered_attributes.into_iter().map(Into::into).collect(),
            time_unix_nano: ex.time.into(),
            span_id: ex.span_id.as_bytes().to_vec(),
            trace_id: ex.trace_id.as_bytes().to_vec(),
            value: Some(ex.value.into()),
        }
    }
}

#[allow(clippy::cast_possible_wrap)]
impl From<wm::DataValue> for ExemplarValue {
    fn from(dv: wm::DataValue) -> Self {
        match dv {
            wm::DataValue::U64(v) => Self::AsInt(v as i64),
            wm::DataValue::S64(v) => Self::AsInt(v),
            wm::DataValue::F64(v) => Self::AsDouble(v),
        }
    }
}

#[allow(clippy::cast_possible_wrap)]
impl From<wm::DataValue> for NumberValue {
    fn from(dv: wm::DataValue) -> Self {
        match dv {
            wm::DataValue::U64(v) => Self::AsInt(v as i64),
            wm::DataValue::S64(v) => Self::AsInt(v),
            wm::DataValue::F64(v) => Self::AsDouble(v),
        }
    }
}

#[allow(clippy::cast_precision_loss)]
impl From<wm::DataValue> for f64 {
    fn from(dv: wm::DataValue) -> Self {
        match dv {
            wm::DataValue::U64(v) => v as Self,
            wm::DataValue::S64(v) => v as Self,
            wm::DataValue::F64(v) => v,
        }
    }
}

impl From<wm::Temporality> for AggregationTemporality {
    fn from(temporality: wm::Temporality) -> Self {
        match temporality {
            wm::Temporality::Cumulative => Self::Cumulative,
            wm::Temporality::Delta => Self::Delta,
            wm::Temporality::LowMemory => Self::Unspecified,
        }
    }
}

impl From<wm::Temporality> for i32 {
    fn from(temporality: wm::Temporality) -> Self {
        AggregationTemporality::from(temporality) as Self
    }
}

#[derive(Default)]
#[repr(i32)]
pub enum DataPointFlags {
    #[default]
    DoNotUse = 0,
    // NoRecordedValueMask = 1,
}
