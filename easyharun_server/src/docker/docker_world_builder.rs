use std::collections::HashMap;
use anyhow::{anyhow, Context};
use bollard::container::ListContainersOptions;
use tracing::{debug, info, trace, warn};

use bollard::models::ContainerSummary;
use easyharun_lib::config::ConfigContainer;
use easyharun_lib::ContainerId;

use crate::container_manager::world::{World, WorldContainer};
use crate::docker::docker_connection::docker_create_connection;
use crate::kv_container::KV;

pub struct DockerRunningContainerInfo {
    pub container_id: ContainerId,
}

pub fn docker_container_info(container: &ContainerSummary) -> Option<DockerRunningContainerInfo> {

    let labels = match &container.labels {
        None => return None,
        Some(s) => s,
    };

    if labels.get("easyharun") != Some(&"1.0.0".to_string()) {
        return None;
    }

    let container_state = match &container.state {
        Some(s) => s,
        None => {
            warn!("got a container without state");
            return None;
        }
    };

    match &container_state[..] {
        "running" | "restarting" | "paused" | "created" => {}, // todo, id container is created, it not yet has ports. proxy / healthcheck must ignore it...
        "exited" | "dead" => {
            return None;
        }
        _ => {
            warn!("unknown container state, {}", &container_state[..]);
            return None;
        }
    };

    let container_id = ContainerId::new(match &container.names {
        None => {
            warn!("container without a name");
            return None;
        }
        Some(s) => {
            match s.first() {
                None => {
                    warn!("container without a name #2");
                    return None;
                },
                Some(s) => s.to_string()
            }
        }
    });

    if KV::is_container_marked_to_be_deleted(&container_id) {
        return None;
    }

    return Some(DockerRunningContainerInfo{
        container_id
    });
}

pub async fn build_world_from_docker() -> Result<World, ::anyhow::Error> {
    trace!("starting to check the docker world");

    let docker = docker_create_connection()?;

    let mut filters : HashMap<String, Vec<String>> = HashMap::new();

    let containers = docker.list_containers(Some(ListContainersOptions {
        all: true,
        size: false,
        limit: None,
        filters
    })).await.context("could not read containers from docker container")?;

    let mut world_containers = vec![];
    for container in containers.iter() {

        let container_id = match docker_container_info(&container) {
            Some(s) => s.container_id,
            None => {
                continue
            }
        };

        match build_world_container(container) {
            Err(e) => {
                warn!("error while scanning container {:?}. error: {:#?}", container.id, e);
            }
            Ok(None) => {
                debug!("container {:?} is ignored.", container.id)
            },
            Ok(Some(c)) => {
                world_containers.push(c);
            }
        };
    }

    Ok(World::new(world_containers))
}

pub fn extract_ports_from_container_summary(container_summary : &ContainerSummary) -> Result<(u32, u32), ::anyhow::Error> {
    match container_summary.ports.clone() {
        Some(s) => match s.first() {
            Some(p) => match p.public_port {
                None => return Err(anyhow!("public port not given")),
                Some(public_port) => return Ok((p.private_port as u32, public_port as u32))
            },
            None => return Err(anyhow!("container without port #2"))
        },
        None => return Err(anyhow!("container without port"))
    };
}

fn build_world_container(container_summary : &ContainerSummary) -> Result<Option<WorldContainer>, ::anyhow::Error> {
    let labels = container_summary.labels.clone().unwrap_or(HashMap::new());

    debug!("inspecting container {}", container_summary.id.as_ref().unwrap_or(&"NO_ID".to_string()));

    let container_id = match container_summary.id.clone() {
        Some(s) => ContainerId::new(s),
        None => return Err(anyhow!("container without id"))
    };

    let name = match labels.get("easyharun_name") {
        Some(s) => s.to_string(),
        None => return Err(anyhow!("container without name"))
    };

    let image = match labels.get("easyharun_image") {
        Some(s) => s.to_string(),
        None => return Err(anyhow!("container without image"))
    };

    let replica_id = match labels.get("easyharun_replica_id") {
        Some(s) => match s.parse::<u32>() {
            Ok(k) => k,
            Err(e) => return Err(anyhow!("invalid easyharun_replica_id (not a number)"))
        },
        None => return Err(anyhow!("container without replica_id"))
    };

    let container_port = match labels.get("easyharun_container_port") {
        Some(s) => match s.parse::<u32>() {
            Ok(k) => k,
            Err(e) => return Err(anyhow!("invalid easyharun_container_port (not a number)"))
        },
        None => return Err(anyhow!("container without easyharun_container_port"))
    };

    Ok(Some(
        WorldContainer {
            container_id: Some(container_id),
            name,
            image,
            replica_id,
            container_port,
        }
    ))
}