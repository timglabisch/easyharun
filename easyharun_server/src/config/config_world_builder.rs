use easyharun_lib::config::ConfigContainer;
use crate::config::config_provider::config_get;
use crate::container_manager::world::{World, WorldContainer};

pub async fn build_world_from_config() -> Result<World, ::anyhow::Error> {
    let config = config_get();

    let mut containers = vec![];

    for config_container in config.container.iter() {
        containers.push(WorldContainer {
            container_id: None,
            name: config_container.name.clone(),
            image: config_container.image.clone(),
            replica_id: config_container.replica_id.clone(),
            container_port: config_container.container_port.clone(),
        });
    }

    Ok(World::new(containers))
}