
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, HashSet};
use anyhow::Context;
use bollard::container::ListContainersOptions;
use tracing::{instrument, trace};

use easyharun_lib::portmapping::PortMappingParser;
use crate::docker::docker_connection::docker_create_connection;
use crate::kv_container::KV;
use crate::proxy::brain::{ProxyBrain, ProxyBrainAction, ProxyBrainActionAdd, ProxyBrainActionRemove};
use crate::proxy::proxy_implementation::proxy_handle::ProxyHandle;
use crate::proxy::proxy_implementation::tcp_proxy::proxy::TcpProxy;
use crate::proxy::world::{ProxyWorld, ProxyWorldEntry, ProxyWorlds};


#[derive(Debug)]
pub struct ProxyManager {
    proxies: HashMap<String, ProxyHandle>
}

impl ProxyManager {

    pub fn create_proxy_world_current(&self) -> ProxyWorld {

        let mut buf = HashMap::new();

        for (_,proxy) in self.proxies.iter() {
            buf.insert(proxy.get_listen_addr().to_string(), ProxyWorldEntry {
                listen_addr: proxy.get_listen_addr().to_string(),
                server_addrs: proxy.get_server_addrs().clone()
            });
        }

        ProxyWorld {
            proxies: buf
        }
    }

    pub async fn create_proxy_world_expected() -> Result<ProxyWorld, ::anyhow::Error> {

        let docker = docker_create_connection().context("docker connection?")?;

        let mut filters = HashMap::new();
        filters.insert("label", vec!["easyharun=\"1.0.0\""]);

        let containers = docker.list_containers(Some(ListContainersOptions {
            all: true,
            size: false,
            limit: None,
            filters
        })).await.context("could not read containers from docker container")?;

        let mut proxies : HashMap<String, ProxyWorldEntry> = HashMap::new();

        for container in containers.iter() {

            if KV::is_container_marked_to_be_deleted(&container.id.as_ref().unwrap_or(&"no-id".to_string())) {
                continue;
            }

            let labels = match &container.labels {
                Some(s) => s,
                None => continue,
            };

            // syntax should be something like "0.0.0.0:1337->"
            let listen = match labels.get("easyharun_listen") {
                Some(s) => s,
                None => continue,
            };

            let portmappings = PortMappingParser::parse(listen).context("port mapping")?;

            for portmapping in portmappings.iter() {
                match proxies.entry(portmapping.listen_addr.to_string()) {
                    Occupied(mut o) => {
                        o.get_mut().server_addrs.insert(portmapping.server_addr.to_string());
                    },
                    Vacant(o) => {
                        o.insert(ProxyWorldEntry {
                            listen_addr: portmapping.listen_addr.clone(),
                            server_addrs: {
                                let mut s = HashSet::new();
                                s.insert(portmapping.server_addr.clone());
                                s
                            }
                        });
                    },
                };
            }
        }

        Ok(ProxyWorld {
            proxies
        })
    }

    pub fn new() -> Self {
        Self {
            proxies: HashMap::new(),
        }
    }

    #[tracing::instrument]
    pub async fn run(&mut self) {
        loop {
            match self.run_inner().await {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Proxy Manager Error: \n{:?}\n\n", e)
                }
            };

            trace!("sleep");
            ::tokio::time::sleep(::tokio::time::Duration::from_millis(200)).await;
            trace!("/sleep");
        }
    }

    pub async fn run_inner(&mut self) -> Result<(), ::anyhow::Error> {
        let worlds = ProxyWorlds {
            current: self.create_proxy_world_current(),
            expected: Self::create_proxy_world_expected().await?
        };

        let actions = ProxyBrain::think(&worlds);

        match self.execute_brain_actions(actions).await {
            Ok(_) => {},
            Err(_) => {
                eprintln!("failed to execute brain actions.");
            }
        };

        Ok(())
    }

    async fn execute_brain_actions(&mut self, actions: Vec<ProxyBrainAction>) -> Result<(), Vec<::anyhow::Error>> {
        let mut errors = vec![];

        for action in actions.into_iter() {
            match self.execute_brain_action(action).await {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("failed to execute brain action:{:?}\n", e);
                    errors.push(e);
                }
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }

    async fn execute_brain_action(&mut self, action: ProxyBrainAction) -> Result<(), ::anyhow::Error> {
        Ok(match action {
            ProxyBrainAction::Add(a) => self.execute_brain_actions_add(a).await?,
            ProxyBrainAction::RemoveAsk(a) => self.execute_brain_actions_remove_ask(a).await?,
        })
    }

    pub async fn execute_brain_actions_add(&mut self, action : ProxyBrainActionAdd) -> Result<(), ::anyhow::Error> {

        match self.proxies.entry(action.listen_addr.to_string()) {
            Occupied(mut o) => {
                o.get_mut().send(ProxyBrainAction::Add(action))?;
            },
            Vacant(o) => {
                let mut proxy = TcpProxy::spawn_and_create_handle(action.listen_addr.to_string());
                proxy.send(ProxyBrainAction::Add(action))?;
                o.insert(proxy);
            },
        };

        Ok(())
    }

    pub async fn execute_brain_actions_remove_ask(&mut self, action : ProxyBrainActionRemove) -> Result<(), ::anyhow::Error> {


        match self.proxies.entry(action.listen_addr.to_string()) {
            Occupied(mut o) => {
                o.get_mut().send(ProxyBrainAction::RemoveAsk(action))?;
            },
            Vacant(_) => {
                eprintln!("could not remove proxy {} - it does not exists", action.listen_addr)
            },
        };

        Ok(())
    }
}



