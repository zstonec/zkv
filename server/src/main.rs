use anyhow::Result;
use tokio::net::{TcpListener, TcpStream};
use server::{MemTable, ServerConnection, Service, SledDb, Storage};
use clap::Parser;

#[macro_use]
extern crate log;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// port
    #[arg(short, long, default_value_t = 8080)]
    port: u32,

    /// storage engine
    #[arg(short, long, value_enum, default_value_t = StorageEngine::Memory)]
    storage: StorageEngine,
}


#[derive(clap::ValueEnum, Clone, Debug)]
enum StorageEngine {
    Memory,
    Sled,
}


#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    let addr = format!("127.0.0.1:{}", args.port);
    info!("Start listening on {}, storage {:?}", addr, args.storage);
    match args.storage {
        StorageEngine::Memory => start_server(&addr, MemTable::new()).await?,
        StorageEngine::Sled => start_server(&addr, SledDb::new("/tmp/sled")).await?
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
