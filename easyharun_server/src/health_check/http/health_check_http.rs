use tokio::sync::mpsc::{channel, Receiver, Sender};
use crate::health_check::health_check_manager::HealthCheckHttpConfig;
use crate::health_check::HealthCheckMsgRecv;

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

    async fn run(self) {
        // todo
    }

    async fn run_inner() {
        // todo
    }
}