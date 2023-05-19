use easyharun_lib::ContainerId;

pub mod health_check_manager;
pub mod http;

#[derive(Debug)]
pub struct HealthCheckMsgRecvCheckFailed {
    container_id: ContainerId,
    target: String,
    reason: String,
}

#[derive(Debug)]
pub struct HealthCheckMsgRecvCheckOk {
    container_id: ContainerId,
    target: String,
}

#[derive(Debug)]
pub enum HealthCheckMsgRecv {
    CheckFailed(HealthCheckMsgRecvCheckFailed),
    CheckOk(HealthCheckMsgRecvCheckOk),
}