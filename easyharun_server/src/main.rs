#![allow(dead_code)]
#![allow(unused_variables)]

mod container_manager;
mod config;
mod docker;
mod brain;
mod kv_container;
mod proxy;

use structopt::StructOpt;
use easyharun_lib::config::Config;
use crate::config::config_provider::config_set;
use crate::container_manager::ContainerManager;
use crate::proxy::proxy_manager::ProxyManager;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {

}

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();

    config_set(Config::read_from_file("./example/basic/easyharun.toml").await.expect("could not read config"));

    let jh_proxymanager = ::tokio::spawn(async move {
        ProxyManager::new().run().await
    });

    let jh_containermanager = ::tokio::spawn(async move {
        ContainerManager::new().run().await
    });

    ::tokio::select! {
        _ = jh_proxymanager => {
            panic!("proximanager crash.");
        }
        _ = jh_containermanager => {
            panic!("containermanager crash.");
        }
    };

    println!("Hello, world!");
}
