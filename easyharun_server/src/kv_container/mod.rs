use std::collections::HashMap;
use std::sync::RwLock;
use lazy_static::lazy_static;
use easyharun_lib::ContainerId;



lazy_static! {
    static ref _KV_CONTAINER : RwLock<HashMap<String, ContainerState>> = RwLock::new(HashMap::new());
    static ref _KV_HEALTH_CHECK : RwLock<HashMap<String, HealthState>> = RwLock::new(HashMap::new());
}

#[derive(Eq, PartialEq)]
pub struct ContainerState {
    should_be_deleted: bool,
}

#[derive(Eq, PartialEq)]
pub struct HealthState {
    container: Option<ContainerId>,
    target: String,
    healthy: bool,
}


impl ContainerState {
    pub fn new_default() -> Self {
        Self {
            should_be_deleted: false,
        }
    }
}

pub struct KV;

impl KV {

    pub fn mark_container_target_healthy(container_id: &ContainerId, target: &str, healthy : bool) {
        if Self::is_target_healthy(target) {
            return;
        }

        // just do a write lock, if we need to change it.
        _KV_HEALTH_CHECK.write().expect("kv write").entry(container_id.as_str().to_string()).or_insert_with(|| HealthState {
            container: Some(container_id.clone()),
            target: target.to_string(),
            healthy,
        }).healthy = healthy;
    }

    pub fn mark_container_to_be_deleted(container_id: &ContainerId) {
        _KV_CONTAINER.write().expect("kv write").entry(container_id.as_str().to_string()).or_insert(ContainerState::new_default()).should_be_deleted = true;
    }

    pub fn is_target_healthy(container_target: &str) -> bool {
        let read = _KV_HEALTH_CHECK.read().expect("could not read kv");

        match read.get(container_target) {
            Some(s) =>  s.healthy, // << has already the right state.
            _ => false,
        }
    }

    pub fn is_container_marked_to_be_deleted(container_id: &ContainerId) -> bool {
        let read = _KV_CONTAINER.read().expect("could not read kv");

        match read.get(container_id.as_str()) {
            Some(s) if s.should_be_deleted == true => true,
            _ => false,
        }
    }
}

