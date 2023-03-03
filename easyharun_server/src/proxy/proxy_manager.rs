use std::borrow::BorrowMut;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use anyhow::Context;
use bollard::container::ListContainersOptions;
use bollard::models::PortMap;
use easyharun_lib::portmapping::PortMappingParser;
use crate::docker::docker_connection::docker_create_connection;
use crate::kv_container::KV;
use crate::proxy::world::{ProxyWorld, ProxyWorldEntry, ProxyWorlds};

pub struct ManagedProxy {
    listen_addr: String,
    server_addrs: Vec<String>,
}

pub struct ProxyManager {
    proxies: HashMap<String, ManagedProxy>
}

impl ProxyManager {

    pub fn create_proxy_world_current(&self) -> ProxyWorld {

        let mut buf = HashMap::new();

        for (_,proxy) in self.proxies.iter() {
            buf.insert(proxy.listen_addr.clone(), ProxyWorldEntry {
                listen_addr: proxy.listen_addr.clone(),
                server_addrs: proxy.server_addrs.clone()
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
                        o.get_mut().server_addrs.push(portmapping.server_addr.to_string());
                    },
                    Vacant(o) => {
                        o.insert(ProxyWorldEntry {
                            listen_addr: portmapping.listen_addr.clone(),
                            server_addrs: portmapping.server_addr.clone()
                        });
                    },
                };
            }
        }

        Ok(ProxyWorld {
            proxies
        })
    }

    pub async fn run(&mut self) -> Result<(), ::anyhow::Error> {
        let worlds = ProxyWorlds {
            current: self.create_proxy_world_current(),
            expected: Self::create_proxy_world_expected().await?
        };

        Ok(())
    }
}



