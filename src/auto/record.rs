// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Context {
    pub xid: XID,
}
impl Context {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Context {
    #[inline]
    fn xid(&self) -> XID {
        self.xid
    }
    #[inline]
    fn from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
#[derive(Clone, Debug, Default)]
pub struct Range8 {
    pub first: Card8,
    pub last: Card8,
}
impl Range8 {}
impl AsByteSequence for Range8 {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.first.as_bytes(&mut bytes[index..]);
        index += self.last.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Range8 from byte buffer");
        let (first, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (last, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Range8 {
                first: first,
                last: last,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.first.size() + self.last.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct Range16 {
    pub first: Card16,
    pub last: Card16,
}
impl Range16 {}
impl AsByteSequence for Range16 {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.first.as_bytes(&mut bytes[index..]);
        index += self.last.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Range16 from byte buffer");
        let (first, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (last, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Range16 {
                first: first,
                last: last,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.first.size() + self.last.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct ExtRange {
    pub major: Range8,
    pub minor: Range16,
}
impl ExtRange {}
impl AsByteSequence for ExtRange {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.major.as_bytes(&mut bytes[index..]);
        index += self.minor.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ExtRange from byte buffer");
        let (major, sz): (Range8, usize) = <Range8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor, sz): (Range16, usize) = <Range16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ExtRange {
                major: major,
                minor: minor,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.major.size() + self.minor.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct Range {
    pub core_requests: Range8,
    pub core_replies: Range8,
    pub ext_requests: ExtRange,
    pub ext_replies: ExtRange,
    pub delivered_events: Range8,
    pub device_events: Range8,
    pub errors: Range8,
    pub client_started: bool,
    pub client_died: bool,
}
impl Range {}
impl AsByteSequence for Range {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.core_requests.as_bytes(&mut bytes[index..]);
        index += self.core_replies.as_bytes(&mut bytes[index..]);
        index += self.ext_requests.as_bytes(&mut bytes[index..]);
        index += self.ext_replies.as_bytes(&mut bytes[index..]);
        index += self.delivered_events.as_bytes(&mut bytes[index..]);
        index += self.device_events.as_bytes(&mut bytes[index..]);
        index += self.errors.as_bytes(&mut bytes[index..]);
        index += self.client_started.as_bytes(&mut bytes[index..]);
        index += self.client_died.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Range from byte buffer");
        let (core_requests, sz): (Range8, usize) = <Range8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (core_replies, sz): (Range8, usize) = <Range8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ext_requests, sz): (ExtRange, usize) = <ExtRange>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ext_replies, sz): (ExtRange, usize) = <ExtRange>::from_bytes(&bytes[index..])?;
        index += sz;
        let (delivered_events, sz): (Range8, usize) = <Range8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_events, sz): (Range8, usize) = <Range8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (errors, sz): (Range8, usize) = <Range8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (client_started, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (client_died, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Range {
                core_requests: core_requests,
                core_replies: core_replies,
                ext_requests: ext_requests,
                ext_replies: ext_replies,
                delivered_events: delivered_events,
                device_events: device_events,
                errors: errors,
                client_started: client_started,
                client_died: client_died,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.core_requests.size()
            + self.core_replies.size()
            + self.ext_requests.size()
            + self.ext_replies.size()
            + self.delivered_events.size()
            + self.device_events.size()
            + self.errors.size()
            + self.client_started.size()
            + self.client_died.size()
    }
}
pub type ElementHeader = Card8;
pub type ClientSpec = Card32;
#[derive(Clone, Debug, Default)]
pub struct ClientInfo {
    pub client_resource: ClientSpec,
    pub ranges: Vec<Range>,
}
impl ClientInfo {}
impl AsByteSequence for ClientInfo {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.client_resource.as_bytes(&mut bytes[index..]);
        index += (self.ranges.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.ranges, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Range>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ClientInfo from byte buffer");
        let (client_resource, sz): (ClientSpec, usize) = <ClientSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ranges, block_len): (Vec<Range>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Range>());
        Some((
            ClientInfo {
                client_resource: client_resource,
                ranges: ranges,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.client_resource.size() + ::core::mem::size_of::<Card32>() + {
            let block_len: usize = self.ranges.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Range>());
            block_len + pad
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct QueryVersionRequest {
    pub req_type: u8,
    pub major_version: Card16,
    pub length: u16,
    pub minor_version: Card16,
}
impl QueryVersionRequest {}
impl AsByteSequence for QueryVersionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.major_version.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.minor_version.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryVersionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryVersionRequest {
                req_type: req_type,
                major_version: major_version,
                length: length,
                minor_version: minor_version,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.major_version.size()
            + self.length.size()
            + self.minor_version.size()
    }
}
impl Request for QueryVersionRequest {
    const OPCODE: u8 = 0;
    const EXTENSION: Option<&'static str> = Some("RECORD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryVersionReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryVersionReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: Card16,
    pub minor_version: Card16,
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
        index += self.major_version.as_bytes(&mut bytes[index..]);
        index += self.minor_version.as_bytes(&mut bytes[index..]);
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
        let (major_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryVersionReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                major_version: major_version,
                minor_version: minor_version,
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
            + self.major_version.size()
            + self.minor_version.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct CreateContextRequest {
    pub req_type: u8,
    pub context: super::record::Context,
    pub length: u16,
    pub element_header: ElementHeader,
    pub client_specs: Vec<ClientSpec>,
    pub ranges: Vec<Range>,
}
impl CreateContextRequest {}
impl AsByteSequence for CreateContextRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.element_header.as_bytes(&mut bytes[index..]);
        index += 3;
        index += (self.client_specs.len() as Card32).as_bytes(&mut bytes[index..]);
        index += (self.ranges.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.client_specs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ClientSpec>());
        let block_len: usize = vector_as_bytes(&self.ranges, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Range>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (super::record::Context, usize) =
            <super::record::Context>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (element_header, sz): (ElementHeader, usize) =
            <ElementHeader>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (client_specs, block_len): (Vec<ClientSpec>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ClientSpec>());
        let (ranges, block_len): (Vec<Range>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Range>());
        Some((
            CreateContextRequest {
                req_type: req_type,
                context: context,
                length: length,
                element_header: element_header,
                client_specs: client_specs,
                ranges: ranges,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context.size()
            + self.length.size()
            + self.element_header.size()
            + 3
            + ::core::mem::size_of::<Card32>()
            + ::core::mem::size_of::<Card32>()
            + {
                let block_len: usize = self.client_specs.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<ClientSpec>());
                block_len + pad
            }
            + {
                let block_len: usize = self.ranges.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Range>());
                block_len + pad
            }
    }
}
impl Request for CreateContextRequest {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = Some("RECORD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct RegisterClientsRequest {
    pub req_type: u8,
    pub context: super::record::Context,
    pub length: u16,
    pub element_header: ElementHeader,
    pub client_specs: Vec<ClientSpec>,
    pub ranges: Vec<Range>,
}
impl RegisterClientsRequest {}
impl AsByteSequence for RegisterClientsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.element_header.as_bytes(&mut bytes[index..]);
        index += 3;
        index += (self.client_specs.len() as Card32).as_bytes(&mut bytes[index..]);
        index += (self.ranges.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.client_specs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ClientSpec>());
        let block_len: usize = vector_as_bytes(&self.ranges, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Range>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing RegisterClientsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (super::record::Context, usize) =
            <super::record::Context>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (element_header, sz): (ElementHeader, usize) =
            <ElementHeader>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (client_specs, block_len): (Vec<ClientSpec>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ClientSpec>());
        let (ranges, block_len): (Vec<Range>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Range>());
        Some((
            RegisterClientsRequest {
                req_type: req_type,
                context: context,
                length: length,
                element_header: element_header,
                client_specs: client_specs,
                ranges: ranges,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context.size()
            + self.length.size()
            + self.element_header.size()
            + 3
            + ::core::mem::size_of::<Card32>()
            + ::core::mem::size_of::<Card32>()
            + {
                let block_len: usize = self.client_specs.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<ClientSpec>());
                block_len + pad
            }
            + {
                let block_len: usize = self.ranges.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Range>());
                block_len + pad
            }
    }
}
impl Request for RegisterClientsRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = Some("RECORD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct UnregisterClientsRequest {
    pub req_type: u8,
    pub context: super::record::Context,
    pub length: u16,
    pub client_specs: Vec<ClientSpec>,
}
impl UnregisterClientsRequest {}
impl AsByteSequence for UnregisterClientsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.client_specs.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.client_specs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ClientSpec>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing UnregisterClientsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (super::record::Context, usize) =
            <super::record::Context>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (client_specs, block_len): (Vec<ClientSpec>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ClientSpec>());
        Some((
            UnregisterClientsRequest {
                req_type: req_type,
                context: context,
                length: length,
                client_specs: client_specs,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context.size()
            + self.length.size()
            + ::core::mem::size_of::<Card32>()
            + {
                let block_len: usize = self.client_specs.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<ClientSpec>());
                block_len + pad
            }
    }
}
impl Request for UnregisterClientsRequest {
    const OPCODE: u8 = 3;
    const EXTENSION: Option<&'static str> = Some("RECORD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GetContextRequest {
    pub req_type: u8,
    pub context: super::record::Context,
    pub length: u16,
}
impl GetContextRequest {}
impl AsByteSequence for GetContextRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (super::record::Context, usize) =
            <super::record::Context>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetContextRequest {
                req_type: req_type,
                context: context,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context.size() + self.length.size()
    }
}
impl Request for GetContextRequest {
    const OPCODE: u8 = 4;
    const EXTENSION: Option<&'static str> = Some("RECORD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetContextReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetContextReply {
    pub reply_type: u8,
    pub enabled: bool,
    pub sequence: u16,
    pub length: u32,
    pub element_header: ElementHeader,
    pub intercepted_clients: Vec<ClientInfo>,
}
impl GetContextReply {}
impl AsByteSequence for GetContextReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.enabled.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.element_header.as_bytes(&mut bytes[index..]);
        index += 3;
        index += (self.intercepted_clients.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 16;
        let block_len: usize = vector_as_bytes(&self.intercepted_clients, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ClientInfo>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetContextReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (enabled, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (element_header, sz): (ElementHeader, usize) =
            <ElementHeader>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 16;
        let (intercepted_clients, block_len): (Vec<ClientInfo>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ClientInfo>());
        Some((
            GetContextReply {
                reply_type: reply_type,
                enabled: enabled,
                sequence: sequence,
                length: length,
                element_header: element_header,
                intercepted_clients: intercepted_clients,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.enabled.size()
            + self.sequence.size()
            + self.length.size()
            + self.element_header.size()
            + 3
            + ::core::mem::size_of::<Card32>()
            + 16
            + {
                let block_len: usize = self.intercepted_clients.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<ClientInfo>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct EnableContextRequest {
    pub req_type: u8,
    pub context: super::record::Context,
    pub length: u16,
}
impl EnableContextRequest {}
impl AsByteSequence for EnableContextRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing EnableContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (super::record::Context, usize) =
            <super::record::Context>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            EnableContextRequest {
                req_type: req_type,
                context: context,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context.size() + self.length.size()
    }
}
impl Request for EnableContextRequest {
    const OPCODE: u8 = 5;
    const EXTENSION: Option<&'static str> = Some("RECORD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = EnableContextReply;
}
#[derive(Clone, Debug, Default)]
pub struct EnableContextReply {
    pub reply_type: u8,
    pub category: Card8,
    pub sequence: u16,
    pub length: u32,
    pub element_header: ElementHeader,
    pub client_swapped: bool,
    pub xid_base: Card32,
    pub server_time: Card32,
    pub rec_sequence_num: Card32,
    pub data: Vec<Byte>,
}
impl EnableContextReply {}
impl AsByteSequence for EnableContextReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.category.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.element_header.as_bytes(&mut bytes[index..]);
        index += self.client_swapped.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.xid_base.as_bytes(&mut bytes[index..]);
        index += self.server_time.as_bytes(&mut bytes[index..]);
        index += self.rec_sequence_num.as_bytes(&mut bytes[index..]);
        index += 8;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing EnableContextReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (category, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (element_header, sz): (ElementHeader, usize) =
            <ElementHeader>::from_bytes(&bytes[index..])?;
        index += sz;
        let (client_swapped, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (xid_base, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (server_time, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (rec_sequence_num, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 8;
        let (data, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize) * (4)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            EnableContextReply {
                reply_type: reply_type,
                category: category,
                sequence: sequence,
                length: length,
                element_header: element_header,
                client_swapped: client_swapped,
                xid_base: xid_base,
                server_time: server_time,
                rec_sequence_num: rec_sequence_num,
                data: data,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.category.size()
            + self.sequence.size()
            + self.length.size()
            + self.element_header.size()
            + self.client_swapped.size()
            + 2
            + self.xid_base.size()
            + self.server_time.size()
            + self.rec_sequence_num.size()
            + 8
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct DisableContextRequest {
    pub req_type: u8,
    pub context: super::record::Context,
    pub length: u16,
}
impl DisableContextRequest {}
impl AsByteSequence for DisableContextRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DisableContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (super::record::Context, usize) =
            <super::record::Context>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DisableContextRequest {
                req_type: req_type,
                context: context,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context.size() + self.length.size()
    }
}
impl Request for DisableContextRequest {
    const OPCODE: u8 = 6;
    const EXTENSION: Option<&'static str> = Some("RECORD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct FreeContextRequest {
    pub req_type: u8,
    pub context: super::record::Context,
    pub length: u16,
}
impl FreeContextRequest {}
impl AsByteSequence for FreeContextRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FreeContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (super::record::Context, usize) =
            <super::record::Context>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            FreeContextRequest {
                req_type: req_type,
                context: context,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context.size() + self.length.size()
    }
}
impl Request for FreeContextRequest {
    const OPCODE: u8 = 7;
    const EXTENSION: Option<&'static str> = Some("RECORD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct HType {
    pub inner: i32,
}
impl HType {
    #[inline]
    pub fn from_server_time(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_from_server_time(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn from_client_time(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_from_client_time(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn from_client_sequence(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_from_client_sequence(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn new(from_server_time: bool, from_client_time: bool, from_client_sequence: bool) -> Self {
        let mut inner: i32 = 0;
        if from_server_time {
            inner |= 1 << 0;
        }
        if from_client_time {
            inner |= 1 << 1;
        }
        if from_client_sequence {
            inner |= 1 << 2;
        }
        HType { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
}
impl AsByteSequence for HType {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        Some((HType { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for HType {
    type Output = HType;
    #[inline]
    fn not(self) -> HType {
        HType { inner: !self.inner }
    }
}
impl core::ops::BitAnd for HType {
    type Output = HType;
    #[inline]
    fn bitand(self, rhs: HType) -> HType {
        HType {
            inner: self.inner & rhs.inner,
        }
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cs {
    CurrentClients = 1,
    FutureClients = 2,
    AllClients = 3,
}
impl AsByteSequence for Cs {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            1 => Some((Self::CurrentClients, sz)),
            2 => Some((Self::FutureClients, sz)),
            3 => Some((Self::AllClients, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for Cs {
    #[inline]
    fn default() -> Cs {
        Cs::CurrentClients
    }
}
#[derive(Clone, Debug, Default)]
pub struct BadContextError {
    pub _error_type: u8,
    pub error_code: u8,
    pub major_code: u8,
    pub minor_code: u8,
    pub sequence: u16,
    pub invalid_record: Card32,
}
impl BadContextError {}
impl AsByteSequence for BadContextError {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self._error_type.as_bytes(&mut bytes[index..]);
        index += self.error_code.as_bytes(&mut bytes[index..]);
        index += self.major_code.as_bytes(&mut bytes[index..]);
        index += self.minor_code.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.invalid_record.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing BadContextError from byte buffer");
        let (_error_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (error_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (invalid_record, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            BadContextError {
                _error_type: _error_type,
                error_code: error_code,
                major_code: major_code,
                minor_code: minor_code,
                sequence: sequence,
                invalid_record: invalid_record,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self._error_type.size()
            + self.error_code.size()
            + self.major_code.size()
            + self.minor_code.size()
            + self.sequence.size()
            + self.invalid_record.size()
    }
}
impl crate::auto::Error for BadContextError {
    const OPCODE: u8 = 0;
}
