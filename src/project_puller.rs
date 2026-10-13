use k8s_openapi::api::{
    batch::v1::{Job, JobSpec},
    core::v1::{Container, PodSpec, PodTemplateSpec},
};
use kube::{
    Api, Client,
    api::{ObjectMeta, PostParams},
};

use crate::project::ProjectSpec;

pub struct ProjectPuller {
    client: Client,
}

#[derive(thiserror::Error, Debug)]
pub enum ProjectPullerError {
    #[error("Failed to create job: {0}")]
    JobCreationError(#[from] kube::Error),
}

impl ProjectPuller {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn pull(
        &self,
        _project: ProjectSpec,
        project_name: &str,
        namespace: &str,
    ) -> Result<(), ProjectPullerError> {
        let jobs = Api::<Job>::namespaced(self.client.clone(), namespace);
        jobs.create(
            &PostParams {
                dry_run: false,
                field_manager: None,
            },
            &Job {
                metadata: ObjectMeta {
                    name: Some(format!("{}-puller", project_name)),
                    ..Default::default()
                },
                spec: Some(JobSpec {
                    template: PodTemplateSpec {
                        spec: Some(PodSpec {
                            restart_policy: Some("Never".to_string()),
                            containers: vec![Container {
                                image: Some("alpine/git:latest".to_string()),
                                name: "puller".to_string(),
                                command: Some(vec!["date".to_string()]),
                                ..Default::default()
                            }],
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                status: None,
            },
        )
        .await
        .map_err(ProjectPullerError::JobCreationError)?;
        Ok(())
    }
}
