//! # WASI Tracing

use std::time::SystemTime;

// use anyhow::Result;
use opentelemetry_otlp::MetricExporter;
use opentelemetry_sdk::error::OTelSdkError;
use opentelemetry_sdk::metrics as sdk;
use opentelemetry_sdk::metrics::exporter::PushMetricExporter;

use crate::Otel;
use crate::generated::wasi::otel::metrics::{self as wm};
use crate::generated::wasi::otel::types;

impl wm::Host for Otel<'_> {
    async fn collect(&mut self, rm: wm::ResourceMetrics) -> Result<(), types::Error> {
        let exporter =
            MetricExporter::builder().with_tonic().build().expect("should build exporter");
        exporter.export(&sdk::data::ResourceMetrics::from(rm)).await?;
        Ok(())
    }

    // async fn output_temporality(
    //     &mut self, kind: wm::InstrumentKind,
    // ) -> wasmtime::Result<wm::Temporality> {
    //     todo!()
    // }
}

impl types::Host for Otel<'_> {
    fn convert_error(&mut self, err: types::Error) -> anyhow::Result<types::Error> {
        tracing::error!("{err}");
        Ok(err)
    }
}

impl From<wm::ResourceMetrics> for sdk::data::ResourceMetrics {
    fn from(span: wm::ResourceMetrics) -> Self {
        todo!()
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
