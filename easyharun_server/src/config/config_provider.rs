use std::sync::{Arc, RwLock};
use lazy_static::lazy_static;
use easyharun_lib::config::Config;

lazy_static! {
    static ref CONFIG : RwLock<Option<Config>> = RwLock::new(None);
}

pub fn config_set(config : Config) {
    let mut lock = CONFIG.write().expect("could not read config lock");
    *lock = Some(config);
}

pub fn config_get() -> Config {
    CONFIG.read().expect("could not read config lock").as_ref().expect("could not read config").clone()
}