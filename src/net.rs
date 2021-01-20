use crate::message::{
    ChannelEncryptRequestBody, ChannelEncryptResultBody, ClientEncryptResponse, NetMessage,
};
use binread::{BinRead, BinReaderExt};
use byteorder::{LittleEndian, WriteBytesExt};
use bytes::BytesMut;
use protobuf::ProtobufEnum;
use std::convert::{TryFrom, TryInto};
use std::io::{Cursor, Write};
use steam_vent_crypto::generate_session_key;
use steam_vent_proto::enums_clientserver::EMsg;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpStream, ToSocketAddrs};

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("{0}")]
    IO(#[from] std::io::Error),
    #[error("Invalid message header")]
    InvalidHeader,
    #[error("Invalid message kind {0}")]
    InvalidMessageKind(i32),
    #[error("Failed to perform crypto handshake")]
    CryptoHandshakeFailed,
    #[error("Difference message expected, expected {0:?}, got {1:?}")]
    DifferentMessage(EMsg, EMsg),
    #[error("{0}")]
    MalformedBody(#[from] crate::message::MalformedBody),
}

pub type Result<T> = std::result::Result<T, NetworkError>;

pub async fn raw_connect<A: ToSocketAddrs>(addr: A) -> Result<(RawSteamReader, RawSteamWriter)> {
    let stream = TcpStream::connect(addr).await?;
    let (read, write) = stream.into_split();
    Ok((
        RawSteamReader {
            tcp: BufReader::new(read),
            buff: BytesMut::with_capacity(1024),
        },
        RawSteamWriter {
            tcp: BufWriter::new(write),
        },
    ))
}

const MAGIC: [u8; 4] = *b"VT01";

#[derive(Debug, Default, Copy, Clone, BinRead)]
#[repr(C)]
struct Header {
    length: u32,
    magic: [u8; 4],
}

impl Header {
    fn validate(&self) -> Result<()> {
        if self.magic != MAGIC {
            Err(NetworkError::InvalidHeader)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct RawNetMessage<'a> {
    kind: EMsg,
    is_protobuf: bool,
    data: &'a [u8],
}

impl<'a> TryFrom<&'a [u8]> for RawNetMessage<'a> {
    type Error = NetworkError;

    fn try_from(value: &'a [u8]) -> Result<Self> {
        let kind = i32::from_le_bytes(
            value[0..4]
                .try_into()
                .map_err(|_| NetworkError::InvalidMessageKind(0))?,
        );

        let is_protobuf = kind < 0;

        let kind = match steam_vent_proto::enums_clientserver::EMsg::from_i32(kind.abs()) {
            Some(kind) => kind,
            None => return Err(NetworkError::InvalidMessageKind(kind)),
        };

        Ok(RawNetMessage {
            kind,
            is_protobuf,
            data: &value[4..],
        })
    }
}

pub struct RawSteamReader {
    tcp: BufReader<OwnedReadHalf>,
    buff: BytesMut,
}

impl RawSteamReader {
    pub async fn read_buff(&mut self) -> Result<&[u8]> {
        let mut header_bytes = [0; 8];
        self.tcp.read_exact(&mut header_bytes).await?;
        let header: Header = Cursor::new(&header_bytes[..]).read_le().unwrap();
        header.validate()?;

        self.buff.resize(header.length as usize, 0);
        self.tcp.read_exact(self.buff.as_mut()).await?;
        Ok(self.buff.as_ref())
    }

    pub async fn read<'a>(&'a mut self) -> Result<RawNetMessage<'a>> {
        self.read_buff().await.and_then(RawNetMessage::try_from)
    }

    pub async fn try_read<T: NetMessage>(&mut self) -> Result<T> {
        let raw = self.read().await?;
        if raw.kind == T::KIND {
            let mut reader = Cursor::new(raw.data);
            Ok(T::read_body(&mut reader)?)
        } else {
            Err(NetworkError::DifferentMessage(T::KIND, raw.kind))
        }
    }
}

pub struct RawSteamWriter {
    tcp: BufWriter<OwnedWriteHalf>,
}

impl RawSteamWriter {
    pub async fn write_message<T: NetMessage>(&mut self, message: &T) -> Result<()> {
        let message_size = message.encode_size();
        let cap = message_size + 8 + 4;
        let mut buff = Vec::with_capacity(cap);

        WriteBytesExt::write_u32::<LittleEndian>(&mut buff, message_size as u32)?;
        Write::write_all(&mut buff, &MAGIC)?;

        WriteBytesExt::write_i32::<LittleEndian>(&mut buff, T::KIND.value())?;
        message.write_body(&mut buff)?;

        self.tcp.write_all(&buff).await?;
        self.tcp.flush().await?;

        Ok(())
    }
}

pub struct SteamReader {
    raw: RawSteamReader,
    key: [u8; 32],
}

pub struct SteamWriter {
    raw: RawSteamWriter,
    key: [u8; 32],
}

pub async fn connect<A: ToSocketAddrs>(addr: A) -> Result<(SteamReader, SteamWriter)> {
    let (mut raw_reader, mut raw_writer) = raw_connect(addr).await?;

    let encrypt_request = raw_reader.try_read::<ChannelEncryptRequestBody>().await?;

    let key = generate_session_key(Some(&encrypt_request.nonce));

    let response = ClientEncryptResponse {
        target_job_id: u64::max_value(),
        source_job_id: u64::max_value(),
        protocol: encrypt_request.protocol,
        encrypted_key: key.encrypted,
    };
    raw_writer.write_message(&response).await?;

    let encrypt_response = raw_reader.try_read::<ChannelEncryptResultBody>().await?;

    if encrypt_response.result != 1 {
        return Err(NetworkError::CryptoHandshakeFailed);
    }

    Ok((
        SteamReader {
            raw: raw_reader,
            key: key.plain,
        },
        SteamWriter {
            raw: raw_writer,
            key: key.plain,
        },
    ))
}
