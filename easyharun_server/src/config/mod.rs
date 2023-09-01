pub mod config_provider;
pub mod config_world_builder;
pub mod config_monitor;

pub struct ConfigMonitor;

use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use easyharun_lib::config::Config;

impl ConfigMonitor {

    pub async fn load_config() -> Config {
        crate::Config::read_from_file("./example/basic/easyharun.toml").await.expect("could not read config")
    }

    fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
        let (mut tx, rx) = channel(1);

        // Automatically select the best implementation for your platform.
        // You can also access each implementation directly e.g. INotifyWatcher.
        let watcher = RecommendedWatcher::new(move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        }, ::notify::Config::default())?;

        Ok((watcher, rx))
    }

    pub async fn async_watch() -> notify::Result<()> {
        let (mut watcher, mut rx) = Self::async_watcher()?;

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch(Path::new("./example/basic/easyharun.toml"), RecursiveMode::Recursive)?;

        while let Some(res) = rx.next().await {
            match res {
                Ok(event) => {
                    Self::load_config().await;
                },
                Err(e) => println!("watch error: {:?}", e),
            }
        }

        Ok(())
    }
}