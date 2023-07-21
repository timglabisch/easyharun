use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use anyhow::{anyhow, Context, Error};
use async_trait::async_trait;
use tokio::select;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tracing::{info, trace, warn};
use easyact::{Actor, ActorConfig, ActorState, ActorStateHandle};
use easyharun_lib::config::{Config, ConfigContainer, ConfigFileHealthCheck};
use easyharun_lib::ContainerId;
use crate::config::config_provider::config_get;
use crate::container_manager::world::WorldContainer;
use crate::docker::docker_world_builder::build_world_from_docker;
use crate::health_check::{HealthCheckMsgRecv, HealthCheckMsgRecvCheckFailed, HealthCheckMsgRecvCheckOk};
use crate::health_check::http::health_check_http::{HealthCheckHttp};
use crate::kv_container::KV;

pub struct HealthCheckHttpConfig {
    pub container_id: ContainerId,
    pub url: String,
    pub timeout_ms: u32,
}

pub enum HealthCheckType {
    HealthCheckTypeHttp(HealthCheckHttpConfig),
}

pub enum HealthCheck {
    Http(ActorStateHandle<()>),
}

impl HealthCheck {
    pub fn kill(&self) {
        match self {
            Self::Http(s) => s.shutdown(),
        };
    }
}

pub struct HealthCheckManager {
    health_checks: HashMap<ContainerId, Vec<(String, HealthCheck)>>,
    actor_state: ActorState<HealthCheckMsgRecv>
}

impl HealthCheckManager {
    pub fn new(actor_state: ActorState<HealthCheckMsgRecv>) -> Self {
        Self {
            health_checks: HashMap::new(),
            actor_state
        }
    }
}

#[async_trait]
impl Actor for HealthCheckManager {
    type MSG = HealthCheckMsgRecv;

    fn get_actor_state(&mut self) -> &mut ActorState<HealthCheckMsgRecv> {
        &mut self.actor_state
    }

    fn timer_duration(&self) -> Option<Duration> {
        Some(Duration::from_millis(500))
    }

    async fn on_timer(&mut self) -> Result<(), Error> {
        self.run_inner_maintain_checks().await
    }

    async fn on_msg(&mut self, msg: HealthCheckMsgRecv) -> Result<(), Error> {
        self.run_inner_got_msg(msg).await
    }
}

impl HealthCheckManager {

    pub async fn run_inner_got_msg(&self, msg : HealthCheckMsgRecv) -> Result<(), ::anyhow::Error> {

        match msg {
            HealthCheckMsgRecv::CheckFailed(msg) => self.on_health_check_failed(msg).await?,
            HealthCheckMsgRecv::CheckOk(msg) => self.on_health_check_ok(msg).await?,
        };

        Ok(())
    }

    pub async fn on_health_check_failed(&self, msg : HealthCheckMsgRecvCheckFailed) -> Result<(), ::anyhow::Error> {
        info!("health check failed {:?}", msg);
        KV::mark_container_target_healthy(&msg.container_id, &msg.target, false);
        Ok(())
    }

    pub async fn on_health_check_ok(&self, msg : HealthCheckMsgRecvCheckOk) -> Result<(), ::anyhow::Error> {
        info!("health check ok {:?}", msg);
        KV::mark_container_target_healthy(&msg.container_id, &msg.target, true);
        Ok(())
    }

    pub async fn run_inner_maintain_checks(&mut self) -> Result<(), ::anyhow::Error> {
        let container_world = build_world_from_docker().await.context("check docker")?;

        let config = config_get();

        let container_ids_that_have_checks_running = {
            let mut buf = HashSet::new();
            for (id, _) in self.health_checks.iter() {
                buf.insert(id.clone());
            }
            buf
        };

        let container_ids_that_exists = {
            let mut buf = HashSet::new();
            for world_container in container_world.get_containers() {
                let container_id = match &world_container.container_id {
                    Some(s) => s.clone(),
                    None => continue,
                };

                buf.insert(container_id.clone());
            }
            buf
        };


        // containers with missing checks.
        {
            let container_ids_for_containers_without_checks = container_ids_that_exists.difference(&container_ids_that_have_checks_running).collect::<Vec<_>>();
            for container_id in container_ids_for_containers_without_checks {

                let world_container = match container_world.get_containers().iter().find(|c|c.container_id == Some(container_id.clone())) {
                    Some(v) => v,
                    None => {
                        warn!("could not find expected container, maybe a bug.");
                        continue;
                    }
                };

                info!("Starting Health Checks for {:?}", world_container.container_id);
                self.health_checks.insert(
                    container_id.clone(),
                    self.build_health_checks_for_container(&world_container, &config).context("could not build health checks")?,
                );
            }
        }


        // containers that have checks running, but the container is not there.
        // so we drop the health check
        {
            let container_ids_with_running_checks_without_running_container = container_ids_that_have_checks_running.difference(&container_ids_that_exists).collect::<Vec<_>>();

            for container_id in container_ids_with_running_checks_without_running_container {
                match self.health_checks.get(container_id) {
                    None => {
                        warn!("could not find expected check, could be a bug.");
                        continue;
                    },
                    Some(checks) => {
                        for (check_name, handle) in checks {
                            info!("Stopping health check because container is not running.");
                            handle.kill();
                        }
                    },
                };

                self.health_checks.remove(container_id);
            }
        }

        Ok(())
    }

    pub fn build_health_checks_for_container(
        &self,
        world_container: &WorldContainer,
        config: &Config,
    ) -> Result<Vec<(String, HealthCheck)>, ::anyhow::Error> {
        let mut buf = vec![];
        for health_check_expected in world_container.health_checks.iter() {
            let health_check = self.build_health_check(
                world_container,
                health_check_expected.as_str(),
                config,
            ).context("build health check")?;

            buf.push((
                health_check_expected.to_string(),
                health_check
            ));
        }

        Ok(buf)
    }

    pub fn template_parse_world_container(template: &str, world_container: &WorldContainer) -> Result<String, ::anyhow::Error> {
        Ok(
            template
            .replace("{{ ", "{{")
            .replace(" }}", "}}")
            .replace(
                "{{container.port_dynamic_host}}",
                world_container.container_port_dynamic_host.context("get container_port_dynamic_host")?.to_string().as_str()
            )
        )
    }

    pub fn build_health_check(&self, world_container: &WorldContainer, health_check_name: &str, config: &Config) -> Result<HealthCheck, ::anyhow::Error> {
        let config_health_check = match config.health_check.iter().find(|x| x.name == health_check_name) {
            Some(s) => s,
            None => return Err(anyhow!("could not find config for health_check {}", health_check_name))
        };

        let container_id = &world_container.container_id.clone().context("container must have a container id")?;

        Ok(match config_health_check.check.as_str() {
            "http" => HealthCheck::Http(self.build_health_check_http(
                container_id.clone(),
                config_health_check.clone(),
                health_check_name,
                world_container
            )?),
            _ => return Err(anyhow!("health_check type {} is not defined", config_health_check.check)),
        })
    }

    pub fn build_health_check_http(
        &self,
        container_id: ContainerId,
        config_file_health_check: ConfigFileHealthCheck,
        health_check_name: &str,
        world_container: &WorldContainer
    ) -> Result<ActorStateHandle<()>, ::anyhow::Error> {
        let health_check_id = format!("{}-{}", health_check_name, container_id.as_str());

        let check_config = HealthCheckHttpConfig {
            container_id: container_id.clone(),
            url: Self::template_parse_world_container(&config_file_health_check.url, world_container).context("template_parse_world_container")?,
            timeout_ms: config_file_health_check.timeout_ms,
        };

        let (_, handle, _) = Actor::spawn(ActorConfig::new("Healh Check Http", "Healh Check").build(), |actor_state| HealthCheckHttp::new(
            health_check_id,
            self.actor_state.create_handle(),
            check_config,
            actor_state,
        ));

        Ok(handle)
    }
}