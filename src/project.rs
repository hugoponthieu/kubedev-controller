use k8s_openapi::serde::{Deserialize, Serialize};
use kube::CustomResource;
use schemars::JsonSchema;

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema, Default)]
#[kube(kind = "Project", group = "kube.dev", version = "v1", namespaced, status = ProjectStatus,
      printcolumn(
          name = "Status",
          type_ = "string",
          json_path = ".status.pull_state"
))]
pub struct ProjectSpec {
    pub project_url: String,
    pub revision: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct ProjectStatus {
    pub pull_state: PullState,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub enum PullState {
    Pulling,
    Pulled,
    Failed,
    Unknown,
}
