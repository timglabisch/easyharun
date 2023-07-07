#![allow(dead_code)]
#![allow(unused_variables)]

use anyhow::Context;
use futures::future::join;
use tokio::task::JoinHandle;
use tracing::Instrument;
use async_trait::async_trait;
use crate::actor::Actor::{Actor, ActorState, ActorStateHandle};
use crate::actor::ActorRegistry::{ActorRegistry, ActorRegistryActor};

pub mod actor;

struct ActorA {
    actor_state: ActorState<String>
}

#[async_trait]
impl Actor for ActorA {
    type MSG = String;

    fn get_actor_state(&mut self) -> &mut ActorState<Self::MSG> {
        &mut self.actor_state
    }

    async fn on_msg(&mut self, msg: Self::MSG) -> Result<(), ::anyhow::Error> {
        println!("got message");

        Ok(())
    }
}

#[tokio::main]
pub async fn main() -> Result<(), ::anyhow::Error> {

    // console_subscriber::init();

    let (registry_jh, registry) = ActorRegistry::spawn_new();


    let (jh_1, handle_a, ready_1) = Actor::spawn("Actor A", "Foo", Some(registry.clone()),|actor_state| ActorA {actor_state} );
    let (jh_2, handle_b, ready_2) = Actor::spawn("Actor B", "Foo", Some(registry.clone()), |actor_state| ActorA {actor_state} );


    println!("{:#?}", handle_a.shutdown().await?.await);

    ready_1.await;
    ready_2.await;


    println!("{:#?}", registry.get_running_actors().await);

    // handle_a.

    join(jh_1, jh_2).await;

    Ok(())
}
