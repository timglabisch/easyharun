use std::collections::HashMap;
use anyhow::{anyhow, Context};
use bollard::container::ListContainersOptions;
use tracing::{debug, info, trace, warn};
use bollard::Docker;
use bollard::models::ContainerSummary;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use crate::container_manager::world::{World, WorldContainer};


pub async fn build_world_from_docker() -> Result<World, ::anyhow::Error> {
    trace!("starting to check the docker world");

    let docker = Docker::connect_with_socket_defaults().context("could not connect docker.")?;

    let mut filters = HashMap::new();
    filters.insert("label", vec!["easyharun=\"1.0.0\""]);

    let containers = docker.list_containers(Some(ListContainersOptions {
        all: true,
        size: false,
        limit: None,
        filters
    })).await.context("could not read containers from docker container")?;

    let mut world_containers = vec![];
    for container in containers.iter() {
        match build_world_container(container) {
            Err(e) => {
                warn!("error while scanning container {:?}. error: {:#?}", container.id, e);
            }
            Ok(None) => {
                info!("container {:?} is ignored.", container.id)
            },
            Ok(Some(c)) => {
                world_containers.push(c);
            }
        };
    }

    Ok(World::new(world_containers))
}

fn build_world_container(container_summary : &ContainerSummary) -> Result<Option<WorldContainer>, ::anyhow::Error> {
    let labels = container_summary.labels.clone().unwrap_or(HashMap::new());

    debug!("inspecting container {}", container_summary.id.as_ref().unwrap_or(&"NO_ID".to_string()));

    let mut container_name = None;
    let mut container_version = None;

    let container_id = match container_summary.id.clone() {
        Some(s) => s,
        None => return Err(anyhow!("container without id"))
    };

    let container_image = match container_summary.image.clone() {
        Some(s) => s,
        None => return Err(anyhow!("container {} has no image", container_id))
    };

    for entry in labels.iter() {
        match &entry.0[..] {
            "easyharun_name" => container_name = Some(entry.1.clone()),
            "easyharun_version" => container_version = Some(entry.1.clone()),
            _ => {
                continue;
            }
        }
    }

    let container_name = match container_name {
        Some(s) => s,
        None => return Err(anyhow!("container {} has no container_name", container_id))
    };

    let container_version = match container_version {
        Some(s) => s,
        None => return Err(anyhow!("container {} has no container_version", container_id))
    };

    Ok(Some(
        WorldContainer {
            id: Some(container_id),
            image: container_image,
            name: container_name,
            version: container_version,
        }
    ))
}