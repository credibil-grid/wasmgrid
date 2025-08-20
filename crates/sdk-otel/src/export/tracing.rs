//! # Tracing
//!
//! Convert OpenTelemetry tracing types in `wasi-otel` types.

use anyhow::Result;
use cfg_if::cfg_if;
#[cfg(feature = "guest-mode")]
use opentelemetry_otlp::{SpanExporter, WithHttpConfig};
use opentelemetry_sdk::error::OTelSdkError;
use opentelemetry_sdk::trace::SpanData;

#[cfg(not(feature = "guest-mode"))]
use crate::generated::wasi::otel::tracing as wasi;

pub fn exporter() -> Result<Exporter> {
    cfg_if! {
        if #[cfg(feature = "guest-mode")] {
            use crate::export::ExportClient;
            let inner = SpanExporter::builder().with_http().with_http_client(ExportClient).build()?;
            Ok(Exporter { inner })
        } else {
            Ok(Exporter{})
        }
    }
}

#[derive(Debug)]
pub struct Exporter {
    #[cfg(feature = "guest-mode")]
    inner: SpanExporter,
    // resource: Resource,
}

impl opentelemetry_sdk::trace::SpanExporter for Exporter {
    #[cfg(feature = "guest-mode")]
    async fn export(&self, span_data: Vec<SpanData>) -> Result<(), OTelSdkError> {
        self.inner.export(span_data).await
    }

    #[cfg(not(feature = "guest-mode"))]
    async fn export(&self, span_data: Vec<SpanData>) -> Result<(), OTelSdkError> {
        for sd in span_data {
            wasi::on_end(&sd.into());
        }
        Ok(())
    }

    #[cfg(feature = "guest-mode")]
    fn set_resource(&mut self, resource: &opentelemetry_sdk::Resource) {
        self.inner.set_resource(resource);
    }
}

cfg_if! {
    if #[cfg(not(feature = "guest-mode"))] {
        use opentelemetry::trace as otel;

        impl From<SpanData> for wasi::SpanData {
            fn from(sd: SpanData) -> Self {
                Self {
                    span_context: sd.span_context.into(),
                    parent_span_id: sd.parent_span_id.to_string(),
                    span_kind: sd.span_kind.into(),
                    name: sd.name.to_string(),
                    start_time: sd.start_time.into(),
                    end_time: sd.end_time.into(),
                    attributes: sd.attributes.into_iter().map(Into::into).collect(),
                    events: sd.events.events.into_iter().map(Into::into).collect(),
                    links: sd.links.links.into_iter().map(Into::into).collect(),
                    status: sd.status.into(),
                    instrumentation_scope: sd.instrumentation_scope.into(),
                    dropped_attributes: sd.dropped_attributes_count,
                    dropped_events: sd.events.dropped_count,
                    dropped_links: sd.links.dropped_count,
                }
            }
        }

        impl From<otel::SpanContext> for wasi::SpanContext {
            fn from(sc: otel::SpanContext) -> Self {
                Self {
                    trace_id: format!("{:x}", sc.trace_id()),
                    span_id: format!("{:x}", sc.span_id()),
                    trace_flags: sc.trace_flags().into(),
                    is_remote: sc.is_remote(),
                    trace_state: sc
                        .trace_state()
                        .header()
                        .split(',')
                        .filter_map(|s| {
                            if let Some((key, value)) = s.split_once('=') {
                                Some((key.to_string(), value.to_string()))
                            } else {
                                None
                            }
                        })
                        .collect(),
                }
            }
        }

        impl From<otel::TraceFlags> for wasi::TraceFlags {
            fn from(tf: otel::TraceFlags) -> Self {
                if tf.is_sampled() { wasi::TraceFlags::SAMPLED } else { wasi::TraceFlags::empty() }
            }
        }

        impl From<otel::SpanKind> for wasi::SpanKind {
            fn from(sk: otel::SpanKind) -> Self {
                match sk {
                    otel::SpanKind::Client => Self::Client,
                    otel::SpanKind::Server => Self::Server,
                    otel::SpanKind::Producer => Self::Producer,
                    otel::SpanKind::Consumer => Self::Consumer,
                    otel::SpanKind::Internal => Self::Internal,
                }
            }
        }

        impl From<otel::Event> for wasi::Event {
            fn from(event: otel::Event) -> Self {
                Self {
                    name: event.name.to_string(),
                    time: event.timestamp.into(),
                    attributes: event.attributes.into_iter().map(Into::into).collect(),
                }
            }
        }

        impl From<otel::Link> for wasi::Link {
            fn from(link: otel::Link) -> Self {
                Self {
                    span_context: link.span_context.into(),
                    attributes: link.attributes.into_iter().map(Into::into).collect(),
                }
            }
        }

        impl From<otel::Status> for wasi::Status {
            fn from(status: otel::Status) -> Self {
                match status {
                    otel::Status::Unset => Self::Unset,
                    otel::Status::Error { description } => Self::Error(description.to_string()),
                    otel::Status::Ok => Self::Ok,
                }
            }
        }
    }
}
