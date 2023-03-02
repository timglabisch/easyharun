use std::collections::HashMap;
use easyharun_lib::config::ConfigContainer;

pub struct World {
    containers: HashMap<String, WorldContainer>
}

pub struct WorldContainer {
    config_container: ConfigContainer
}