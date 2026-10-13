use k8s_openapi::serde::{Deserialize, Serialize};
use kube::CustomResource;
use schemars::JsonSchema;

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema, Default)]
#[kube(kind = "Project", group = "kube.rs", version = "v1", namespaced)]
pub struct ProjectSpec {
    pub project_url: String,
    pub revision: String,
}
