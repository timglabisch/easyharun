use tonic::{Request, Status, Response};

use crate::actor::ActorRegistry::ActorRegistry;
use crate::proto::proto_actor::{PingRequest, PingResponse, ActorsRunningGetRequest, ActorsRunningGetResponse, ActorsRunningGetResponseItem};

use crate::proto::proto_actor::actor_service_server::ActorService;

#[derive(Debug)]
pub struct GrpcServiceActor {
    actor_registry: ActorRegistry,
}

impl GrpcServiceActor {
    pub fn new(actor_registry : ActorRegistry) -> Self {
        Self {
            actor_registry
        }
    }
}

#[tonic::async_trait]
impl ActorService for GrpcServiceActor {
    async fn ping(
        &self,
        request: Request<PingRequest>,
    ) -> Result<Response<PingResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = PingResponse {
            id: format!("Hello {}!", request.into_inner().id).into(),
        };

        Ok(Response::new(reply))
    }

    async fn actors_running_get(
        &self,
        _request: Request<ActorsRunningGetRequest>,
    ) -> Result<Response<ActorsRunningGetResponse>, Status> {

        let running_actors = match self.actor_registry.get_running_actors().await {
            Ok(v) => v,
            Err(_e) => return Err(Status::internal("could not read running actors."))
        };

        Ok(Response::new(ActorsRunningGetResponse{
            items: running_actors.iter().map(|v| ActorsRunningGetResponseItem {
                actor_id: v.actor_id.0.to_string(),
                actor_name: v.actor_name.to_string(),
                actor_type: v.actor_type.to_string(),
            }).collect::<Vec<_>>()
        }))
    }
}