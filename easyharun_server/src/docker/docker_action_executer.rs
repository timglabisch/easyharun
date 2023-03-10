use std::collections::HashMap;
use anyhow::Context;
use bollard::container::{Config, CreateContainerOptions, StartContainerOptions};
use bollard::Docker;
use bollard::image::CreateImageOptions;
use tracing::{debug, info};
use uuid::Uuid;
use crate::brain::brain_action::{BrainAction, ContainerStart, ContainerStop};
use crate::docker::docker_connection::docker_create_connection;
use crate::kv_container::KV;
use futures::StreamExt;

pub struct DockerActionExecuter;

impl DockerActionExecuter {
    #[tracing::instrument]
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
            Self::execute_container_start(&docker, container_start).await.context("starting docker container")?
        }

        Ok(())
    }

    async fn execute_container_start(docker: &Docker, container: &ContainerStart) -> Result<(), ::anyhow::Error> {
        info!("execute_containers_start");

        let options = CreateImageOptions{
            from_image: container.image.clone(),
            ..Default::default()
        };

        let mut create_image = docker.create_image(Some(options), None, None);

        while let Some(v) = create_image.next().await {
            info!("getting image ...");
        }
        info!("got image ...");


        let name = Uuid::new_v4();

        let labels = {
            let mut buf = HashMap::new();
            buf.insert("easyharun_name".to_string(), container.name.to_string());
            buf.insert("easyharun".to_string(), "1.0.0".to_string());
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
            Self::execute_container_stop(container_stop).await?
        }

        Ok(())
    }

    async fn execute_container_stop(container: &ContainerStop) -> Result<(), ::anyhow::Error> {

        info!("execute_containers_start");
        KV::mark_container_to_be_deleted(&container.id);

        Ok(())
    }
}