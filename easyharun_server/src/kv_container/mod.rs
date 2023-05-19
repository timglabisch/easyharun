use std::collections::HashMap;
use std::sync::RwLock;
use lazy_static::lazy_static;
use easyharun_lib::ContainerId;


lazy_static! {
    static ref _KV : RwLock<HashMap<String, ContainerState>> = RwLock::new(HashMap::new());
}

#[derive(Eq, PartialEq)]
pub struct ContainerState {
    should_be_deleted: bool,
    healthy: bool,
}

impl ContainerState {
    pub fn new_default() -> Self {
        Self {
            should_be_deleted: false,
            healthy: false,
        }
    }
}

pub struct KV;

impl KV {

    pub fn mark_container_healthy(container_id: &ContainerId, healthy : bool) {
        {
            let read = _KV.read().expect("could not read kv");

            match read.get(container_id.as_str()) {
                Some(s) if s.healthy == healthy => {
                    return; // << has already the right state.
                },
                _ => false,
            };
        }

        // just do a write lock, if we need to change it.
        _KV.write().expect("kv write").entry(container_id.as_str().to_string()).or_insert(ContainerState::new_default()).healthy = healthy;
    }

    pub fn mark_container_to_be_deleted(container_id: &ContainerId) {
        _KV.write().expect("kv write").entry(container_id.as_str().to_string()).or_insert(ContainerState::new_default()).should_be_deleted = true;
    }

    pub fn is_container_marked_to_be_deleted(container_id: &ContainerId) -> bool {
        let read = _KV.read().expect("could not read kv");

        match read.get(container_id.as_str()) {
            Some(s) if s.should_be_deleted == true => true,
            _ => false,
        }
    }
}

