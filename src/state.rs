use k8s_openapi::api::{batch::v1::Job, core::v1::PersistentVolumeClaim};
use kube::{Api, Client};

use crate::project_puller::ProjectPuller;

pub struct ControllerState {
    jobs: Api<Job>,
    pvcs: Api<PersistentVolumeClaim>,
    pub project_puller: ProjectPuller,
}

impl ControllerState {
    pub fn new(client: &Client) -> Self {
        let pvcs = Api::<PersistentVolumeClaim>::all(client.clone());
        let jobs = Api::<Job>::all(client.clone());
        let project_puller = ProjectPuller::new(client.clone());
        Self {
            jobs,
            pvcs,
            project_puller,
        }
    }
}
