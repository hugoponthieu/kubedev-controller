use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::{ExporterBuildError, SpanExporter, WithExportConfig};
use opentelemetry_sdk::{
    Resource,
    trace::{SdkTracer, SdkTracerProvider},
};
use tracing_subscriber::{
    EnvFilter, fmt,
    layer::SubscriberExt,
    util::{SubscriberInitExt, TryInitError},
};

use crate::config::{LogFormat, TelemetryConfig};

#[derive(thiserror::Error, Debug)]
pub enum TelemetryError {
    #[error("Could not filter env: {0}")]
    Tracing(#[from] tracing_subscriber::filter::ParseError),
    #[error("Could not setup registry: {0}")]
    Registry(#[from] TryInitError),
    #[error("Could not build exporter: {0}")]
    BuildExporter(#[from] ExporterBuildError),
}

pub fn init(config: &TelemetryConfig) -> Result<(), TelemetryError> {
    let filter = EnvFilter::try_new(&config.log_filter)?;
    let tracer = init_tracer(config.opentelemetry_endpoint_url.clone())?;
    match config.log_format {
        LogFormat::Text => {
            tracing_subscriber::registry()
                .with(filter)
                .with(tracing_opentelemetry::layer().with_tracer(tracer))
                .with(fmt::layer())
                .try_init()?;
        }
        LogFormat::Json => {
            tracing_subscriber::registry()
                .with(filter)
                .with(
                    fmt::layer()
                        .json()
                        .with_ansi(false)
                        .with_current_span(true)
                        .with_span_list(true),
                )
                .with(tracing_opentelemetry::layer().with_tracer(tracer))
                .try_init()?;
        }
    }
    Ok(())
}

fn init_tracer(endpoint: String) -> Result<SdkTracer, TelemetryError> {
    let exporter = SpanExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
        .build()?;

    let resource = Resource::builder()
        .with_service_name("kubedev-controller")
        .build();

    Ok(SdkTracerProvider::builder()
        .with_resource(resource)
        .with_batch_exporter(exporter)
        .build()
        .tracer("tracing-otel-subscriber"))
}
