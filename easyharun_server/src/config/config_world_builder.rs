use easyharun_lib::config::ConfigContainer;
use crate::config::config_provider::config_get;
use crate::container_manager::world::{World, WorldContainer};

pub async fn build_world_from_config() -> Result<World, ::anyhow::Error> {
    let config = config_get();

    let mut containers = vec![];

    for config_container in config.container.iter() {

        let world_container = build_world_container_from_config_container(config_container);

        for _ in 0..config_container.replicas {
            containers.push(world_container.clone());
        }
    }

    Ok(World::new(containers))
}

pub fn build_world_container_from_config_container(config_container : &ConfigContainer) -> WorldContainer {
    WorldContainer {
        id: None,
        version: config_container.version.clone(),
        image: config_container.image.clone(),
        name: config_container.name.clone(),
        internal_id: None,
    }
}