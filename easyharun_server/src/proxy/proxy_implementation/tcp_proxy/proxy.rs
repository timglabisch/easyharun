use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use std::error::Error;
use anyhow::{anyhow, Context};
use futures::FutureExt;
use structopt::clap::app_from_crate;
use tokio::sync::mpsc::UnboundedReceiver;
use crate::brain::brain_action::BrainAction;

use crate::proxy::brain::{ProxyBrainAction, ProxyBrainActionAdd, ProxyBrainActionRemove};
use crate::proxy::proxy_implementation::proxy_handle::{ProxyHandle};


pub struct TcpProxy {
    listen_addr: String,
    server_addrs: Vec<String>,
    stats_requests_all: u64,
    recv: UnboundedReceiver<ProxyBrainAction>,
}

impl TcpProxy {

    pub fn new(
        listen_addr: String,
        recv: UnboundedReceiver<ProxyBrainAction>
    ) -> Self {
        Self {
            listen_addr,
            server_addrs: vec![],
            stats_requests_all: 0,
            recv,
        }
    }

    pub fn spawn_and_create_handle(
        listen_addr: String
    ) -> ProxyHandle {

        let (sender, recv) = ::tokio::sync::mpsc::unbounded_channel::<ProxyBrainAction>();

        let listen_addr_clone = listen_addr.to_string();
        let jh = ::tokio::spawn(async move {
            Self::new(listen_addr_clone, recv).run().await.expect("tcp proxy failed");
            ()
        });

        ProxyHandle::new(
            sender,
            jh,
            listen_addr
        )
    }

    pub async fn run(mut self) -> Result<(), ::anyhow::Error> {

        let listener = TcpListener::bind(&self.listen_addr).await?;

        loop {
            ::tokio::select! {
                action = self.recv.recv() => {

                    let action = match action {
                        None => continue,
                        Some(s) => s,
                    };

                    self.handle_action(action);
                },
                accept = listener.accept() => {
                    self.handle_accept(accept);
                }
            }
        }
    }

    fn handle_action(&mut self, action : ProxyBrainAction) {
        match action {
            ProxyBrainAction::Add(action) => self.handle_action_add(action),
            ProxyBrainAction::RemoveAsk(action) => self.handle_action_remove(action),
        };
    }

    fn handle_action_add(&mut self, action : ProxyBrainActionAdd) {
        if self.server_addrs.contains(&action.server_addr) {
            return;
        }

        self.server_addrs.push(action.server_addr);
    }

    fn handle_action_remove(&mut self, action : ProxyBrainActionRemove) {
        self.server_addrs.retain(|x| x != &action.server_addr);
    }

    fn pick_server(&self) -> Result<String, ::anyhow::Error> {
        let server_addrs_len = self.server_addrs.len() as u64;

        if server_addrs_len == 0 {
            return Err(anyhow!("no backend server..."));
        }

        Ok(self.server_addrs[(server_addrs_len % self.stats_requests_all) as usize].to_string())
    }

    fn handle_accept(&mut self, data : Result<(TcpStream, std::net::SocketAddr), std::io::Error>) -> Result<(), ::anyhow::Error> {
        let inbound = match data {
            Ok(v) => v.0,
            Err(e) => {
                eprintln!("proxy, could not accept tcp");
                return Err(anyhow!(e));
            }
        };

        let backend_server_addr = self.pick_server().context("could not pick a backend server")?;

        let transfer = Self::transfer(inbound, backend_server_addr).map(|r| {
            if let Err(e) = r {
                println!("Failed to transfer; error={}", e);
            }
        });

        tokio::spawn(transfer);

        Ok(())
    }

    async fn transfer(mut inbound: TcpStream, proxy_addr: String) -> Result<(), Box<dyn Error>> {
        let mut outbound = TcpStream::connect(proxy_addr).await?;

        let (mut ri, mut wi) = inbound.split();
        let (mut ro, mut wo) = outbound.split();

        let client_to_server = async {
            io::copy(&mut ri, &mut wo).await?;
            wo.shutdown().await
        };

        let server_to_client = async {
            io::copy(&mut ro, &mut wi).await?;
            wi.shutdown().await
        };

        tokio::try_join!(client_to_server, server_to_client)?;

        Ok(())
    }
}