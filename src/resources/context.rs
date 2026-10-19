use std::collections::BTreeMap;

use k8s_openapi::apimachinery::pkg::apis::meta::v1::OwnerReference;

use crate::{config::Config, project::ProjectSpec};

pub struct BuildContext<'a> {
    pub project_name: &'a str,
    pub project_spec: &'a ProjectSpec,
    pub owner_references: &'a OwnerReference,
    pub config: &'a Config,
    pub names: ResourceNames,
    pub labels: ResourceLabels,
}

impl<'a> BuildContext<'a> {
    pub fn new(
        project_name: &'a str,
        project_spec: &'a ProjectSpec,
        owner_references: &'a OwnerReference,
        config: &'a Config,
    ) -> Self {
        Self {
            project_name,
            project_spec,
            owner_references,
            config,
            names: ResourceNames::for_project(project_name),
            labels: ResourceLabels::for_project(project_name),
        }
    }
}

#[derive(Clone)]
pub struct ResourceLabels(BTreeMap<String, String>);

impl ResourceLabels {
    pub fn for_project(project_name: &str) -> Self {
        Self(BTreeMap::from([
            (
                "app.kubernetes.io/instance".to_string(),
                project_name.to_string(),
            ),
            (
                "app.kubernetes.io/managed-by".to_string(),
                "kubedev-controller".to_string(),
            ),
            ("kube.dev/project".to_string(), project_name.to_string()),
        ]))
    }

    pub fn to_map(&self) -> BTreeMap<String, String> {
        self.0.clone()
    }
}

pub struct ResourceNames {
    pub pvc: String,
    pub pull_job: String,
    pub deployment: String,
}

impl ResourceNames {
    pub fn for_project(project_name: &str) -> Self {
        Self {
            pvc: format!("{project_name}-workspace"),
            pull_job: project_name.to_string(),
            deployment: format!("{project_name}-workspace"),
        }
    }
}
