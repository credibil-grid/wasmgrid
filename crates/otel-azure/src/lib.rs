use std::collections::HashMap;
use std::time::Duration;

use opentelemetry::propagation::TextMapPropagator;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::trace::Tracer;
use tokio::process::Command;
use tokio::time::sleep;
use tracing::Span;
use tracing_attributes::instrument;
use tracing_opentelemetry::{OpenTelemetryLayer, OpenTelemetrySpanExt};
use tracing_subscriber::Registry;

pub  fn telemetry() -> anyhow::Result<OpenTelemetryLayer<Registry, Tracer>> {
    let tracer = opentelemetry_application_insights::new_pipeline_from_env()
        .expect("env var APPLICATIONINSIGHTS_CONNECTION_STRING should exist")
        .with_client(reqwest::Client::new())
        .install_batch(opentelemetry_sdk::runtime::Tokio);

    Ok(tracing_opentelemetry::layer().with_tracer(tracer))

    // let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    // let subscriber = Registry::default().with(telemetry)
    // tracing::subscriber::set_global_default(subscriber).expect("setting global default failed");

    // match traceparent {
    //     Some(traceparent) => {
    //         let mut extractor = HashMap::new();
    //         extractor.insert("traceparent".to_string(), traceparent);
    //         let propagator = TraceContextPropagator::new();
    //         let cx = propagator.extract(&extractor);
    //         let span = tracing::info_span!("child", otel.kind = "server");
    //         span.set_parent(cx);
    //         let _guard = span.enter();
    //         run_in_child_process().await;
    //     }
    //     _ => {
    //         let span = tracing::info_span!("root");
    //         let _guard = span.enter();
    //         spawn_children(5, process_name).await;
    //     }
    // }

    // opentelemetry::global::shutdown_tracer_provider();

    // Ok(())
}

#[instrument]
async fn spawn_children(n: u32, process_name: String) {
    for _ in 0..n {
        spawn_child_process(&process_name).await;
    }
}

#[instrument(fields(otel.kind = "client"))]
async fn spawn_child_process(process_name: &str) {
    let mut injector = HashMap::new();
    let propagator = TraceContextPropagator::new();
    propagator.inject_context(&Span::current().context(), &mut injector);
    let mut child = Command::new(process_name)
        .arg(injector.remove("traceparent").expect("propagator should inject traceparent"))
        .spawn()
        .expect("failed to spawn");
    child.wait().await.expect("awaiting process failed");
}

#[instrument]
async fn run_in_child_process() {
    tracing::info!("leaf fn");
    sleep(Duration::from_millis(50)).await
}
