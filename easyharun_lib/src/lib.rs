pub mod config;
pub mod portmapping;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct ContainerId {
    id: String,
}

impl ContainerId {
    pub fn new(id: String) -> Self {
        Self {
            id
        }
    }

    pub fn as_str(&self) -> &str {
        self.id.as_str()
    }
}