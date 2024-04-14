use protobuf::{RepeatedField, Message};
use crate::{
    gc::{App, ClientToGCMessage},
    net::PROTO_MASK,
    net::NetworkError,
    connection::Connection,
    proto::steammessages_base::CMsgProtoBufHeader,
};
use super::proto::{
    econ_gcmessages::EGCItemMsg,
    base_gcmessages::{
        CMsgSetItemPositions,
        CMsgSetItemPositions_ItemPosition,
        CMsgUseItem,
        CMsgFulfillDynamicRecipeComponent,
        CMsgRecipeComponent,
        CMsgGCRemoveCustomizationAttributeSimple,
    },
};
use super::request::{self, ItemCustomization};
use byteorder::{LittleEndian, WriteBytesExt};
use bytes::{BufMut, BytesMut};
use std::io::Write;

pub const JOBID_NONE: u64 = u64::MAX;

#[derive(Debug)]
pub struct CS2 {
    source_job_id: u64,
}

impl App for CS2 {
    const APPID: u32 = 730;
}

impl CS2 {
    pub fn new() -> Self {
        Self {
            source_job_id: 0,
        }
    }
    
    fn next_jobid(&mut self) -> u64 {
        self.source_job_id += 1;
        self.source_job_id
    }

    async fn send(
        &self,
        connection: &mut Connection,
        msg: ClientToGCMessage,
    ) -> Result<u64, NetworkError> {
        connection.send_gc(msg).await
    }

    pub fn inspect_item(
        &mut self,
        connection: &mut Connection,
        link: String,
    ) -> Result<u64, NetworkError> {
        
    }

    fn proto_payload<Msg: Message>(
        &mut self,
        message: Msg,
        msg_type: i32,
    ) -> Result<Vec<u8>, std::io::Error> {
        fn encode_size(source_job_id: u64) -> usize {
            let mut proto_header = CMsgProtoBufHeader::new();
            proto_header.set_jobid_source(source_job_id);
            
            4 + 4 + proto_header.compute_size() as usize
        }
        
        fn write_header<W: WriteBytesExt>(
            writer: &mut W,
            msg_type: i32,
            source_job_id: u64,
        ) -> std::io::Result<()> {
            let mut proto_header = CMsgProtoBufHeader::new();
            // proto_header.set_jobid_target(self.target_job_id);
            proto_header.set_jobid_source(source_job_id);
            
            writer.write_u32::<LittleEndian>(msg_type as u32 | PROTO_MASK)?;
            writer.write_u32::<LittleEndian>(proto_header.compute_size())?;
            proto_header.write_to_writer(writer)?;
            Ok(())
        }
        
        let source_job_id = self.next_jobid();
        let mut buff = BytesMut::with_capacity(
            encode_size(source_job_id) + message.compute_size() as usize
        );
        let mut writer = (&mut buff).writer();
        
        write_header(&mut writer, msg_type, source_job_id)?;
        message.write_to_writer(&mut writer)?;
    
        Ok(buff.to_vec())
    }
    
    fn payload(
        &mut self,
        message: BytesMut,
    ) -> Result<Vec<u8>, std::io::Error> {
        fn encode_size() -> usize {
            2 + 8 + 8 + 4
        }
        
        fn write_header<W: WriteBytesExt>(
            writer: &mut W,
            source_job_id: u64,
        ) -> std::io::Result<()> {
            writer.write_u16::<LittleEndian>(1)?;
            writer.write_u64::<LittleEndian>(JOBID_NONE)?;
            writer.write_u64::<LittleEndian>(source_job_id)?;
            Ok(())
        }
        
        let source_job_id = self.next_jobid();
        let mut buff = BytesMut::with_capacity(
            encode_size() + message.len() as usize
        );
        let mut writer = (&mut buff).writer();
        
        write_header(&mut writer, source_job_id)?;
        
        writer.write(&message[..])?;
    
        Ok(buff.to_vec())
    }
}