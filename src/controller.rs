use std::sync::Arc;

use futures::StreamExt;
use k8s_openapi::api::{
    apps::v1::Deployment,
    batch::v1::Job,
    core::v1::{PersistentVolumeClaim, Service},
};
use kube::{Api, Client, runtime::Controller};

use crate::{
    config::Config,
    project::Project,
    reconcile::{error_policy, reconcile},
    state::ControllerState,
};

#[derive(thiserror::Error, Debug)]
pub enum StartControllerError {
    #[error("Could not start the controller")]
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
        .for_each(|_| async move {})
        .await;
    Ok(())
}
