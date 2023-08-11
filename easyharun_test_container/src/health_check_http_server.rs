use axum::{
    routing::{get},
    http::StatusCode, Router,
};

pub struct HealthCheckHttpServer {}

pub async fn handle_request() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}

impl HealthCheckHttpServer {
    pub async fn run() {
        let app = Router::new()
            .route("/", get(handle_request));
        
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}