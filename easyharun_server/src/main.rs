#![allow(dead_code)]
#![allow(unused_variables)]

mod container_manager;
mod config;
mod docker;
mod brain;
mod kv_container;
mod proxy;
mod health_check;

use structopt::StructOpt;
use easyharun_lib::config::Config;
use crate::config::config_provider::config_set;
use crate::config::ConfigMonitor;
use crate::container_manager::ContainerManager;
use crate::health_check::health_check_manager::HealthCheckManager;
use crate::proxy::proxy_manager::ProxyManager;
use easyact::{Actor, actor_run_grpc_server, ActorConfig, ActorRegistry};

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {

}

#[tokio::main]
pub async fn main() {

    tracing_subscriber::fmt::init();

    ConfigMonitor::load_config().await;

    let (registry_jh, registry_actor) = ActorRegistry::spawn_new();
    registry_actor.register_as_default();

    let registry_actor_copy_server = registry_actor.clone();
    let jh_actor_proto_server = ::tokio::spawn(async move {
        actor_run_grpc_server("0.0.0.0:50051", registry_actor_copy_server).await
    });

    let jh_config_watch = ::tokio::spawn(async move {
        ConfigMonitor::async_watch().await
    });

    let jh_proxymanager = ::tokio::spawn(async move {
        ProxyManager::new().run().await
    });


    let (jh_containermanager, handle_containermanager, _) = Actor::spawn(ActorConfig::new("ContainerManager", "Manager").build(), |actor_state| ContainerManager { actor_state });

    let jh_healh_check_manager = ::tokio::spawn(async move {
        HealthCheckManager::new().run().await
    });

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
