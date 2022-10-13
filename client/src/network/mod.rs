use bytes::BytesMut;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
use protocol::error::KvError;
use protocol::network::frame::{FrameCoder, read_frame};
use protocol::pb::commands::{CommandRequest, CommandResponse};


pub struct ClientConnection<S> {
    stream: S
}

impl <S> ClientConnection<S> where S: AsyncRead + AsyncWrite + Send + Unpin,
{
    pub fn new(stream: S) -> Self {
        Self {
            stream
        }
    }

    pub async fn process(&mut self, cmd: CommandRequest) -> Result<CommandResponse, KvError> {
        self.send(cmd).await?;
        Ok(self.recv().await?)
    }

    async fn send(&mut self, cmd: CommandRequest) -> Result<(), KvError> {
        let mut buf = BytesMut::new();
        cmd.encode_frame(&mut buf)?;
        let encoded = buf.freeze();
        self.stream.write_all(&encoded[..]).await?;
        Ok(())
    }

    async fn recv(&mut self) -> Result<CommandResponse, KvError> {
        let mut buf = BytesMut::new();
        read_frame(&mut self.stream, &mut buf).await?;
        CommandResponse::decode_frame(&mut buf)
    }
}