use std::fmt::{Debug, Display, Formatter};

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use anyhow::Error;
use async_trait::async_trait;
use futures::future::OptionFuture;


use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::oneshot::{Sender};
use tokio::task::JoinHandle;

use tokio_util::sync::CancellationToken;
use tracing::{warn};
use crate::actor::ActorRegistry::DEFAULT_ACTOR_REGISTRY;
use crate::actor::ActorRegistry::{ActorRegistry, ActorRegistryMsg, ActorRegistryMsgRegister, ActorRegistryMsgUnregister};
use crate::actor::HasCancellationToken::HasCancellationToken;

pub static ACTOR_ID_GEN : AtomicU64 = AtomicU64::new(0);

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


#[derive(Debug)]
pub struct ActorState<MSG> where MSG: Send, MSG : Sync, MSG: Sized, MSG: Unpin {
    id: ActorId,
    name: String,
    actor_type: String,
    cancellation_tokens_others: Vec<CancellationToken>,
    cancellation_token_self: CancellationToken,
    inbox: Option<Receiver<ActorMsg<MSG>>>,
    inbox_sender: ::tokio::sync::mpsc::Sender<ActorMsg<MSG>>,
    metrics: ActorStateMetrics,
    pub shutdown: bool,
    shutdown_notify: Vec<::tokio::sync::oneshot::Sender<()>>
}

impl<MSG> ActorState<MSG> where MSG: Send, MSG : Sync, MSG: Sized, MSG: Unpin {
    pub fn create_handle(&self) -> ActorStateHandle<MSG> {
        ActorStateHandle {
            id: self.id.clone(),
            sender: self.inbox_sender.clone(),
            cancellation_token_self: self.cancellation_token_self.clone()
        }
    }
}

#[derive(Debug)]
pub struct ActorStateHandle<MSG> where MSG: Send, MSG : Sync, MSG: Sized, MSG: Unpin {
    id: ActorId,
    sender: ::tokio::sync::mpsc::Sender<ActorMsg<MSG>>,
    cancellation_token_self: CancellationToken,
}

impl<MSG> HasCancellationToken for ActorStateHandle<MSG> where MSG: Send, MSG : Sync, MSG: Sized, MSG: Unpin {
    fn get_cancellation_token(&self) -> CancellationToken {
        self.cancellation_token_self.clone()
    }
}

impl<MSG> ActorStateHandle<MSG> where MSG: Send, MSG : Sync, MSG: Sized, MSG: Unpin {
    pub fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            sender: self.sender.clone(),
            cancellation_token_self: self.cancellation_token_self.clone(),
        }
    }
}

#[async_trait]
pub trait ActorStateHandleManageable {
    async fn shutdown(&self) -> Result<::tokio::sync::oneshot::Receiver<()>, ::anyhow::Error>;
}

impl Debug for Box<dyn ActorStateHandleManageable + 'static + Send + Sync> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("ActorStateHandleManageable{}")
    }
}

#[async_trait]
impl<MSG> ActorStateHandleManageable for ActorStateHandle<MSG> where MSG: Debug, MSG: Send + 'static, MSG : Sync, MSG: Sized, MSG: Unpin {
    async fn shutdown(&self) -> Result<tokio::sync::oneshot::Receiver<()>, Error> {
        self.shutdown().await
    }
}

impl<MSG> ActorStateHandle<MSG> where MSG: Send + 'static, MSG : Sync, MSG: Sized, MSG: Unpin {
    pub async fn send(&self, msg : MSG) -> Result<(), SendError<ActorMsg<MSG>>> {
        self.sender.send(ActorMsg::Msg(msg)).await
    }

    pub fn get_cancellation_token(&self) -> CancellationToken {
        self.cancellation_token_self.clone()
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

        let _res = self.sender.send(ActorMsg::Shutdown(ActorMsgShutdown {notify: Some(s)})).await?;

        Ok(r)
    }
}

impl<MSG> ActorState<MSG> where MSG: Send, MSG : Sync, MSG: Sized, MSG: Unpin {

    pub fn new_root(
        name: String,
        actor_type: String,
        cancellation_tokens: Vec<CancellationToken>,
    ) -> (ActorStateHandle<MSG>, Self) {
        Self::new(name, actor_type, cancellation_tokens)
    }

    fn new(
        name: String,
        actor_type: String,
        cancellation_tokens: Vec<CancellationToken>
    ) -> (ActorStateHandle<MSG>, Self) {

        let (inbox_sender, inbox) = channel::<ActorMsg<MSG>>(1000);

        let id = ActorId(ACTOR_ID_GEN.fetch_add(1, Ordering::Relaxed));

        println!("id {}", id);

        let cancellation_token_self = CancellationToken::new();

        let s = Self {
            id: id.clone(),
            cancellation_token_self: cancellation_token_self.clone(),
            metrics: ActorStateMetrics::default(),
            inbox: Some(inbox),
            inbox_sender: inbox_sender,
            cancellation_tokens_others: cancellation_tokens,
            name,
            actor_type,
            shutdown: false,
            shutdown_notify: vec![],
        };

        (
            s.create_handle(),
            s
        )
    }
}

pub struct ActorConfig {
    actor_name: String,
    actor_type: String,
    registry: Option<ActorRegistry>,
    cancellation_tokens: Vec<CancellationToken>,
}

pub struct ActorConfigBuilder {
    actor_name: String,
    actor_type: String,
    registry: Option<ActorRegistry>,
    cancellation_tokens: Vec<CancellationToken>,
}

impl ActorConfigBuilder {

    pub fn registry(mut self, registry: &ActorRegistry) -> Self {
        self.registry = Some(registry.clone());
        self
    }

    pub fn cancel_on_token(mut self, token : CancellationToken) -> Self {
        self.cancellation_tokens.push(token);
        self
    }

    pub fn cancel_on(mut self, could_cancel: &dyn HasCancellationToken) -> Self {
        self.cancellation_tokens.push(could_cancel.get_cancellation_token());
        self
    }

    pub fn build(self) -> ActorConfig {
        ActorConfig {
            actor_name: self.actor_name,
            actor_type: self.actor_type,
            registry: self.registry,
            cancellation_tokens: self.cancellation_tokens,
        }
    }
}

impl ActorConfig {
    pub fn new<N>(actor_name : N, actor_type : N) -> ActorConfigBuilder where N : AsRef<str> {
        ActorConfigBuilder {
            actor_name: actor_name.as_ref().to_string(),
            actor_type: actor_type.as_ref().to_string(),
            registry: match DEFAULT_ACTOR_REGISTRY.clone().take() {
                Some(v) => Some(v),
                None => None,
            },
            cancellation_tokens: vec![],
        }
    }
}

#[async_trait]
pub trait Actor: Sized + Send + Sync + 'static {
    type MSG: Send + Sync + Sized + Unpin + Debug;

    fn get_actor_state(&mut self) -> &mut ActorState<Self::MSG>;

    fn timer_duration(&self) -> Option<Duration> {
        None
    }

    async fn on_timer(&mut self) -> Result<(), ::anyhow::Error> {
        Ok(())
    }

    fn spawn<F>(actor_config : ActorConfig, func: F) -> (JoinHandle<()>, ActorStateHandle<Self::MSG>, ::tokio::sync::oneshot::Receiver<()>)
        where F: FnOnce(ActorState<Self::MSG>) -> Self
    {
        let (handle, actor_state) = ActorState::new_root(
            actor_config.actor_name,
            actor_config.actor_type,
            actor_config.cancellation_tokens
        );

        let (ready_shot_s, ready_shot_r) = ::tokio::sync::oneshot::channel();

        let _name = format!("Actor {}", actor_state.name);

        let mut this = func(actor_state);
        let actor_handle_manage = Box::new(handle.clone());
        let jh = ::tokio::spawn(async move {
            if let Some(ref r) = actor_config.registry {
                match r.send(ActorRegistryMsg::Register(ActorRegistryMsgRegister {
                    actor_id: this.get_actor_state().id.clone(),
                    actor_name: this.get_actor_state().name.clone(),
                    actor_type: this.get_actor_state().actor_type.clone(),
                    actor_handle_manage
                })).await {
                    Ok(_) => {},
                    Err(_e) => {
                        warn!("could not send registry that actor {} was registered", this.get_actor_state().name)
                    }
                };
            };

            match ready_shot_s.send(()) {
                Ok(_) => {},
                Err(_) => {}
            };

            let res = this.run_loop().await;

            if let Some(ref r) = actor_config.registry {
                match r.send(ActorRegistryMsg::Unregister(ActorRegistryMsgUnregister {
                    actor_id: this.get_actor_state().id.clone(),
                })).await {
                    Ok(_) => {},
                    Err(_e) => {
                        warn!("could not send registry that actor {} was unregistered", this.get_actor_state().name)
                    }
                }
            };

            for notify_shutdown in this.get_actor_state().shutdown_notify.drain(..) {
                // we ignore errors for now.
                match notify_shutdown.send(()) {
                    Ok(_) => {},
                    Err(_e) => {}
                }
            }


            this.get_actor_state().cancellation_token_self.cancel();
            res
        });

        (jh, handle, ready_shot_r)
    }

    async fn run_loop_inner(&mut self) -> Result<(), ::anyhow::Error> {

        let cancellation_tokens_others = self.get_actor_state().cancellation_tokens_others.clone();
        let mut cancellations = cancellation_tokens_others.iter().map(|v| Box::pin(v.cancelled())).collect::<Vec<_>>();
        let mut inbox = self.get_actor_state().inbox.take().expect("expect inbox");
        let mut timer_wakeup = Box::pin(OptionFuture::from(self.timer_duration().and_then(|x| Some(::tokio::time::sleep(x)) )));

        println!("yay? {}", cancellations.len() > 0);

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
                _ = &mut timer_wakeup => {
                    match self.on_timer().await {
                        Ok(_) => {},
                        Err(e) => {
                            eprint!("oh no! #2 {:?}", e);
                        }
                    }
                    // reset the timer.
                    timer_wakeup = Box::pin(OptionFuture::from(self.timer_duration().and_then(|x| Some(::tokio::time::sleep(x)) )));
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
                Err(_e) => {
                    self.get_actor_state().metrics.errors += 1;
                    warn!("Actor Error {}", self.get_actor_state().name);
                }
            };

            ::tokio::time::sleep(::tokio::time::Duration::from_millis(500)).await;
        }

    }


}