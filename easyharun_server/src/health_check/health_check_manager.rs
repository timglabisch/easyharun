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
    pub container_id: ContainerId,
    pub url: String,
}

pub enum HealthCheckType {
    HealthCheckTypeHttp(HealthCheckHttpConfig),
}

pub enum HealthCheck {
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

            if self.health_checks.get(&container_id).is_some() {
                continue;
            }

            self.health_checks.insert(
                container_id,
                self.build_health_checks_for_container(&world_container, &config).context("could not build health checks")?
            );
        }

        // die health checks selbst sind in der config definiert
        // allerdings ist in docker definiert, welche health checks relevant sind.
        // wenn es einen check gibt, der in der config fehlt, wird der check als fehlerhaft markiert.


        Ok(())
    }

    pub fn build_health_checks_for_container(
        &self,
        world_container: &WorldContainer,
        config: &Config
    ) -> Result<Vec<(String, HealthCheck)>, ::anyhow::Error> {
        let mut buf = vec![];
        for health_check_expected in world_container.health_checks.iter() {
            let health_check = self.build_health_check(
                &world_container.container_id.clone().context("container must have a container id")?,
                health_check_expected.as_str(),
                config
            ).context("build health check")?;

            buf.push((
                health_check_expected.to_string(),
                health_check
            ));
        }

        Ok(buf)
    }

    pub fn build_health_check(&self, container_id: &ContainerId, health_check_name: &str, config: &Config) -> Result<HealthCheck, ::anyhow::Error> {
        let config_health_check = match config.health_check.iter().find(|x| x.name == health_check_name) {
            Some(s) => s,
            None => return Err(anyhow!("could not find config for health_check {}", health_check_name))
        };

        Ok(match config_health_check.check.as_str() {
            "http" => HealthCheck::Http(HealthCheckHttp::new(
                format!("{}-{}", health_check_name, container_id.as_str()),
                self.sender.clone(),
                HealthCheckHttpConfig {
                    container_id: container_id.clone(),
                    url: config_health_check.url.to_string(),
                }
            )),
            _ => return Err(anyhow!("health_check type {} is not defined", config_health_check.check)),
        })
    }
}