use k8s_openapi::{
    api::{
        apps::v1::{Deployment, DeploymentSpec},
        batch::v1::{Job, JobSpec},
        core::v1::{
            Container, ContainerPort, EnvVar, PersistentVolumeClaim, PersistentVolumeClaimSpec,
            PersistentVolumeClaimVolumeSource, PodSpec, PodTemplateSpec, Service, ServicePort,
            ServiceSpec, Volume, VolumeMount, VolumeResourceRequirements,
        },
    },
    apimachinery::pkg::api::resource::Quantity,
    apimachinery::pkg::apis::meta::v1::{LabelSelector, OwnerReference},
    apimachinery::pkg::util::intstr::IntOrString,
};
use kube::{
    Api, Client,
    api::{ObjectMeta, Patch, PatchParams, PostParams},
};
use std::collections::BTreeMap;

use crate::{config::Config, project::ProjectSpec};

pub struct ProjectPuller {
    client: Client,
    config: Config,
}

const WAIT_FOR_PROJECT_SCRIPT: &str = r#"
for attempt in $(seq 1 150); do
  if [ -s /workspace/repository/.clone-complete ]; then
    chown -R 1000:1000 /workspace/repository
    echo "Project is ready"
    exit 0
  fi

  echo "Waiting for the project clone to complete (${attempt}/150)"
  sleep 2
done

echo "Timed out waiting for /workspace/repository/.clone-complete" >&2
exit 1
"#;

#[derive(thiserror::Error, Debug)]
pub enum ProjectPullerError {
    #[error("Kubernetes API request failed: {0}")]
    Kubernetes(#[from] kube::Error),
}

impl ProjectPuller {
    pub fn new(client: Client, config: Config) -> Self {
        Self { client, config }
    }

    pub async fn pull(
        &self,
        project: ProjectSpec,
        project_name: &str,
        namespace: &str,
        owner_reference: OwnerReference,
    ) -> Result<(), ProjectPullerError> {
        let pvc_name = format!("{}-workspace", project_name);

        self.ensure_pvc(namespace, &pvc_name, owner_reference.clone())
            .await?;
        self.ensure_job(
            project,
            project_name,
            namespace,
            &pvc_name,
            owner_reference.clone(),
        )
        .await?;
        self.ensure_workspace(project_name, namespace, &pvc_name, owner_reference.clone())
            .await?;
        self.ensure_workspace_service(project_name, namespace, owner_reference)
            .await
    }

    async fn ensure_pvc(
        &self,
        namespace: &str,
        pvc_name: &str,
        owner_reference: OwnerReference,
    ) -> Result<(), ProjectPullerError> {
        let pvcs = Api::<PersistentVolumeClaim>::namespaced(self.client.clone(), namespace);

        if pvcs.get_opt(pvc_name).await?.is_some() {
            return Ok(());
        }

        let pvc = PersistentVolumeClaim {
            metadata: ObjectMeta {
                name: Some(pvc_name.to_string()),
                owner_references: Some(vec![owner_reference]),
                ..Default::default()
            },
            spec: Some(PersistentVolumeClaimSpec {
                access_modes: Some(vec!["ReadWriteMany".to_string()]),
                resources: Some(VolumeResourceRequirements {
                    requests: Some(BTreeMap::from([(
                        "storage".to_string(),
                        Quantity("1Gi".to_string()),
                    )])),
                    ..Default::default()
                }),
                // The cluster's default StorageClass is used. Its reclaim policy
                // must be Delete for the backing volume to be removed with the PVC.
                storage_class_name: None,
                ..Default::default()
            }),
            status: None,
        };

        match pvcs.create(&PostParams::default(), &pvc).await {
            Ok(_) => Ok(()),
            Err(kube::Error::Api(response)) if response.code == 409 => Ok(()),
            Err(error) => Err(error.into()),
        }
    }

    async fn ensure_job(
        &self,
        project: ProjectSpec,
        project_name: &str,
        namespace: &str,
        pvc_name: &str,
        owner_reference: OwnerReference,
    ) -> Result<(), ProjectPullerError> {
        let jobs = Api::<Job>::namespaced(self.client.clone(), namespace);

        if jobs.get_opt(project_name).await?.is_some() {
            return Ok(());
        }

        let job = Job {
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
                            image: Some(self.config.project_puller_image()),
                            image_pull_policy: Some("IfNotPresent".to_string()),
                            name: "puller".to_string(),
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
                            volume_mounts: Some(vec![VolumeMount {
                                name: "workspace".to_string(),
                                mount_path: "/workspace".to_string(),
                                read_only: Some(false),
                                ..Default::default()
                            }]),
                            ..Default::default()
                        }],
                        volumes: Some(vec![Volume {
                            name: "workspace".to_string(),
                            persistent_volume_claim: Some(PersistentVolumeClaimVolumeSource {
                                claim_name: pvc_name.to_string(),
                                read_only: Some(false),
                            }),
                            ..Default::default()
                        }]),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }),
            status: None,
        };

        match jobs.create(&PostParams::default(), &job).await {
            Ok(_) => Ok(()),
            Err(kube::Error::Api(response)) if response.code == 409 => Ok(()),
            Err(error) => Err(error.into()),
        }
    }

    async fn ensure_workspace(
        &self,
        project_name: &str,
        namespace: &str,
        pvc_name: &str,
        owner_reference: OwnerReference,
    ) -> Result<(), ProjectPullerError> {
        let deployments = Api::<Deployment>::namespaced(self.client.clone(), namespace);
        let workspace_name = format!("{}-workspace", project_name);

        let labels = BTreeMap::from([
            (
                "app.kubernetes.io/name".to_string(),
                "kubedev-workspace".to_string(),
            ),
            (
                "kubedev-controller/project".to_string(),
                project_name.to_string(),
            ),
        ]);
        let workspace_mount = VolumeMount {
            name: "workspace".to_string(),
            mount_path: "/workspace".to_string(),
            read_only: Some(false),
            ..Default::default()
        };

        let deployment = Deployment {
            metadata: ObjectMeta {
                name: Some(workspace_name.clone()),
                owner_references: Some(vec![owner_reference]),
                ..Default::default()
            },
            spec: Some(DeploymentSpec {
                replicas: Some(1),
                selector: LabelSelector {
                    match_labels: Some(labels.clone()),
                    ..Default::default()
                },
                template: PodTemplateSpec {
                    metadata: Some(ObjectMeta {
                        labels: Some(labels),
                        ..Default::default()
                    }),
                    spec: Some(PodSpec {
                        init_containers: Some(vec![Container {
                            name: "wait-for-project".to_string(),
                            image: Some(self.config.git_image()),
                            command: Some(vec!["/bin/sh".to_string()]),
                            args: Some(vec![
                                "-ec".to_string(),
                                WAIT_FOR_PROJECT_SCRIPT.to_string(),
                            ]),
                            volume_mounts: Some(vec![workspace_mount.clone()]),
                            ..Default::default()
                        }]),
                        containers: vec![Container {
                            name: "workspace".to_string(),
                            image: Some(self.config.workspace_image()),
                            image_pull_policy: Some("IfNotPresent".to_string()),
                            env: Some(vec![EnvVar {
                                name: "BUILDKIT_HOST".to_string(),
                                value: Some(
                                    "tcp://buildkitd.buildkit.svc.cluster.local:1234".to_string(),
                                ),
                                ..Default::default()
                            }]),
                            ports: Some(vec![ContainerPort {
                                container_port: 22,
                                name: Some("ssh".to_string()),
                                protocol: Some("TCP".to_string()),
                                ..Default::default()
                            }]),
                            volume_mounts: Some(vec![workspace_mount]),
                            ..Default::default()
                        }],
                        restart_policy: Some("Always".to_string()),
                        volumes: Some(vec![Volume {
                            name: "workspace".to_string(),
                            persistent_volume_claim: Some(PersistentVolumeClaimVolumeSource {
                                claim_name: pvc_name.to_string(),
                                read_only: Some(false),
                            }),
                            ..Default::default()
                        }]),
                        ..Default::default()
                    }),
                },
                ..Default::default()
            }),
            status: None,
        };

        deployments
            .patch(
                &workspace_name,
                &PatchParams::apply("kubedev-controller").force(),
                &Patch::Apply(&deployment),
            )
            .await?;

        Ok(())
    }

    async fn ensure_workspace_service(
        &self,
        project_name: &str,
        namespace: &str,
        owner_reference: OwnerReference,
    ) -> Result<(), ProjectPullerError> {
        let services = Api::<Service>::namespaced(self.client.clone(), namespace);
        let workspace_name = format!("{}-workspace", project_name);

        let service = Service {
            metadata: ObjectMeta {
                name: Some(workspace_name.clone()),
                owner_references: Some(vec![owner_reference]),
                annotations: Some(BTreeMap::from([
                    ("tailscale.com/hostname".to_string(), workspace_name.clone()),
                    ("tailscale.com/tags".to_string(), "tag:k8s".to_string()),
                ])),
                ..Default::default()
            },
            spec: Some(ServiceSpec {
                type_: Some("LoadBalancer".to_string()),
                load_balancer_class: Some("tailscale".to_string()),
                selector: Some(BTreeMap::from([(
                    "kubedev-controller/project".to_string(),
                    project_name.to_string(),
                )])),
                ports: Some(vec![ServicePort {
                    name: Some("ssh".to_string()),
                    port: 22,
                    protocol: Some("TCP".to_string()),
                    target_port: Some(IntOrString::String("ssh".to_string())),
                    ..Default::default()
                }]),
                ..Default::default()
            }),
            status: None,
        };

        services
            .patch(
                &workspace_name,
                &PatchParams::apply("kubedev-controller").force(),
                &Patch::Apply(&service),
            )
            .await?;

        Ok(())
    }
}
