pub mod health_check_manager;
pub mod http;

pub struct HealthCheckMsgRecvCheckFailed {
    target: HealthCheckTarget,
}

pub enum HealthCheckMsgRecv {
    CheckFailed(HealthCheckMsgRecvCheckFailed),
}

pub struct HealthCheckTarget {
    target_id: String,
}