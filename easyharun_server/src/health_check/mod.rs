pub mod health_check_manager;
pub mod http;

#[derive(Debug)]
pub struct HealthCheckMsgRecvCheckFailed {
    target: String,
    reason: String,
}

#[derive(Debug)]
pub struct HealthCheckMsgRecvCheckOk {
    target: String,
}

#[derive(Debug)]
pub enum HealthCheckMsgRecv {
    CheckFailed(HealthCheckMsgRecvCheckFailed),
    CheckOk(HealthCheckMsgRecvCheckOk),
}