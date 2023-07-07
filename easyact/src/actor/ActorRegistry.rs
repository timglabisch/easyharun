use std::collections::hash_map::Entry::Vacant;
use std::collections::hash_map::Entry::Occupied;
use std::collections::HashMap;
use std::fmt::{Debug};
use std::sync::atomic::{Ordering};
use async_trait::async_trait;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::oneshot::error::RecvError;
use tokio::task::JoinHandle;
use tracing::warn;
use crate::actor::Actor::{Actor, ActorConfig, ActorId, ActorMsg, ActorState, ActorStateHandle};

pub struct ActorRegistry {
    inner: ActorStateHandle<ActorRegistryMsg>,
}

impl ActorRegistry {
    pub fn spawn_new() -> (JoinHandle<()>, ActorRegistry) {
        let (jh, handle, _) = Actor::spawn(ActorConfig::new("Registry", "Registry").build(), |actor_state| ActorRegistryActor {
            actors: HashMap::new(),
            actor_state
        });

        (jh, ActorRegistry {inner : handle})
    }

    pub fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }

    pub async fn get_running_actors(&self) -> Result<HashMap<ActorId, ActorRegistryEntry>, ::anyhow::Error> {

        let (s, r) = ::tokio::sync::oneshot::channel();

        self.send(ActorRegistryMsg::GetRunningActors(ActorRegistryMsgGetRunningActors {
            shot: s,
        })).await?;

        Ok(r.await?)
    }

    pub async fn send(&self, msg : ActorRegistryMsg) -> Result<(), SendError<ActorMsg<ActorRegistryMsg>>> {
        self.inner.send(msg).await
    }
}

pub struct ActorRegistryActor {
    actors: HashMap<ActorId, ActorRegistryEntry>,
    actor_state: ActorState<ActorRegistryMsg>
}

#[derive(Debug, Clone)]
pub struct ActorRegistryEntry {
    actor_id: ActorId,
    actor_name: String,
    actor_type: String,
}

#[derive(Debug)]
pub struct ActorRegistryMsgRegister {
    pub actor_id: ActorId,
    pub actor_name: String,
    pub actor_type: String,
}

#[derive(Debug)]
pub struct ActorRegistryMsgUnregister {
    pub actor_id: ActorId,
}

#[derive(Debug)]
pub struct ActorRegistryMsgGetRunningActors {
    shot: ::tokio::sync::oneshot::Sender<HashMap<ActorId, ActorRegistryEntry>>
}

#[derive(Debug)]
pub enum ActorRegistryMsg {
    Register(ActorRegistryMsgRegister),
    Unregister(ActorRegistryMsgUnregister),
    GetRunningActors(ActorRegistryMsgGetRunningActors),
}

#[async_trait]
impl Actor for ActorRegistryActor {
    type MSG = ActorRegistryMsg;

    fn get_actor_state(&mut self) -> &mut ActorState<Self::MSG> {
        &mut self.actor_state
    }

    async fn on_msg(&mut self, msg: Self::MSG) -> Result<(), ::anyhow::Error> {
        match msg {
            ActorRegistryMsg::Register(entry) => self.on_msg_register(entry),
            ActorRegistryMsg::Unregister(entry) => self.on_msg_unregister(entry),
            ActorRegistryMsg::GetRunningActors(entry) => self.on_msg_get_running_actors(entry),
        };

        Ok(())
    }
}

impl ActorRegistryActor {

    fn on_msg_get_running_actors(&mut self, msg: ActorRegistryMsgGetRunningActors) {
        msg.shot.send(self.actors.clone());
    }

    fn on_msg_register(&mut self, msg: ActorRegistryMsgRegister) {
        match self.actors.entry(msg.actor_id.clone()) {
            Occupied(mut o) => {
                warn!("could not reregister actor {:?}", msg)
            }
            Vacant(o) => {
                o.insert(ActorRegistryEntry {
                    actor_id: msg.actor_id.clone(),
                    actor_name: msg.actor_name,
                    actor_type: msg.actor_type,
                });
            }
        }
    }

    fn on_msg_unregister(&mut self, msg: ActorRegistryMsgUnregister) {
        match self.actors.entry(msg.actor_id.clone()) {
            Occupied(mut o) => {
                o.remove();
            }
            Vacant(o) => {
                warn!("could not remove actor {:?}", msg)
            }
        }
    }
}