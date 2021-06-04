// MIT/Apache2 License

use super::auto::xproto::QueryExtensionReply;
use alloc::borrow::Cow;

/// Data related to an extension.
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Extension {
    pub major_opcode: u8,
    pub first_event: u8,
    pub first_error: u8,
}

impl Extension {
    #[inline]
    pub(crate) fn from_reply(qer: QueryExtensionReply) -> crate::Result<Extension> {
        if qer.present {
            Ok(Self {
                major_opcode: qer.major_opcode,
                first_event: qer.first_event,
                first_error: qer.first_error,
            })
        } else {
            //Err(crate::BreadError::ExtensionNotPresent(ext_name))
            unimplemented!("Extension not present")
        }
    }
}

/// Version of an extension.
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ExtensionVersion {
    pub major: u32,
    pub minor: u32,
}
