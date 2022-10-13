
use anyhow::Result;
use tokio::net::{TcpListener, TcpStream};
use server::{ServerConnection, Service};
use server::memory::MemTable;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> Result<()>{
    env_logger::init();
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    info!("listener bind to {}", addr);
    let service = Service::new(MemTable::new());
    loop {
        let (stream, addr) = listener.accept().await?;
        info!("client addr {}", addr);
        let connection = ServerConnection::new(stream, service.clone());
        tokio::spawn(async move { connection.process().await });
    }
}
