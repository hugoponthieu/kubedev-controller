use kube::runtime::controller::Action;
use kube::{Resource, ResourceExt};
use std::sync::Arc;
use std::time::Duration;

use crate::project::{Project, ProjectSpec};
use crate::project_puller::ProjectPullerError;
use crate::state::ControllerState;

#[derive(thiserror::Error, Debug)]
pub enum ReconcileError {
    #[error("Failed to pull project: {0}")]
    PullProject(#[from] ProjectPullerError),
}

pub async fn reconcile(
    project: Arc<Project>,
    ctx: Arc<ControllerState>,
) -> Result<Action, ReconcileError> {
    let namespace = project
        .namespace()
        .expect("a namespaced Project must have a namespace");
    let project_name = project.name_any();
    let owner_reference = project
        .controller_owner_ref(&())
        .expect("Project must have a UID");
    if !ctx
        .project_puller
        .check_pulling_job(&namespace, &project_name)
        .await
        .map_err(ReconcileError::PullProject)?
    {
        ctx.project_puller
            .pull(
                ProjectSpec::default(),
                &project_name,
                &namespace,
                owner_reference,
            )
            .await
            .map_err(ReconcileError::PullProject)?;
    }
    println!("reconcile request: {}", project.name_any());
    Ok(Action::await_change())
}

pub fn error_policy(
    _object: Arc<Project>,
    err: &ReconcileError,
    _ctx: Arc<ControllerState>,
) -> Action {
    println!("reconcile failed: {}", err);
    Action::requeue(Duration::from_secs(5))
}
