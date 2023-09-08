
use crate::config::config_provider::{ConfigReader};
use crate::container_manager::world::{World, WorldContainer};

pub async fn build_world_from_config(config_reader : &ConfigReader) -> Result<World, ::anyhow::Error> {
    let config = config_reader.get_copy().await;

    let mut containers = vec![];

    for config_container in config.container.iter() {
        containers.push(WorldContainer {
            container_id: None,
            container_port_mapping: None,
            name: config_container.name.clone(),
            image: config_container.image.clone(),
            replica_id: config_container.replica_id.clone(),
            container_ports: config_container.container_ports.clone(),
            proxies: config_container.proxies.clone(),
            health_checks: config_container.health_checks.clone()
        });
    }

    Ok(World::new(containers))
}