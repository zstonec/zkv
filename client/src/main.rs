
use tokio::net::TcpStream;
use anyhow::Result;
use client::network::ClientConnection;
use protocol::pb::commands::CommandRequest;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let addr = "127.0.0.1:8080";

    // 连接服务器
    let stream = TcpStream::connect(addr).await?;

    let mut connection = ClientConnection::new(stream);

    let cmd = CommandRequest::new_set( "hello", "world".to_string().into(), 10);

    let data = connection.process(cmd).await?;
    info!("{:?}", data);

    let cmd = CommandRequest::new_get( "hello");
    let data = connection.process(cmd).await?;
    info!("{:?}", data);

    let cmd = CommandRequest::new_del( vec!["hello".into()]);
    let data = connection.process(cmd).await?;
    info!("{:?}", data);


    let cmd = CommandRequest::new_get( "hello");
    let data = connection.process(cmd).await?;
    info!("{:?}", data);

    Ok(())
}
