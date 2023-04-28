use anyhow::anyhow;
use easyharun_lib::ContainerId;
use crate::container_manager::world::WorldContainer;

#[derive(Debug, Clone)]
pub enum BrainAction {
    ContainersStart(Vec<ContainerStart>),
    ContainersStop(Vec<ContainerStop>),
    NoOp,
}

#[derive(Debug, Clone)]
pub struct ContainerStop {
    pub id: ContainerId,
    pub world_container: WorldContainer,
}

impl ContainerStop {
    pub fn new_from_world_container(world_container : &WorldContainer) -> Result<Self, ::anyhow::Error> {
        Ok(Self {
            id: match &world_container.container_id {
                Some(id) => id.clone(),
                None => return Err(anyhow!("could not stop container without id"))
            },
            world_container: world_container.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ContainerStart {
    pub container_world: WorldContainer,
}

impl ContainerStart {
    pub fn new_from_world_container(container_world : &WorldContainer) -> Self {
        Self {
            container_world: container_world.clone(),
        }
    }
}