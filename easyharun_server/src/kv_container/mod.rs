use std::collections::HashMap;
use std::sync::Arc;
use ::tokio::sync::RwLock;
use easyharun_lib::ContainerId;


#[derive(Eq, PartialEq, Debug)]
pub struct ContainerState {
    should_be_deleted: bool,
}

#[derive(Eq, PartialEq, Debug)]
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

#[derive(Clone, Debug)]
pub struct KV {
    container: Arc<RwLock<HashMap<String, ContainerState>>>,
    health: Arc<RwLock<HashMap<String, HealthState>>>,
}

impl KV {

    pub fn new() -> KV {
        Self {
            container: Arc::new(RwLock::new(HashMap::new())),
            health: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub async fn mark_container_target_healthy(&self, container_id: &ContainerId, target: &str, healthy : bool) {
        if self.is_target_healthy(target).await {
            return;
        }

        // just do a write lock, if we need to change it.
        self.health.write().await.entry(container_id.as_str().to_string()).or_insert_with(|| HealthState {
            container: Some(container_id.clone()),
            target: target.to_string(),
            healthy,
        }).healthy = healthy;
    }

    pub async fn mark_container_to_be_deleted(&self, container_id: &ContainerId) {
        self.container.write().await.entry(container_id.as_str().to_string()).or_insert(ContainerState::new_default()).should_be_deleted = true;
    }

    pub async fn is_target_healthy(&self, container_target: &str) -> bool {
        let read = self.health.read().await;

        match read.get(container_target) {
            Some(s) =>  s.healthy, // << has already the right state.
            _ => false,
        }
    }

    pub async fn is_container_marked_to_be_deleted(&self, container_id: &ContainerId) -> bool {
        let read = self.container.read().await;

        match read.get(container_id.as_str()) {
            Some(s) if s.should_be_deleted == true => true,
            _ => false,
        }
    }
}

