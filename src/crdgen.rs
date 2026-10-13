use kube::CustomResourceExt;
use kubedev_controller::project::Project;

fn main() {
    print!("{}", serde_yaml::to_string(&Project::crd()).unwrap())
}
