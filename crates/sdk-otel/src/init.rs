//! # Initializer
//!
//! Initialize the OpenTelemetry collectors and exporters.

use std::{env, mem};

use anyhow::{Context as _, Result};
use async_trait::async_trait;
use bytes::Bytes;
use http::{Request, Response};
use opentelemetry::trace::TracerProvider;
use opentelemetry::{Context, ContextGuard, KeyValue, global};
use opentelemetry_http::{HttpClient, HttpError};
use opentelemetry_otlp::{MetricExporter, SpanExporter, WithExportConfig, WithHttpConfig};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_sdk::trace::SdkTracerProvider;
use sdk_http::Client;
use serde::Deserialize;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

use crate::propagate::{ContextPropagator, Processor, Propagator};

/// Telemetry initializer.
#[derive(Debug, Default)]
pub struct Otel {
    /// The name of the application to for the purposes of identifying the
    /// service in telemetry data.
    app_name: String,

    /// The name of the environment, e.g. "production", "staging", "development".
    env_name: Option<String>,

    /// The OpenTelemetry metrics collection endpoint.
    endpoint: Option<String>,
}

impl Otel {
    /// Create a new telemetry initializer.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            app_name: name.into(),
            env_name: None,
            endpoint: None,
        }
    }

    /// Override the default app name.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.app_name = name.into();
        self
    }

    /// Set the environment name.
    #[must_use]
    pub fn env(mut self, env_name: impl Into<String>) -> Self {
        self.env_name = Some(env_name.into());
        self
    }

    /// Set the OpenTelemetry endpoint.
    #[must_use]
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }

    /// Initialize telemetry with the provided configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the telemetry system fails to initialize, such as if
    /// the OpenTelemetry exporter cannot be created or if setting the global
    /// subscriber fails.
    pub fn init(self) -> Result<()> {
        let config = Config {
            app_name: self.app_name,
            env_name: self.env_name,
            endpoint: self.endpoint,
        };

        let resource = Resource::from(config.clone());

        // metrics
        let meter_provider = init_metrics(&config, resource.clone())?;
        global::set_meter_provider(meter_provider);

        // tracer
        let tracer_provider = init_traces(&config, resource)?;
        global::set_tracer_provider(tracer_provider.clone());

        // tracing
        let env_filter = EnvFilter::from_default_env();
        let fmt_layer =
            tracing_subscriber::fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE);
        let tracer = tracer_provider.tracer(config.app_name.clone());
        let tracing_layer = tracing_opentelemetry::layer().with_tracer(tracer);

        let subscriber = Registry::default().with(env_filter).with(fmt_layer).with(tracing_layer);
        tracing::subscriber::set_global_default(subscriber)?;

        Ok(())
    }

    #[must_use]
    pub fn with_host_context(self) -> Result<ContextGuard> {
        self.init().context("initializing telemetry")?;
        let context = ContextPropagator::new().extract(&Context::current()).attach();
        Ok(context)
    }
}

fn init_traces(config: &Config, resource: Resource) -> Result<SdkTracerProvider> {
    let mut builder = SpanExporter::builder().with_http().with_http_client(OtelClient);
    if let Some(endpoint) = &config.endpoint {
        builder = builder.with_endpoint(endpoint);
    }
    let exporter = builder.build()?;
    let processor = Processor::new();

    Ok(SdkTracerProvider::builder()
        .with_resource(resource)
        .with_span_processor(processor)
        .with_simple_exporter(exporter)
        .build())
}

fn init_metrics(config: &Config, resource: Resource) -> Result<SdkMeterProvider> {
    let mut builder = MetricExporter::builder().with_http().with_http_client(OtelClient);
    if let Some(endpoint) = &config.endpoint {
        builder = builder.with_endpoint(endpoint);
    }
    let exporter = builder.build()?;
    Ok(SdkMeterProvider::builder().with_resource(resource).with_periodic_exporter(exporter).build())
}

/// Telemetry configuration.
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    /// The name of the application to for the purposes of identifying the
    /// service in telemetry data.
    pub app_name: String,

    /// The name of the environment, e.g. "production", "staging", "development".
    pub env_name: Option<String>,

    /// The OpenTelemetry metrics collection endpoint.
    pub endpoint: Option<String>,
}

impl From<Config> for Resource {
    fn from(config: Config) -> Self {
        Self::builder()
            .with_service_name(config.app_name.clone())
            .with_attributes(vec![
                KeyValue::new(
                    "deployment.environment",
                    config.env_name.clone().unwrap_or_else(|| "unknown".to_string()),
                ),
                KeyValue::new("service.namespace", config.app_name),
                KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
                KeyValue::new(
                    "service.instance.id",
                    env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string()),
                ),
                KeyValue::new("telemetry.sdk.name", "opentelemetry"),
                KeyValue::new("instrumentation.provider", "opentelemetry"),
            ])
            .build()
    }
}

#[derive(Debug)]
struct OtelClient;

#[async_trait]
impl HttpClient for OtelClient {
    async fn send_bytes(&self, request: Request<Bytes>) -> Result<Response<Bytes>, HttpError> {
        let mut response = Client::new()
            .post(request.uri())
            .headers(request.headers())
            .body(request.into_body().to_vec())
            .send::<Vec<u8>>()?;

        let headers = mem::take(response.headers_mut());
        let mut http_response =
            Response::builder().status(response.status()).body(response.body().clone().into())?;
        *http_response.headers_mut() = headers;

        Ok(http_response)
    }
}
