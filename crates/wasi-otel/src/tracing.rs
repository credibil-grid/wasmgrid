//! # WASI Tracing

use std::collections::HashMap;
use std::env;

use anyhow::{Context, Result};
use credibil_otel::init;
use http::header::CONTENT_TYPE;
use opentelemetry::trace::{self as otel, TraceContextExt};
use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;
use opentelemetry_proto::tonic::resource::v1::Resource;
use opentelemetry_proto::tonic::trace::v1::span::{Event, Link};
use opentelemetry_proto::tonic::trace::v1::status::StatusCode;
use opentelemetry_proto::tonic::trace::v1::{ResourceSpans, ScopeSpans, Span, Status};
use prost::Message;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::generated::wasi::otel as wasi_otel;
use crate::generated::wasi::otel::tracing::{self as wasi};
use crate::{DEF_HTTP_ADDR, Host};

impl wasi_otel::tracing::Host for Host<'_> {
    async fn context(&mut self) -> Result<wasi::SpanContext> {
        let ctx = tracing::Span::current().context();
        let span = ctx.span();
        Ok(wasi::SpanContext::from(span.span_context()))
    }

    async fn export(&mut self, span_data: Vec<wasi::SpanData>) -> Result<(), wasi::Error> {
        let http_client = self.http_client.clone();

        // export to collector in background to avoid blocking
        tokio::spawn(async move {
            // convert to opentelemetry export format
            let resource_spans = resource_spans(span_data, init::resource());
            let request = ExportTraceServiceRequest { resource_spans };

            let body = Message::encode_to_vec(&request);
            let addr = env::var("OTEL_HTTP_ADDR").unwrap_or_else(|_| DEF_HTTP_ADDR.to_string());

            // export to collector
            if let Err(e) = http_client
                .post(format!("{addr}/v1/traces"))
                .header(CONTENT_TYPE, "application/x-protobuf")
                .body(body)
                .send()
                .await
                .context("sending traces")
            {
                tracing::error!("failed to send traces: {e}");
            }
        });

        Ok(())
    }
}

pub fn resource_spans(
    spans: Vec<wasi::SpanData>, resource: &opentelemetry_sdk::Resource,
) -> Vec<ResourceSpans> {
    // group spans by InstrumentationScope
    let scope_map = spans.into_iter().fold(
        HashMap::new(),
        |mut scope_map: HashMap<wasi::InstrumentationScope, Vec<wasi::SpanData>>, span| {
            let instrumentation = span.instrumentation_scope.clone();
            scope_map.entry(instrumentation).or_default().push(span);
            scope_map
        },
    );

    // convert into ScopeSpans
    let scope_spans = scope_map
        .into_values()
        .map(|spans| ScopeSpans {
            scope: Some(spans[0].instrumentation_scope.clone().into()),
            schema_url: resource.schema_url().map(Into::into).unwrap_or_default(),
            spans: spans.into_iter().map(Into::into).collect(),
        })
        .collect();

    // create ResourceSpans
    vec![ResourceSpans {
        resource: Some(Resource {
            attributes: resource.iter().map(Into::into).collect(),
            dropped_attributes_count: 0,
            entity_refs: vec![],
        }),
        scope_spans,
        schema_url: resource.schema_url().map(Into::into).unwrap_or_default(),
    }]
}

impl From<wasi::SpanData> for Span {
    fn from(span: wasi::SpanData) -> Self {
        let trace_state = span.span_context.trace_state;
        let trace_state =
            trace_state.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join(",");

        Self {
            trace_id: hex::decode(span.span_context.trace_id).unwrap_or_default(),
            span_id: hex::decode(span.span_context.span_id).unwrap_or_default(),
            trace_state,
            parent_span_id: hex::decode(span.parent_span_id).unwrap_or_default(),
            flags: span.span_context.trace_flags.into(),
            name: span.name,
            kind: span.span_kind as i32,
            start_time_unix_nano: span.start_time.into(),
            end_time_unix_nano: span.end_time.into(),
            attributes: span.attributes.into_iter().map(Into::into).collect(),
            dropped_attributes_count: span.dropped_attributes,
            events: span.events.into_iter().map(Into::into).collect(),
            dropped_events_count: span.dropped_events,
            links: span.links.into_iter().map(Into::into).collect(),
            dropped_links_count: span.dropped_links,
            status: Some(span.status.into()),
        }
    }
}

impl From<&otel::SpanContext> for wasi::SpanContext {
    fn from(ctx: &otel::SpanContext) -> Self {
        Self {
            trace_id: ctx.trace_id().to_string(),
            span_id: ctx.span_id().to_string(),
            trace_flags: ctx.trace_flags().into(),
            is_remote: ctx.is_remote(),
            trace_state: ctx
                .trace_state()
                .header()
                .split(',')
                .filter_map(|s| {
                    if let Some((key, ctx)) = s.split_once('=') {
                        Some((key.to_string(), ctx.to_string()))
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }
}

impl From<otel::TraceFlags> for wasi::TraceFlags {
    fn from(value: otel::TraceFlags) -> Self {
        if value.is_sampled() { Self::SAMPLED } else { Self::empty() }
    }
}

impl From<wasi::TraceFlags> for u32 {
    fn from(value: wasi::TraceFlags) -> Self {
        if value.contains(wasi::TraceFlags::SAMPLED) {
            Self::from(otel::TraceFlags::SAMPLED.to_u8())
        } else {
            Self::from(otel::TraceFlags::NOT_SAMPLED.to_u8())
        }
    }
}

impl From<wasi::Event> for Event {
    fn from(event: wasi::Event) -> Self {
        Self {
            time_unix_nano: event.time.into(),
            name: event.name,
            attributes: event.attributes.into_iter().map(Into::into).collect(),
            dropped_attributes_count: 0,
        }
    }
}

impl From<wasi::Link> for Link {
    fn from(link: wasi::Link) -> Self {
        let attrs = link.attributes.into_iter().map(Into::into).collect();

        let trace_state = link.span_context.trace_state;
        let trace_state =
            trace_state.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join(",");

        Self {
            trace_id: hex::decode(link.span_context.trace_id).unwrap_or_default(),
            span_id: hex::decode(link.span_context.span_id).unwrap_or_default(),
            trace_state,
            attributes: attrs,
            dropped_attributes_count: 0,
            flags: link.span_context.trace_flags.into(),
        }
    }
}

impl From<wasi::Status> for Status {
    fn from(value: wasi::Status) -> Self {
        match value {
            wasi::Status::Unset => Self::default(),
            wasi::Status::Error(description) => Self {
                code: StatusCode::Error.into(),
                message: description,
            },
            wasi::Status::Ok => Self {
                code: StatusCode::Ok.into(),
                message: String::new(),
            },
        }
    }
}
