use bollard::container::StartContainerOptions;
use bollard::image::CreateImageOptions;
use bollard::{
    container::{Config, CreateContainerOptions, ListContainersOptions},
    models::{HostConfig, Mount, MountTypeEnum},
    volume::{CreateVolumeOptions, ListVolumesOptions},
    Docker,
};
use futures_util::stream::TryStreamExt;
use std::collections::HashMap;
use std::default::Default;

const VOLUME_NAME: &str = "etcd_data";
const IMAGE_NAME: &str = "quay.io/coreos/etcd:v3.5.6";
const CONTAINER_NAME: &str = "etcd";

pub async fn check_requirements() {
    println!("Checking etcd requirements... ");

    if is_etcd_container_created().await {
        println!("etcd is up and running.");
    } else {
        panic!("etcd is not running. It will be set up with `cargo micro init`");
    }
}

async fn is_etcd_container_created() -> bool {
    let docker =
        Docker::connect_with_local_defaults().expect("Failed to connect to docker. Not installed?");

    let mut list_container_filters = HashMap::new();
    list_container_filters.insert("name", vec!["etcd"]);

    let containers = &docker
        .list_containers(Some(ListContainersOptions {
            all: false,
            filters: list_container_filters,
            ..Default::default()
        }))
        .await
        .expect("Failed to list containers");

    !containers.is_empty()
}

/// Starts the etcd container.
/// If the container never starts before, it creates a volume and get the needed container.
pub async fn start() {
    println!("Starting etcd...");
    let docker =
        Docker::connect_with_local_defaults().expect("Failed to connect to docker. Not installed?");

    // Check if volume is already created
    let mut filters = HashMap::new();
    filters.insert("name", vec![VOLUME_NAME]);

    let options = ListVolumesOptions { filters };
    let volumes = docker
        .list_volumes(Some(options))
        .await
        .expect("Failed to list volumes");

    // Checks if volume is already created
    if volumes
        .volumes
        .expect("No volumes found in list response.")
        .len()
        == 0
    {
        println!("Creating volume for etcd data...");
        let config = CreateVolumeOptions {
            name: VOLUME_NAME,
            ..Default::default()
        };

        docker
            .create_volume(config)
            .await
            .expect("Failed to create volume");
        println!("Volume created.");
    }

    if !is_etcd_container_created().await {
        println!("Pulling etcd image...");

        let options = Some(CreateImageOptions {
            // repo: "quay.io/coreos",
            from_image: "quay.io/coreos/etcd:v3.5.6",
            // tag: IMAGE_TAG,
            ..Default::default()
        });
        docker
            .create_image(options, None, None)
            .try_collect::<Vec<_>>()
            .await
            .expect("Failed to pull image");
        println!("Pulled etcd image.");

        println!("Creating etcd container...");

        let options = Some(CreateContainerOptions {
            name: CONTAINER_NAME,
        });

        let mut exposed_ports = HashMap::new();
        exposed_ports.insert("2379/tcp", HashMap::new());
        exposed_ports.insert("2380/tcp", HashMap::new());

        // Taken from https://etcd.io/docs/v3.5/op-guide/container/#docker
        let cmd = vec!["etcd", "--data-dir=/etcd-data"];

        let config = Config {
            image: Some(IMAGE_NAME),
            exposed_ports: Some(exposed_ports),
            host_config: Some(HostConfig {
                mounts: Some(vec![Mount {
                    target: Some("/etcd-data".to_string()),
                    source: Some(VOLUME_NAME.to_string()),
                    typ: Some(MountTypeEnum::VOLUME),
                    ..Default::default()
                }]),
                ..Default::default()
            }),
            cmd: Some(cmd),
            ..Default::default()
        };

        docker
            .create_container(options, config)
            .await
            .expect("Failed to create container");
        println!("Container created.");
    }

    println!("Starting etcd container...");
    docker
        .start_container(CONTAINER_NAME, None::<StartContainerOptions<&str>>)
        .await
        .expect("Failed to start container");
    println!("etcd is up and running.");
}

/// Stops the etcd container.
pub async fn stop() {
    println!("Stopping etcd...");
    let docker =
        Docker::connect_with_local_defaults().expect("Failed to connect to docker. Not installed?");

    docker
        .stop_container(CONTAINER_NAME, None)
        .await
        .expect("Cannot stop etcd");

    println!("etcd stopped.");
}

/// Cleanup the etcd container.
/// Removes the volume and the container.
pub async fn cleanup() {
    println!("Cleaning up etcd...");
    todo!("Cleanup etcd from docker: volume, container, image");
}
