use crate::net::{NetMessageHeader, NetworkError, RawNetMessage};
use crate::proto::steammessages_clientserver::CMsgClientCMList;
use crate::service_method::{ServiceMethodRequest, ServiceMethodResponse};
use binread::BinRead;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use bytes::{Buf, BytesMut};
use crc::{Crc, CRC_32_ISO_HDLC};
use flate2::read::GzDecoder;
use futures_util::{
    future::ready,
    stream::{iter, once},
    StreamExt,
};
use log::{debug, trace};
use protobuf::{Message, ProtobufError};
use std::any::type_name;
use std::fmt::Debug;
use std::io::{Cursor, Read, Write};
use steam_vent_proto::{
    enums_clientserver::EMsg,
    steammessages_base::CMsgMulti,
    steammessages_clientserver::{
        CMsgClientServersAvailable,
        CMsgClientGamesPlayed
    },
    steammessages_clientserver_friends::{
        CMsgClientFriendMsg,
        CMsgClientFriendMsgIncoming,
        CMsgClientAddFriend,
        CMsgClientAddFriendResponse,
        CMsgClientRemoveFriend,
        CMsgClientHideFriend,
        CMsgClientFriendsList,
        CMsgClientFriendsGroupsList,
        CMsgClientPlayerNicknameList,
        CMsgClientRequestFriendData,
        CMsgClientChangeStatus,
        CMsgClientPersonaState,
        CMsgClientFriendProfileInfo,
        CMsgClientFriendProfileInfoResponse,
        CMsgClientGetEmoticonList,
        CMsgClientEmoticonList,
    },    
    steammessages_clientserver_login::{
        CMsgClientLoggedOff,
        CMsgClientLogon, 
        CMsgClientLogonResponse,
        CMsgClientNewLoginKey,
        CMsgClientNewLoginKeyAccepted,
        CMsgClientHeartBeat,
        CMsgClientRequestWebAPIAuthenticateUserNonce,
        CMsgClientRequestWebAPIAuthenticateUserNonceResponse,
        CMsgClientLogOff,
        CMsgClientServerTimestampRequest,
        CMsgClientServerTimestampResponse,
        CMsgClientAccountInfo,
        CMsgClientChallengeRequest,
        CMsgClientChallengeResponse
    },
    steammessages_clientserver_2::{
        CMsgClientUserNotifications,
        CMsgClientUpdateMachineAuth,
        CMsgClientUpdateMachineAuthResponse
    }
};
use thiserror::Error;
use tokio_stream::Stream;

#[derive(Error, Debug)]
#[error("Malformed message body for {0:?}: {1}")]
pub struct MalformedBody(pub EMsg, pub MessageBodyError);

#[derive(Error, Debug)]
pub enum MessageBodyError {
    #[error("{0}")]
    Protobuf(#[from] ProtobufError),
    #[error("{0}")]
    BinRead(#[from] binread::Error),
    #[error("{0}")]
    IO(#[from] std::io::Error),
    #[error("{0}")]
    Other(String),
    #[error("malformed child: {0}")]
    MalformedChild(Box<NetworkError>),
}

impl From<String> for MessageBodyError {
    fn from(e: String) -> Self {
        MessageBodyError::Other(e)
    }
}

pub trait NetMessage: Sized + Debug {
    const KIND: EMsg;
    const IS_PROTOBUF: bool = false;

    fn read_body(_data: BytesMut, _header: &NetMessageHeader) -> Result<Self, MalformedBody> {
        panic!("Reading not implemented for {}", type_name::<Self>())
    }

    fn write_body<W: Write>(&self, _writer: W) -> Result<(), std::io::Error> {
        panic!("Writing not implemented for {}", type_name::<Self>())
    }

    fn encode_size(&self) -> usize {
        panic!("Writing not implemented for {}", type_name::<Self>())
    }

    fn process_header(&self, _header: &mut NetMessageHeader) {}
}

#[derive(Debug, BinRead)]
pub struct SetIgnoreFriend {
    pub steamid: u64,
    pub steamid_other: u64,
    pub block: u8,
}

impl NetMessage for SetIgnoreFriend {
    const KIND: EMsg = EMsg::k_EMsgClientSetIgnoreFriend;

    fn write_body<W: Write>(&self, mut writer: W) -> Result<(), std::io::Error> {
        trace!("writing body of {:?} message", Self::KIND);
        writer.write_u64::<LittleEndian>(self.steamid)?;
        writer.write_u64::<LittleEndian>(self.steamid_other)?;
        writer.write_u8(self.block)?;
        
        Ok(())
    }
}

#[derive(Debug, BinRead)]
pub struct ChannelEncryptRequest {
    pub protocol: u32,
    pub universe: u32,
    pub nonce: [u8; 16],
}

impl NetMessage for ChannelEncryptRequest {
    const KIND: EMsg = EMsg::k_EMsgChannelEncryptRequest;

    fn read_body(data: BytesMut, _header: &NetMessageHeader) -> Result<Self, MalformedBody> {
        trace!("reading body of {:?} message", Self::KIND);
        let mut reader = Cursor::new(data);
        ChannelEncryptRequest::read(&mut reader).map_err(|e| MalformedBody(Self::KIND, e.into()))
    }
}

#[derive(Debug, BinRead)]
pub struct ChannelEncryptResult {
    pub result: u32,
}

impl NetMessage for ChannelEncryptResult {
    const KIND: EMsg = EMsg::k_EMsgChannelEncryptResult;

    fn read_body(data: BytesMut, _header: &NetMessageHeader) -> Result<Self, MalformedBody> {
        trace!("reading body of {:?} message", Self::KIND);
        let mut reader = Cursor::new(data);
        ChannelEncryptResult::read(&mut reader).map_err(|e| MalformedBody(Self::KIND, e.into()))
    }
}

#[derive(Debug)]
pub struct ClientEncryptResponse {
    pub protocol: u32,
    pub encrypted_key: Vec<u8>,
}

const CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

impl NetMessage for ClientEncryptResponse {
    const KIND: EMsg = EMsg::k_EMsgChannelEncryptResponse;

    fn write_body<W: Write>(&self, mut writer: W) -> Result<(), std::io::Error> {
        trace!("writing body of {:?} message", Self::KIND);
        writer.write_u64::<LittleEndian>(u64::MAX)?;
        writer.write_u64::<LittleEndian>(u64::MAX)?;
        writer.write_u32::<LittleEndian>(self.protocol)?;
        writer.write_u32::<LittleEndian>(self.encrypted_key.len() as u32)?;
        writer.write_all(&self.encrypted_key)?;

        let mut digest = CRC.digest();
        digest.update(&self.encrypted_key);
        writer.write_u32::<LittleEndian>(digest.finalize())?;
        writer.write_u32::<LittleEndian>(0)?;
        Ok(())
    }

    fn encode_size(&self) -> usize {
        8 + 8 + 4 + 4 + self.encrypted_key.len() + 4 + 4
    }
}

#[derive(Debug)]
pub struct Multi {
    pub messages: Vec<RawNetMessage>,
}

enum MaybeZipReader {
    Raw(Cursor<Vec<u8>>),
    Zipped(GzDecoder<Cursor<Vec<u8>>>),
}

impl Read for MaybeZipReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            MaybeZipReader::Raw(raw) => raw.read(buf),
            MaybeZipReader::Zipped(zipped) => zipped.read(buf),
        }
    }
}

impl NetMessage for Multi {
    const KIND: EMsg = EMsg::k_EMsgMulti;

    fn read_body(data: BytesMut, _header: &NetMessageHeader) -> Result<Self, MalformedBody> {
        trace!("reading body of {:?} message", Self::KIND);
        Ok(Multi {
            messages: Self::iter(data.reader())?
                .collect::<Result<Vec<_>, NetworkError>>()
                .map_err(|e| {
                    MalformedBody(Self::KIND, MessageBodyError::MalformedChild(Box::new(e)))
                })?,
        })
    }
}

impl Multi {
    pub fn iter<R: Read>(
        reader: R,
    ) -> Result<impl Iterator<Item = Result<RawNetMessage, NetworkError>>, MalformedBody> {
        MultiBodyIter::new(reader)
    }
}

pub fn flatten_multi<S: Stream<Item = Result<RawNetMessage, NetworkError>>>(
    source: S,
) -> impl Stream<Item = Result<RawNetMessage, NetworkError>> {
    source.flat_map(|res| match res {
        Ok(next) if next.kind == EMsg::k_EMsgMulti => {
            let reader = Cursor::new(next.data);
            let multi = match MultiBodyIter::new(reader) {
                Err(e) => return once(ready(Err(e.into()))).right_stream(),
                Ok(iter) => iter,
            };
            iter(multi).left_stream()
        }
        res => once(ready(res)).right_stream(),
    })
}

struct MultiBodyIter<R> {
    reader: R,
}

impl MultiBodyIter<MaybeZipReader> {
    pub fn new<R: Read>(mut reader: R) -> Result<Self, MalformedBody> {
        let mut multi = CMsgMulti::parse_from_reader(&mut reader)
            .map_err(|e| MalformedBody(EMsg::k_EMsgMulti, e.into()))?;

        let data = match multi.get_size_unzipped() {
            0 => MaybeZipReader::Raw(Cursor::new(multi.take_message_body())),
            _ => MaybeZipReader::Zipped(GzDecoder::new(Cursor::new(multi.take_message_body()))),
        };

        Ok(MultiBodyIter { reader: data })
    }
}

impl<R: Read> Iterator for MultiBodyIter<R> {
    type Item = Result<RawNetMessage, NetworkError>;

    fn next(&mut self) -> Option<Self::Item> {
        let size = match self.reader.read_u32::<LittleEndian>() {
            Ok(size) => size,
            Err(_) => return None,
        };

        let mut msg_data = BytesMut::with_capacity(size as usize);
        msg_data.resize(size as usize, 0);
        if let Err(e) = self.reader.read_exact(&mut msg_data) {
            return Some(Err(NetworkError::IO(e)));
        }
        let raw = match RawNetMessage::read(msg_data) {
            Ok(raw) => raw,
            Err(e) => return Some(Err(e)),
        };

        debug!("Reading child message {:?}", raw.kind);

        Some(Ok(raw))
    }
}

impl<Request: ServiceMethodRequest> NetMessage for Request {
    const KIND: EMsg = EMsg::k_EMsgServiceMethodCallFromClient;
    const IS_PROTOBUF: bool = true;

    fn read_body(data: BytesMut, _header: &NetMessageHeader) -> Result<Self, MalformedBody> {
        trace!("reading body of protobuf message {:?}", Self::KIND);
        Request::parse_from_reader(&mut data.reader())
            .map_err(|e| MalformedBody(Self::KIND, e.into()))
    }

    fn write_body<W: Write>(&self, mut writer: W) -> Result<(), std::io::Error> {
        trace!("writing body of protobuf message {:?}", Self::KIND);
        self.write_to_writer(&mut writer)
            .map_err(|_| std::io::Error::from(std::io::ErrorKind::InvalidData))
    }

    fn encode_size(&self) -> usize {
        self.compute_size() as usize
    }

    fn process_header(&self, header: &mut NetMessageHeader) {
        header.target_job_name = Some(Request::NAME.into())
    }
}

#[derive(Debug)]
pub struct ServiceMethodResponseMessage {
    job_name: String,
    body: BytesMut,
}

impl ServiceMethodResponseMessage {
    pub fn into_response<Request: ServiceMethodRequest>(
        self,
    ) -> Result<Request::Response, NetworkError> {
        if self.job_name == Request::NAME {
            Ok(
                Request::Response::parse_from_reader(&mut self.body.reader())
                    .map_err(|e| MalformedBody(Self::KIND, e.into()))?,
            )
        } else {
            Err(NetworkError::DifferentServiceMethod(
                Request::NAME,
                self.job_name,
            ))
        }
    }
}

#[derive(Debug)]
pub struct ServiceMethodRequestMessage {
    job_name: String,
    body: BytesMut,
}

impl ServiceMethodRequestMessage {
    pub fn into_message<Request: ServiceMethodRequest>(
        self,
    ) -> Result<Request, NetworkError> {
        if self.job_name == Request::NAME {
            Ok(
                Request::parse_from_reader(&mut self.body.reader())
                    .map_err(|e| MalformedBody(Self::KIND, e.into()))?,
            )
        } else {
            Err(NetworkError::DifferentServiceMethod(
                Request::NAME,
                self.job_name,
            ))
        }
    }
}

impl NetMessage for ServiceMethodRequestMessage {
    const KIND: EMsg = EMsg::k_EMsgServiceMethod;
    const IS_PROTOBUF: bool = true;

    fn read_body(data: BytesMut, header: &NetMessageHeader) -> Result<Self, MalformedBody> {
        trace!("reading body of protobuf message {:?}", Self::KIND);
        Ok(ServiceMethodRequestMessage {
            job_name: header
                .target_job_name
                .as_deref()
                .unwrap_or_default()
                .to_string(),
            body: data,
        })
    }
}

impl NetMessage for ServiceMethodResponseMessage {
    const KIND: EMsg = EMsg::k_EMsgServiceMethodResponse;
    const IS_PROTOBUF: bool = true;

    fn read_body(data: BytesMut, header: &NetMessageHeader) -> Result<Self, MalformedBody> {
        trace!("reading body of protobuf message {:?}", Self::KIND);
        Ok(ServiceMethodResponseMessage {
            job_name: header
                .target_job_name
                .as_deref()
                .unwrap_or_default()
                .to_string(),
            body: data,
        })
    }
}

macro_rules! proto_msg {
    ($kind:expr => $ty:ident) => {
        impl NetMessage for $ty {
            const KIND: EMsg = $kind;
            const IS_PROTOBUF: bool = true;

            fn read_body(
                data: BytesMut,
                _header: &NetMessageHeader,
            ) -> Result<Self, MalformedBody> {
                trace!("reading body of protobuf message {:?}", Self::KIND);
                $ty::parse_from_reader(&mut data.reader())
                    .map_err(|e| MalformedBody(Self::KIND, e.into()))
            }

            fn write_body<W: Write>(&self, mut writer: W) -> Result<(), std::io::Error> {
                trace!("writing body of protobuf message {:?}", Self::KIND);
                self.write_to_writer(&mut writer)
                    .map_err(|_| std::io::Error::from(std::io::ErrorKind::InvalidData))
            }

            fn encode_size(&self) -> usize {
                self.compute_size() as usize
            }
        }
    };
}

// gc_msg!(EMsg::k_EMsgClientToGC => CMsgGCClient);
// proto_msg!(EMsg::k_EMsgClientFromGC => CMsgGCClient);
proto_msg!(EMsg::k_EMsgClientGamesPlayed => CMsgClientGamesPlayed);
proto_msg!(EMsg::k_EMsgClientFriendMsg => CMsgClientFriendMsg);
proto_msg!(EMsg::k_EMsgClientFriendMsgIncoming => CMsgClientFriendMsgIncoming);
proto_msg!(EMsg::k_EMsgClientAddFriend => CMsgClientAddFriend);
proto_msg!(EMsg::k_EMsgClientAddFriendResponse => CMsgClientAddFriendResponse);
proto_msg!(EMsg::k_EMsgClientRemoveFriend => CMsgClientRemoveFriend);
proto_msg!(EMsg::k_EMsgClientHideFriend => CMsgClientHideFriend);
proto_msg!(EMsg::k_EMsgClientFriendsList => CMsgClientFriendsList);
proto_msg!(EMsg::k_EMsgClientFriendsGroupsList => CMsgClientFriendsGroupsList);
proto_msg!(EMsg::k_EMsgClientPlayerNicknameList => CMsgClientPlayerNicknameList);
proto_msg!(EMsg::k_EMsgClientRequestFriendData => CMsgClientRequestFriendData);
proto_msg!(EMsg::k_EMsgClientChangeStatus => CMsgClientChangeStatus);
proto_msg!(EMsg::k_EMsgClientPersonaState => CMsgClientPersonaState);
proto_msg!(EMsg::k_EMsgClientFriendProfileInfo => CMsgClientFriendProfileInfo);
proto_msg!(EMsg::k_EMsgClientFriendProfileInfoResponse => CMsgClientFriendProfileInfoResponse);
proto_msg!(EMsg::k_EMsgClientGetEmoticonList => CMsgClientGetEmoticonList);
proto_msg!(EMsg::k_EMsgClientEmoticonList => CMsgClientEmoticonList);
proto_msg!(EMsg::k_EMsgClientServerTimestampRequest => CMsgClientServerTimestampRequest);
proto_msg!(EMsg::k_EMsgClientServerTimestampResponse => CMsgClientServerTimestampResponse);
proto_msg!(EMsg::k_EMsgClientAccountInfo => CMsgClientAccountInfo);
proto_msg!(EMsg::k_EMsgClientChallengeRequest => CMsgClientChallengeRequest);
proto_msg!(EMsg::k_EMsgClientChallengeResponse => CMsgClientChallengeResponse);
proto_msg!(EMsg::k_EMsgClientLogOff => CMsgClientLogOff);
proto_msg!(EMsg::k_EMsgClientUserNotifications => CMsgClientUserNotifications);
proto_msg!(EMsg::k_EMsgClientRequestWebAPIAuthenticateUserNonce => CMsgClientRequestWebAPIAuthenticateUserNonce);
proto_msg!(EMsg::k_EMsgClientRequestWebAPIAuthenticateUserNonceResponse => CMsgClientRequestWebAPIAuthenticateUserNonceResponse);
proto_msg!(EMsg::k_EMsgClientHeartBeat => CMsgClientHeartBeat);
proto_msg!(EMsg::k_EMsgClientUpdateMachineAuthResponse => CMsgClientUpdateMachineAuthResponse);
proto_msg!(EMsg::k_EMsgClientUpdateMachineAuth => CMsgClientUpdateMachineAuth);
proto_msg!(EMsg::k_EMsgClientNewLoginKeyAccepted => CMsgClientNewLoginKeyAccepted);
proto_msg!(EMsg::k_EMsgClientNewLoginKey => CMsgClientNewLoginKey);
proto_msg!(EMsg::k_EMsgClientLogon => CMsgClientLogon);
proto_msg!(EMsg::k_EMsgClientLoggedOff => CMsgClientLoggedOff);
proto_msg!(EMsg::k_EMsgClientLogOnResponse => CMsgClientLogonResponse);
proto_msg!(EMsg::k_EMsgClientServersAvailable => CMsgClientServersAvailable);
proto_msg!(EMsg::k_EMsgClientCMList => CMsgClientCMList);
