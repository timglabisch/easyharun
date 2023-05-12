use std::time::Duration;
use anyhow::Context;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tracing::trace;
use crate::health_check::health_check_manager::HealthCheckHttpConfig;
use crate::health_check::{HealthCheckMsgRecv, HealthCheckMsgRecvCheckFailed};

pub enum HealthCheckHttpHandleMsg {
    Kill,
}

pub struct HealthCheckHttpHandle {
    message_send: Sender<HealthCheckHttpHandleMsg>,
}

impl HealthCheckHttpHandle {
    pub fn kill(&self) {
        self.message_send.send(HealthCheckHttpHandleMsg::Kill);
    }
}

pub struct HealthCheckHttp {
    health_check_id: String,
    sender: Sender<HealthCheckMsgRecv>,
    message_recv: Receiver<HealthCheckHttpHandleMsg>,
    check_config: HealthCheckHttpConfig,
}

impl HealthCheckHttp {
    pub fn new(
        health_check_id: String,
        sender: Sender<HealthCheckMsgRecv>,
        check_config: HealthCheckHttpConfig
    ) -> HealthCheckHttpHandle {

        let (message_send, message_recv) = channel(100);

        let check = Self {
            health_check_id,
            sender,
            message_recv,
            check_config
        };

        ::tokio::spawn(async move {
            check.run().await
        });

        HealthCheckHttpHandle {
            message_send,
        }
    }

    async fn run(mut self) {
        loop {
            match self.run_inner().await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Health Check Manager Error: \n{:?}\n\n", e)
                }
            };

            trace!("sleep");
            ::tokio::time::sleep(::tokio::time::Duration::from_millis(100)).await;
            trace!("/sleep");
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
                    target: url.to_string()
                }))
            },
            Ok(v) => v,
        };


        Ok(())

    }
}