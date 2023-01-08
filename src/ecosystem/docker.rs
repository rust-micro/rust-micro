use bollard::Docker;

pub async fn check_requirements() {
    println!("Checking docker requirements...");
    let docker =
        Docker::connect_with_local_defaults().expect("Failed to connect to docker. Not installed?");
    let version = docker
        .version()
        .await
        .expect("Failed to get docker version");
    println!("{:?}", version);
}
