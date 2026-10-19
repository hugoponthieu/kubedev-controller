mod context;
mod pull_job;
mod pvc;
use std::collections::BTreeMap;

fn workspace_labels(project_name: &str) -> BTreeMap<String, String> {
    BTreeMap::from([
        (
            "app.kubernetes.io/name".to_string(),
            "kubedev-workspace".to_string(),
        ),
        (
            "kubedev-controller/project".to_string(),
            project_name.to_string(),
        ),
    ])
}
