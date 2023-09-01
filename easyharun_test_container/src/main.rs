use rand::prelude::*;
use tokio::signal::unix::{signal, SignalKind};
use tonic::transport::Server;
use crate::container_service_grpc::ContainerServiceImpl;
use crate::health_check_http_server::HealthCheckHttpServer;
use crate::proto::container_service_server::ContainerServiceServer;

pub mod container_service_grpc;
pub mod proto;
pub mod health_check_http_server;

#[derive(Clone)]
pub struct ServerInfo {
    name: String,
    id: String,
}

pub enum MainMsg {
    Kill
}

#[tokio::main]
async fn main() -> Result<(), ::anyhow::Error> {

    let server_info = ServerInfo {
        name: ::std::env::var("SERVER_NAME").expect("env server name must be given"),
        id: {
            let mut rng = rand::thread_rng();
            rng.gen::<f64>().to_string()
        }
    };

    let (main_channel_sender, mut main_channel_recv) = ::tokio::sync::mpsc::channel::<MainMsg>(100);

    let jh_health_check_server_info = server_info.clone();
    let mut jh_health_check = ::tokio::spawn (async move {
        HealthCheckHttpServer::run(jh_health_check_server_info.clone()).await;
    });

    let jh_server_server_info = server_info.clone();
    let jh_sever_main_channel_sender = main_channel_sender.clone();
    let mut jh_server = ::tokio::spawn(async move {
        Server::builder()
            .add_service(ContainerServiceServer::new(ContainerServiceImpl::new(
                jh_server_server_info,
                jh_sever_main_channel_sender
            )))
            .serve("0.0.0.0:5345".parse().expect("...")).await
    });

    println!("booted, health check is running at http://127.0.0.1:3000");

    let mut sig_quit = signal(SignalKind::quit())?;
    let mut sig_term = signal(SignalKind::terminate())?;

    loop {
        ::tokio::select! {
            _ = &mut jh_health_check => {
                eprintln!("health check crashed");
            },
            _ = &mut jh_server => {
                eprintln!("server check crashed");
            },
            msg = main_channel_recv.recv() => {
                match (msg) {
                    None => {},
                    Some(MainMsg::Kill) => {
                        return Ok(());
                    }
                }
            },
            _ = sig_quit.recv() => {
                println!("Signal quit, quit.");
                return Ok(());
            },
            _ = sig_term.recv() => {
                println!("Signal term, quit.");
                return Ok(());
            }
            _ = ::tokio::signal::ctrl_c() => {
                println!("Signal ctrl_c, quit.");
                return Ok(());
            }
        }
    }
}
