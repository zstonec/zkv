use bytes::{Buf, BufMut, BytesMut};
use prost::Message;
use tokio::io::{AsyncRead, AsyncReadExt};
use crate::pb::commands::{CommandRequest, CommandResponse};
use crate::error::KvError;


pub const LEN_LEN: usize = 4;
const MAX_FRAME: usize = 2 * 1024 * 1024 * 1024;
const COMPRESSION_LIMIT: usize = 14236;
const COMPRESSION_BIT: usize = 1 << 31;

pub trait FrameCoder
    where Self: Message + Sized + Default
{

    fn encode_frame(&self, buf: &mut BytesMut) -> Result<(), KvError> {
        let size = self.encoded_len();
        if size > MAX_FRAME {
            return Err(KvError::FrameError);
        }
        buf.put_u32(size as _);
        if size > COMPRESSION_LIMIT {

        } else {
            self.encode(buf)?;
        }
        Ok(())
    }

    fn decode_frame(buf: &mut BytesMut) -> Result<Self, KvError> {
        let header = buf.get_u32() as usize;
        let (len, compressed) = decode_header(header);
        if compressed {
            Err(KvError::FrameError)
        } else {
            let msg = Self::decode(&buf[..len])?;
            buf.advance(len);
            Ok(msg)
        }
    }
}

impl FrameCoder for CommandRequest {
}

impl FrameCoder for CommandResponse {
}

fn decode_header(header: usize) -> (usize, bool) {
    let len = header & !COMPRESSION_BIT;
    let compressed = header & COMPRESSION_BIT == COMPRESSION_BIT;
    (len, compressed)
}

pub async fn read_frame<S>(stream: &mut S, buf: &mut BytesMut) -> Result<(), KvError>
where
    S: AsyncRead + Unpin + Send,
{
    let header = stream.read_u32().await? as usize;
    let (len, _compressed) = decode_header(header);
    buf.reserve(LEN_LEN + len);
    buf.put_u32(header as _);
    unsafe {buf.advance_mut(len)};
    stream.read_exact(&mut buf[LEN_LEN..]).await?;
    Ok(())
}
