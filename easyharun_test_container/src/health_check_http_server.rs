pub static HTTP_HEALTH_CHECK_STATUS_CODE : AtomicU16 = AtomicU16::new(200);

use std::sync::Arc;
use std::sync::atomic::{AtomicU16, Ordering};
use axum::{
    routing::{get},
    http::StatusCode, Router,
};
use axum::extract::State;
use crate::ServerInfo;

pub struct HealthCheckHttpServer {}

pub async fn handle_request(
    State(server_info): State<ServerInfo>
) -> (StatusCode, &'static str) {
    (StatusCode::from_u16(HTTP_HEALTH_CHECK_STATUS_CODE.load(Ordering::Relaxed)).expect("invalid status code"), "Check")
}

impl HealthCheckHttpServer {
    pub async fn run(server_infos: ServerInfo) {

        let app = Router::new()
            .route("/", get(handle_request))
            .with_state(server_infos);
        
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}