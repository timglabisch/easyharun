use anyhow::Context;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigFile {
    pub proxy: Vec<ConfigFileProxy>,
    pub container: Vec<ConfigFileContainer>,
    pub health_check: Vec<ConfigFileHealthCheck>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigFileHealthCheck {
    pub name: String,
    pub check: String,
    pub url: String,
    pub timeout_ms: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigFileProxy {
    pub name: String,
    pub listen: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigFileContainer {
    pub name: String,
    pub image: String,
    pub replicas: u32,
    pub container_port: u32,
    pub health_checks: Vec<String>,
    pub proxies: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigContainer {
    pub name: String,
    pub image: String,
    pub replica_id: u32,
    pub container_port: u32,
    pub health_checks: Vec<String>,
    pub proxies: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub proxy: Vec<ConfigFileProxy>,
    pub container: Vec<ConfigContainer>,
    pub health_check: Vec<ConfigFileHealthCheck>
}

impl Config {

    pub async fn read_from_file(file : &str) -> Result<Self, ::anyhow::Error> {
        let contents = ::tokio::fs::read(file).await.context(format!("reading file {}", &file))?;

        let config_file = ::toml::from_str::<ConfigFile>(
            &String::from_utf8(contents).context(format!("config file {} does not contains vaild uft8", &file))?
        ).context(format!("could not parse toml file {}", &file))?;


        let container = {
            let mut buffer = vec![];

            for config_file_container in config_file.container {
                for replica_id in 0..config_file_container.replicas {
                    buffer.push(ConfigContainer {
                        replica_id,
                        proxies: config_file_container.proxies.clone(),
                        name: config_file_container.name.clone(),
                        image: config_file_container.image.clone(),
                        health_checks: config_file_container.health_checks.clone(),
                        container_port: config_file_container.container_port
                    })
                }
            }

            buffer
        };

        Ok(Config {
            proxy: config_file.proxy,
            health_check: config_file.health_check,
            container
        })
    }
}