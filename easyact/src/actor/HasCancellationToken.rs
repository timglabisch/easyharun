use tokio_util::sync::CancellationToken;

pub trait HasCancellationToken {
    fn get_cancellation_token(&self) -> CancellationToken;
}