use k8s_openapi::{
    api::{
        apps::v1::{Deployment, DeploymentSpec},
        core::v1::{
            Container, ContainerPort, EnvVar, PersistentVolumeClaimVolumeSource, PodSpec,
            PodTemplateSpec, Volume, VolumeMount,
        },
    },
    apimachinery::pkg::apis::meta::v1::LabelSelector,
};
use kube::api::ObjectMeta;

use crate::resources::context::BuildContext;

pub fn build_deployment(ctx: BuildContext<'_>) -> Deployment {
    let labels = ctx.labels.to_map();

    let workspace_mount = VolumeMount {
        name: "workspace".to_string(),
        mount_path: "/workspace".to_string(),
        read_only: Some(false),
        ..Default::default()
    };

    Deployment {
        metadata: ObjectMeta {
            name: Some(ctx.names.deployment.clone()),
            labels: Some(labels.clone()),
            owner_references: Some(vec![ctx.owner_references.clone()]),
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
                        image: Some(ctx.config.git_image()),
                        command: Some(vec!["/bin/sh".to_string()]),
                        args: Some(vec!["-ec".to_string(), WAIT_FOR_PROJECT_SCRIPT.to_string()]),
                        volume_mounts: Some(vec![workspace_mount.clone()]),
                        ..Default::default()
                    }]),
                    containers: vec![Container {
                        name: "workspace".to_string(),
                        image: Some(ctx.config.workspace_image()),
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
                            claim_name: ctx.names.pvc.clone(),
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
    }
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
