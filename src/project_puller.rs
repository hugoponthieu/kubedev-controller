use k8s_openapi::{
    api::{
        apps::v1::Deployment,
        batch::v1::{Job, JobStatus},
        core::v1::{PersistentVolumeClaim, Service},
    },
    apimachinery::pkg::apis::meta::v1::OwnerReference,
};
use kube::{
    Api, Client,
    api::{Patch, PatchParams, PostParams},
};

use crate::{
    project::{Project, ProjectSpec, ProjectStatus, PullState},
    resources::{
        context::{BuildContext, ContainerImages},
        deployment::build_deployment,
        pull_job::build_pull_job,
        pvc::build_pvc,
        service::build_service,
    },
};

pub struct ProjectPuller {
    client: Client,
    images: ContainerImages,
}

#[derive(thiserror::Error, Debug)]
pub enum ProjectPullerError {
    #[error("Kubernetes API request failed: {0}")]
    Kubernetes(#[from] kube::Error),
}

impl ProjectPuller {
    pub fn new(client: Client, images: impl Into<ContainerImages>) -> Self {
        Self {
            client,
            images: images.into(),
        }
    }

    #[tracing::instrument(
        name = "kubernetes.set_status",
        level = "info",
        skip_all,
        fields(resource_kind = "Project", operation = "patch_status")
    )]
    pub async fn set_project_status(
        &self,
        project_name: &str,
        namespace: &str,
        project_status: &ProjectStatus,
    ) -> Result<Project, ProjectPullerError> {
        let projects = Api::<Project>::namespaced(self.client.clone(), namespace);

        let patch = serde_json::json!({
            "status": project_status
        });
        let project = projects
            .patch_status(project_name, &PatchParams::default(), &Patch::Merge(patch))
            .await?;
        Ok(project)
    }

    pub async fn pull(
        &self,
        project: ProjectSpec,
        project_name: &str,
        namespace: &str,
        owner_reference: &OwnerReference,
    ) -> Result<ProjectStatus, ProjectPullerError> {
        let context = &BuildContext::new(project_name, &project, owner_reference, &self.images);

        let pvc = build_pvc(context);
        let job = build_pull_job(context);
        let deployment = build_deployment(context);
        let service = build_service(context);

        self.ensure_pvc(namespace, &context.names.pvc, &pvc).await?;
        let pull_state = self
            .ensure_job(namespace, &context.names.pull_job, &job)
            .await?;
        let project_status = ProjectStatus { pull_state };
        self.set_project_status(project_name, namespace, &project_status)
            .await?;
        self.ensure_workspace(namespace, &context.names.deployment, &deployment)
            .await?;
        self.ensure_workspace_service(namespace, &context.names.service, &service)
            .await?;
        Ok(project_status)
    }

    #[tracing::instrument(
        name = "kubernetes.apply",
        level = "info",
        skip_all,
        fields(resource_kind = "PersistentVolumeClaim", operation = "apply")
    )]
    async fn ensure_pvc(
        &self,
        namespace: &str,
        name: &str,
        desired: &PersistentVolumeClaim,
    ) -> Result<(), ProjectPullerError> {
        let pvcs = Api::<PersistentVolumeClaim>::namespaced(self.client.clone(), namespace);

        if pvcs.get_opt(name).await?.is_some() {
            return Ok(());
        }

        match pvcs.create(&PostParams::default(), desired).await {
            Ok(_) => Ok(()),
            Err(kube::Error::Api(response)) if response.code == 409 => Ok(()),
            Err(error) => Err(error.into()),
        }
    }

    fn get_job_state_from_status(job_status: JobStatus) -> PullState {
        if let Some(n_failed) = job_status.succeeded
            && n_failed > 0
        {
            PullState::Pulled
        } else if let Some(n_active) = job_status.active
            && n_active > 0
        {
            PullState::Pulling
        } else if let Some(n_failed) = job_status.failed
            && n_failed > 0
        {
            PullState::Failed
        } else {
            PullState::Unknown
        }
    }

    #[tracing::instrument(
        name = "kubernetes.apply",
        level = "info",
        skip_all,
        fields(resource_kind = "Job", operation = "apply")
    )]
    async fn ensure_job(
        &self,
        namespace: &str,
        name: &str,
        desired: &Job,
    ) -> Result<PullState, ProjectPullerError> {
        let jobs = Api::<Job>::namespaced(self.client.clone(), namespace);

        if let Some(job) = jobs.get_opt(name).await?
            && let Some(job_status) = job.status
        {
            return Ok(Self::get_job_state_from_status(job_status));
        }

        match jobs.create(&PostParams::default(), desired).await {
            Ok(_) => Ok(PullState::Pulling),
            Err(kube::Error::Api(response)) if response.code == 409 => Ok(PullState::Failed),
            Err(error) => Err(error.into()),
        }
    }

    #[tracing::instrument(
        name = "kubernetes.apply",
        level = "info",
        skip_all,
        fields(resource_kind = "Deployment", operation = "apply")
    )]
    async fn ensure_workspace(
        &self,
        namespace: &str,
        name: &str,
        desired: &Deployment,
    ) -> Result<(), ProjectPullerError> {
        let deployments = Api::<Deployment>::namespaced(self.client.clone(), namespace);

        deployments
            .patch(
                name,
                &PatchParams::apply("kubedev-controller").force(),
                &Patch::Apply(desired),
            )
            .await?;

        Ok(())
    }

    #[tracing::instrument(
        name = "kubernetes.apply",
        level = "info",
        skip_all,
        fields(resource_kind = "Service", operation = "apply")
    )]
    async fn ensure_workspace_service(
        &self,
        namespace: &str,
        name: &str,
        desired: &Service,
    ) -> Result<(), ProjectPullerError> {
        let services = Api::<Service>::namespaced(self.client.clone(), namespace);

        services
            .patch(
                name,
                &PatchParams::apply("kubedev-controller").force(),
                &Patch::Apply(desired),
            )
            .await?;

        Ok(())
    }
}
