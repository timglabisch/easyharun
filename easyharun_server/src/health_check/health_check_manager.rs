use crate::health_check::http::health_check_http::HealthCheckHttpHandle;

pub struct HealthCheckHttpConfig {
    container_id: String,
    url: String,
}

pub enum HealthCheckType {
    HealthCheckTypeHttp(HealthCheckHttpConfig),
}

enum HealthCheck {
    Http(HealthCheckHttpHandle),
}

pub struct HealthCheckManager {
    health_checks: Vec<HealthCheck>,
}