// This file is generated by rust-protobuf 2.24.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `steammessages_remoteclient_service.steamclient.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_24_1;

static file_descriptor_proto_data: &'static [u8] = b"\
    \n4steammessages_remoteclient_service.steamclient.proto\x1a,steammessage\
    s_unified_base.steamclient.proto\x1a1steammessages_remoteclient_service_\
    messages.proto2\xd4\x0c\n\x0cRemoteClient\x12\x88\x01\n\x0eGetPairingInf\
    o\x12%.CRemoteClient_GetPairingInfo_Request\x1a&.CRemoteClient_GetPairin\
    gInfo_Response\"'\x82\xb5\x18#Get\x20pairing\x20info\x20for\x20an\x20ent\
    ered\x20PIN\x12~\n\x0cNotifyOnline\x12\".CRemoteClient_Online_Notificati\
    on\x1a\x0b.NoResponse\"=\x82\xb5\x189Let\x20the\x20service\x20know\x20we\
    're\x20available\x20for\x20status\x20listeners\x12n\n\x11NotifyReplyPack\
    et\x12'.CRemoteClient_ReplyPacket_Notification\x1a\x0b.NoResponse\"#\x82\
    \xb5\x18\x1fSend\x20a\x20reply\x20to\x20a\x20remote\x20client\x12\x9f\
    \x01\n\x12AllocateTURNServer\x12).CRemoteClient_AllocateTURNServer_Reque\
    st\x1a*.CRemoteClient_AllocateTURNServer_Response\"2\x82\xb5\x18.Allocat\
    e\x20a\x20TURN\x20server\x20for\x20a\x20streaming\x20session\x12\xa7\x01\
    \n\x13AllocateRelayServer\x12*.CRemoteClient_AllocateRelayServer_Request\
    \x1a+.CRemoteClient_AllocateRelayServer_Response\"7\x82\xb5\x183Allocate\
    \x20a\x20UDP\x20relay\x20server\x20for\x20a\x20streaming\x20session\x12}\
    \n\x0bAllocateSDR\x12\".CRemoteClient_AllocateSDR_Request\x1a#.CRemoteCl\
    ient_AllocateSDR_Response\"%\x82\xb5\x18!Allocate\x20SDR\x20resources\
    \x20for\x20an\x20app\x12\x83\x01\n\x18SendSteamBroadcastPacket\x12*.CRem\
    oteClient_SteamBroadcast_Notification\x1a\x0b.NoResponse\".\x82\xb5\x18*\
    Broadcast\x20a\x20packet\x20to\x20remote\x20Steam\x20clients\x12{\n\x16S\
    endSteamToSteamPacket\x12(.CRemoteClient_SteamToSteam_Notification\x1a\
    \x0b.NoResponse\"*\x82\xb5\x18&Send\x20a\x20packet\x20to\x20a\x20remote\
    \x20Steam\x20client\x12\xa8\x01\n\x1cSendRemotePlaySessionStarted\x12#.C\
    RemotePlay_SessionStarted_Request\x1a$.CRemotePlay_SessionStarted_Respon\
    se\"=\x82\xb5\x189Let\x20the\x20server\x20know\x20that\x20we\x20started\
    \x20a\x20Remote\x20Play\x20session\x12\x94\x01\n\x1cSendRemotePlaySessio\
    nStopped\x12(.CRemotePlay_SessionStopped_Notification\x1a\x0b.NoResponse\
    \"=\x82\xb5\x189Let\x20the\x20server\x20know\x20that\x20we\x20stopped\
    \x20a\x20Remote\x20Play\x20session\x12\x88\x01\n\x1cSendRemotePlayTogeth\
    erPacket\x12!.CRemotePlayTogether_Notification\x1a\x0b.NoResponse\"8\x82\
    \xb5\x184Send\x20a\x20Remote\x20Play\x20Together\x20packet\x20to\x20a\
    \x20Steam\x20client\x1a.\x82\xb5\x18*Methods\x20for\x20Steam\x20remote\
    \x20client\x20operations2\x94\x07\n\x17RemoteClientSteamClient\x12\x90\
    \x01\n\x1aNotifyRegisterStatusUpdate\x120.CRemoteClient_RegisterStatusUp\
    date_Notification\x1a\x0b.NoResponse\"3\x82\xb5\x18/Register\x20for\x20s\
    tatus\x20updates\x20with\x20a\x20Steam\x20client\x12\x96\x01\n\x1cNotify\
    UnregisterStatusUpdate\x122.CRemoteClient_UnregisterStatusUpdate_Notific\
    ation\x1a\x0b.NoResponse\"5\x82\xb5\x181Unregister\x20for\x20status\x20u\
    pdates\x20with\x20a\x20Steam\x20client\x12p\n\x12NotifyRemotePacket\x12(\
    .CRemoteClient_RemotePacket_Notification\x1a\x0b.NoResponse\"#\x82\xb5\
    \x18\x1fSend\x20a\x20packet\x20to\x20a\x20Steam\x20client\x12\x85\x01\n\
    \x1aNotifySteamBroadcastPacket\x12*.CRemoteClient_SteamBroadcast_Notific\
    ation\x1a\x0b.NoResponse\".\x82\xb5\x18*Broadcast\x20a\x20packet\x20to\
    \x20remote\x20Steam\x20clients\x12\x91\x01\n\x18NotifySteamToSteamPacket\
    \x12(.CRemoteClient_SteamToSteam_Notification\x1a\x0b.NoResponse\">\x82\
    \xb5\x18:Send\x20a\x20packet\x20to\x20a\x20Steam\x20client\x20from\x20a\
    \x20remote\x20Steam\x20client\x12\x8a\x01\n\x1eNotifyRemotePlayTogetherP\
    acket\x12!.CRemotePlayTogether_Notification\x1a\x0b.NoResponse\"8\x82\
    \xb5\x184Send\x20a\x20Remote\x20Play\x20Together\x20packet\x20to\x20a\
    \x20Steam\x20client\x1a2\x82\xb5\x18*Methods\x20for\x20Steam\x20remote\
    \x20client\x20operations\xc0\xb5\x18\x02B\x03\x80\x01\x01J\xfc\x0c\n\x06\
    \x12\x04\0\0P\x01\n\t\n\x02\x03\0\x12\x03\0\06\n\t\n\x02\x03\x01\x12\x03\
    \x01\0;\n\x08\n\x01\x08\x12\x03\x03\0\"\n\t\n\x02\x08\x10\x12\x03\x03\0\
    \"\n\n\n\x02\x06\0\x12\x04\x05\03\x01\n\n\n\x03\x06\0\x01\x12\x03\x05\
    \x08\x14\n\n\n\x03\x06\0\x03\x12\x03\x06\x08T\n\r\n\x06\x06\0\x03\xd0\
    \x86\x03\x12\x03\x06\x08T\n\x0c\n\x04\x06\0\x02\0\x12\x04\x08\x08\n\t\n\
    \x0c\n\x05\x06\0\x02\0\x01\x12\x03\x08\x0c\x1a\n\x0c\n\x05\x06\0\x02\0\
    \x02\x12\x03\x08\x1cA\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x08Lr\n\x0c\n\
    \x05\x06\0\x02\0\x04\x12\x03\t\x10T\n\x0f\n\x08\x06\0\x02\0\x04\xd0\x86\
    \x03\x12\x03\t\x10T\n\x0c\n\x04\x06\0\x02\x01\x12\x04\x0c\x08\x0e\t\n\
    \x0c\n\x05\x06\0\x02\x01\x01\x12\x03\x0c\x0c\x18\n\x0c\n\x05\x06\0\x02\
    \x01\x02\x12\x03\x0c\x1a<\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03\x0cGR\n\
    \x0c\n\x05\x06\0\x02\x01\x04\x12\x03\r\x10j\n\x0f\n\x08\x06\0\x02\x01\
    \x04\xd0\x86\x03\x12\x03\r\x10j\n\x0c\n\x04\x06\0\x02\x02\x12\x04\x10\
    \x08\x12\t\n\x0c\n\x05\x06\0\x02\x02\x01\x12\x03\x10\x0c\x1d\n\x0c\n\x05\
    \x06\0\x02\x02\x02\x12\x03\x10\x1fF\n\x0c\n\x05\x06\0\x02\x02\x03\x12\
    \x03\x10Q\\\n\x0c\n\x05\x06\0\x02\x02\x04\x12\x03\x11\x10P\n\x0f\n\x08\
    \x06\0\x02\x02\x04\xd0\x86\x03\x12\x03\x11\x10P\n\x0c\n\x04\x06\0\x02\
    \x03\x12\x04\x14\x08\x16\t\n\x0c\n\x05\x06\0\x02\x03\x01\x12\x03\x14\x0c\
    \x1e\n\x0c\n\x05\x06\0\x02\x03\x02\x12\x03\x14\x20I\n\x0c\n\x05\x06\0\
    \x02\x03\x03\x12\x03\x14T~\n\x0c\n\x05\x06\0\x02\x03\x04\x12\x03\x15\x10\
    _\n\x0f\n\x08\x06\0\x02\x03\x04\xd0\x86\x03\x12\x03\x15\x10_\n\x0c\n\x04\
    \x06\0\x02\x04\x12\x04\x18\x08\x1a\t\n\x0c\n\x05\x06\0\x02\x04\x01\x12\
    \x03\x18\x0c\x1f\n\x0c\n\x05\x06\0\x02\x04\x02\x12\x03\x18!K\n\r\n\x05\
    \x06\0\x02\x04\x03\x12\x04\x18V\x81\x01\n\x0c\n\x05\x06\0\x02\x04\x04\
    \x12\x03\x19\x10d\n\x0f\n\x08\x06\0\x02\x04\x04\xd0\x86\x03\x12\x03\x19\
    \x10d\n\x0c\n\x04\x06\0\x02\x05\x12\x04\x1c\x08\x1e\t\n\x0c\n\x05\x06\0\
    \x02\x05\x01\x12\x03\x1c\x0c\x17\n\x0c\n\x05\x06\0\x02\x05\x02\x12\x03\
    \x1c\x19;\n\x0c\n\x05\x06\0\x02\x05\x03\x12\x03\x1cFi\n\x0c\n\x05\x06\0\
    \x02\x05\x04\x12\x03\x1d\x10R\n\x0f\n\x08\x06\0\x02\x05\x04\xd0\x86\x03\
    \x12\x03\x1d\x10R\n\x0c\n\x04\x06\0\x02\x06\x12\x04\x20\x08\"\t\n\x0c\n\
    \x05\x06\0\x02\x06\x01\x12\x03\x20\x0c$\n\x0c\n\x05\x06\0\x02\x06\x02\
    \x12\x03\x20&P\n\x0c\n\x05\x06\0\x02\x06\x03\x12\x03\x20[f\n\x0c\n\x05\
    \x06\0\x02\x06\x04\x12\x03!\x10[\n\x0f\n\x08\x06\0\x02\x06\x04\xd0\x86\
    \x03\x12\x03!\x10[\n\x0c\n\x04\x06\0\x02\x07\x12\x04$\x08&\t\n\x0c\n\x05\
    \x06\0\x02\x07\x01\x12\x03$\x0c\"\n\x0c\n\x05\x06\0\x02\x07\x02\x12\x03$\
    $L\n\x0c\n\x05\x06\0\x02\x07\x03\x12\x03$Wb\n\x0c\n\x05\x06\0\x02\x07\
    \x04\x12\x03%\x10W\n\x0f\n\x08\x06\0\x02\x07\x04\xd0\x86\x03\x12\x03%\
    \x10W\n\x0c\n\x04\x06\0\x02\x08\x12\x04(\x08*\t\n\x0c\n\x05\x06\0\x02\
    \x08\x01\x12\x03(\x0c(\n\x0c\n\x05\x06\0\x02\x08\x02\x12\x03(*M\n\x0c\n\
    \x05\x06\0\x02\x08\x03\x12\x03(X|\n\x0c\n\x05\x06\0\x02\x08\x04\x12\x03)\
    \x10j\n\x0f\n\x08\x06\0\x02\x08\x04\xd0\x86\x03\x12\x03)\x10j\n\x0c\n\
    \x04\x06\0\x02\t\x12\x04,\x08.\t\n\x0c\n\x05\x06\0\x02\t\x01\x12\x03,\
    \x0c(\n\x0c\n\x05\x06\0\x02\t\x02\x12\x03,*R\n\x0c\n\x05\x06\0\x02\t\x03\
    \x12\x03,]h\n\x0c\n\x05\x06\0\x02\t\x04\x12\x03-\x10j\n\x0f\n\x08\x06\0\
    \x02\t\x04\xd0\x86\x03\x12\x03-\x10j\n\x0c\n\x04\x06\0\x02\n\x12\x040\
    \x082\t\n\x0c\n\x05\x06\0\x02\n\x01\x12\x030\x0c(\n\x0c\n\x05\x06\0\x02\
    \n\x02\x12\x030*K\n\x0c\n\x05\x06\0\x02\n\x03\x12\x030Va\n\x0c\n\x05\x06\
    \0\x02\n\x04\x12\x031\x10e\n\x0f\n\x08\x06\0\x02\n\x04\xd0\x86\x03\x12\
    \x031\x10e\n\n\n\x02\x06\x01\x12\x045\0P\x01\n\n\n\x03\x06\x01\x01\x12\
    \x035\x08\x1f\n\n\n\x03\x06\x01\x03\x12\x036\x08T\n\r\n\x06\x06\x01\x03\
    \xd0\x86\x03\x12\x036\x08T\n\n\n\x03\x06\x01\x03\x12\x037\x08K\n\r\n\x06\
    \x06\x01\x03\xd8\x86\x03\x12\x037\x08K\n\x0c\n\x04\x06\x01\x02\0\x12\x04\
    9\x08;\t\n\x0c\n\x05\x06\x01\x02\0\x01\x12\x039\x0c&\n\x0c\n\x05\x06\x01\
    \x02\0\x02\x12\x039(X\n\x0c\n\x05\x06\x01\x02\0\x03\x12\x039cn\n\x0c\n\
    \x05\x06\x01\x02\0\x04\x12\x03:\x10`\n\x0f\n\x08\x06\x01\x02\0\x04\xd0\
    \x86\x03\x12\x03:\x10`\n\x0c\n\x04\x06\x01\x02\x01\x12\x04=\x08?\t\n\x0c\
    \n\x05\x06\x01\x02\x01\x01\x12\x03=\x0c(\n\x0c\n\x05\x06\x01\x02\x01\x02\
    \x12\x03=*\\\n\x0c\n\x05\x06\x01\x02\x01\x03\x12\x03=gr\n\x0c\n\x05\x06\
    \x01\x02\x01\x04\x12\x03>\x10b\n\x0f\n\x08\x06\x01\x02\x01\x04\xd0\x86\
    \x03\x12\x03>\x10b\n\x0c\n\x04\x06\x01\x02\x02\x12\x04A\x08C\t\n\x0c\n\
    \x05\x06\x01\x02\x02\x01\x12\x03A\x0c\x1e\n\x0c\n\x05\x06\x01\x02\x02\
    \x02\x12\x03A\x20H\n\x0c\n\x05\x06\x01\x02\x02\x03\x12\x03AS^\n\x0c\n\
    \x05\x06\x01\x02\x02\x04\x12\x03B\x10P\n\x0f\n\x08\x06\x01\x02\x02\x04\
    \xd0\x86\x03\x12\x03B\x10P\n\x0c\n\x04\x06\x01\x02\x03\x12\x04E\x08G\t\n\
    \x0c\n\x05\x06\x01\x02\x03\x01\x12\x03E\x0c&\n\x0c\n\x05\x06\x01\x02\x03\
    \x02\x12\x03E(R\n\x0c\n\x05\x06\x01\x02\x03\x03\x12\x03E]h\n\x0c\n\x05\
    \x06\x01\x02\x03\x04\x12\x03F\x10[\n\x0f\n\x08\x06\x01\x02\x03\x04\xd0\
    \x86\x03\x12\x03F\x10[\n\x0c\n\x04\x06\x01\x02\x04\x12\x04I\x08K\t\n\x0c\
    \n\x05\x06\x01\x02\x04\x01\x12\x03I\x0c$\n\x0c\n\x05\x06\x01\x02\x04\x02\
    \x12\x03I&N\n\x0c\n\x05\x06\x01\x02\x04\x03\x12\x03IYd\n\x0c\n\x05\x06\
    \x01\x02\x04\x04\x12\x03J\x10k\n\x0f\n\x08\x06\x01\x02\x04\x04\xd0\x86\
    \x03\x12\x03J\x10k\n\x0c\n\x04\x06\x01\x02\x05\x12\x04M\x08O\t\n\x0c\n\
    \x05\x06\x01\x02\x05\x01\x12\x03M\x0c*\n\x0c\n\x05\x06\x01\x02\x05\x02\
    \x12\x03M,M\n\x0c\n\x05\x06\x01\x02\x05\x03\x12\x03MXc\n\x0c\n\x05\x06\
    \x01\x02\x05\x04\x12\x03N\x10e\n\x0f\n\x08\x06\x01\x02\x05\x04\xd0\x86\
    \x03\x12\x03N\x10e\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}