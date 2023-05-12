pub mod health_check_manager;
pub mod http;

#[derive(Debug)]
pub struct HealthCheckMsgRecvCheckFailed {
    target: HealthCheckTarget,
}

pub enum HealthCheckMsgRecv {
    CheckFailed(HealthCheckMsgRecvCheckFailed),
}

#[derive(Debug)]
pub struct HealthCheckTarget {
    target_id: String,
}