use kube::runtime::controller::Action;
use kube::{Resource, ResourceExt};
use std::sync::Arc;
use std::time::Duration;
use tracing::{info, instrument};

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

#[instrument(
    skip(ctx, project),
    fields(project_name = %project.name_any())
)]
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
            &owner_reference,
        )
        .await
        .map_err(ReconcileError::PullProject)?;

    info!(event = "reconciliation.completed",
        project_name = %project_name, 
        "Reconciliation completed");
    Ok(Action::await_change())
}

pub fn error_policy(
    _project: Arc<Project>,
    err: &ReconcileError,
    _ctx: Arc<ControllerState>,
) -> Action {
    let requeue_after_seconds = 5_u64;
    info!(event = "reconciliation.failed",error = %err,requeue_after_seconds = requeue_after_seconds, "Failed to reconcile; retry scheduled");
    Action::requeue(Duration::from_secs(requeue_after_seconds))
}
