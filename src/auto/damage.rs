// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

use super::xfixes::*;
use super::xproto::*;
#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Damage {
    pub xid: XID,
}
impl Damage {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Damage {
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
pub struct QueryVersionRequest {
    pub req_type: u8,
    pub length: u16,
    pub client_major_version: Card32,
    pub client_minor_version: Card32,
}
impl QueryVersionRequest {}
impl AsByteSequence for QueryVersionRequest {
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
        log::trace!("Deserializing QueryVersionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (client_major_version, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (client_minor_version, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryVersionRequest {
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
impl Request for QueryVersionRequest {
    const OPCODE: u8 = 0;
    const EXTENSION: Option<&'static str> = Some("DAMAGE");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryVersionReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryVersionReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: Card32,
    pub minor_version: Card32,
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
        index += 16;
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
        let (major_version, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_version, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 16;
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
            + 16
    }
}
#[derive(Clone, Debug, Default)]
pub struct CreateRequest {
    pub req_type: u8,
    pub damage: Damage,
    pub length: u16,
    pub drawable: Drawable,
    pub level: ReportLevel,
}
impl CreateRequest {}
impl AsByteSequence for CreateRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.damage.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.level.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (damage, sz): (Damage, usize) = <Damage>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (level, sz): (ReportLevel, usize) = <ReportLevel>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            CreateRequest {
                req_type: req_type,
                damage: damage,
                length: length,
                drawable: drawable,
                level: level,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.damage.size()
            + self.length.size()
            + self.drawable.size()
            + self.level.size()
            + 3
    }
}
impl Request for CreateRequest {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = Some("DAMAGE");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReportLevel {
    RawRectangles = 0,
    DeltaRectangles = 1,
    BoundingBox = 2,
    NonEmpty = 3,
}
impl AsByteSequence for ReportLevel {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::RawRectangles, sz)),
            1 => Some((Self::DeltaRectangles, sz)),
            2 => Some((Self::BoundingBox, sz)),
            3 => Some((Self::NonEmpty, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for ReportLevel {
    #[inline]
    fn default() -> ReportLevel {
        ReportLevel::RawRectangles
    }
}
#[derive(Clone, Debug, Default)]
pub struct DestroyRequest {
    pub req_type: u8,
    pub damage: Damage,
    pub length: u16,
}
impl DestroyRequest {}
impl AsByteSequence for DestroyRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.damage.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DestroyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (damage, sz): (Damage, usize) = <Damage>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DestroyRequest {
                req_type: req_type,
                damage: damage,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.damage.size() + self.length.size()
    }
}
impl Request for DestroyRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = Some("DAMAGE");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct SubtractRequest {
    pub req_type: u8,
    pub damage: Damage,
    pub length: u16,
    pub repair: Region,
    pub parts: Region,
}
impl SubtractRequest {}
impl AsByteSequence for SubtractRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.damage.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.repair.as_bytes(&mut bytes[index..]);
        index += self.parts.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SubtractRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (damage, sz): (Damage, usize) = <Damage>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (repair, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (parts, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SubtractRequest {
                req_type: req_type,
                damage: damage,
                length: length,
                repair: repair,
                parts: parts,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.damage.size()
            + self.length.size()
            + self.repair.size()
            + self.parts.size()
    }
}
impl Request for SubtractRequest {
    const OPCODE: u8 = 3;
    const EXTENSION: Option<&'static str> = Some("DAMAGE");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct AddRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub region: Region,
}
impl AddRequest {}
impl AsByteSequence for AddRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.region.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AddRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (region, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            AddRequest {
                req_type: req_type,
                length: length,
                drawable: drawable,
                region: region,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.drawable.size() + self.region.size()
    }
}
impl Request for AddRequest {
    const OPCODE: u8 = 4;
    const EXTENSION: Option<&'static str> = Some("DAMAGE");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct NotifyEvent {
    pub event_type: u8,
    pub level: ReportLevel,
    pub sequence: u16,
    pub drawable: Drawable,
    pub damage: Damage,
    pub timestamp: Timestamp,
    pub area: Rectangle,
    pub geometry: Rectangle,
}
impl NotifyEvent {}
impl AsByteSequence for NotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.level.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.damage.as_bytes(&mut bytes[index..]);
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += self.area.as_bytes(&mut bytes[index..]);
        index += self.geometry.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing NotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (level, sz): (ReportLevel, usize) = <ReportLevel>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (damage, sz): (Damage, usize) = <Damage>::from_bytes(&bytes[index..])?;
        index += sz;
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (area, sz): (Rectangle, usize) = <Rectangle>::from_bytes(&bytes[index..])?;
        index += sz;
        let (geometry, sz): (Rectangle, usize) = <Rectangle>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            NotifyEvent {
                event_type: event_type,
                level: level,
                sequence: sequence,
                drawable: drawable,
                damage: damage,
                timestamp: timestamp,
                area: area,
                geometry: geometry,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.level.size()
            + self.sequence.size()
            + self.drawable.size()
            + self.damage.size()
            + self.timestamp.size()
            + self.area.size()
            + self.geometry.size()
    }
}
impl crate::auto::Event for NotifyEvent {
    const OPCODE: u8 = 0;
}
