use std::collections::{HashMap, HashSet};
use easyharun_lib::ContainerId;


#[derive(Debug, Clone)]
pub struct Worlds {
    pub current: World,
    pub expected: World,
}

pub struct WorldDiff {
    containers_exists_but_should_not_exists: Vec<WorldContainer>,
    containers_does_not_exists_but_should_exists: Vec<WorldContainer>,
}

impl WorldDiff {
    pub fn containers_exists_but_should_not_exists(&self) -> &Vec<WorldContainer> {
        &self.containers_exists_but_should_not_exists
    }
    pub fn containers_does_not_exists_but_should_exists(&self) -> &Vec<WorldContainer> {
        &self.containers_does_not_exists_but_should_exists
    }
}

impl Worlds {
    pub fn build_diff_world(&self) -> WorldDiff {
        let current_map = {
            let mut map = HashMap::new();
            for c in &self.current.containers {
                map.insert(c.get_identifier(), c.clone());
            }
            map
        };

        let expected_map = {
            let mut map = HashMap::new();
            for c in &self.expected.containers {
                map.insert(c.get_identifier(), c.clone());
            }
            map
        };

        let mut containers_exists_but_should_not_exists = vec![];

        for (container_identifier, container) in &current_map {
            if !expected_map.contains_key(container_identifier) {
                containers_exists_but_should_not_exists.push(container.clone());
            }
        }

        let mut containers_does_not_exists_but_should_exists = vec![];

        for (container_identifier, container) in &expected_map {
            if !current_map.contains_key(container_identifier) {
                containers_does_not_exists_but_should_exists.push(container.clone());
            }
        }

        WorldDiff {
            containers_does_not_exists_but_should_exists,
            containers_exists_but_should_not_exists
        }
    }
}

#[derive(Debug, Clone)]
pub struct World {
    containers: Vec<WorldContainer>
}

impl World {
    pub fn new(containers: Vec<WorldContainer>) -> Self {
        Self {
            containers
        }
    }

    pub fn get_containers(&self) -> &Vec<WorldContainer> {
        &self.containers
    }
}

#[derive(Debug, Clone, Default)]
pub struct WorldContainer {
    pub container_id: Option<ContainerId>,
    pub name: String,
    pub image: String,
    pub replica_id: u32,
    pub container_port: u32,
    pub health_checks: Vec<String>,
    pub proxies: Vec<String>,
}

impl WorldContainer {
    pub fn get_identifier(&self) -> String {
        format!("{}|{}|{}|{}", self.name, self.image, self.replica_id, self.container_port)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn build_world_diff_all_are_same() {

        let worlds = Worlds {
            expected: World::new(vec![
                WorldContainer {
                    container_port: 80,
                    image: "foo:latest".to_string(),
                    ..Default::default()
                }
            ]),
            current: World::new(vec![
                WorldContainer {
                    container_port: 80,
                    image: "foo:latest".to_string(),
                    ..Default::default()
                }
            ]),
        };

        let diff = worlds.build_diff_world();

        assert_eq!(0, diff.containers_exists_but_should_not_exists.len());
        assert_eq!(0, diff.containers_does_not_exists_but_should_exists.len());
    }

    #[test]
    fn build_world_diff_all_are_same_multiple() {

        let worlds = Worlds {
            expected: World::new(vec![
                WorldContainer {
                    container_port: 80,
                    image: "foo:latest".to_string(),
                    ..Default::default()
                },
                WorldContainer {
                    container_port: 80,
                    image: "foo:latest".to_string(),
                    ..Default::default()
                }
            ]),
            current: World::new(vec![
                WorldContainer {
                    container_port: 80,
                    image: "foo:latest".to_string(),
                    ..Default::default()
                },
                WorldContainer {
                    container_port: 80,
                    image: "foo:latest".to_string(),
                    ..Default::default()
                }
            ]),
        };

        let diff = worlds.build_diff_world();

        assert_eq!(0, diff.containers_exists_but_should_not_exists.len());
        assert_eq!(0, diff.containers_does_not_exists_but_should_exists.len());
    }

    #[test]
    fn build_world_diff_one_container_is_missing() {

        let worlds = Worlds {
            expected: World::new(vec![
                WorldContainer {
                    container_port: 80,
                    image: "foo:latest".to_string(),
                    ..Default::default()
                },
                WorldContainer {
                    container_port: 80,
                    image: "foo:latest".to_string(),
                    ..Default::default()
                }
            ]),
            current: World::new(vec![
                WorldContainer {
                    container_port: 80,
                    image: "foo:latest".to_string(),
                    ..Default::default()
                }
            ]),
        };

        let diff = worlds.build_diff_world();

        assert_eq!(0, diff.containers_exists_but_should_not_exists.len());
        assert_eq!(1, diff.containers_does_not_exists_but_should_exists.len());
    }


    #[test]
    fn build_world_diff_one_container_too_much() {

        let worlds = Worlds {
            expected: World::new(vec![
                WorldContainer {
                    container_port: 80,
                    image: "foo:latest".to_string(),
                    ..Default::default()
                },
            ]),
            current: World::new(vec![
                WorldContainer {
                    container_port: 80,
                    image: "foo:latest".to_string(),
                    ..Default::default()
                },
                WorldContainer {
                    container_port: 80,
                    image: "foo:latest".to_string(),
                    ..Default::default()
                }
            ]),
        };

        let diff = worlds.build_diff_world();

        assert_eq!(1, diff.containers_exists_but_should_not_exists.len());
        assert_eq!(0, diff.containers_does_not_exists_but_should_exists.len());
    }
}