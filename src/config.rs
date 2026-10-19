use clap::Parser;

#[derive(Clone, Debug, Parser)]
#[command(name = "kubedev-controller")]
pub struct Config {
    #[arg(long, env = "KUBEDEV_PROJECT_PULLER_VERSION", default_value = "0.0.1")]
    pub project_puller_version: String,

    #[arg(long, env = "KUBEDEV_WORKSPACE_VERSION", default_value = "0.0.4")]
    pub workspace_version: String,

    #[arg(long, env = "KUBEDEV_GIT_VERSION", default_value = "latest")]
    pub git_version: String,
}

impl Config {
    pub fn project_puller_image(&self) -> String {
        format!("hugo08/project-puller:{}", self.project_puller_version)
    }

    pub fn workspace_image(&self) -> String {
        format!("hugo08/home-dev-container:{}", self.workspace_version)
    }

    pub fn git_image(&self) -> String {
        format!("alpine/git:{}", self.git_version)
    }
}
