
use tonic::transport::Server;
use crate::container_service_grpc::ContainerServiceImpl;
use crate::proto::container_service_server::ContainerServiceServer;

pub mod container_service_grpc;
pub mod proto;
pub mod health_check_http_server;

#[tokio::main]
async fn main() -> Result<(), ::anyhow::Error> {

    Server::builder()
        .add_service(ContainerServiceServer::new(ContainerServiceImpl::new()))
        .serve("0.0.0.0:5345".parse()?)
        .await?;

    Ok(())
}
