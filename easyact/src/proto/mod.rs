use tonic::{Request, Response, Status, transport::Server};

use crate::actor::ActorRegistry::ActorRegistry;

use self::{proto_actor::actor_service_server::{ActorServiceServer}, service_actor::GrpcServiceActor};

pub mod proto_actor {
    tonic::include_proto!("proto_actor"); // The string specified here must match the proto package name
}

pub mod service_actor;

pub async fn grpc_server_run(actor_registry: ActorRegistry) -> Result<(), ::anyhow::Error> {
    let addr = "[::1]:50051".parse()?;
    let grpc_service = GrpcServiceActor::new(actor_registry);

    Server::builder()
        .add_service(ActorServiceServer::new(grpc_service))
        .serve(addr)
        .await?;

    Ok(())
}