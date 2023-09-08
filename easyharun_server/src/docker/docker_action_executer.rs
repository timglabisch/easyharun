use std::collections::HashMap;
use anyhow::Context;
use bollard::container::{Config, CreateContainerOptions, StartContainerOptions};
use bollard::Docker;
use bollard::image::CreateImageOptions;
use bollard::models::{HostConfig, PortBinding};
use tracing::{debug, info};
use uuid::Uuid;
use crate::brain::brain_action::{BrainAction, ContainerStart, ContainerStop};
use crate::docker::docker_connection::docker_create_connection;
use crate::kv_container::KV;
use futures::StreamExt;
use serde_json::json;

pub struct DockerActionExecuter {
    kv: KV,
}


impl DockerActionExecuter {
    pub fn new(kv: KV) -> Self {
        Self {
            kv
        }
    }

    pub async fn execute(&self, action: &BrainAction) -> Result<(), ::anyhow::Error> {
        match action {
            BrainAction::ContainersStart(c) => return self.execute_containers_start(c).await,
            BrainAction::ContainersStop(c) => return self.execute_containers_stop(c).await,
            BrainAction::NoOp => Ok(()),
        }
    }

    async fn execute_containers_start(&self, action: &Vec<ContainerStart>) -> Result<(), ::anyhow::Error> {
        let docker = docker_create_connection()?;

        for container_start in action.iter() {
            self.execute_container_start(&docker, container_start).await.context("starting docker container")?
        }

        Ok(())
    }

    async fn execute_container_start(&self, docker: &Docker, container_start: &ContainerStart) -> Result<(), ::anyhow::Error> {
        debug!("execute_containers_start");

        let container = &container_start.container_world;

        let options = CreateImageOptions{
            from_image: container.image.clone(),
            ..Default::default()
        };

        let mut create_image = docker.create_image(Some(options), None, None);

        while let Some(v) = create_image.next().await {
            debug!("getting image ...");
        }
        debug!("got image ...");


        let name = Uuid::new_v4();

        let labels = {
            let mut buf = HashMap::new();
            buf.insert("easyharun".to_string(), "1.0.0".to_string());
            buf.insert("easyharun_name".to_string(), container.name.to_string());
            buf.insert("easyharun_image".to_string(), container.image.to_string());
            buf.insert("easyharun_replica_id".to_string(), container.replica_id.to_string());
            buf.insert("easyharun_container_ports".to_string(), container.container_ports.iter().map(|x|x.to_string()).collect::<Vec<_>>().join(","));
            buf.insert("easyharun_health_checks".to_string(), container.health_checks.join(","));
            buf.insert("easyharun_proxies".to_string(), json!(container.proxies.clone()).to_string());

            buf
        };

        let mut port_bindings = ::std::collections::HashMap::new();
        for port in container.container_ports.iter() {
            port_bindings.insert(
                format!("{}", port),
                Some(vec![PortBinding {
                    host_ip: Some(String::from("0.0.0.0")),
                    host_port: None, // we let the os pick the port
                }]),
            );
        }

        let exposed_ports = {
            let mut exposed_ports = HashMap::new();

            for port in container.container_ports.iter() {
                let empty = HashMap::<(), ()>::new();
                let exposed_port = format!("{}", port);
                exposed_ports.insert(exposed_port, empty);
            }
            exposed_ports
        };

        let config = Config {
            image: Some(container.image.clone()),
            labels: Some(labels),
            host_config: Some(HostConfig {
                port_bindings: Some(port_bindings),
                ..Default::default()
            }),
            exposed_ports: Some(exposed_ports),
            ..Default::default()
        };

        let _ = &docker
            .create_container(
                Some(CreateContainerOptions {
                    name: name.to_string(),
                    platform: None,
                }),
                config,
            )
            .await?;

        debug!("execute_containers_start");
        let _ = &docker
            .start_container(&name.to_string(), None::<StartContainerOptions<String>>)
            .await?;

        Ok(())
    }


    async fn execute_containers_stop(&self, action: &Vec<ContainerStop>) -> Result<(), ::anyhow::Error> {
        let docker = docker_create_connection()?;

        for container_stop in action.iter() {
            self.execute_container_stop(container_stop).await?
        }

        Ok(())
    }

    async fn execute_container_stop(&self, container: &ContainerStop) -> Result<(), ::anyhow::Error> {

        info!("execute_containers_stop");
        self.kv.mark_container_to_be_deleted(&container.id).await;

        Ok(())
    }
}