use std::collections::HashSet;
use anyhow::{anyhow, Context};
use tokio::task::JoinHandle;
use crate::proxy::brain::ProxyBrainAction;


pub struct ProxyHandle {
    sender: ::tokio::sync::mpsc::UnboundedSender<ProxyBrainAction>,
    jh: JoinHandle<()>,
    listen_addr: String,
    server_addrs: HashSet<String>,
}

impl ProxyHandle {
    pub fn new(
        sender: ::tokio::sync::mpsc::UnboundedSender<ProxyBrainAction>,
        jh: JoinHandle<()>,
        listen_addr: String,
    ) -> Self {
        Self {
            sender,
            jh,
            listen_addr,
            server_addrs: HashSet::new(),
        }
    }
}

impl ProxyHandle {
    pub fn send(&mut self, msg : ProxyBrainAction) -> Result<(), ::anyhow::Error> {

        // we need to manage this messages twice.
        // the handle must know all server_addrs.
        match &msg {
            &ProxyBrainAction::Add(v) => {
                self.server_addrs.remove(&v.server_addr);
            },
            &ProxyBrainAction::RemoveAsk(v) => {
                self.server_addrs.insert(v.server_addr.to_string());
            },
        };

        Ok(
            self.sender.send(msg)
                .map_err(|x| anyhow!("could not send {}", x))
                .context("could not send to proxy")?
        )
    }

    pub fn get_listen_addr(&self) -> &str {
        &self.listen_addr
    }
    pub fn get_server_addrs(&self) -> &HashSet<String> {
        &self.server_addrs
    }
}