use k8s_openapi::{
    api::{
        batch::v1::{Job, JobSpec},
        core::v1::{Container, EnvVar, PodSpec, PodTemplateSpec},
    },
    apimachinery::pkg::apis::meta::v1::OwnerReference,
};
use kube::{
    Api, Client,
    api::{ObjectMeta, PostParams},
};

use crate::project::ProjectSpec;

pub struct ProjectPuller {
    client: Client,
}

const SCRIPT: &str = r#"
: "${REPOSITORY_URL:?REPOSITORY_URL is required}"

REVISION="${REVISION:-main}"
TARGET_DIR="${TARGET_DIR:-/workspace/repository}"

mkdir -p "$(dirname "$TARGET_DIR")"

if [ -d "$TARGET_DIR/.git" ]; then
  echo "Updating existing repository"
  git -C "$TARGET_DIR" remote set-url origin "$REPOSITORY_URL"
else
  echo "Initializing repository"
  rm -rf "$TARGET_DIR"
  mkdir -p "$TARGET_DIR"
  git -C "$TARGET_DIR" init
  git -C "$TARGET_DIR" remote add origin "$REPOSITORY_URL"
fi

git -C "$TARGET_DIR" fetch --depth=1 origin "$REVISION"
git -C "$TARGET_DIR" checkout --detach --force FETCH_HEAD
git -C "$TARGET_DIR" clean -ffd

COMMIT_SHA="$(git -C "$TARGET_DIR" rev-parse HEAD)"
printf '%s\n' "$COMMIT_SHA" > "$TARGET_DIR/.clone-complete.tmp"
mv "$TARGET_DIR/.clone-complete.tmp" "$TARGET_DIR/.clone-complete"

echo "Repository ready at commit $COMMIT_SHA"
"#;

#[derive(thiserror::Error, Debug)]
pub enum ProjectPullerError {
    #[error("Failed to create job: {0}")]
    JobCreation(kube::Error),
    #[error("Failure while checking a job: {0}")]
    Check(kube::Error),
}

impl ProjectPuller {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn pull(
        &self,
        project: ProjectSpec,
        project_name: &str,
        namespace: &str,
        owner_reference: OwnerReference,
    ) -> Result<(), ProjectPullerError> {
        let jobs = Api::<Job>::namespaced(self.client.clone(), namespace);
        jobs.create(
            &PostParams {
                dry_run: false,
                field_manager: None,
            },
            &Job {
                metadata: ObjectMeta {
                    name: Some(project_name.to_string()),
                    owner_references: Some(vec![owner_reference]),
                    ..Default::default()
                },
                spec: Some(JobSpec {
                    template: PodTemplateSpec {
                        spec: Some(PodSpec {
                            restart_policy: Some("Never".to_string()),
                            containers: vec![Container {
                                image: Some("alpine/git:latest".to_string()),
                                name: "puller".to_string(),
                                command: Some(vec!["/bin/sh".to_string()]),
                                args: Some(vec!["-ec".to_string(), SCRIPT.to_string()]),
                                env: Some(vec![
                                    EnvVar {
                                        name: "REPOSITORY_URL".to_string(),
                                        value: Some(project.project_url),
                                        ..Default::default()
                                    },
                                    EnvVar {
                                        name: "REVISION".to_string(),
                                        value: Some(project.revision),
                                        ..Default::default()
                                    },
                                    EnvVar {
                                        name: "TARGET_DIR".to_string(),
                                        value: Some("/workspace/repository".to_string()),
                                        ..Default::default()
                                    },
                                ]),
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
        .map_err(ProjectPullerError::JobCreation)?;
        Ok(())
    }

    pub async fn pulling_job_exists(
        &self,
        namespace: &str,
        project_name: &str,
    ) -> Result<bool, ProjectPullerError> {
        let jobs = Api::<Job>::namespaced(self.client.clone(), namespace);
        Ok(jobs
            .get_opt(project_name)
            .await
            .map_err(ProjectPullerError::Check)?
            .is_some())
    }
}
