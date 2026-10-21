use kube::Client;

use crate::project_puller::ProjectPuller;
use crate::resources::context::ContainerImages;

pub struct ControllerState {
    pub project_puller: ProjectPuller,
}

impl ControllerState {
    pub fn new(client: &Client, images: impl Into<ContainerImages>) -> Self {
        let project_puller = ProjectPuller::new(client.clone(), images);
        Self { project_puller }
    }
}
