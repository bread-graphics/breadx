// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

use super::xproto::*;
#[derive(Clone, Debug, Default)]
pub struct GetVersionRequest {
    pub req_type: u8,
    pub major_version: Card8,
    pub length: u16,
    pub minor_version: Card16,
}
impl GetVersionRequest {}
impl AsByteSequence for GetVersionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.major_version.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.minor_version.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetVersionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_version, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (minor_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetVersionRequest {
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
            + 1
            + self.minor_version.size()
    }
}
impl Request for GetVersionRequest {
    const OPCODE: u8 = 0;
    const EXTENSION: Option<&'static str> = Some("XTEST");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetVersionReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetVersionReply {
    pub reply_type: u8,
    pub major_version: Card8,
    pub sequence: u16,
    pub length: u32,
    pub minor_version: Card16,
}
impl GetVersionReply {}
impl AsByteSequence for GetVersionReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.major_version.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.minor_version.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetVersionReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_version, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetVersionReply {
                reply_type: reply_type,
                major_version: major_version,
                sequence: sequence,
                length: length,
                minor_version: minor_version,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.major_version.size()
            + self.sequence.size()
            + self.length.size()
            + self.minor_version.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct CompareCursorRequest {
    pub req_type: u8,
    pub window: Window,
    pub length: u16,
    pub cursor: Cursor,
}
impl CompareCursorRequest {}
impl AsByteSequence for CompareCursorRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.cursor.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CompareCursorRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cursor, sz): (Cursor, usize) = <Cursor>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CompareCursorRequest {
                req_type: req_type,
                window: window,
                length: length,
                cursor: cursor,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.window.size() + self.length.size() + self.cursor.size()
    }
}
impl Request for CompareCursorRequest {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = Some("XTEST");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = CompareCursorReply;
}
#[derive(Clone, Debug, Default)]
pub struct CompareCursorReply {
    pub reply_type: u8,
    pub same: bool,
    pub sequence: u16,
    pub length: u32,
}
impl CompareCursorReply {}
impl AsByteSequence for CompareCursorReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.same.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CompareCursorReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (same, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CompareCursorReply {
                reply_type: reply_type,
                same: same,
                sequence: sequence,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + self.same.size() + self.sequence.size() + self.length.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct FakeInputRequest {
    pub req_type: u8,
    pub ty: Byte,
    pub length: u16,
    pub detail: Byte,
    pub time: Card32,
    pub root: Window,
    pub root_x: Int16,
    pub root_y: Int16,
    pub deviceid: Card8,
}
impl FakeInputRequest {}
impl AsByteSequence for FakeInputRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += 8;
        index += self.root_x.as_bytes(&mut bytes[index..]);
        index += self.root_y.as_bytes(&mut bytes[index..]);
        index += 7;
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FakeInputRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Byte, usize) = <Byte>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Byte, usize) = <Byte>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (time, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 8;
        let (root_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 7;
        let (deviceid, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            FakeInputRequest {
                req_type: req_type,
                ty: ty,
                length: length,
                detail: detail,
                time: time,
                root: root,
                root_x: root_x,
                root_y: root_y,
                deviceid: deviceid,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.ty.size()
            + self.length.size()
            + self.detail.size()
            + 2
            + self.time.size()
            + self.root.size()
            + 8
            + self.root_x.size()
            + self.root_y.size()
            + 7
            + self.deviceid.size()
    }
}
impl Request for FakeInputRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = Some("XTEST");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GrabControlRequest {
    pub req_type: u8,
    pub impervious: bool,
    pub length: u16,
}
impl GrabControlRequest {}
impl AsByteSequence for GrabControlRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.impervious.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GrabControlRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (impervious, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            GrabControlRequest {
                req_type: req_type,
                impervious: impervious,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.impervious.size() + self.length.size() + 3
    }
}
impl Request for GrabControlRequest {
    const OPCODE: u8 = 3;
    const EXTENSION: Option<&'static str> = Some("XTEST");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cursor {
    None = 0,
    Current = 1,
}
impl AsByteSequence for Cursor {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::None, sz)),
            1 => Some((Self::Current, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for Cursor {
    #[inline]
    fn default() -> Cursor {
        Cursor::None
    }
}
