use anyhow::Result;
use tokio::net::{TcpListener, TcpStream};
use server::{MemTable, ServerConnection, Service, SledDb, Storage};
use clap::Parser;
use server::config::{ServerConfig, StorageConfig};

#[macro_use]
extern crate log;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: String,
}


#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    let config = ServerConfig::load(&args.config)?;
    info!("config: {} {:?}", args.config, config);
    match config.storage {
        StorageConfig::MemTable => start_server(&config.general.addr, MemTable::new()).await?,
        StorageConfig::SledDb(path) => start_server(&config.general.addr, SledDb::new(path)).await?
    }
    Ok(())
}

pub async fn start_server<Store: Storage>(addr: &str, store: Store) -> Result<()> {
    let service = Service::new(store);
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, addr) = listener.accept().await?;
        info!("client addr {}", addr);
        let connection = ServerConnection::new(stream, service.clone());
        tokio::spawn(async move { connection.process().await });
    }
}
