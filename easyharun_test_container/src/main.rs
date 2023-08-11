use rand::prelude::*;
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

#[tokio::main]
async fn main() -> Result<(), ::anyhow::Error> {

    let server_info = ServerInfo {
        name: ::std::env::var("SERVER_NAME").expect("env server name must be given"),
        id: {
            let mut rng = rand::thread_rng();
            rng.gen::<f64>().to_string()
        }
    };

    HealthCheckHttpServer::run(server_info.clone()).await;

    Server::builder()
        .add_service(ContainerServiceServer::new(ContainerServiceImpl::new(server_info)))
        .serve("0.0.0.0:5345".parse()?)
        .await?;

    Ok(())
}
