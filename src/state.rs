use kube::Client;

use crate::project_puller::ProjectPuller;

pub struct ControllerState {
    pub project_puller: ProjectPuller,
}

impl ControllerState {
    pub fn new(client: &Client) -> Self {
        let project_puller = ProjectPuller::new(client.clone());
        Self { project_puller }
    }
}
