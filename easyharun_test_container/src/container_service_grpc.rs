use async_trait::async_trait;
use tonic::{Request, Response, Status};
use crate::proto::container_service_server::ContainerService;
use crate::proto::{GetStatusRequest, GetStatusResponse, KillServerRequest, KillServerResponse, MakeHealthcheckFailRequest, MakeHealthcheckFailResponse, MakeHealthcheckPassRequest, MakeHealthcheckPassResponse};

pub struct ContainerServiceImpl {}

impl ContainerServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl ContainerService for ContainerServiceImpl {
    async fn get_status(&self, request: Request<GetStatusRequest>) -> Result<Response<GetStatusResponse>, Status> {
        todo!()
    }

    async fn kill_server(&self, request: Request<KillServerRequest>) -> Result<Response<KillServerResponse>, Status> {
        todo!()
    }

    async fn make_healthcheck_pass(&self, request: Request<MakeHealthcheckPassRequest>) -> Result<Response<MakeHealthcheckPassResponse>, Status> {
        todo!()
    }

    async fn make_healthcheck_fail(&self, request: Request<MakeHealthcheckFailRequest>) -> Result<Response<MakeHealthcheckFailResponse>, Status> {
        todo!()
    }
}