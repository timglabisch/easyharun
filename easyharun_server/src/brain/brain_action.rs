use anyhow::anyhow;
use crate::container_manager::world::WorldContainer;

pub enum BrainAction {
    ContainersStart(Vec<ContainerStart>),
    ContainersStop(Vec<ContainerStop>),
    NoOp,
}

pub struct ContainerStop {
    pub id: String,
    pub name: String,
    pub image: String,
}

impl ContainerStop {
    pub fn new_from_world_container(world_container : &WorldContainer) -> Result<Self, ::anyhow::Error> {
        Ok(Self {
            id: match &world_container.id {
                Some(id) => id.to_string(),
                None => return Err(anyhow!("could not stop container without id"))
            },
            name: world_container.name.to_string(),
            image: world_container.image.to_string(),
        })
    }
}

pub struct ContainerStart {
    pub name: String,
    pub image: String,
}

impl ContainerStart {
    pub fn new_from_world_container(world_container : &WorldContainer) -> Self {
        Self {
            name: world_container.name.to_string(),
            image: world_container.image.to_string(),
        }
    }
}