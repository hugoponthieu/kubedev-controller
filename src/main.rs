use clap::Parser;
use kubedev_controller::{
    config::Config,
    controller::{StartControllerError, start_controller},
    telemetry::{self, TelemetryError},
};
use tracing::error;

#[derive(thiserror::Error, Debug)]
pub enum ControllerError {
    #[error("{0}")]
    StartController(#[from] StartControllerError),
    #[error("{0}")]
    Telemetry(#[from] TelemetryError),
}

#[tokio::main]
async fn main() -> Result<(), ControllerError> {
    let config = Config::parse();
    telemetry::init(&config.telemetry)?;
    if let Err(err) = start_controller(&config).await {
        error!(
            event = "controller.failed",
            error = %err,
            "Controller stopped with an error"
        );
        return Err(err.into());
    }

    Ok(())
}
