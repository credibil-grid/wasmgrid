//! # WASI Tracing

use std::collections::HashMap;

use anyhow::{Context, Result};
use credibil_otel::init;
use http::header::CONTENT_TYPE;
use opentelemetry::trace::{
    TraceContextExt, {self as otel},
};
use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;
// use opentelemetry_proto::tonic::common::v1::InstrumentationScope;
use opentelemetry_proto::tonic::resource::v1::Resource;
use opentelemetry_proto::tonic::trace::v1::span::{Event, Link};
use opentelemetry_proto::tonic::trace::v1::status::StatusCode;
use opentelemetry_proto::tonic::trace::v1::{ResourceSpans, ScopeSpans, Span, Status};
use prost::Message;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::generated::wasi::otel as wasi_otel;
use crate::generated::wasi::otel::tracing::{self as wt};
use crate::{OTEL_ADDR, Otel};

impl wasi_otel::tracing::Host for Otel<'_> {
    async fn current_span_context(&mut self) -> Result<wt::SpanContext> {
        let ctx = tracing::Span::current().context();
        let span = ctx.span();
        Ok(wt::SpanContext::from(span.span_context()))
    }

    async fn export(&mut self, span_data: Vec<wt::SpanData>) -> Result<()> {
        let resource_spans = resource_spans(span_data, init::resource());
        let request = ExportTraceServiceRequest { resource_spans };

        let body = Message::encode_to_vec(&request);
        let addr = option_env!("OTEL_ADDR").unwrap_or(OTEL_ADDR);

        // export to collector
        self.http_client
            .post(format!("{addr}/v1/traces"))
            .header(CONTENT_TYPE, "application/x-protobuf")
            .body(body)
            .send()
            .await
            .context("sending traces")?;

        Ok(())
    }
}

pub fn resource_spans(
    spans: Vec<wt::SpanData>, resource: &opentelemetry_sdk::Resource,
) -> Vec<ResourceSpans> {
    // Group spans by their instrumentation scope
    let scope_map = spans.into_iter().fold(
        HashMap::new(),
        |mut scope_map: HashMap<opentelemetry::InstrumentationScope, Vec<wt::SpanData>>, span| {
            let instrumentation =
                opentelemetry::InstrumentationScope::from(span.instrumentation_scope.clone());
            scope_map.entry(instrumentation).or_default().push(span);
            scope_map
        },
    );

    // Convert the grouped spans into ScopeSpans
    let scope_spans = scope_map
        .into_iter()
        .map(|(_, spans)| ScopeSpans {
            scope: Some(spans[0].instrumentation_scope.clone().into()),
            schema_url: resource.schema_url().map(Into::into).unwrap_or_default(),
            spans: spans.into_iter().map(|span_data| span_data.into()).collect(),
        })
        .collect();

    // Wrap ScopeSpans into a single ResourceSpans
    vec![ResourceSpans {
        resource: Some(Resource {
            attributes: resource.iter().into_iter().map(Into::into).collect(),
            dropped_attributes_count: 0,
            entity_refs: vec![],
        }),
        scope_spans,
        schema_url: resource.schema_url().map(Into::into).unwrap_or_default(),
    }]
}

impl From<wt::SpanData> for Span {
    fn from(span: wt::SpanData) -> Self {
        let trace_state = span.span_context.trace_state;
        let trace_state =
            trace_state.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join(",");

        Self {
            trace_id: hex::decode(span.span_context.trace_id).unwrap_or_default(),
            span_id: hex::decode(span.span_context.span_id).unwrap_or_default(),
            trace_state,
            parent_span_id: hex::decode(span.parent_span_id).unwrap_or_default(),
            flags: span.span_context.trace_flags.into(),
            name: span.name.into(),
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

impl From<&otel::SpanContext> for wt::SpanContext {
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

// impl From<wt::SpanContext> for otel::SpanContext {
//     fn from(ctx: wt::SpanContext) -> Self {
//         let trace_id = otel::TraceId::from_hex(&ctx.trace_id).unwrap_or(otel::TraceId::INVALID);
//         let span_id = otel::SpanId::from_hex(&ctx.span_id).unwrap_or(otel::SpanId::INVALID);
//         let trace_state = otel::TraceState::from_key_value(ctx.trace_state)
//             .unwrap_or_else(|_| otel::TraceState::default());
//         Self::new(trace_id, span_id, ctx.trace_flags.into(), ctx.is_remote, trace_state)
//     }
// }

impl From<otel::TraceFlags> for wt::TraceFlags {
    fn from(value: otel::TraceFlags) -> Self {
        if value.is_sampled() { Self::SAMPLED } else { Self::empty() }
    }
}

impl From<wt::TraceFlags> for u32 {
    fn from(value: wt::TraceFlags) -> Self {
        if value.contains(wt::TraceFlags::SAMPLED) {
            otel::TraceFlags::SAMPLED.to_u8() as Self
        } else {
            otel::TraceFlags::NOT_SAMPLED.to_u8() as Self
        }
    }
}

// impl From<wt::SpanKind> for i32 {
//     fn from(value: wt::SpanKind) -> Self {
//         match value {
//             wt::SpanKind::Client => Self::Client,
//             wt::SpanKind::Server => Self::Server,
//             wt::SpanKind::Producer => Self::Producer,
//             wt::SpanKind::Consumer => Self::Consumer,
//             wt::SpanKind::Internal => Self::Internal,
//         }
//     }
// }

impl From<wt::Event> for Event {
    fn from(event: wt::Event) -> Self {
        Self {
            time_unix_nano: event.time.into(),
            name: event.name,
            attributes: event.attributes.into_iter().map(Into::into).collect(),
            dropped_attributes_count: 0,
        }
    }
}

impl From<wt::Link> for Link {
    fn from(link: wt::Link) -> Self {
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

impl From<wt::Status> for Status {
    fn from(value: wt::Status) -> Self {
        match value {
            wt::Status::Unset => Self::default(),
            wt::Status::Error(description) => Self {
                code: StatusCode::Error.into(),
                message: description.into(),
            },
            wt::Status::Ok => Self {
                code: StatusCode::Ok.into(),
                message: String::new(),
            },
        }
    }
}

mod tests {
    #[test]
    fn from_hex() {
        let hex = "ff59dca6e9c68f0adddf8aba67294b16";
        let bytes = hex::decode(hex).unwrap();
        // assert_eq!(span_context.trace_id, hex);
        println!("Bytes: {:?}", bytes);
    }
}
