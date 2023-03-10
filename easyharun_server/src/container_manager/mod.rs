use anyhow::Context;
use tracing::{debug, trace, warn};
use crate::brain::brain::Brain;
use crate::config::config_world_builder::build_world_from_config;
use crate::container_manager::world::Worlds;
use crate::docker::docker_action_executer::DockerActionExecuter;
use crate::docker::docker_world_builder::build_world_from_docker;

pub mod world;

#[derive(Debug)]
pub struct ContainerManager;

impl ContainerManager {

    pub fn new() -> Self {
        Self {}
    }

    #[tracing::instrument]
    pub async fn run(&self) {

        loop {
            match self.tick().await {
                Ok(_) => {},
                Err(e) => {
                    warn!("tick error");
                    eprintln!("Container Manager Error {:?}", e);
                    panic!("kill");
                }
            };

            trace!("sleep");
            ::tokio::time::sleep(::tokio::time::Duration::from_millis(100)).await;
            trace!("/sleep");
        }

    }

    #[tracing::instrument]
    async fn tick(&self) -> Result<(), ::anyhow::Error> {
        let worlds = Worlds {
            expected: build_world_from_config().await.context("could not build world from config")?,
            current: build_world_from_docker().await.context("could not build world from docker")?
        };

        debug!("created worlds");

        let next_action = Brain::think_about_next_action(&worlds).context("brain error, could not resolve brain action.")?;

        DockerActionExecuter::execute(&next_action).await.context("docker action executer")?;

        Ok(())
    }
}