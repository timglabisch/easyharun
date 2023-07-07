use std::fmt::{Debug, Display, Formatter};
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use async_trait::async_trait;
use futures::select;
use pin_project_lite::pin_project;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::oneshot::{Sender};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};
use crate::actor::ActorRegistry::{ActorRegistry, ActorRegistryMsg, ActorRegistryMsgRegister, ActorRegistryMsgUnregister};

static ACTOR_ID_GEN : AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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
pub struct ActorMsgShutdown {
    notify: Option<Sender<()>>,
}

#[derive(Debug)]
pub struct ActorMsgPingResponse {
    actor_id: ActorId,
}

#[derive(Debug)]
pub enum ActorMsg<MSG> {
    Msg(MSG),
    Ping(ActorMsgPing),
    Shutdown(ActorMsgShutdown)
}

#[derive(Debug, Default)]
pub struct ActorStateMetrics {
    msg_msg: u64,
    msg_ping: u64,
    errors: u64,
}


pub struct ActorState<MSG> where MSG: Send, MSG : Sync, MSG: Sized, MSG: Unpin {
    id: ActorId,
    name: String,
    actor_type: String,
    cancellation_tokens_others: Vec<CancellationToken>,
    cancellation_token_self: CancellationToken,
    inbox: Option<Receiver<ActorMsg<MSG>>>,
    metrics: ActorStateMetrics,
    shutdown: bool,
    shutdown_notify: Vec<::tokio::sync::oneshot::Sender<()>>
}

pub struct ActorStateHandle<MSG> where MSG: Send, MSG : Sync, MSG: Sized, MSG: Unpin {
    id: ActorId,
    sender: ::tokio::sync::mpsc::Sender<ActorMsg<MSG>>
}

impl<MSG> ActorStateHandle<MSG> where MSG: Send, MSG : Sync, MSG: Sized, MSG: Unpin {
    pub fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            sender: self.sender.clone(),
        }
    }
}

impl<MSG> ActorStateHandle<MSG> where MSG: Send + 'static, MSG : Sync, MSG: Sized, MSG: Unpin {
    pub async fn send(&self, msg : MSG) -> Result<(), SendError<ActorMsg<MSG>>> {
        self.sender.send(ActorMsg::Msg(msg)).await
    }

    pub async fn ping(&self) -> Result<ActorMsgPingResponse, ::anyhow::Error> where MSG: Debug {
        let (s, r) = ::tokio::sync::oneshot::channel();

        let ping_request = ActorMsgPing {
            actor_id: self.id.clone(),
            shot: s,
        };

        self.sender.send(ActorMsg::Ping(ping_request)).await?;

        Ok(r.await?)
    }

    pub async fn shutdown(&self) -> Result<::tokio::sync::oneshot::Receiver<()>, ::anyhow::Error> where MSG: Debug {

        let (s, r) = ::tokio::sync::oneshot::channel();

        let res = self.sender.send(ActorMsg::Shutdown(ActorMsgShutdown {notify: Some(s)})).await?;

        Ok(r)
    }
}

impl<MSG> ActorState<MSG> where MSG: Send, MSG : Sync, MSG: Sized, MSG: Unpin {

    pub fn new_root(
        name: String,
        actor_type: String,
    ) -> (ActorStateHandle<MSG>, Self) {
        Self::new(name, actor_type, None)
    }

    fn new(
        name: String,
        actor_type: String,
        parent_cancellation_token: Option<CancellationToken>
    ) -> (ActorStateHandle<MSG>, Self) {

        let (inbox_sender, inbox) = channel::<ActorMsg<MSG>>(1000);

        let id = ActorId(ACTOR_ID_GEN.fetch_add(1, Ordering::Relaxed));

        println!("id {}", id);

        let s = Self {
            id: id.clone(),
            cancellation_token_self: CancellationToken::new(),
            metrics: ActorStateMetrics::default(),
            inbox: Some(inbox),
            cancellation_tokens_others: vec![],
            name,
            actor_type,
            shutdown: false,
            shutdown_notify: vec![],
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
pub trait Actor: Sized + Send + Sync + 'static {
    type MSG: Send + Sync + Sized + Unpin + Debug;

    fn get_actor_state(&mut self) -> &mut ActorState<Self::MSG>;

    fn spawn<N, F>(actor_name: N, actor_type: N, registry: Option<ActorRegistry>, func: F) -> (JoinHandle<()>, ActorStateHandle<Self::MSG>, ::tokio::sync::oneshot::Receiver<()>)
        where F: FnOnce(ActorState<Self::MSG>) -> Self,
        N : AsRef<str> {

        let (handle, actor_state) = ActorState::new_root(
            actor_name.as_ref().to_string(),
            actor_type.as_ref().to_string()
        );

        let (ready_shot_s, ready_shot_r) = ::tokio::sync::oneshot::channel();

        let name = format!("Actor {}", actor_state.name);

        let mut this = func(actor_state);
        let jh = ::tokio::spawn(async move {
            if let Some(ref r) = registry {
                match r.send(ActorRegistryMsg::Register(ActorRegistryMsgRegister {
                    actor_id: this.get_actor_state().id.clone(),
                    actor_name: this.get_actor_state().name.clone(),
                    actor_type: this.get_actor_state().actor_type.clone(),
                })).await {
                    Ok(_) => {},
                    Err(e) => {
                        warn!("could not send registry that actor {} was registered", this.get_actor_state().name)
                    }
                };
            };

            match ready_shot_s.send(()) {
                Ok(_) => {},
                Err(_) => {}
            };

            let res = this.run_loop().await;

            if let Some(ref r) = registry {
                match r.send(ActorRegistryMsg::Unregister(ActorRegistryMsgUnregister {
                    actor_id: this.get_actor_state().id.clone(),
                })).await {
                    Ok(_) => {},
                    Err(e) => {
                        warn!("could not send registry that actor {} was unregistered", this.get_actor_state().name)
                    }
                }
            };

            for notify_shutdown in this.get_actor_state().shutdown_notify.drain(..) {
                // we ignore errors for now.
                match notify_shutdown.send(()) {
                    Ok(_) => {},
                    Err(e) => {}
                }
            }

            res
        });

        (jh, handle, ready_shot_r)
    }

    async fn run_loop_inner(&mut self) -> Result<(), ::anyhow::Error> {

        let cancellation_tokens_others = self.get_actor_state().cancellation_tokens_others.clone();
        let mut cancellations = cancellation_tokens_others.iter().map(|v| Box::pin(v.cancelled())).collect::<Vec<_>>();
        let mut inbox = self.get_actor_state().inbox.take().expect("expect inbox");

        loop {
            ::tokio::select! {
                msg = inbox.recv() => {
                    match msg {
                        Some(v) => match self.on_loop_inner_msg(v).await {
                            Ok(_) => {},
                            Err(e) => {
                                eprint!("oh no! {:?}", e);
                            }
                        },
                        None => {}
                    }
                },
                _ = async { ::futures::future::select_all(&mut cancellations).await }, if cancellations.len() > 0 => {
                    println!("parent cancelled. we kill ourself");
                    return Ok(());
                }
            }

            if self.get_actor_state().shutdown {
                println!("Shutdown");
                return Ok(());
            }
        }
    }

    async fn on_loop_inner_msg(&mut self, msg : ActorMsg<Self::MSG>) -> Result<(), ::anyhow::Error> {
        match msg {
            ActorMsg::Msg(msg) => return self.on_msg(msg).await,
            ActorMsg::Ping(ping) => {
                ping.shot.send(ActorMsgPingResponse {
                    actor_id: self.get_actor_state().id.clone(),
                });

                Ok(())
            },
            ActorMsg::Shutdown(msg) => {
                self.get_actor_state().shutdown = true;
                if let Some(notify) = msg.notify {
                    self.get_actor_state().shutdown_notify.push(notify);
                }
                Ok(())
            }
        }
    }

    async fn on_msg(&mut self, msg: Self::MSG) -> Result<(), ::anyhow::Error>;

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