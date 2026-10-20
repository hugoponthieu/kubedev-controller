use std::collections::BTreeMap;

use k8s_openapi::{
    api::core::v1::{PersistentVolumeClaim, PersistentVolumeClaimSpec, VolumeResourceRequirements},
    apimachinery::pkg::api::resource::Quantity,
};
use kube::api::ObjectMeta;

use crate::resources::context::BuildContext;

pub fn build_pvc(ctx: &BuildContext<'_>) -> PersistentVolumeClaim {
    PersistentVolumeClaim {
        metadata: ObjectMeta {
            name: Some(ctx.names.pvc.clone()),
            labels: Some(ctx.labels.to_map()),
            owner_references: Some(vec![ctx.owner_references.clone()]),
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
    }
}
