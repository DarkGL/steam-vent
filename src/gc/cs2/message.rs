use crate::gc::GCMessage;
use super::proto::{
    econ_gcmessages,
    base_gcmessages,
    gcsdk_gcmessages,
    tf_gcmessages,
};

macro_rules! gc_msg {
    ($req:path) => {
        impl GCMessage for $req {}
    };
}