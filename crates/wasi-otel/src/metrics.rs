//! # WASI Tracing

use std::env;

use anyhow::Result;
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

use crate::generated::wasi::otel::{metrics as wasi, metrics, types};
use crate::{DEF_HTTP_ADDR, Host};

impl metrics::Host for Host<'_> {
    async fn export(&mut self, rm: wasi::ResourceMetrics) -> Result<(), wasi::Error> {
        let http_client = self.http_client.clone();

        // export to collector in background to avoid blocking
        tokio::spawn(async move {
            // convert to opentelemetry export format
            let request = ExportMetricsServiceRequest::from(rm);
            let body = Message::encode_to_vec(&request);
            let addr = env::var("OTEL_HTTP_ADDR").unwrap_or_else(|_| DEF_HTTP_ADDR.to_string());

            // export to collector
            if let Err(e) = http_client
                .post(format!("{addr}/v1/metrics"))
                .header(CONTENT_TYPE, "application/x-protobuf")
                .body(body)
                .send()
                .await
            {
                tracing::error!("failed to send metrics: {e}");
            }
        });

        Ok(())
    }
}

impl types::Host for Host<'_> {
    fn convert_error(&mut self, err: wasi::Error) -> anyhow::Result<wasi::Error> {
        tracing::error!("{err}");
        Ok(err)
    }
}

impl From<OTelSdkError> for wasi::Error {
    fn from(err: OTelSdkError) -> Self {
        match err {
            OTelSdkError::AlreadyShutdown => Self::AlreadyShutdown,
            OTelSdkError::Timeout(duration) => Self::Timeout(duration.as_secs()),
            OTelSdkError::InternalFailure(msg) => Self::InternalFailure(msg),
        }
    }
}

impl From<anyhow::Error> for wasi::Error {
    fn from(err: anyhow::Error) -> Self {
        Self::InternalFailure(err.to_string())
    }
}

impl From<wasi::ResourceMetrics> for ExportMetricsServiceRequest {
    fn from(rm: wasi::ResourceMetrics) -> Self {
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

impl From<wasi::Resource> for Resource {
    fn from(resource: wasi::Resource) -> Self {
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

impl From<wasi::KeyValue> for KeyValue {
    fn from(value: wasi::KeyValue) -> Self {
        Self {
            key: value.key,
            value: Some(value.value.into()),
        }
    }
}

impl From<wasi::Value> for AnyValue {
    fn from(value: wasi::Value) -> Self {
        let v: Value = match value {
            wasi::Value::Bool(v) => Value::BoolValue(v),
            wasi::Value::S64(v) => Value::IntValue(v),
            wasi::Value::F64(v) => Value::DoubleValue(v),
            wasi::Value::String(v) => Value::StringValue(v),
            wasi::Value::BoolArray(items) => Value::ArrayValue(ArrayValue {
                values: items
                    .into_iter()
                    .map(|v| Self {
                        value: Some(Value::BoolValue(v)),
                    })
                    .collect(),
            }),
            wasi::Value::S64Array(items) => Value::ArrayValue(ArrayValue {
                values: items
                    .into_iter()
                    .map(|v| Self {
                        value: Some(Value::IntValue(v)),
                    })
                    .collect(),
            }),
            wasi::Value::F64Array(items) => Value::ArrayValue(ArrayValue {
                values: items
                    .into_iter()
                    .map(|v| Self {
                        value: Some(Value::DoubleValue(v)),
                    })
                    .collect(),
            }),
            wasi::Value::StringArray(items) => Value::ArrayValue(ArrayValue {
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

impl From<wasi::ScopeMetrics> for ScopeMetrics {
    fn from(sm: wasi::ScopeMetrics) -> Self {
        let schema_url = sm.scope.clone().schema_url.unwrap_or_default();

        Self {
            scope: Some(sm.scope.into()),
            metrics: sm.metrics.into_iter().map(Into::into).collect(),
            schema_url,
        }
    }
}

impl From<wasi::InstrumentationScope> for InstrumentationScope {
    fn from(data: wasi::InstrumentationScope) -> Self {
        Self {
            name: data.name,
            version: data.version.unwrap_or_default(),
            attributes: data.attributes.into_iter().map(Into::into).collect(),
            dropped_attributes_count: 0,
        }
    }
}

impl From<wasi::Metric> for Metric {
    fn from(metric: wasi::Metric) -> Self {
        Self {
            name: metric.name,
            description: metric.description,
            unit: metric.unit,
            metadata: vec![],
            data: Some(match metric.data {
                wasi::AggregatedMetrics::F64(data)
                | wasi::AggregatedMetrics::U64(data)
                | wasi::AggregatedMetrics::S64(data) => data.into(),
            }),
        }
    }
}

impl From<wasi::MetricData> for MetricData {
    fn from(data: wasi::MetricData) -> Self {
        match data {
            wasi::MetricData::Gauge(gauge) => Self::Gauge(gauge.into()),
            wasi::MetricData::Sum(sum) => Self::Sum(sum.into()),
            wasi::MetricData::Histogram(hist) => Self::Histogram(hist.into()),
            wasi::MetricData::ExponentialHistogram(hist) => Self::ExponentialHistogram(hist.into()),
        }
    }
}

impl From<wasi::Gauge> for Gauge {
    fn from(gauge: wasi::Gauge) -> Self {
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

impl From<wasi::Sum> for Sum {
    fn from(sum: wasi::Sum) -> Self {
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

impl From<wasi::Histogram> for Histogram {
    fn from(hist: wasi::Histogram) -> Self {
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

impl From<wasi::ExponentialHistogram> for ExponentialHistogram {
    fn from(hist: wasi::ExponentialHistogram) -> Self {
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

impl From<wasi::Exemplar> for Exemplar {
    fn from(ex: wasi::Exemplar) -> Self {
        Self {
            filtered_attributes: ex.filtered_attributes.into_iter().map(Into::into).collect(),
            time_unix_nano: ex.time.into(),
            span_id: hex::decode(ex.span_id).unwrap_or_default(),
            trace_id: hex::decode(ex.trace_id).unwrap_or_default(),
            value: Some(ex.value.into()),
        }
    }
}

#[allow(clippy::cast_possible_wrap)]
impl From<wasi::DataValue> for ExemplarValue {
    fn from(dv: wasi::DataValue) -> Self {
        match dv {
            wasi::DataValue::U64(v) => Self::AsInt(v as i64),
            wasi::DataValue::S64(v) => Self::AsInt(v),
            wasi::DataValue::F64(v) => Self::AsDouble(v),
        }
    }
}

#[allow(clippy::cast_possible_wrap)]
impl From<wasi::DataValue> for NumberValue {
    fn from(dv: wasi::DataValue) -> Self {
        match dv {
            wasi::DataValue::U64(v) => Self::AsInt(v as i64),
            wasi::DataValue::S64(v) => Self::AsInt(v),
            wasi::DataValue::F64(v) => Self::AsDouble(v),
        }
    }
}

#[allow(clippy::cast_precision_loss)]
impl From<wasi::DataValue> for f64 {
    fn from(dv: wasi::DataValue) -> Self {
        match dv {
            wasi::DataValue::U64(v) => v as Self,
            wasi::DataValue::S64(v) => v as Self,
            wasi::DataValue::F64(v) => v,
        }
    }
}

impl From<wasi::Temporality> for AggregationTemporality {
    fn from(temporality: wasi::Temporality) -> Self {
        match temporality {
            wasi::Temporality::Cumulative => Self::Cumulative,
            wasi::Temporality::Delta => Self::Delta,
            wasi::Temporality::LowMemory => Self::Unspecified,
        }
    }
}

impl From<wasi::Temporality> for i32 {
    fn from(temporality: wasi::Temporality) -> Self {
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
