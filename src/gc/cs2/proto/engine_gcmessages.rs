// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `engine_gcmessages.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CEngineGotvSyncPacket)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CEngineGotvSyncPacket {
    // message fields
    // @@protoc_insertion_point(field:CEngineGotvSyncPacket.match_id)
    pub match_id: ::std::option::Option<u64>,
    // @@protoc_insertion_point(field:CEngineGotvSyncPacket.instance_id)
    pub instance_id: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:CEngineGotvSyncPacket.signupfragment)
    pub signupfragment: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:CEngineGotvSyncPacket.currentfragment)
    pub currentfragment: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:CEngineGotvSyncPacket.tickrate)
    pub tickrate: ::std::option::Option<f32>,
    // @@protoc_insertion_point(field:CEngineGotvSyncPacket.tick)
    pub tick: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:CEngineGotvSyncPacket.rtdelay)
    pub rtdelay: ::std::option::Option<f32>,
    // @@protoc_insertion_point(field:CEngineGotvSyncPacket.rcvage)
    pub rcvage: ::std::option::Option<f32>,
    // @@protoc_insertion_point(field:CEngineGotvSyncPacket.keyframe_interval)
    pub keyframe_interval: ::std::option::Option<f32>,
    // @@protoc_insertion_point(field:CEngineGotvSyncPacket.cdndelay)
    pub cdndelay: ::std::option::Option<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:CEngineGotvSyncPacket.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CEngineGotvSyncPacket {
    fn default() -> &'a CEngineGotvSyncPacket {
        <CEngineGotvSyncPacket as ::protobuf::Message>::default_instance()
    }
}

impl CEngineGotvSyncPacket {
    pub fn new() -> CEngineGotvSyncPacket {
        ::std::default::Default::default()
    }

    // optional uint64 match_id = 1;

    pub fn match_id(&self) -> u64 {
        self.match_id.unwrap_or(0)
    }

    pub fn clear_match_id(&mut self) {
        self.match_id = ::std::option::Option::None;
    }

    pub fn has_match_id(&self) -> bool {
        self.match_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_match_id(&mut self, v: u64) {
        self.match_id = ::std::option::Option::Some(v);
    }

    // optional uint32 instance_id = 2;

    pub fn instance_id(&self) -> u32 {
        self.instance_id.unwrap_or(0)
    }

    pub fn clear_instance_id(&mut self) {
        self.instance_id = ::std::option::Option::None;
    }

    pub fn has_instance_id(&self) -> bool {
        self.instance_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_instance_id(&mut self, v: u32) {
        self.instance_id = ::std::option::Option::Some(v);
    }

    // optional uint32 signupfragment = 3;

    pub fn signupfragment(&self) -> u32 {
        self.signupfragment.unwrap_or(0)
    }

    pub fn clear_signupfragment(&mut self) {
        self.signupfragment = ::std::option::Option::None;
    }

    pub fn has_signupfragment(&self) -> bool {
        self.signupfragment.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signupfragment(&mut self, v: u32) {
        self.signupfragment = ::std::option::Option::Some(v);
    }

    // optional uint32 currentfragment = 4;

    pub fn currentfragment(&self) -> u32 {
        self.currentfragment.unwrap_or(0)
    }

    pub fn clear_currentfragment(&mut self) {
        self.currentfragment = ::std::option::Option::None;
    }

    pub fn has_currentfragment(&self) -> bool {
        self.currentfragment.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentfragment(&mut self, v: u32) {
        self.currentfragment = ::std::option::Option::Some(v);
    }

    // optional float tickrate = 5;

    pub fn tickrate(&self) -> f32 {
        self.tickrate.unwrap_or(0.)
    }

    pub fn clear_tickrate(&mut self) {
        self.tickrate = ::std::option::Option::None;
    }

    pub fn has_tickrate(&self) -> bool {
        self.tickrate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tickrate(&mut self, v: f32) {
        self.tickrate = ::std::option::Option::Some(v);
    }

    // optional uint32 tick = 6;

    pub fn tick(&self) -> u32 {
        self.tick.unwrap_or(0)
    }

    pub fn clear_tick(&mut self) {
        self.tick = ::std::option::Option::None;
    }

    pub fn has_tick(&self) -> bool {
        self.tick.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tick(&mut self, v: u32) {
        self.tick = ::std::option::Option::Some(v);
    }

    // optional float rtdelay = 8;

    pub fn rtdelay(&self) -> f32 {
        self.rtdelay.unwrap_or(0.)
    }

    pub fn clear_rtdelay(&mut self) {
        self.rtdelay = ::std::option::Option::None;
    }

    pub fn has_rtdelay(&self) -> bool {
        self.rtdelay.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rtdelay(&mut self, v: f32) {
        self.rtdelay = ::std::option::Option::Some(v);
    }

    // optional float rcvage = 9;

    pub fn rcvage(&self) -> f32 {
        self.rcvage.unwrap_or(0.)
    }

    pub fn clear_rcvage(&mut self) {
        self.rcvage = ::std::option::Option::None;
    }

    pub fn has_rcvage(&self) -> bool {
        self.rcvage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rcvage(&mut self, v: f32) {
        self.rcvage = ::std::option::Option::Some(v);
    }

    // optional float keyframe_interval = 10;

    pub fn keyframe_interval(&self) -> f32 {
        self.keyframe_interval.unwrap_or(0.)
    }

    pub fn clear_keyframe_interval(&mut self) {
        self.keyframe_interval = ::std::option::Option::None;
    }

    pub fn has_keyframe_interval(&self) -> bool {
        self.keyframe_interval.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keyframe_interval(&mut self, v: f32) {
        self.keyframe_interval = ::std::option::Option::Some(v);
    }

    // optional uint32 cdndelay = 11;

    pub fn cdndelay(&self) -> u32 {
        self.cdndelay.unwrap_or(0)
    }

    pub fn clear_cdndelay(&mut self) {
        self.cdndelay = ::std::option::Option::None;
    }

    pub fn has_cdndelay(&self) -> bool {
        self.cdndelay.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cdndelay(&mut self, v: u32) {
        self.cdndelay = ::std::option::Option::Some(v);
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "match_id",
            |m: &CEngineGotvSyncPacket| { &m.match_id },
            |m: &mut CEngineGotvSyncPacket| { &mut m.match_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "instance_id",
            |m: &CEngineGotvSyncPacket| { &m.instance_id },
            |m: &mut CEngineGotvSyncPacket| { &mut m.instance_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "signupfragment",
            |m: &CEngineGotvSyncPacket| { &m.signupfragment },
            |m: &mut CEngineGotvSyncPacket| { &mut m.signupfragment },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "currentfragment",
            |m: &CEngineGotvSyncPacket| { &m.currentfragment },
            |m: &mut CEngineGotvSyncPacket| { &mut m.currentfragment },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "tickrate",
            |m: &CEngineGotvSyncPacket| { &m.tickrate },
            |m: &mut CEngineGotvSyncPacket| { &mut m.tickrate },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "tick",
            |m: &CEngineGotvSyncPacket| { &m.tick },
            |m: &mut CEngineGotvSyncPacket| { &mut m.tick },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "rtdelay",
            |m: &CEngineGotvSyncPacket| { &m.rtdelay },
            |m: &mut CEngineGotvSyncPacket| { &mut m.rtdelay },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "rcvage",
            |m: &CEngineGotvSyncPacket| { &m.rcvage },
            |m: &mut CEngineGotvSyncPacket| { &mut m.rcvage },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "keyframe_interval",
            |m: &CEngineGotvSyncPacket| { &m.keyframe_interval },
            |m: &mut CEngineGotvSyncPacket| { &mut m.keyframe_interval },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "cdndelay",
            |m: &CEngineGotvSyncPacket| { &m.cdndelay },
            |m: &mut CEngineGotvSyncPacket| { &mut m.cdndelay },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CEngineGotvSyncPacket>(
            "CEngineGotvSyncPacket",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CEngineGotvSyncPacket {
    const NAME: &'static str = "CEngineGotvSyncPacket";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.match_id = ::std::option::Option::Some(is.read_uint64()?);
                },
                16 => {
                    self.instance_id = ::std::option::Option::Some(is.read_uint32()?);
                },
                24 => {
                    self.signupfragment = ::std::option::Option::Some(is.read_uint32()?);
                },
                32 => {
                    self.currentfragment = ::std::option::Option::Some(is.read_uint32()?);
                },
                45 => {
                    self.tickrate = ::std::option::Option::Some(is.read_float()?);
                },
                48 => {
                    self.tick = ::std::option::Option::Some(is.read_uint32()?);
                },
                69 => {
                    self.rtdelay = ::std::option::Option::Some(is.read_float()?);
                },
                77 => {
                    self.rcvage = ::std::option::Option::Some(is.read_float()?);
                },
                85 => {
                    self.keyframe_interval = ::std::option::Option::Some(is.read_float()?);
                },
                88 => {
                    self.cdndelay = ::std::option::Option::Some(is.read_uint32()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.match_id {
            my_size += ::protobuf::rt::uint64_size(1, v);
        }
        if let Some(v) = self.instance_id {
            my_size += ::protobuf::rt::uint32_size(2, v);
        }
        if let Some(v) = self.signupfragment {
            my_size += ::protobuf::rt::uint32_size(3, v);
        }
        if let Some(v) = self.currentfragment {
            my_size += ::protobuf::rt::uint32_size(4, v);
        }
        if let Some(v) = self.tickrate {
            my_size += 1 + 4;
        }
        if let Some(v) = self.tick {
            my_size += ::protobuf::rt::uint32_size(6, v);
        }
        if let Some(v) = self.rtdelay {
            my_size += 1 + 4;
        }
        if let Some(v) = self.rcvage {
            my_size += 1 + 4;
        }
        if let Some(v) = self.keyframe_interval {
            my_size += 1 + 4;
        }
        if let Some(v) = self.cdndelay {
            my_size += ::protobuf::rt::uint32_size(11, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.match_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.instance_id {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.signupfragment {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.currentfragment {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.tickrate {
            os.write_float(5, v)?;
        }
        if let Some(v) = self.tick {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.rtdelay {
            os.write_float(8, v)?;
        }
        if let Some(v) = self.rcvage {
            os.write_float(9, v)?;
        }
        if let Some(v) = self.keyframe_interval {
            os.write_float(10, v)?;
        }
        if let Some(v) = self.cdndelay {
            os.write_uint32(11, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CEngineGotvSyncPacket {
        CEngineGotvSyncPacket::new()
    }

    fn clear(&mut self) {
        self.match_id = ::std::option::Option::None;
        self.instance_id = ::std::option::Option::None;
        self.signupfragment = ::std::option::Option::None;
        self.currentfragment = ::std::option::Option::None;
        self.tickrate = ::std::option::Option::None;
        self.tick = ::std::option::Option::None;
        self.rtdelay = ::std::option::Option::None;
        self.rcvage = ::std::option::Option::None;
        self.keyframe_interval = ::std::option::Option::None;
        self.cdndelay = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CEngineGotvSyncPacket {
        static instance: CEngineGotvSyncPacket = CEngineGotvSyncPacket {
            match_id: ::std::option::Option::None,
            instance_id: ::std::option::Option::None,
            signupfragment: ::std::option::Option::None,
            currentfragment: ::std::option::Option::None,
            tickrate: ::std::option::Option::None,
            tick: ::std::option::Option::None,
            rtdelay: ::std::option::Option::None,
            rcvage: ::std::option::Option::None,
            keyframe_interval: ::std::option::Option::None,
            cdndelay: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CEngineGotvSyncPacket {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CEngineGotvSyncPacket").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CEngineGotvSyncPacket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CEngineGotvSyncPacket {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17engine_gcmessages.proto\x1a\x20google/protobuf/descriptor.proto\"\
    \xd0\x02\n\x15CEngineGotvSyncPacket\x12\x19\n\x08match_id\x18\x01\x20\
    \x01(\x04R\x07matchId\x12\x1f\n\x0binstance_id\x18\x02\x20\x01(\rR\ninst\
    anceId\x12&\n\x0esignupfragment\x18\x03\x20\x01(\rR\x0esignupfragment\
    \x12(\n\x0fcurrentfragment\x18\x04\x20\x01(\rR\x0fcurrentfragment\x12\
    \x1a\n\x08tickrate\x18\x05\x20\x01(\x02R\x08tickrate\x12\x12\n\x04tick\
    \x18\x06\x20\x01(\rR\x04tick\x12\x18\n\x07rtdelay\x18\x08\x20\x01(\x02R\
    \x07rtdelay\x12\x16\n\x06rcvage\x18\t\x20\x01(\x02R\x06rcvage\x12+\n\x11\
    keyframe_interval\x18\n\x20\x01(\x02R\x10keyframeInterval\x12\x1a\n\x08c\
    dndelay\x18\x0b\x20\x01(\rR\x08cdndelayJ\xdd\x05\n\x06\x12\x04\0\0\r\x01\
    \n\t\n\x02\x03\0\x12\x03\0\0*\n\n\n\x02\x04\0\x12\x04\x02\0\r\x01\n\n\n\
    \x03\x04\0\x01\x12\x03\x02\x08\x1d\n\x0b\n\x04\x04\0\x02\0\x12\x03\x03\
    \x08%\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x03\x08\x10\n\x0c\n\x05\x04\0\
    \x02\0\x05\x12\x03\x03\x11\x17\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x03\
    \x18\x20\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03#$\n\x0b\n\x04\x04\0\x02\
    \x01\x12\x03\x04\x08(\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\x04\x08\x10\
    \n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x04\x11\x17\n\x0c\n\x05\x04\0\x02\
    \x01\x01\x12\x03\x04\x18#\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x04&'\n\
    \x0b\n\x04\x04\0\x02\x02\x12\x03\x05\x08+\n\x0c\n\x05\x04\0\x02\x02\x04\
    \x12\x03\x05\x08\x10\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x05\x11\x17\n\
    \x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x05\x18&\n\x0c\n\x05\x04\0\x02\x02\
    \x03\x12\x03\x05)*\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x06\x08,\n\x0c\n\
    \x05\x04\0\x02\x03\x04\x12\x03\x06\x08\x10\n\x0c\n\x05\x04\0\x02\x03\x05\
    \x12\x03\x06\x11\x17\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x06\x18'\n\
    \x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x06*+\n\x0b\n\x04\x04\0\x02\x04\x12\
    \x03\x07\x08$\n\x0c\n\x05\x04\0\x02\x04\x04\x12\x03\x07\x08\x10\n\x0c\n\
    \x05\x04\0\x02\x04\x05\x12\x03\x07\x11\x16\n\x0c\n\x05\x04\0\x02\x04\x01\
    \x12\x03\x07\x17\x1f\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x07\"#\n\x0b\
    \n\x04\x04\0\x02\x05\x12\x03\x08\x08!\n\x0c\n\x05\x04\0\x02\x05\x04\x12\
    \x03\x08\x08\x10\n\x0c\n\x05\x04\0\x02\x05\x05\x12\x03\x08\x11\x17\n\x0c\
    \n\x05\x04\0\x02\x05\x01\x12\x03\x08\x18\x1c\n\x0c\n\x05\x04\0\x02\x05\
    \x03\x12\x03\x08\x1f\x20\n\x0b\n\x04\x04\0\x02\x06\x12\x03\t\x08#\n\x0c\
    \n\x05\x04\0\x02\x06\x04\x12\x03\t\x08\x10\n\x0c\n\x05\x04\0\x02\x06\x05\
    \x12\x03\t\x11\x16\n\x0c\n\x05\x04\0\x02\x06\x01\x12\x03\t\x17\x1e\n\x0c\
    \n\x05\x04\0\x02\x06\x03\x12\x03\t!\"\n\x0b\n\x04\x04\0\x02\x07\x12\x03\
    \n\x08\"\n\x0c\n\x05\x04\0\x02\x07\x04\x12\x03\n\x08\x10\n\x0c\n\x05\x04\
    \0\x02\x07\x05\x12\x03\n\x11\x16\n\x0c\n\x05\x04\0\x02\x07\x01\x12\x03\n\
    \x17\x1d\n\x0c\n\x05\x04\0\x02\x07\x03\x12\x03\n\x20!\n\x0b\n\x04\x04\0\
    \x02\x08\x12\x03\x0b\x08.\n\x0c\n\x05\x04\0\x02\x08\x04\x12\x03\x0b\x08\
    \x10\n\x0c\n\x05\x04\0\x02\x08\x05\x12\x03\x0b\x11\x16\n\x0c\n\x05\x04\0\
    \x02\x08\x01\x12\x03\x0b\x17(\n\x0c\n\x05\x04\0\x02\x08\x03\x12\x03\x0b+\
    -\n\x0b\n\x04\x04\0\x02\t\x12\x03\x0c\x08&\n\x0c\n\x05\x04\0\x02\t\x04\
    \x12\x03\x0c\x08\x10\n\x0c\n\x05\x04\0\x02\t\x05\x12\x03\x0c\x11\x17\n\
    \x0c\n\x05\x04\0\x02\t\x01\x12\x03\x0c\x18\x20\n\x0c\n\x05\x04\0\x02\t\
    \x03\x12\x03\x0c#%\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(::protobuf::descriptor::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CEngineGotvSyncPacket::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
