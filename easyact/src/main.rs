#![allow(dead_code)]
#![allow(unused_variables)]


use anyhow::Error;
use futures::future::{join3};


use async_trait::async_trait;
use easyact::actor::ActorTask::{ActorTask, ActorTaskConfig, ActorTaskState};
use crate::actor::Actor::{Actor, ActorConfig, ActorState};
use crate::actor::ActorRegistry::{ActorRegistry};
use crate::actor::CancellationTokenRegistry::CancellationTokenRegistry;

pub mod actor;
pub mod proto;

struct ActorA {
    actor_state: ActorState<String>,
}

struct ActorTaskA {
    actor_state: ActorTaskState,
}


#[async_trait]
impl ActorTask for ActorTaskA {
    type RES = ();

    fn get_actor_state(&mut self) -> &mut ActorTaskState { &mut self.actor_state }

    async fn run(&mut self) -> Result<Self::RES, Error> {
        return Ok(());
    }
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

    let (registry_jh, registry_actor) = ActorRegistry::spawn_new();
    registry_actor.register_as_default();

    let (a, registry_cancellation) = CancellationTokenRegistry::spawn_new();

    let token = registry_cancellation.create_or_get_token("foo").await;


    let (jh_1, handle_a, ready_1) = Actor::spawn(ActorConfig::new("Actor A", "Foo").build(), |actor_state| ActorA { actor_state });
    let (jh_2, handle_b, ready_2) = Actor::spawn(ActorConfig::new("Actor B", "Foo").cancel_on(&handle_a).build(), |actor_state| ActorA { actor_state });

    ActorTask::spawn(ActorTaskConfig::new("Task A", "Group!").build(), |actor_state| ActorTaskA { actor_state });


    //println!("{:#?}", );

    // handle_a.shutdown().await?.await;

    ready_1.await;
    ready_2.await;

    // ::tokio::time::sleep(Duration::from_secs(1)).await;


    println!("{:#?}", registry_actor.get_running_actors().await);

    // handle_a.

    join3(jh_1, jh_2, proto::actor_run_grpc_server("0.0.0.0:50051", registry_actor)).await;

    Ok(())
}
