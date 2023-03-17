use anyhow::Context;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub container: Vec<ConfigContainer>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigContainer {
    pub name: String,
    pub image: String,
    pub replicas: u32,
    pub container_port: u32,
    pub target_port: u32,
    pub health_check_cmd: Option<String>,
}

impl Config {
    pub async fn read_from_file(file : &str) -> Result<Self, ::anyhow::Error> {
        let contents = ::tokio::fs::read(file).await.context(format!("reading file {}", &file))?;

        Ok(::toml::from_str::<Config>(
            &String::from_utf8(contents).context(format!("config file {} does not contains vaild uft8", &file))?
        ).context(format!("could not parse toml file {}", &file))?)
    }
}