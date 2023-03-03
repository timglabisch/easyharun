use std::collections::HashMap;
use std::sync::RwLock;
use lazy_static::lazy_static;


lazy_static! {
    static ref _KV : RwLock<HashMap<String, ContainerState>> = RwLock::new(HashMap::new());
}

#[derive(Eq, PartialEq)]
pub struct ContainerState {
    should_be_deleted: bool,
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

    pub fn mark_container_to_be_deleted(container_id: &str) {
        _KV.write().expect("kv write").entry(container_id.to_string()).or_insert(ContainerState::new_default()).should_be_deleted = true;
    }

    pub fn is_container_marked_to_be_deleted(container_id: &str) -> bool {
        let read = _KV.read().expect("could not read kv");

        match read.get(container_id) {
            Some(s) if s.should_be_deleted == true => true,
            _ => false,
        }
    }
}

