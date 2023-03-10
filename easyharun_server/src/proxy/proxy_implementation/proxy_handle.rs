use anyhow::{anyhow, Context};
use tokio::task::JoinHandle;

pub enum ProxyHandleMsg {

}

pub struct ProxyHandle {
    sender: ::tokio::sync::mpsc::UnboundedSender<ProxyHandleMsg>,
    jh: JoinHandle<()>,
}

impl ProxyHandle {
    pub fn new(
        sender: ::tokio::sync::mpsc::UnboundedSender<ProxyHandleMsg>,
        jh: JoinHandle<()>
    ) -> Self {
        Self { sender, jh }
    }
}

impl ProxyHandle {
    pub fn send(&self, msg : ProxyHandleMsg) -> Result<(), ::anyhow::Error> {
        Ok(
            self.sender.send(msg)
                .map_err(|x| anyhow!("could not send {}", x))
                .context("could not send to proxy")?
        )
    }
}