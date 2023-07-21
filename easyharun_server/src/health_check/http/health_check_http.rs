use std::time::Duration;
use anyhow::{Context, Error};
use async_trait::async_trait;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tracing::trace;
use easyact::{Actor, ActorState, ActorStateHandle};
use crate::health_check::health_check_manager::HealthCheckHttpConfig;
use crate::health_check::{HealthCheckMsgRecv, HealthCheckMsgRecvCheckFailed, HealthCheckMsgRecvCheckOk};


pub struct HealthCheckHttp {
    health_check_id: String,
    sender: ActorStateHandle<HealthCheckMsgRecv>,
    actor_state: ActorState<()>,
    check_config: HealthCheckHttpConfig,
}

#[async_trait]
impl Actor for HealthCheckHttp {
    type MSG = ();

    fn get_actor_state(&mut self) -> &mut ActorState<Self::MSG> {
        &mut self.actor_state
    }

    fn timer_duration(&self) -> Option<Duration> {
        Some(::tokio::time::Duration::from_millis(100))
    }

    async fn on_timer(&mut self) -> Result<(), Error> {
        self.run_inner().await
    }

    async fn on_msg(&mut self, msg: Self::MSG) -> Result<(), Error> {
        Ok(())
    }
}


impl HealthCheckHttp {
    pub fn new(
        health_check_id: String,
        sender: ActorStateHandle<HealthCheckMsgRecv>,
        check_config: HealthCheckHttpConfig,
        actor_state: ActorState<()>
    ) -> Self {

        HealthCheckHttp {
            health_check_id,
            sender,
            actor_state,
            check_config,
        }
    }


    async fn run_inner(&mut self) -> Result<(), ::anyhow::Error> {

        let url = self.check_config.url.as_str();

        let request = ::tokio::time::timeout(
            Duration::from_millis(self.check_config.timeout_ms as u64),
            ::reqwest::get(url)
        );

        let response = match request.await.context("health check request") {
            Err(e) => {
                self.sender.send(HealthCheckMsgRecv::CheckFailed(HealthCheckMsgRecvCheckFailed {
                    container_id: self.check_config.container_id.clone(),
                    target: url.to_string(),
                    reason: "Timeout".to_string(),
                })).await?;

                return Ok(());
            },
            Ok(raw_resp) => match  raw_resp {
                Ok(res) => res,
                Err(e) => {
                    self.sender.send(HealthCheckMsgRecv::CheckFailed(HealthCheckMsgRecvCheckFailed {
                        container_id: self.check_config.container_id.clone(),
                        target: url.to_string(),
                        reason: format!("Error : {:?}", e),
                    })).await?;

                    return Ok(());
                },
            },
        };

        if !response.status().is_success() {
            self.sender.send(HealthCheckMsgRecv::CheckFailed(HealthCheckMsgRecvCheckFailed {
                container_id: self.check_config.container_id.clone(),
                target: url.to_string(),
                reason: format!("Unsuccessful response : {:?}", response),
            })).await?;

            return Ok(());
        }

        self.sender.send(HealthCheckMsgRecv::CheckOk(HealthCheckMsgRecvCheckOk {
            container_id: self.check_config.container_id.clone(),
            target: url.to_string(),
        })).await?;

        Ok(())

    }
}