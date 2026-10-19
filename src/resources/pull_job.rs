use k8s_openapi::api::{
    batch::v1::{Job, JobSpec},
    core::v1::{
        Container, EnvVar, PersistentVolumeClaimVolumeSource, PodSpec, PodTemplateSpec, Volume,
        VolumeMount,
    },
};
use kube::api::ObjectMeta;

use crate::resources::context::BuildContext;

pub fn build_pull_job(ctx: BuildContext<'_>) -> Job {
    Job {
        metadata: ObjectMeta {
            name: Some(ctx.project_name.to_string()),
            labels: Some(ctx.labels.to_map()),
            owner_references: Some(vec![ctx.owner_references.clone()]),
            ..Default::default()
        },
        spec: Some(JobSpec {
            template: PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    labels: Some(ctx.labels.to_map()),
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    restart_policy: Some("Never".to_string()),
                    containers: vec![Container {
                        image: Some(ctx.config.project_puller_image()),
                        image_pull_policy: Some("IfNotPresent".to_string()),
                        name: "puller".to_string(),
                        env: Some(vec![
                            EnvVar {
                                name: "REPOSITORY_URL".to_string(),
                                value: Some(ctx.project_spec.project_url.clone()),
                                ..Default::default()
                            },
                            EnvVar {
                                name: "REVISION".to_string(),
                                value: Some(ctx.project_spec.revision.clone()),
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
                            claim_name: ctx.names.pvc,
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
    }
}
