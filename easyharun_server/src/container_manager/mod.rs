use std::time::Duration;
use anyhow::{Context, Error};
use tracing::{debug, info};
use easyact::{Actor, ActorState};
use crate::brain::brain::Brain;
use crate::config::config_world_builder::build_world_from_config;
use crate::container_manager::world::Worlds;
use crate::docker::docker_action_executer::DockerActionExecuter;
use crate::docker::docker_world_builder::build_world_from_docker;
use async_trait::async_trait;
use crate::config::config_provider::ConfigReader;

pub mod world;

#[derive(Debug)]
pub struct ContainerManager {
    pub actor_state: ActorState<()>,
    pub config_reader: ConfigReader,
}


#[async_trait]
impl Actor for ContainerManager {
    type MSG = ();

    fn get_actor_state(&mut self) -> &mut ActorState<Self::MSG> {
        &mut self.actor_state
    }

    fn timer_duration(&self) -> Option<Duration> {
        Some(Duration::from_millis(500))
    }

    #[tracing::instrument]
    async fn on_timer(&mut self) -> Result<(), Error> {

        let worlds = Worlds {
            expected: build_world_from_config(&self.config_reader).await.context("could not build world from config")?,
            current: build_world_from_docker().await.context("could not build world from docker")?
        };

        debug!("created worlds");

        let next_action = Brain::think_about_next_action(&worlds).context("brain error, could not resolve brain action.")?;

        info!("execute action {:?}", next_action);
        DockerActionExecuter::execute(&next_action).await.context("docker action executer")?;

        Ok(())

    }

    async fn on_msg(&mut self, msg: Self::MSG) -> Result<(), Error> {
        Ok(())
    }
}