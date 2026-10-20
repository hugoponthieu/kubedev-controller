use std::collections::BTreeMap;

use k8s_openapi::{
    api::core::v1::{Service, ServicePort, ServiceSpec},
    apimachinery::pkg::util::intstr::IntOrString,
};
use kube::api::ObjectMeta;

use crate::resources::context::BuildContext;

pub fn build_service(ctx: &BuildContext<'_>) -> Service {
    Service {
        metadata: ObjectMeta {
            name: Some(ctx.names.service.clone()),
            owner_references: Some(vec![ctx.owner_references.clone()]),
            annotations: Some(BTreeMap::from([
                (
                    "tailscale.com/hostname".to_string(),
                    ctx.names.service.clone(),
                ),
                ("tailscale.com/tags".to_string(), "tag:k8s".to_string()),
            ])),
            ..Default::default()
        },
        spec: Some(ServiceSpec {
            type_: Some("LoadBalancer".to_string()),
            load_balancer_class: Some("tailscale".to_string()),
            selector: Some(BTreeMap::from([(
                "kubedev-controller/project".to_string(),
                ctx.project_name.to_string(),
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
    }
}
