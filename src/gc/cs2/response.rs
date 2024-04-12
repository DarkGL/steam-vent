use bytes::BytesMut;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;
use std::fmt::Debug;

pub type GCBytesMessageError = std::io::Error;

pub trait GCResponseMessage: Debug + Sized {
    
    fn from_payload(payload: BytesMut) -> Result<Self, GCBytesMessageError>;
}