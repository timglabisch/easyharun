use std::collections::HashMap;
use anyhow::Context;
use tracing::trace;
use crate::config::config_provider::config_get;
use crate::docker::docker_world_builder::build_world_from_docker;
use crate::health_check::http::health_check_http::HealthCheckHttpHandle;

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
    health_checks: HashMap<String, Vec<(String, HealthCheck)>>,
}

impl HealthCheckManager {
    pub async fn run(mut self) {
        loop {
            match self.run_inner().await {
                Ok(_) => {},
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
        for config_health_check in config.container.iter() {

        }

        for container in container_world.get_containers() {

        }

        // die health checks selbst sind in der config definiert
        // allerdings ist in docker definiert, welche health checks relevant sind.
        // wenn es einen check gibt, der in der config fehlt, wird der check als fehlerhaft markiert.



        Ok(())
    }
}