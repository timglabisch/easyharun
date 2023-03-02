use std::collections::HashMap;
use easyharun_lib::config::ConfigContainer;

#[derive(Debug, Clone)]
pub struct Worlds {
    pub current: World,
    pub expected: World,
}

#[derive(Debug, Clone)]
pub struct World {
    containers: Vec<WorldContainer>
}

impl World {
    pub fn new(containers: Vec<WorldContainer>) -> Self {

        let containers = containers.into_iter().enumerate().map(|(id, mut world_container)| {
            world_container.internal_id = Some(id as u64);
            world_container
        }).collect();

        Self {
            containers
        }
    }

    pub fn get_containers(&self) -> &Vec<WorldContainer> {
        &self.containers
    }
}

#[derive(Debug, Clone)]
pub struct WorldContainer {
    pub internal_id: Option<u64>,
    pub id: Option<String>,
    pub name: String,
    pub image: String,
    pub version: String,
}

impl WorldContainer {
    pub fn get_internal_id(&self) -> u64 {
        self.internal_id.expect("internal id must be given ...")
    }
}