use std::collections::HashMap;
use easyharun_lib::config::ConfigContainer;

#[derive(Debug, Clone)]
pub struct World {
    pub containers: Vec<WorldContainer>
}

#[derive(Debug, Clone)]
pub struct WorldContainer {
    pub id: Option<String>,
    pub name: String,
    pub image: String,
    pub version: String,
}