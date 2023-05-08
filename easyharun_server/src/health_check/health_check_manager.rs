use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use anyhow::{anyhow, Context};
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tracing::trace;
use easyharun_lib::config::{Config, ConfigContainer};
use easyharun_lib::ContainerId;
use crate::config::config_provider::config_get;
use crate::container_manager::world::WorldContainer;
use crate::docker::docker_world_builder::build_world_from_docker;
use crate::health_check::HealthCheckMsgRecv;
use crate::health_check::http::health_check_http::{HealthCheckHttp, HealthCheckHttpHandle};

pub struct HealthCheckHttpConfig {
    container_id: String,
    url: String,
}

pub enum HealthCheckType {
    HealthCheckTypeHttp(HealthCheckHttpConfig),
}

enum HealthCheck {
    Http(HealthCheckHttpHandle),
}

pub struct HealthCheckManager {
    sender: Sender<HealthCheckMsgRecv>,
    recv: Receiver<HealthCheckMsgRecv>,
    health_checks: HashMap<ContainerId, Vec<(String, HealthCheck)>>,
}

impl HealthCheckManager {
    pub fn new() -> HealthCheckManager {
        let (sender, recv) = channel(1000);

        Self {
            sender,
            recv,
            health_checks: HashMap::new(),
        }
    }

    pub async fn run(mut self) {
        loop {
            match self.run_inner().await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Health Check Manager Error: \n{:?}\n\n", e)
                }
            };

            trace!("sleep");
            ::tokio::time::sleep(::tokio::time::Duration::from_millis(500)).await;
            trace!("/sleep");
        }
    }

    pub async fn run_inner(&mut self) -> Result<(), ::anyhow::Error> {
        let container_world = build_world_from_docker().await.context("check docker")?;

        let config = config_get();


        // any checks we could kill?
        let health_check_configs = {
            let mut buf = HashMap::new();
            for config_health_check in config.health_check.iter() {
                buf.insert(config_health_check.name.clone(), config_health_check.clone());
            }

            buf
        };

        for world_container in container_world.get_containers() {
            let container_id = match &world_container.container_id {
                Some(s) => s.clone(),
                None => continue,
            };

            // removing or adding a health check will result in a new container.
            // so we've 2 cases.
            // 1. all health checks are missing
            match self.health_checks.entry(container_id) {
                Occupied(mut o) => {
                    continue;
                }
                Vacant(o) => {
                    o.insert(self::build_world_from_docker());
                }
            }
        }

        // die health checks selbst sind in der config definiert
        // allerdings ist in docker definiert, welche health checks relevant sind.
        // wenn es einen check gibt, der in der config fehlt, wird der check als fehlerhaft markiert.


        Ok(())
    }

    pub fn build_health_checks_for_container(world_container: &WorldContainer, config: &Config) -> Result<Vec<(String, HealthCheck)>, ::anyhow::Error> {
        let mut buf = vec![];
        for health_check_expected in world_container.health_checks.iter() {

            let container_id = match world_container.container_id {
                Some(s) => s,
                None => return Err(anyhow!("container not found, should not happen")),
            };

            buf.insert((
                health_check_expected.to_string(),
                self::build_health_check(world_container.container_id)?
            ))
        }

        Ok(buf)
    }

    pub fn build_health_check(container_id: ContainerId, name: &str, config: &Config) -> Result<HealthCheck, ::anyhow::Error> {
        let config_health_check = match config.health_check.iter().find(|x| x.name == name) {
            Some(s) => s,
            None => return Err(anyhow!("could not find config for health_check {}", name))
        };

        match config_health_check.check {
            "http" => HealthCheck::Http(HealthCheckHttp::new(
                format!("{}-{}", name, container_id.as_str()),
            ))
            _ => return Err(anyhow!("health_check type {} is not defined", config_health_check.check)),
        }
    }
}