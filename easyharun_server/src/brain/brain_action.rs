use anyhow::anyhow;
use crate::container_manager::world::WorldContainer;

#[derive(Debug, Clone)]
pub enum BrainAction {
    ContainersStart(Vec<ContainerStart>),
    ContainersStop(Vec<ContainerStop>),
    NoOp,
}

#[derive(Debug, Clone)]
pub struct ContainerStop {
    pub id: String,
    pub image: String,
}

impl ContainerStop {
    pub fn new_from_world_container(world_container : &WorldContainer) -> Result<Self, ::anyhow::Error> {
        Ok(Self {
            id: match &world_container.id {
                Some(id) => id.to_string(),
                None => return Err(anyhow!("could not stop container without id"))
            },
            image: world_container.image.to_string(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ContainerStart {
    pub container_port: u32,
    pub target_port: u32,
    pub image: String,
}

impl ContainerStart {
    pub fn new_from_world_container(world_container : &WorldContainer) -> Self {
        Self {
            container_port: world_container.container_port,
            target_port: world_container.host_port,
            image: world_container.image.to_string(),
        }
    }
}