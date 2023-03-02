use crate::brain::brain_action::BrainAction;
use crate::container_manager::world::Worlds;

pub struct Brain {}

impl Brain {
    pub fn think_about_next_action(worlds : &Worlds) -> Result<BrainAction, ::anyhow::Error> {

        match Self::think_about_starting_new_containers(worlds)? {
            Some(s) => return Ok(s),
            None => {},
        }

        Ok(BrainAction::NoOp)
    }

    pub fn think_about_starting_new_containers(worlds : &Worlds) -> Result<Option<BrainAction>, ::anyhow::Error> {

        for expected_container in worlds.expected.get_containers() {

        }

        Ok(None)
    }
}