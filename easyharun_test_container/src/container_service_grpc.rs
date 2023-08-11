use std::sync::atomic::Ordering::Relaxed;
use async_trait::async_trait;
use tonic::{Request, Response, Status};
use crate::health_check_http_server::HTTP_HEALTH_CHECK_STATUS_CODE;
use crate::proto::container_service_server::ContainerService;
use crate::proto::{GetStatusRequest, GetStatusResponse, KillServerRequest, KillServerResponse, MakeHealthcheckFailRequest, MakeHealthcheckFailResponse, MakeHealthcheckPassRequest, MakeHealthcheckPassResponse};
use crate::ServerInfo;

pub struct ContainerServiceImpl {
    server_info: ServerInfo
}

impl ContainerServiceImpl {
    pub fn new(
        server_info: ServerInfo
    ) -> Self {
        Self { server_info }
    }
}

#[async_trait]
impl ContainerService for ContainerServiceImpl {
    async fn get_status(&self, _request: Request<GetStatusRequest>) -> Result<Response<GetStatusResponse>, Status> {

        Ok(Response::new(GetStatusResponse {
            id: self.server_info.id.clone(),
            name: self.server_info.name.clone(),
            status: HTTP_HEALTH_CHECK_STATUS_CODE.load(Relaxed) as u64,
        }))
    }

    async fn kill_server(&self, _request: Request<KillServerRequest>) -> Result<Response<KillServerResponse>, Status> {
        todo!()
    }

    async fn make_healthcheck_pass(&self, _request: Request<MakeHealthcheckPassRequest>) -> Result<Response<MakeHealthcheckPassResponse>, Status> {
        todo!()
    }

    async fn make_healthcheck_fail(&self, _request: Request<MakeHealthcheckFailRequest>) -> Result<Response<MakeHealthcheckFailResponse>, Status> {
        todo!()
    }
}