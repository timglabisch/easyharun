use anyhow::Context;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    service: Vec<ConfigService>,
    container: Vec<ConfigContainer>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigService {
    name: String,
    port: u64,
    target: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigContainer {
    name: String,
    version: String,
    image: String,
    replicas: String,
    container_port: String,
    target_port: String,
    health_check_cmd: Option<String>,
}

impl Config {
    pub async fn read_from_file(file : &str) -> Result<Self, ::anyhow::Error> {
        let contents = ::tokio::fs::read(file).await.context(format!("reading file {}", &file))?;

        Ok(::toml::from_str::<Config>(
            &String::from_utf8(contents).context(format!("config file {} does not contains vaild uft8", &file))?
        ).context(format!("could not parse toml file {}", &file))?)
    }
}