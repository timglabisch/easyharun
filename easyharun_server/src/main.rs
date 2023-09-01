#![allow(dead_code)]
#![allow(unused_variables)]

mod container_manager;
mod config;
mod docker;
mod brain;
mod kv_container;
mod proxy;
mod health_check;
mod _test_integration;

use structopt::StructOpt;
use easyharun_lib::config::Config;

use crate::config::ConfigMonitor;
use crate::container_manager::ContainerManager;
use crate::health_check::health_check_manager::HealthCheckManager;
use crate::proxy::proxy_manager::ProxyManager;
use easyact::{Actor, actor_run_grpc_server, ActorConfig, ActorRegistry};
use crate::config::config_provider::ConfigProvider;
use crate::kv_container::KV;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {

}

#[tokio::main]
pub async fn main() {

    tracing_subscriber::fmt::init();

    let (config_reader, config_writer) = ConfigProvider::new(ConfigMonitor::load_config().await);
    let kv = KV::new();

    let (registry_jh, registry_actor) = ActorRegistry::spawn_new();
    registry_actor.register_as_default();

    let registry_actor_copy_server = registry_actor.clone();
    let jh_actor_proto_server = ::tokio::spawn(async move {
        actor_run_grpc_server("0.0.0.0:50051", registry_actor_copy_server).await
    });

    let jh_config_watch = ::tokio::spawn(async move {
        ConfigMonitor::async_watch(config_writer).await
    });

    let (jh_proxymanager, handle_proxymanager, _) = Actor::spawn(ActorConfig::new("ProxyManager", "Manager").build(), |actor_state| ProxyManager::new(
        actor_state,
        config_reader.clone(),
        kv.clone()
    ));


    let (jh_containermanager, handle_containermanager, _) = Actor::spawn(ActorConfig::new("ContainerManager", "Manager").build(), |actor_state| ContainerManager {
        actor_state,
        config_reader: config_reader.clone(),
        kv: kv.clone()
    });


    let (jh_healh_check_manager, handle_healh_check_manager, _) = Actor::spawn(ActorConfig::new("HealthCheckManager", "Manager").build(), |actor_state| HealthCheckManager::new(
        actor_state,
        config_reader.clone(),
        kv.clone()
    ));


    ::tokio::select! {
        _ = jh_proxymanager => {
            panic!("proximanager crash.");
        }
        _ = jh_config_watch => {
            panic!("config_watch crash.");
        }
        _ = jh_containermanager => {
            panic!("containermanager crash.");
        }
        _ = jh_healh_check_manager => {
            panic!("jh_healh_check_manager crash.");
        }
    };
}
