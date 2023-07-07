use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use async_trait::async_trait;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use crate::actor::Actor::{Actor, ActorConfig, ActorConfigBuilder, ActorState, ActorStateHandle};

#[derive(Debug)]
pub struct ActorCancellationTokenMsgGetOrCreate {
    obj: Vec<u8>,
    shot: ::tokio::sync::oneshot::Sender<CancellationToken>,
}

#[derive(Debug)]
pub struct ActorCancellationTokenMsgDelete {
    obj: Vec<u8>,
    shot: ::tokio::sync::oneshot::Sender<Option<CancellationToken>>,
}

#[derive(Debug)]
pub enum ActorCancellationTokenMsg {
    GetOrCreate(ActorCancellationTokenMsgGetOrCreate),
    Delete(ActorCancellationTokenMsgDelete)
}

#[derive(Debug)]
pub struct CancellationTokenRegistryActor {
    actor_state: ActorState<ActorCancellationTokenMsg>,
    tokens: HashMap<Vec<u8>, CancellationToken>
}

pub struct CancellationTokenRegistry {
    inner: ActorStateHandle<ActorCancellationTokenMsg>,
}

pub trait CancellationTokenIdent {
    fn to_cancellation_token_ident(&self) -> String;
}

impl<T> CancellationTokenIdent for T where T: AsRef<str> {
    fn to_cancellation_token_ident(&self) -> String {
        format!("_string_{}", self.as_ref())
    }
}

impl CancellationTokenRegistry {
    pub fn spawn_new() -> (JoinHandle<()>, Self) {
        let (jh, handle, _) = Actor::spawn(ActorConfig::new("Registry Cancellation", "Registry").build(), |actor_state| CancellationTokenRegistryActor {
            tokens: HashMap::new(),
            actor_state
        });

        (jh, Self {inner : handle})
    }

    pub async fn create_or_get_token<T>(&self, obj : T) -> Result<CancellationToken, ::anyhow::Error> where T: CancellationTokenIdent {
        let ident = obj.to_cancellation_token_ident();
        let (s, r) = ::tokio::sync::oneshot::channel();

        self.inner.send(ActorCancellationTokenMsg::GetOrCreate(ActorCancellationTokenMsgGetOrCreate {
            obj: ident.into_bytes(),
            shot: s,
        })).await?;

        Ok(r.await?)
    }
}


#[async_trait]
impl Actor for CancellationTokenRegistryActor {
    type MSG = ActorCancellationTokenMsg;

    fn get_actor_state(&mut self) -> &mut ActorState<Self::MSG> {
        &mut self.actor_state
    }

    async fn on_msg(&mut self, msg: Self::MSG) -> Result<(), ::anyhow::Error> {
        match msg {
            ActorCancellationTokenMsg::GetOrCreate(entry) => {
                let token = match self.tokens.entry(entry.obj) {
                    Entry::Occupied(e) => {
                        e.get().clone()
                    },
                    Entry::Vacant(e) => {
                        let token = CancellationToken::new();
                        e.insert(token.clone());
                        token
                    }
                };

                match entry.shot.send(token) {
                    Ok(_) => {},
                    Err(_) => {},
                }
            },
            ActorCancellationTokenMsg::Delete(entry) => {
                let token = match self.tokens.entry(entry.obj) {
                    Entry::Occupied(e) => Some(e.remove()),
                    Entry::Vacant(e) => None
                };

                match entry.shot.send(token) {
                    Ok(_) => {},
                    Err(_) => {},
                }
            }
        };

        Ok(())
    }
}