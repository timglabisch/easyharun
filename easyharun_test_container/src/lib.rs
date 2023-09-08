use std::time::Duration;
use anyhow::anyhow;
use tonic::transport::Channel;
use crate::proto::container_service_client::ContainerServiceClient;

pub mod proto;

pub struct TestClient {}

impl TestClient {
    pub async fn connect<S>(url: S, timeout : Duration) -> Result<ContainerServiceClient<Channel>, ::anyhow::Error> where S: AsRef<str> {


        let mut sleep = Box::pin(::tokio::time::sleep(timeout));

        loop {
            let client = crate::proto::container_service_client::ContainerServiceClient::connect(url.as_ref().to_string());
            ::tokio::select! {
                _client = client => {
                    match _client {
                        Ok(v) => return Ok(v),
                        Err(_) => {
                            ::tokio::time::sleep(Duration::from_millis(50)).await;
                            continue;
                        }
                    }
                },
                _ = &mut sleep => {
                    return Err(anyhow!("timeout"))
                }
            }
        }
    }
}