use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use std::error::Error;
use anyhow::{anyhow, Context};
use futures::FutureExt;
use tokio::sync::mpsc::UnboundedReceiver;
use crate::proxy::proxy_implementation::proxy_handle::{ProxyHandle, ProxyHandleMsg};


pub struct TcpProxy {
    listen_addr: String,
    server_addrs: Vec<String>,
    stats_requests_all: u64,
}

impl TcpProxy {

    pub fn new(
        listen_addr: String,
        recv: UnboundedReceiver<ProxyHandleMsg>
    ) -> Self {
        Self {
            listen_addr,
            server_addrs: vec![],
            stats_requests_all: 0,
        }
    }

    pub fn spawn_and_create_handle(
        listen_addr: String
    ) -> ProxyHandle {

        let (sender, recv) = ::tokio::sync::mpsc::unbounded_channel::<ProxyHandleMsg>();

        let jh = ::tokio::spawn(async move {
            Self::new(listen_addr, recv).run().await.expect("tcp proxy failed");
            Ok(())
        });

        ProxyHandle {
            jh,
            sender
        }
    }

    pub async fn run(mut self) -> Result<(), ::anyhow::Error> {

        let listener = TcpListener::bind(&self.listen_addr).await?;

        while let Ok((inbound, _)) = listener.accept().await {
            match self.handle_accept(inbound) {
                Ok(()) => {},
                Err(e) => {
                    eprintln!("tcp proxy error: {:?}", e)
                }
            }
        }

        Ok(())
    }

    fn pick_server(&self) -> Result<String, ::anyhow::Error> {
        let server_addrs_len = self.server_addrs.len() as u64;

        if server_addrs_len == 0 {
            return Err(anyhow!("no backend server..."));
        }

        Ok(self.server_addrs[(server_addrs_len % self.stats_requests_all) as usize].to_string())
    }

    fn handle_accept(&mut self, inbound : TcpStream) -> Result<(), ::anyhow::Error> {
        let backend_server_addr = self.pick_server().context("could not pick a backend server")?;

        let transfer = Self::transfer(inbound, backend_server_addr).map(|r| {
            if let Err(e) = r {
                println!("Failed to transfer; error={}", e);
            }
        });

        tokio::spawn(transfer);

        Ok(())
    }

    async fn transfer(mut inbound: TcpStream, proxy_addr: String) -> Result<(), Box<dyn Error>> {
        let mut outbound = TcpStream::connect(proxy_addr).await?;

        let (mut ri, mut wi) = inbound.split();
        let (mut ro, mut wo) = outbound.split();

        let client_to_server = async {
            io::copy(&mut ri, &mut wo).await?;
            wo.shutdown().await
        };

        let server_to_client = async {
            io::copy(&mut ro, &mut wi).await?;
            wi.shutdown().await
        };

        tokio::try_join!(client_to_server, server_to_client)?;

        Ok(())
    }
}