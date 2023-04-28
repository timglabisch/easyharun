use std::collections::{HashSet};
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

        let mut ids = HashSet::new();
        for current_container in self.current.containers.iter() {

            if ids.contains(&current_container.get_internal_id()) {
                continue;
            }

            for expected_container in self.expected.containers.iter() {

                if ids.contains(&expected_container.get_internal_id()) {
                    continue;
                }

                if Self::container_statisfies_container(current_container, expected_container) {
                    ids.insert(current_container.get_internal_id());
                    ids.insert(expected_container.get_internal_id());
                    break;
                }
            }
        }

        let mut containers_exists_but_should_not_exists = vec![];
        for current_container in self.current.containers.iter() {
            if ids.contains(&current_container.get_internal_id()) {
                continue;
            }

            containers_exists_but_should_not_exists.push(current_container.clone());
        }

        let mut containers_does_not_exists_but_should_exists = vec![];
        for expected_container in self.expected.containers.iter() {
            if ids.contains(&expected_container.get_internal_id()) {
                continue;
            }

            containers_does_not_exists_but_should_exists.push(expected_container.clone());
        }

        WorldDiff {
            containers_does_not_exists_but_should_exists,
            containers_exists_but_should_not_exists
        }
    }

    fn container_statisfies_container(container_a : &WorldContainer, container_b : &WorldContainer) -> bool {
        if container_a.container_port != container_b.container_port {
            return false;
        }

        if container_a.image != container_b.image {
            return false;
        }

        true
    }
}

#[derive(Debug, Clone)]
pub struct World {
    containers: Vec<WorldContainer>
}

impl World {
    pub fn new(containers: Vec<WorldContainer>, unique_world_name: &str) -> Self {

        let containers = containers.into_iter().enumerate().map(|(id, mut world_container)| {
            world_container.internal_id = Some(format!("{}_{}", unique_world_name, id));
            world_container
        }).collect();

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
    pub internal_id: Option<String>,
    pub id: Option<ContainerId>,
    pub image: String,
    pub container_port: u32,
    pub host_port: u32,
}

impl WorldContainer {
    pub fn get_internal_id(&self) -> String {
        self.internal_id.as_ref().expect("internal id must be given ...").as_str().to_string()
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
            ], "1"),
            current: World::new(vec![
                WorldContainer {
                    container_port: 80,
                    image: "foo:latest".to_string(),
                    ..Default::default()
                }
            ], "2"),
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
            ], "1"),
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
            ], "2"),
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
            ], "1"),
            current: World::new(vec![
                WorldContainer {
                    container_port: 80,
                    image: "foo:latest".to_string(),
                    ..Default::default()
                }
            ], "2"),
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
            ], "1"),
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
            ], "2"),
        };

        let diff = worlds.build_diff_world();

        assert_eq!(1, diff.containers_exists_but_should_not_exists.len());
        assert_eq!(0, diff.containers_does_not_exists_but_should_exists.len());
    }
}