

use std::collections::hash_map::Entry::Vacant;
use std::collections::hash_map::Entry::Occupied;
use std::collections::HashMap;
use std::fmt::{Debug};

use std::sync::OnceLock;
use async_trait::async_trait;

use tokio::sync::mpsc::error::SendError;

use tokio::task::JoinHandle;
use tracing::warn;
use crate::actor::Actor::{Actor, ActorConfig, ActorId, ActorMsg, ActorState, ActorStateHandle, ActorStateHandleManageable};

pub static DEFAULT_ACTOR_REGISTRY: OnceLock<ActorRegistry> = OnceLock::new();


#[derive(Debug)]
pub struct ActorRegistry {
    inner: ActorStateHandle<ActorRegistryMsg>,
}

impl Clone for ActorRegistry {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl ActorRegistry {
    pub fn spawn_new() -> (JoinHandle<()>, ActorRegistry) {
        let (jh, handle, _) = Actor::spawn(ActorConfig::new("Registry", "Registry").build(), |actor_state| ActorRegistryActor {
            actors: HashMap::new(),
            actor_state
        });

        (jh, ActorRegistry {inner : handle})
    }

    pub fn register_as_default(&self) {
        DEFAULT_ACTOR_REGISTRY.set(self.clone()).expect("could not register actor registry as default");
    }

    pub async fn get_running_actors(&self) -> Result<Vec<ActorRegistryMsgGetRunningActorsEntry>, ::anyhow::Error> {

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

#[derive(Debug)]
pub struct ActorRegistryEntry {
    pub actor_id: ActorId,
    pub actor_name: String,
    pub actor_type: String,
    pub actor_handle_manage: Box<dyn ActorStateHandleManageable + 'static + Send + Sync>
}

#[derive(Debug)]
pub struct ActorRegistryMsgGetRunningActorsEntry {
    pub actor_id: ActorId,
    pub actor_name: String,
    pub actor_type: String,
}

#[derive(Debug)]
pub struct ActorRegistryMsgRegister {
    pub actor_id: ActorId,
    pub actor_name: String,
    pub actor_type: String,
    pub actor_handle_manage: Box<dyn ActorStateHandleManageable + 'static + Send + Sync>
}

#[derive(Debug)]
pub struct ActorRegistryMsgUnregister {
    pub actor_id: ActorId,
}

#[derive(Debug)]
pub struct ActorRegistryMsgGetRunningActors {
    shot: ::tokio::sync::oneshot::Sender<Vec<ActorRegistryMsgGetRunningActorsEntry>>
}

#[derive(Debug)]
pub struct ActorRegistryMsgShutdown {
    actor_id: ActorId,
}

#[derive(Debug)]
pub enum ActorRegistryMsg {
    Register(ActorRegistryMsgRegister),
    Unregister(ActorRegistryMsgUnregister),
    GetRunningActors(ActorRegistryMsgGetRunningActors),
    Shutdown(ActorRegistryMsgShutdown)
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
            ActorRegistryMsg::Shutdown(entry) => self.on_msg_shutdown(entry).await,
        };

        Ok(())
    }
}

impl ActorRegistryActor {

    fn on_msg_get_running_actors(&mut self, msg: ActorRegistryMsgGetRunningActors) {
        msg.shot.send(self.actors.iter().map(|(_, v)| ActorRegistryMsgGetRunningActorsEntry {
            actor_id: v.actor_id.clone(),
            actor_name: v.actor_name.clone(),
            actor_type: v.actor_type.clone(),
        }).collect::<Vec<_>>());
    }

    async fn on_msg_shutdown(&mut self, msg: ActorRegistryMsgShutdown) {
        let actor = match self.actors.iter().find(|(id, _)| id.0 == msg.actor_id.0) {
            None => return,
            Some((_id, entry)) => entry,
        };

        actor.actor_handle_manage.shutdown().await;
    }

    fn on_msg_register(&mut self, msg: ActorRegistryMsgRegister) {
        match self.actors.entry(msg.actor_id.clone()) {
            Occupied(_o) => {
                warn!("could not reregister actor {:?}", msg)
            }
            Vacant(o) => {
                o.insert(ActorRegistryEntry {
                    actor_id: msg.actor_id.clone(),
                    actor_name: msg.actor_name,
                    actor_type: msg.actor_type,
                    actor_handle_manage: msg.actor_handle_manage
                });
            }
        }
    }

    fn on_msg_unregister(&mut self, msg: ActorRegistryMsgUnregister) {
        match self.actors.entry(msg.actor_id.clone()) {
            Occupied(o) => {
                o.remove();
            }
            Vacant(_o) => {
                warn!("could not remove actor {:?}", msg)
            }
        }
    }
}