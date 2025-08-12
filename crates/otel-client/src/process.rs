use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use anyhow::Result;
use opentelemetry::Context;
use opentelemetry_sdk::error::OTelSdkError;
use opentelemetry_sdk::trace::{Span, SpanData, SpanProcessor};

use crate::otel::wasi::otel::tracing;

#[derive(Debug)]
pub struct Processor {
    is_shutdown: AtomicBool,
}

impl Processor {
    /// Create a new `Processor`.
    pub fn new() -> Self {
        Self {
            is_shutdown: AtomicBool::new(false),
        }
    }
}

impl Default for Processor {
    fn default() -> Self {
        Self::new()
    }
}

impl SpanProcessor for Processor {
    fn on_start(&self, span: &mut Span, _: &Context) {
        if self.is_shutdown.load(Ordering::Relaxed) {
            return;
        }
        if let Some(span_data) = span.exported_data() {
            let span_context = tracing::SpanContext::from(span_data.clone().span_context);
            let span_data = tracing::SpanData::from(span_data);
            tracing::on_start(&span_data, &span_context);
        }
    }

    fn on_end(&self, span: SpanData) {
        if self.is_shutdown.load(Ordering::Relaxed) {
            return;
        }
        tracing::on_end(&span.into());
    }

    fn force_flush(&self) -> Result<(), OTelSdkError> {
        if self.is_shutdown.load(Ordering::Relaxed) {
            return Err(OTelSdkError::AlreadyShutdown);
        }
        Ok(())
    }

    fn shutdown(&self) -> Result<(), OTelSdkError> {
        let result = self.force_flush();
        if self.is_shutdown.swap(true, Ordering::Relaxed) {
            return Err(OTelSdkError::InternalFailure("Processor already shutdown".into()));
        }
        result
    }

    fn shutdown_with_timeout(&self, _: Duration) -> Result<(), OTelSdkError> {
        todo!()
    }
}
