use std::sync::Arc;

use futures::StreamExt;
use k8s_openapi::api::{
    apps::v1::Deployment,
    batch::v1::Job,
    core::v1::{PersistentVolumeClaim, Service},
};
use kube::{
    Api, Client,
    runtime::{Controller, controller::Error as ControllerRuntimeError},
};
use tracing::{error, warn};

use crate::{
    config::Config,
    project::Project,
    reconcile::{error_policy, reconcile},
    state::ControllerState,
};

#[derive(thiserror::Error, Debug)]
pub enum StartControllerError {
    #[error("Could not start the controller: {0}")]
    Kubernetes(#[from] kube::Error),
}

pub async fn start_controller(config: &Config) -> Result<(), StartControllerError> {
    let client = Client::try_default().await?;
    let project = Api::<Project>::all(client.clone());
    let jobs = Api::<Job>::all(client.clone());
    let pvcs = Api::<PersistentVolumeClaim>::all(client.clone());
    let deployments = Api::<Deployment>::all(client.clone());
    let services = Api::<Service>::all(client.clone());
    let controller_state = Arc::new(ControllerState::new(&client, config));

    Controller::new(project.clone(), Default::default())
        .owns(jobs, Default::default())
        .owns(pvcs, Default::default())
        .owns(deployments, Default::default())
        .owns(services, Default::default())
        .run(reconcile, error_policy, controller_state)
        .for_each(|result| async move {
            match result {
                Ok(_) | Err(ControllerRuntimeError::ReconcilerFailed(_, _)) => {}
                Err(ControllerRuntimeError::ObjectNotFound(object_ref)) => warn!(
                    event = "controller.object_not_found",
                    object_ref = %object_ref,
                    "Scheduled object was not found in the controller store"
                ),
                Err(err) => error!(
                    event = "controller.stream_error",
                    error = %err,
                    "Controller stream error"
                ),
            }
        })
        .await;
    Ok(())
}
