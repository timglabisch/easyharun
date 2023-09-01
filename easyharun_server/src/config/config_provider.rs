use std::sync::Arc;
use tokio::sync::{RwLock};
use easyharun_lib::config::Config;

pub struct ConfigProvider;

impl ConfigProvider {
    pub fn new(config: Config) -> (ConfigReader, ConfigReaderWriter) {
        let arc = Arc::new(RwLock::new(config));

        (
            ConfigReader {config: arc.clone()},
            ConfigReaderWriter {config: arc},
        )
    }
}

#[derive(Clone, Debug)]
pub struct ConfigReader {
    config: Arc<RwLock<Config>>,
}

#[derive(Clone, Debug)]
pub struct ConfigReaderWriter {
    config: Arc<RwLock<Config>>,
}

impl ConfigReader {
    pub async fn get_copy(&self) -> Config {
        self.config.read().await.clone()
    }
}

impl ConfigReaderWriter {
    pub async fn get_copy(&self) -> Config {
        self.config.read().await.clone()
    }

    pub async fn set(&self, config : Config) {
        let mut w = self.config.write().await;
        *w = config;
    }
}