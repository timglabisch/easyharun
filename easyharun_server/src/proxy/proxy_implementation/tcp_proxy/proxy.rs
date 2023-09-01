use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use std::error::Error;
use anyhow::{anyhow, Context};
use async_trait::async_trait;
use futures::FutureExt;

use tokio::sync::mpsc::UnboundedReceiver;
use tracing::{info, warn};
use easyact::actor::ActorTask::{ActorTask, ActorTaskConfig, ActorTaskState};

use crate::kv_container::KV;

use crate::proxy::brain::{ProxyBrainAction, ProxyBrainActionAdd, ProxyBrainActionRemove};
use crate::proxy::proxy_implementation::proxy_handle::{ProxyHandle};


pub struct TcpProxy {
    listen_addr: String,
    server_addrs: Vec<String>,
    stats_requests_all: u64,
    recv: UnboundedReceiver<ProxyBrainAction>,
    actor_state: ActorTaskState,
    kv: KV,
}

#[async_trait]
impl ActorTask for TcpProxy {
    type RES = ();

    fn get_actor_state(&mut self) -> &mut ActorTaskState {
        &mut self.actor_state
    }

    async fn run(&mut self) -> Result<Self::RES, ::anyhow::Error> {

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
                    self.handle_accept(accept).await;
                }
            }
        }
    }
}

impl TcpProxy {

    pub fn spawn_and_create_handle(
        listen_addr: String,
        kv: KV,
    ) -> ProxyHandle {

        let (sender, recv) = ::tokio::sync::mpsc::unbounded_channel::<ProxyBrainAction>();

        let listen_addr_clone = listen_addr.to_string();

        let jh = ActorTask::spawn(ActorTaskConfig::new(
            format!("Tcp Proxy {}", &listen_addr),
            "Proxy",
        ).build(), |actor_state| Self {
            listen_addr: listen_addr_clone,
            server_addrs: vec![],
            stats_requests_all: 0,
            recv,
            actor_state,
            kv
        });

        ProxyHandle::new(
            sender,
            jh,
            listen_addr
        )
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

    async fn pick_server(&self) -> Result<String, ::anyhow::Error> {
        let server_addrs_len = self.server_addrs.len() as u64;

        if server_addrs_len == 0 {
            return Err(anyhow!("no backend server..."));
        }


        // we pick the first server that is healthy // todo
        for i in 0..server_addrs_len {

            let id = ((self.stats_requests_all + i) % server_addrs_len) as usize;
            let server = &self.server_addrs[id];


            // todo, the read lock might be expensive.
            if !self.kv.is_target_healthy(server).await {
                info!("skip (unhealthy) {}", server);
                continue;
            }

            return Ok(self.server_addrs[id].to_string());
        }

        warn!("all servers are unhealthy, we pick the first one.");
        let id = ((self.stats_requests_all) % server_addrs_len) as usize;
        Ok(self.server_addrs[id].to_string())
    }

    async fn handle_accept(&mut self, data : Result<(TcpStream, std::net::SocketAddr), std::io::Error>) -> Result<(), ::anyhow::Error> {

        self.stats_requests_all += 1;

        let inbound = match data {
            Ok(v) => v.0,
            Err(e) => {
                eprintln!("proxy, could not accept tcp");
                return Err(anyhow!(e));
            }
        };

        let backend_server_addr = self.pick_server().await.context("could not pick a backend server")?;

        ::tokio::spawn(async move {

            println!("accept {backend_server_addr}");

            match Self::transfer(inbound, backend_server_addr).await {
                Err(e) => {
                    println!("Failed to transfer; error={}", e);
                }
                Ok(_) => {}
            };

        });

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