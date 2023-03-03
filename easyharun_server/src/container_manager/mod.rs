use anyhow::Context;
use crate::brain::brain::Brain;
use crate::config::config_world_builder::build_world_from_config;
use crate::container_manager::world::Worlds;
use crate::docker::docker_world_builder::build_world_from_docker;

pub mod world;

pub struct ContainerManager {

}

pub async fn run() {

}

pub async fn tick() -> Result<(), ::anyhow::Error> {
    let worlds = Worlds {
        expected: build_world_from_config().await.context("could not build world from config")?,
        current: build_world_from_docker().await.context("could not build world from docker")?
    };

    let next_action = Brain::think_about_next_action(&worlds);



    Ok(())
}