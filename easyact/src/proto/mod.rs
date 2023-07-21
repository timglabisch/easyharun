use tonic::{Request, Response, Status, transport::Server};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};


use crate::actor::ActorRegistry::ActorRegistry;

use self::{proto_actor::actor_service_server::{ActorServiceServer}, service_actor::GrpcServiceActor};

pub mod proto_actor {
    tonic::include_proto!("proto_actor"); // The string specified here must match the proto package name
}

pub mod service_actor;

pub async fn actor_run_grpc_server<S : AsRef<str>>(
    addr: S,
    actor_registry: ActorRegistry
) -> Result<(), ::anyhow::Error> {
    let addr = addr.as_ref().parse()?;
    let grpc_service = GrpcServiceActor::new(actor_registry);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Server::builder()
        .accept_http1(true)
        .layer(cors)
        .add_service(::tonic_web::enable(ActorServiceServer::new(grpc_service)))
        .serve(addr)
        .await?;

    Ok(())
}