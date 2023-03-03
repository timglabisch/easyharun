use crate::brain::brain_action::{BrainAction, ContainerStart, ContainerStop};
use crate::container_manager::world::{WorldDiff, Worlds};

pub struct Brain {}

impl Brain {
    pub fn think_about_next_action(worlds : &Worlds) -> Result<BrainAction, ::anyhow::Error> {

        let world_diff = worlds.build_diff_world();

        match Self::think_about_starting_new_containers(&world_diff)? {
            Some(s) => return Ok(s),
            None => {},
        }

        match Self::think_about_stopping_existing_containers(&world_diff)? {
            Some(s) => return Ok(s),
            None => {},
        }

        Ok(BrainAction::NoOp)
    }

    pub fn think_about_starting_new_containers(world_diff : &WorldDiff) -> Result<Option<BrainAction>, ::anyhow::Error> {
        match world_diff.containers_does_not_exists_but_should_exists().first() {
            None => Ok(None),
            Some(s) => Ok(Some(BrainAction::ContainersStart(vec![
                ContainerStart::new_from_world_container(s)
            ])))
        }
    }

    pub fn think_about_stopping_existing_containers(world_diff : &WorldDiff) -> Result<Option<BrainAction>, ::anyhow::Error> {
        match world_diff.containers_exists_but_should_not_exists().first() {
            None => Ok(None),
            Some(s) => Ok(Some(BrainAction::ContainersStop(vec![
                ContainerStop::new_from_world_container(s)?
            ])))
        }
    }
}