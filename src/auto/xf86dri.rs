// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DrmClipRect {
    pub x1: Int16,
    pub y1: Int16,
    pub x2: Int16,
    pub x3: Int16,
}
impl DrmClipRect {}
impl AsByteSequence for DrmClipRect {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.x1.as_bytes(&mut bytes[index..]);
        index += self.y1.as_bytes(&mut bytes[index..]);
        index += self.x2.as_bytes(&mut bytes[index..]);
        index += self.x3.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DrmClipRect from byte buffer");
        let (x1, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y1, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x2, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x3, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DrmClipRect {
                x1: x1,
                y1: y1,
                x2: x2,
                x3: x3,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.x1.size() + self.y1.size() + self.x2.size() + self.x3.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryVersionRequest {
    pub req_type: u8,
    pub length: u16,
}
impl QueryVersionRequest {}
impl AsByteSequence for QueryVersionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryVersionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryVersionRequest {
                req_type: req_type,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size()
    }
}
impl Request for QueryVersionRequest {
    const OPCODE: u8 = 0;
    const EXTENSION: Option<&'static str> = Some("XFree86-DRI");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryVersionReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryVersionReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub dri_major_version: Card16,
    pub dri_minor_version: Card16,
    pub dri_minor_patch: Card32,
}
impl QueryVersionReply {}
impl AsByteSequence for QueryVersionReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.dri_major_version.as_bytes(&mut bytes[index..]);
        index += self.dri_minor_version.as_bytes(&mut bytes[index..]);
        index += self.dri_minor_patch.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryVersionReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dri_major_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dri_minor_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dri_minor_patch, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryVersionReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                dri_major_version: dri_major_version,
                dri_minor_version: dri_minor_version,
                dri_minor_patch: dri_minor_patch,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + self.dri_major_version.size()
            + self.dri_minor_version.size()
            + self.dri_minor_patch.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryDirectRenderingCapableRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card32,
}
impl QueryDirectRenderingCapableRequest {}
impl AsByteSequence for QueryDirectRenderingCapableRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryDirectRenderingCapableRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryDirectRenderingCapableRequest {
                req_type: req_type,
                length: length,
                screen: screen,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.screen.size()
    }
}
impl Request for QueryDirectRenderingCapableRequest {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = Some("XFree86-DRI");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryDirectRenderingCapableReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryDirectRenderingCapableReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub is_capable: bool,
}
impl QueryDirectRenderingCapableReply {}
impl AsByteSequence for QueryDirectRenderingCapableReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.is_capable.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryDirectRenderingCapableReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (is_capable, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryDirectRenderingCapableReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                is_capable: is_capable,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + self.is_capable.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct OpenConnectionRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card32,
}
impl OpenConnectionRequest {}
impl AsByteSequence for OpenConnectionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing OpenConnectionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            OpenConnectionRequest {
                req_type: req_type,
                length: length,
                screen: screen,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.screen.size()
    }
}
impl Request for OpenConnectionRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = Some("XFree86-DRI");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = OpenConnectionReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct OpenConnectionReply<'a> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub sarea_handle_low: Card32,
    pub sarea_handle_high: Card32,
    pub bus_id: Cow<'a, str>,
}
impl<'a> OpenConnectionReply {}
impl<'a> AsByteSequence for OpenConnectionReply<'a> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.sarea_handle_low.as_bytes(&mut bytes[index..]);
        index += self.sarea_handle_high.as_bytes(&mut bytes[index..]);
        index += (self.bus_id.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = string_as_bytes(&self.bus_id, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing OpenConnectionReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sarea_handle_low, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sarea_handle_high, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (bus_id, block_len): (Cow<'static, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            OpenConnectionReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                sarea_handle_low: sarea_handle_low,
                sarea_handle_high: sarea_handle_high,
                bus_id: bus_id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + self.sarea_handle_low.size()
            + self.sarea_handle_high.size()
            + ::core::mem::size_of::<Card32>()
            + 12
            + {
                let block_len: usize = self.bus_id.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CloseConnectionRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card32,
}
impl CloseConnectionRequest {}
impl AsByteSequence for CloseConnectionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CloseConnectionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CloseConnectionRequest {
                req_type: req_type,
                length: length,
                screen: screen,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.screen.size()
    }
}
impl Request for CloseConnectionRequest {
    const OPCODE: u8 = 3;
    const EXTENSION: Option<&'static str> = Some("XFree86-DRI");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetClientDriverNameRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card32,
}
impl GetClientDriverNameRequest {}
impl AsByteSequence for GetClientDriverNameRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetClientDriverNameRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetClientDriverNameRequest {
                req_type: req_type,
                length: length,
                screen: screen,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.screen.size()
    }
}
impl Request for GetClientDriverNameRequest {
    const OPCODE: u8 = 4;
    const EXTENSION: Option<&'static str> = Some("XFree86-DRI");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetClientDriverNameReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetClientDriverNameReply<'b> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub client_driver_major_version: Card32,
    pub client_driver_minor_version: Card32,
    pub client_driver_patch_version: Card32,
    pub client_driver_name: Cow<'b, str>,
}
impl<'b> GetClientDriverNameReply {}
impl<'b> AsByteSequence for GetClientDriverNameReply<'b> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self
            .client_driver_major_version
            .as_bytes(&mut bytes[index..]);
        index += self
            .client_driver_minor_version
            .as_bytes(&mut bytes[index..]);
        index += self
            .client_driver_patch_version
            .as_bytes(&mut bytes[index..]);
        index += (self.client_driver_name.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 8;
        let block_len: usize = string_as_bytes(&self.client_driver_name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetClientDriverNameReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (client_driver_major_version, sz): (Card32, usize) =
            <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (client_driver_minor_version, sz): (Card32, usize) =
            <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (client_driver_patch_version, sz): (Card32, usize) =
            <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 8;
        let (client_driver_name, block_len): (Cow<'static, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            GetClientDriverNameReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                client_driver_major_version: client_driver_major_version,
                client_driver_minor_version: client_driver_minor_version,
                client_driver_patch_version: client_driver_patch_version,
                client_driver_name: client_driver_name,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + self.client_driver_major_version.size()
            + self.client_driver_minor_version.size()
            + self.client_driver_patch_version.size()
            + ::core::mem::size_of::<Card32>()
            + 8
            + {
                let block_len: usize = self.client_driver_name.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CreateContextRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card32,
    pub visual: Card32,
    pub context: Card32,
}
impl CreateContextRequest {}
impl AsByteSequence for CreateContextRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.visual.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (visual, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CreateContextRequest {
                req_type: req_type,
                length: length,
                screen: screen,
                visual: visual,
                context: context,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.screen.size()
            + self.visual.size()
            + self.context.size()
    }
}
impl Request for CreateContextRequest {
    const OPCODE: u8 = 5;
    const EXTENSION: Option<&'static str> = Some("XFree86-DRI");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = CreateContextReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CreateContextReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub hw_context: Card32,
}
impl CreateContextReply {}
impl AsByteSequence for CreateContextReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.hw_context.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateContextReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hw_context, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CreateContextReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                hw_context: hw_context,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + self.hw_context.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DestroyContextRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card32,
    pub context: Card32,
}
impl DestroyContextRequest {}
impl AsByteSequence for DestroyContextRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DestroyContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DestroyContextRequest {
                req_type: req_type,
                length: length,
                screen: screen,
                context: context,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.screen.size() + self.context.size()
    }
}
impl Request for DestroyContextRequest {
    const OPCODE: u8 = 6;
    const EXTENSION: Option<&'static str> = Some("XFree86-DRI");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CreateDrawableRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card32,
    pub drawable: Card32,
}
impl CreateDrawableRequest {}
impl AsByteSequence for CreateDrawableRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateDrawableRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CreateDrawableRequest {
                req_type: req_type,
                length: length,
                screen: screen,
                drawable: drawable,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.screen.size() + self.drawable.size()
    }
}
impl Request for CreateDrawableRequest {
    const OPCODE: u8 = 7;
    const EXTENSION: Option<&'static str> = Some("XFree86-DRI");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = CreateDrawableReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CreateDrawableReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub hw_drawable_handle: Card32,
}
impl CreateDrawableReply {}
impl AsByteSequence for CreateDrawableReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.hw_drawable_handle.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateDrawableReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hw_drawable_handle, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CreateDrawableReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                hw_drawable_handle: hw_drawable_handle,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + self.hw_drawable_handle.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DestroyDrawableRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card32,
    pub drawable: Card32,
}
impl DestroyDrawableRequest {}
impl AsByteSequence for DestroyDrawableRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DestroyDrawableRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DestroyDrawableRequest {
                req_type: req_type,
                length: length,
                screen: screen,
                drawable: drawable,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.screen.size() + self.drawable.size()
    }
}
impl Request for DestroyDrawableRequest {
    const OPCODE: u8 = 8;
    const EXTENSION: Option<&'static str> = Some("XFree86-DRI");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetDrawableInfoRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card32,
    pub drawable: Card32,
}
impl GetDrawableInfoRequest {}
impl AsByteSequence for GetDrawableInfoRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDrawableInfoRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetDrawableInfoRequest {
                req_type: req_type,
                length: length,
                screen: screen,
                drawable: drawable,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.screen.size() + self.drawable.size()
    }
}
impl Request for GetDrawableInfoRequest {
    const OPCODE: u8 = 9;
    const EXTENSION: Option<&'static str> = Some("XFree86-DRI");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetDrawableInfoReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetDrawableInfoReply<'c, 'd> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub drawable_table_index: Card32,
    pub drawable_table_stamp: Card32,
    pub drawable_origin_x: Int16,
    pub drawable_origin_y: Int16,
    pub drawable_size_w: Int16,
    pub drawable_size_h: Int16,
    pub back_x: Int16,
    pub back_y: Int16,
    pub clip_rects: Cow<'c, [DrmClipRect]>,
    pub back_clip_rects: Cow<'d, [DrmClipRect]>,
}
impl<'c, 'd> GetDrawableInfoReply {}
impl<'c, 'd> AsByteSequence for GetDrawableInfoReply<'c, 'd> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable_table_index.as_bytes(&mut bytes[index..]);
        index += self.drawable_table_stamp.as_bytes(&mut bytes[index..]);
        index += self.drawable_origin_x.as_bytes(&mut bytes[index..]);
        index += self.drawable_origin_y.as_bytes(&mut bytes[index..]);
        index += self.drawable_size_w.as_bytes(&mut bytes[index..]);
        index += self.drawable_size_h.as_bytes(&mut bytes[index..]);
        index += (self.clip_rects.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.back_x.as_bytes(&mut bytes[index..]);
        index += self.back_y.as_bytes(&mut bytes[index..]);
        index += (self.back_clip_rects.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.clip_rects, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<DrmClipRect>());
        let block_len: usize = vector_as_bytes(&self.back_clip_rects, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<DrmClipRect>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDrawableInfoReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable_table_index, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable_table_stamp, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable_origin_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable_origin_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable_size_w, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable_size_h, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (back_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (back_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (clip_rects, block_len): (Cow<'static, [DrmClipRect]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<DrmClipRect>());
        let (back_clip_rects, block_len): (Cow<'static, [DrmClipRect]>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<DrmClipRect>());
        Some((
            GetDrawableInfoReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                drawable_table_index: drawable_table_index,
                drawable_table_stamp: drawable_table_stamp,
                drawable_origin_x: drawable_origin_x,
                drawable_origin_y: drawable_origin_y,
                drawable_size_w: drawable_size_w,
                drawable_size_h: drawable_size_h,
                back_x: back_x,
                back_y: back_y,
                clip_rects: clip_rects,
                back_clip_rects: back_clip_rects,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + self.drawable_table_index.size()
            + self.drawable_table_stamp.size()
            + self.drawable_origin_x.size()
            + self.drawable_origin_y.size()
            + self.drawable_size_w.size()
            + self.drawable_size_h.size()
            + ::core::mem::size_of::<Card32>()
            + self.back_x.size()
            + self.back_y.size()
            + ::core::mem::size_of::<Card32>()
            + {
                let block_len: usize = self.clip_rects.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<DrmClipRect>());
                block_len + pad
            }
            + {
                let block_len: usize = self.back_clip_rects.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<DrmClipRect>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetDeviceInfoRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card32,
}
impl GetDeviceInfoRequest {}
impl AsByteSequence for GetDeviceInfoRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDeviceInfoRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetDeviceInfoRequest {
                req_type: req_type,
                length: length,
                screen: screen,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.screen.size()
    }
}
impl Request for GetDeviceInfoRequest {
    const OPCODE: u8 = 10;
    const EXTENSION: Option<&'static str> = Some("XFree86-DRI");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetDeviceInfoReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetDeviceInfoReply<'e> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub framebuffer_handle_low: Card32,
    pub framebuffer_handle_high: Card32,
    pub framebuffer_origin_offset: Card32,
    pub framebuffer_size: Card32,
    pub framebuffer_stride: Card32,
    pub device_private: Cow<'e, [Card32]>,
}
impl<'e> GetDeviceInfoReply {}
impl<'e> AsByteSequence for GetDeviceInfoReply<'e> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.framebuffer_handle_low.as_bytes(&mut bytes[index..]);
        index += self.framebuffer_handle_high.as_bytes(&mut bytes[index..]);
        index += self.framebuffer_origin_offset.as_bytes(&mut bytes[index..]);
        index += self.framebuffer_size.as_bytes(&mut bytes[index..]);
        index += self.framebuffer_stride.as_bytes(&mut bytes[index..]);
        index += (self.device_private.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.device_private, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDeviceInfoReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (framebuffer_handle_low, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (framebuffer_handle_high, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (framebuffer_origin_offset, sz): (Card32, usize) =
            <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (framebuffer_size, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (framebuffer_stride, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_private, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            GetDeviceInfoReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                framebuffer_handle_low: framebuffer_handle_low,
                framebuffer_handle_high: framebuffer_handle_high,
                framebuffer_origin_offset: framebuffer_origin_offset,
                framebuffer_size: framebuffer_size,
                framebuffer_stride: framebuffer_stride,
                device_private: device_private,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + self.framebuffer_handle_low.size()
            + self.framebuffer_handle_high.size()
            + self.framebuffer_origin_offset.size()
            + self.framebuffer_size.size()
            + self.framebuffer_stride.size()
            + ::core::mem::size_of::<Card32>()
            + {
                let block_len: usize = self.device_private.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct AuthConnectionRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card32,
    pub magic: Card32,
}
impl AuthConnectionRequest {}
impl AsByteSequence for AuthConnectionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.magic.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AuthConnectionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (magic, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            AuthConnectionRequest {
                req_type: req_type,
                length: length,
                screen: screen,
                magic: magic,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.screen.size() + self.magic.size()
    }
}
impl Request for AuthConnectionRequest {
    const OPCODE: u8 = 11;
    const EXTENSION: Option<&'static str> = Some("XFree86-DRI");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = AuthConnectionReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct AuthConnectionReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub authenticated: Card32,
}
impl AuthConnectionReply {}
impl AsByteSequence for AuthConnectionReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.authenticated.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AuthConnectionReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (authenticated, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            AuthConnectionReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                authenticated: authenticated,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + self.authenticated.size()
    }
}
