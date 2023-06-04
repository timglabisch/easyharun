#![allow(dead_code)]
#![allow(unused_variables)]

use futures::future::join;
use tokio::task::JoinHandle;
use crate::actor::wrapper::{Actor, ActorState, ActorStateHandle};

pub mod actor;

struct ActorA {
    actor_state: ActorState<String>
}

impl ActorA {
    pub fn spawn() -> (JoinHandle<()>, ActorStateHandle<String>) {

        let (handle, actor_state) = ActorState::new_root("ActorA");

        let jh = ::tokio::spawn(async move {
            Self {
                actor_state
            }.run_loop().await
        });

        (jh, handle)
    }
}

impl Actor for ActorA {
    type MSG = String;

    fn get_actor_state(&mut self) -> &mut ActorState<Self::MSG> {
        &mut self.actor_state
    }
}

#[tokio::main]
pub async fn main() {
    let (jh_1, state_a) = ActorA::spawn();
    let (jh_2, state_b) = ActorA::spawn();

    join(jh_1, jh_2).await;
}
 