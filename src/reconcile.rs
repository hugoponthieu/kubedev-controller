use kube::runtime::controller::Action;
use kube::{Resource, ResourceExt};
use std::sync::Arc;
use std::time::Duration;
use tracing::{Span, error, info, instrument};

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

impl ReconcileError {
    pub const fn reason(&self) -> &'static str {
        match self {
            Self::PullProject(_) => "project_pull_failed",
            Self::Namespace => "namespace_missing",
            Self::OwnerReference => "owner_reference_missing",
        }
    }
}

#[instrument(
    skip(ctx, project),
    fields(
        project_name = %project.name_any(),
        result = tracing::field::Empty,
        reason = tracing::field::Empty,
    )
)]
pub async fn reconcile(
    project: Arc<Project>,
    ctx: Arc<ControllerState>,
) -> Result<Action, ReconcileError> {
    let result = reconcile_project(&project, &ctx).await;
    let span = Span::current();

    match &result {
        Ok(_) => {
            span.record("result", "success");
            info!(
                event = "reconciliation.completed",
                "Reconciliation completed"
            );
        }
        Err(err) => {
            span.record("result", "failure");
            span.record("reason", err.reason());
            error!(
                event = "reconciliation.failed",
                reason = err.reason(),
                error = %err,
                "Failed to reconcile"
            );
        }
    }

    result
}

async fn reconcile_project(
    project: &Project,
    ctx: &ControllerState,
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
    Ok(Action::await_change())
}

pub fn error_policy(
    _project: Arc<Project>,
    err: &ReconcileError,
    _ctx: Arc<ControllerState>,
) -> Action {
    let requeue_after_seconds = 5_u64;
    info!(
        event = "reconciliation.retry_scheduled",
        reason = err.reason(),
        requeue_after_seconds,
        "Reconciliation retry scheduled"
    );
    Action::requeue(Duration::from_secs(requeue_after_seconds))
}
