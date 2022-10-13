use bytes::BytesMut;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
use tokio::net::TcpStream;
use protocol::error::KvError;
use protocol::network::frame::{FrameCoder, read_frame};
use protocol::pb::commands::{CommandRequest, CommandResponse};
use crate::{Service, Storage};


pub struct ServerConnection<Stream, Store> {
    stream: Stream,
    service: Service<Store>,
}

impl<Stream, Store> ServerConnection<Stream, Store>
    where
        Stream: AsyncRead + AsyncWrite + Send + Unpin,
        Store: Storage
{

    pub fn new(stream: Stream, service: Service<Store>) -> Self {
        Self {
            stream,
            service
        }
    }

    pub async fn process(mut self) -> Result<(), KvError> {
        while let Ok(cmd) = self.recv().await {
            let res = self.service.execute(cmd);
            self.send(res).await?;
        }
        Ok(())
    }

    async fn recv(&mut self) -> Result<CommandRequest, KvError> {
        let mut buf = BytesMut::new();
        let stream = &mut self.stream;
        read_frame(stream, &mut buf).await?;
        CommandRequest::decode_frame(&mut buf)
    }

    async fn send(&mut self, msg: CommandResponse) -> Result<(), KvError> {
        let mut buf = BytesMut::new();
        msg.encode_frame(&mut buf)?;
        let encoded = buf.freeze();
        self.stream.write_all(&encoded[..]).await?;
        Ok(())
    }
}