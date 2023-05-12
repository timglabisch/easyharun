pub mod health_check_manager;
pub mod http;

#[derive(Debug)]
pub struct HealthCheckMsgRecvCheckFailed {
    target: String,
}

pub enum HealthCheckMsgRecv {
    CheckFailed(HealthCheckMsgRecvCheckFailed),
}