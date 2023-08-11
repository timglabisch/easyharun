use std::sync::atomic::Ordering;
use anyhow::Error;
use async_trait::async_trait;
use tokio::sync::oneshot::Receiver;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::warn;
use crate::actor::Actor::{ACTOR_ID_GEN, ActorId, ActorStateHandleManageable};
use crate::actor::ActorRegistry::{ActorRegistry, ActorRegistryMsg, ActorRegistryMsgRegister, DEFAULT_ACTOR_REGISTRY};
use crate::actor::HasCancellationToken::HasCancellationToken;

pub struct ActorTaskState {
    id: ActorId,
    name: String,
    actor_type: String,
    cancellation_tokens_others: Vec<CancellationToken>,
    cancellation_token_self: CancellationToken,
}

pub struct ActorTaskSpawn;



pub struct ActorTaskConfig {
    actor_name: String,
    actor_type: String,
    registry: Option<ActorRegistry>,
    cancellation_tokens: Vec<CancellationToken>,
}

impl ActorTaskConfig {
    pub fn new<N, T>(actor_name : N, actor_type: T) -> ActorTaskConfigBuilder
        where N : AsRef<str>,
        T: AsRef<str>
    {
        ActorTaskConfigBuilder {
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

pub struct ActorTaskConfigBuilder {
    actor_name: String,
    actor_type: String,
    registry: Option<ActorRegistry>,
    cancellation_tokens: Vec<CancellationToken>,
}

impl ActorTaskConfigBuilder {

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

    pub fn build(self) -> ActorTaskConfig {
        ActorTaskConfig {
            actor_name: self.actor_name,
            actor_type: self.actor_type,
            registry: self.registry,
            cancellation_tokens: self.cancellation_tokens,
        }
    }
}

struct ActorStateHandleManageableHandle {

}

#[async_trait]
impl ActorStateHandleManageable for ActorStateHandleManageableHandle  {
    async fn shutdown(&self) -> Result<Receiver<()>, Error> {
        todo!()
    }
}

#[async_trait]
pub trait ActorTask where Self:Sized + Send + Sync + 'static {

    type RES : Send + Sync;

    fn get_actor_state(&mut self) -> &mut ActorTaskState;

    async fn run(&mut self) -> Result<Self::RES, ::anyhow::Error>;

    fn spawn<F>(actor_config : ActorTaskConfig, func: F) -> JoinHandle<Result<Self::RES, ::anyhow::Error>> where F: FnOnce(ActorTaskState) -> Self + 'static, F: Send + Sync, Self: 'static {

        let actor_task_state = ActorTaskState {
            id: ActorId(ACTOR_ID_GEN.fetch_add(1, Ordering::Relaxed)),
            name: actor_config.actor_name,
            actor_type: actor_config.actor_type,
            cancellation_tokens_others: actor_config.cancellation_tokens,
            cancellation_token_self: CancellationToken::new(),
        };

        let jh = ::tokio::spawn(async move {
            let mut this = func(actor_task_state);

            if let Some(ref r) = actor_config.registry {
                match r.send(ActorRegistryMsg::Register(ActorRegistryMsgRegister {
                    actor_id: this.get_actor_state().id.clone(),
                    actor_name: this.get_actor_state().name.clone(),
                    actor_type: this.get_actor_state().actor_type.clone(),
                    actor_handle_manage: Box::new(ActorStateHandleManageableHandle {}),
                })).await {
                    Ok(_) => {},
                    Err(_e) => {
                        warn!("could not send registry that actor {} was registered", this.get_actor_state().name)
                    }
                };
            };

            this.run().await
        });

        jh
    }
}