use std::collections::HashMap;
use std::sync::RwLock;
use lazy_static::lazy_static;


lazy_static! {
    static ref _KV : RwLock<HashMap<String, ContainerState>> = RwLock::new(HashMap::new());
}

#[derive(Eq, PartialEq)]
pub enum ContainerState {
    Running,
    ToBeDeleted,
}

pub struct KV;

impl KV {
    pub fn update_container_state(container_id: &str, container_state : ContainerState) {
        _KV.write().expect("kv write").insert(container_id.to_string(), container_state);
    }

    pub fn mark_container_to_be_deleted(container_id: &str) {
        Self::update_container_state(container_id, ContainerState::ToBeDeleted);
    }

    pub fn is_container_marked_to_be_deleted(container_id: &str) -> bool {
        _KV.read().expect("could not read kv").get(container_id) == Some(&ContainerState::ToBeDeleted)
    }
}

