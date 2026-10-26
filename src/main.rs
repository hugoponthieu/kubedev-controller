use clap::Parser;
use kubedev_controller::{
    config::Config,
    controller::{StartControllerError, start_controller},
    health::HealthState,
    http::{HttpServerError, start_server},
    telemetry::{self, TelemetryError},
};
use tracing::error;

#[derive(thiserror::Error, Debug)]
pub enum ControllerError {
    #[error("{0}")]
    StartController(#[from] StartControllerError),
    #[error("{0}")]
    Telemetry(#[from] TelemetryError),
    #[error("{0}")]
    HttpServer(#[from] HttpServerError),
}

#[tokio::main]
async fn main() -> Result<(), ControllerError> {
    let config = &Config::parse();
    let health_state = HealthState::new();
    telemetry::init(&config.telemetry)?;
    tokio::select! {
        controller_res = start_controller(config,health_state.clone())=>{
            if let Err(err) = controller_res {
                error!(
                    event = "controller.failed",
                    error = %err,
                    "Controller stopped with an error"
                );
                return Err(err.into());
            }
        },
        server_res = start_server(&config.server, health_state)=>{
            if let Err(err) = server_res {
                error!(
                    event = "server.failed",
                    error = %err,
                    "Health server stopped with an error"
                );
                return Err(err.into());
            }
        }
    };

    Ok(())
}
