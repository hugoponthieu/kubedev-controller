use kube::Client;

use crate::config::Config;
use crate::project_puller::ProjectPuller;

pub struct ControllerState {
    pub project_puller: ProjectPuller,
}

impl ControllerState {
    pub fn new(client: &Client, config: Config) -> Self {
        let project_puller = ProjectPuller::new(client.clone(), config);
        Self { project_puller }
    }
}
