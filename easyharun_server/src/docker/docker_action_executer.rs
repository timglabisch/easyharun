use std::collections::HashMap;
use bollard::container::{Config, CreateContainerOptions, StartContainerOptions, UpdateContainerOptions};
use bollard::Docker;
use uuid::Uuid;
use crate::brain::brain_action::{BrainAction, ContainerStart, ContainerStop};
use crate::docker::docker_connection::docker_create_connection;

struct DockerActionExecuter;

impl DockerActionExecuter {
    pub async fn execute(action: &BrainAction) -> Result<(), ::anyhow::Error> {
        match action {
            BrainAction::ContainersStart(c) => return Self::execute_containers_start(c).await,
            BrainAction::ContainersStop(c) => return Self::execute_containers_stop(c).await,
            BrainAction::NoOp => Ok(()),
        }
    }

    async fn execute_containers_start(action: &Vec<ContainerStart>) -> Result<(), ::anyhow::Error> {
        let docker = docker_create_connection()?;

        for container_start in action.iter() {
            Self::execute_container_start(&docker, container_start).await?
        }

        Ok(())
    }

    async fn execute_container_start(docker: &Docker, container: &ContainerStart) -> Result<(), ::anyhow::Error> {
        let name = Uuid::new_v4();

        let labels = {
            let mut buf = HashMap::new();
            buf.insert("easyharun".to_string(), "1".to_string());
            buf
        };

        let config = Config {
            image: Some(container.image.clone()),
            labels: Some(labels),
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

        let _ = &docker
            .start_container(&name.to_string(), None::<StartContainerOptions<String>>)
            .await?;

        Ok(())
    }


    async fn execute_containers_stop(action: &Vec<ContainerStop>) -> Result<(), ::anyhow::Error> {
        let docker = docker_create_connection()?;

        for container_stop in action.iter() {
            Self::execute_container_stop(&docker, container_stop).await?
        }

        Ok(())
    }

    async fn execute_container_stop(docker: &Docker, container: &ContainerStop) -> Result<(), ::anyhow::Error> {

        docker.

        docker.update_container(&container.id, UpdateContainerOptions {
            ..Default::default()
        });

        Ok(())
    }
}