#![allow(dead_code)]
#![allow(unused_variables)]

use futures::future::join;
use tokio::task::JoinHandle;
use tracing::Instrument;
use crate::actor::wrapper::{Actor, ActorState, ActorStateHandle};

pub mod actor;

struct ActorA {
    actor_state: ActorState<String>
}


impl Actor for ActorA {
    type MSG = String;

    fn get_actor_state(&mut self) -> &mut ActorState<Self::MSG> {
        &mut self.actor_state
    }
}

#[tokio::main]
pub async fn main() {

    console_subscriber::init();

    let (jh_1, state_a) = Actor::spawn_as_actor(|actor_state| ActorA {actor_state} );
    let (jh_2, state_b) = Actor::spawn_as_actor(|actor_state| ActorA {actor_state} );

    join(jh_1, jh_2).await;
}
