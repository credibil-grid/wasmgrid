//! # WASI Tracing



use anyhow::Result;
use opentelemetry::trace::{
    TraceContextExt, {self as otel},
};
use opentelemetry_sdk::trace as sdk;
use opentelemetry_sdk::trace::SpanExporter as _;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::Otel;
use crate::generated::wasi::otel as wasi_otel;
use crate::generated::wasi::otel::tracing::{self as wt};

impl wasi_otel::tracing::Host for Otel<'_> {
    async fn on_start(&mut self, _: wt::SpanData, _parent: wt::SpanContext) -> Result<()> {
        Ok(())
    }

    async fn on_end(&mut self, span_data: wt::SpanData) -> Result<()> {
        self.exporter.export(vec![sdk::SpanData::from(span_data)]).await?;
        Ok(())
    }

    async fn current_span_context(&mut self) -> Result<wt::SpanContext> {
        let ctx = Span::current().context();
        let span = ctx.span();
        Ok(wt::SpanContext::from(span.span_context()))
    }
}

impl From<wt::SpanData> for sdk::SpanData {
    fn from(span: wt::SpanData) -> Self {
        let parent_span_id =
            otel::SpanId::from_hex(&span.parent_span_id).unwrap_or(otel::SpanId::INVALID);

        let mut events = sdk::SpanEvents::default();
        events.events = span.events.into_iter().map(Into::into).collect();

        let mut links = sdk::SpanLinks::default();
        links.links = span.links.into_iter().map(Into::into).collect();

        Self {
            span_context: span.span_context.into(),
            parent_span_id,
            span_kind: span.span_kind.into(),
            name: span.name.into(),
            start_time: span.start_time.into(),
            end_time: span.end_time.into(),
            attributes: span.attributes.into_iter().map(Into::into).collect(),
            dropped_attributes_count: span.dropped_attributes,
            events,
            links,
            status: span.status.into(),
            instrumentation_scope: span.instrumentation_scope.into(),
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

impl From<wt::SpanContext> for otel::SpanContext {
    fn from(ctx: wt::SpanContext) -> Self {
        let trace_id = otel::TraceId::from_hex(&ctx.trace_id).unwrap_or(otel::TraceId::INVALID);
        let span_id = otel::SpanId::from_hex(&ctx.span_id).unwrap_or(otel::SpanId::INVALID);
        let trace_state = otel::TraceState::from_key_value(ctx.trace_state)
            .unwrap_or_else(|_| otel::TraceState::default());
        Self::new(trace_id, span_id, ctx.trace_flags.into(), ctx.is_remote, trace_state)
    }
}

impl From<otel::TraceFlags> for wt::TraceFlags {
    fn from(value: otel::TraceFlags) -> Self {
        if value.is_sampled() { Self::SAMPLED } else { Self::empty() }
    }
}

impl From<wt::TraceFlags> for otel::TraceFlags {
    fn from(value: wt::TraceFlags) -> Self {
        if value.contains(wt::TraceFlags::SAMPLED) { Self::SAMPLED } else { Self::NOT_SAMPLED }
    }
}

impl From<wt::SpanKind> for otel::SpanKind {
    fn from(value: wt::SpanKind) -> Self {
        match value {
            wt::SpanKind::Client => Self::Client,
            wt::SpanKind::Server => Self::Server,
            wt::SpanKind::Producer => Self::Producer,
            wt::SpanKind::Consumer => Self::Consumer,
            wt::SpanKind::Internal => Self::Internal,
        }
    }
}



impl From<wt::Event> for otel::Event {
    fn from(value: wt::Event) -> Self {
        let attrs = value.attributes.into_iter().map(Into::into).collect();
        Self::new(value.name, value.time.into(), attrs, 0)
    }
}

impl From<wt::Link> for otel::Link {
    fn from(value: wt::Link) -> Self {
        let attrs = value.attributes.into_iter().map(Into::into).collect();
        Self::new(value.span_context.into(), attrs, 0)
    }
}

impl From<wt::Status> for otel::Status {
    fn from(value: wt::Status) -> Self {
        match value {
            wt::Status::Unset => Self::Unset,
            wt::Status::Error(description) => Self::Error {
                description: description.into(),
            },
            wt::Status::Ok => Self::Ok,
        }
    }
}
