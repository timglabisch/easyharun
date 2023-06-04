use std::fmt::{Debug, Display, Formatter};
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use async_trait::async_trait;
use futures::select;
use pin_project_lite::pin_project;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::channel;
use tokio::sync::oneshot::{Sender};
use tokio_util::sync::CancellationToken;
use tracing::warn;

const ACTOR_ID_GEN : AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone)]
pub struct ActorId(pub u64);

impl Display for ActorId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_str())
    }
}

#[derive(Debug)]
pub struct ActorMsgPing {
    actor_id: ActorId,
    shot: Sender<ActorMsgPingResponse>
}

#[derive(Debug)]
pub struct ActorMsgPingResponse {
    actor_id: ActorId,
}

#[derive(Debug)]
enum ActorMsg<MSG> {
    Msg(MSG),
    Ping(ActorMsgPing),
}

#[derive(Debug, Default)]
pub struct ActorStateMetrics {
    msg_msg: u64,
    msg_ping: u64,
    errors: u64,
}


pub struct ActorState<MSG> where MSG: Send, MSG : Sync, MSG: Sized, MSG: Unpin {
    id: ActorId,
    name: &'static str,
    parent_cancellation_token: Option<CancellationToken>,
    cancellation_token: CancellationToken,
    inbox: Receiver<ActorMsg<MSG>>,
    metrics: ActorStateMetrics,
}

pub struct ActorStateHandle<MSG> where MSG: Send, MSG : Sync, MSG: Sized, MSG: Unpin {
    id: ActorId,
    sender: ::tokio::sync::mpsc::Sender<ActorMsg<MSG>>
}

impl<MSG> ActorState<MSG> where MSG: Send, MSG : Sync, MSG: Sized, MSG: Unpin {

    pub fn new_root(name: &'static str) -> (ActorStateHandle<MSG>, Self) {
        Self::new(name, None)
    }

    pub fn new_child(name: &'static str, parent_cancellation_token: CancellationToken) -> (ActorStateHandle<MSG>, Self) {
        Self::new(name, Some(parent_cancellation_token))
    }

    fn new(name: &'static str, parent_cancellation_token: Option<CancellationToken>) -> (ActorStateHandle<MSG>, Self) {

        let (inbox_sender, inbox) = channel::<ActorMsg<MSG>>(1000);

        let id = ActorId(ACTOR_ID_GEN.fetch_add(1, Ordering::Relaxed));

        let s = Self {
            id: id.clone(),
            cancellation_token: CancellationToken::new(),
            metrics: ActorStateMetrics::default(),
            inbox,
            parent_cancellation_token: None,
            name
        };

        (
            ActorStateHandle {
                id: id,
                sender: inbox_sender
            },
            s
        )
    }
}


#[async_trait]
pub trait Actor{
    type MSG: Send + Sync + Sized + Unpin + Debug;

    fn get_actor_state(&mut self) -> &mut ActorState<Self::MSG>;

    async fn run_loop_inner(&mut self) -> Result<(), ::anyhow::Error> {

        let mut state = self.get_actor_state();

        loop {
            ::tokio::select! {
                msg = state.inbox.recv() => {
                    println!("msg: {:#?}", msg);
                },
                _ = async { state.parent_cancellation_token.as_mut().expect("crash here").cancelled().await }, if state.parent_cancellation_token.is_some() => {
                    println!("parent cancelled. we kill ourself");
                    return Ok(());
                }
            }
        }
    }

    async fn run_loop(&mut self) {

        loop {
            match self.run_loop_inner().await {
                Ok(_) => return,
                Err(e) => {
                    self.get_actor_state().metrics.errors += 1;
                    warn!("Actor Error {}", self.get_actor_state().name);
                }
            };

            ::tokio::time::sleep(::tokio::time::Duration::from_millis(500)).await;
        }

    }


}