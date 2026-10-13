use futures::StreamExt;
use k8s_openapi::api::batch::v1::Job;
use kube::{Api, Client, runtime::controller::Controller};
use kubedev_controller::{
    project,
    reconcile::{error_policy, reconcile},
    state::ControllerState,
};
use project::Project;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    let client = Client::try_default().await?;
    let project = Api::<Project>::all(client.clone());
    let jobs = Api::<Job>::all(client.clone());
    let controller_state = Arc::new(ControllerState::new(&client));

    Controller::new(project.clone(), Default::default())
        .owns(jobs, Default::default())
        .run(reconcile, error_policy, controller_state)
        .for_each(|_| futures::future::ready(()))
        .await;

    Ok(())
}
