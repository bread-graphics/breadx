// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct GetVersionRequest {
    pub req_type: u8,
    pub length: u16,
    pub client_major_version: Card16,
    pub client_minor_version: Card16,
}
impl GetVersionRequest {}
impl AsByteSequence for GetVersionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.client_major_version.as_bytes(&mut bytes[index..]);
        index += self.client_minor_version.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetVersionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (client_major_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (client_minor_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetVersionRequest {
                req_type: req_type,
                length: length,
                client_major_version: client_major_version,
                client_minor_version: client_minor_version,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.client_major_version.size()
            + self.client_minor_version.size()
    }
}
impl Request for GetVersionRequest {
    const OPCODE: u8 = 0;
    const EXTENSION: Option<&'static str> = Some("XC-MISC");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetVersionReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetVersionReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major_version: Card16,
    pub server_minor_version: Card16,
}
impl GetVersionReply {}
impl AsByteSequence for GetVersionReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.server_major_version.as_bytes(&mut bytes[index..]);
        index += self.server_minor_version.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetVersionReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (server_major_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (server_minor_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetVersionReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                server_major_version: server_major_version,
                server_minor_version: server_minor_version,
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
            + self.server_major_version.size()
            + self.server_minor_version.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetXidRangeRequest {
    pub req_type: u8,
    pub length: u16,
}
impl GetXidRangeRequest {}
impl AsByteSequence for GetXidRangeRequest {
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
        log::trace!("Deserializing GetXidRangeRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetXidRangeRequest {
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
impl Request for GetXidRangeRequest {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = Some("XC-MISC");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetXidRangeReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetXidRangeReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub start_id: Card32,
    pub count: Card32,
}
impl GetXidRangeReply {}
impl AsByteSequence for GetXidRangeReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.start_id.as_bytes(&mut bytes[index..]);
        index += self.count.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetXidRangeReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (start_id, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (count, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetXidRangeReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                start_id: start_id,
                count: count,
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
            + self.start_id.size()
            + self.count.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetXidListRequest {
    pub req_type: u8,
    pub length: u16,
    pub count: Card32,
}
impl GetXidListRequest {}
impl AsByteSequence for GetXidListRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.count.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetXidListRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (count, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetXidListRequest {
                req_type: req_type,
                length: length,
                count: count,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.count.size()
    }
}
impl Request for GetXidListRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = Some("XC-MISC");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetXidListReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetXidListReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ids: Vec<Card32>,
}
impl GetXidListReply {}
impl AsByteSequence for GetXidListReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.ids.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.ids, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetXidListReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        let (ids, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            GetXidListReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                ids: ids,
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
            + ::core::mem::size_of::<Card32>()
            + 20
            + {
                let block_len: usize = self.ids.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
