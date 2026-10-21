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
}

pub fn init(config: &TelemetryConfig) -> Result<(), TelemetryError> {
    let filter = EnvFilter::try_new(&config.log_filter)?;

    match config.log_format {
        LogFormat::Text => {
            tracing_subscriber::registry()
                .with(filter)
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
                .try_init()?;
        }
    }
    Ok(())
}
