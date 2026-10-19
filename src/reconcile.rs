use kube::runtime::controller::Action;
use kube::{Resource, ResourceExt};
use std::sync::Arc;
use std::time::Duration;

use crate::project::Project;
use crate::project_puller::ProjectPullerError;
use crate::state::ControllerState;

#[derive(thiserror::Error, Debug)]
pub enum ReconcileError {
    #[error("Failed to pull project: {0}")]
    PullProject(#[from] ProjectPullerError),
    #[error("Failed to get namespace")]
    Namespace,
    #[error("No owner reference")]
    OwnerReference,
}

pub async fn reconcile(
    project: Arc<Project>,
    ctx: Arc<ControllerState>,
) -> Result<Action, ReconcileError> {
    let namespace = project.namespace().ok_or(ReconcileError::Namespace)?;
    let project_name = project.name_any();
    let owner_reference = project
        .controller_owner_ref(&())
        .ok_or(ReconcileError::OwnerReference)?;
    let _ = ctx
        .project_puller
        .pull(
            project.spec.clone(),
            &project_name,
            &namespace,
            owner_reference,
        )
        .await
        .map_err(ReconcileError::PullProject)?;
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
