// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

use super::xproto::*;
pub type Op = Card8;
pub type Kind = Card8;
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Sk {
    Bounding = 0,
    Clip = 1,
    Input = 2,
}
impl AsByteSequence for Sk {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Bounding, sz)),
            1 => Some((Self::Clip, sz)),
            2 => Some((Self::Input, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for Sk {
    #[inline]
    fn default() -> Sk {
        Sk::Bounding
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
    const EXTENSION: Option<&'static str> = Some("SHAPE");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryVersionReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
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
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct RectanglesRequest<'a> {
    pub req_type: u8,
    pub length: u16,
    pub operation: So,
    pub destination_kind: Sk,
    pub ordering: ClipOrdering,
    pub destination_window: Window,
    pub x_offset: Int16,
    pub y_offset: Int16,
    pub rectangles: Cow<'a, [Rectangle]>,
}
impl<'a> RectanglesRequest<'a> {}
impl<'a> AsByteSequence for RectanglesRequest<'a> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.operation.as_bytes(&mut bytes[index..]);
        index += self.destination_kind.as_bytes(&mut bytes[index..]);
        index += self.ordering.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.destination_window.as_bytes(&mut bytes[index..]);
        index += self.x_offset.as_bytes(&mut bytes[index..]);
        index += self.y_offset.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.rectangles, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing RectanglesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (operation, sz): (So, usize) = <So>::from_bytes(&bytes[index..])?;
        index += sz;
        let (destination_kind, sz): (Sk, usize) = <Sk>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ordering, sz): (ClipOrdering, usize) = <ClipOrdering>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (destination_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x_offset, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y_offset, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (rectangles, block_len): (Cow<'_, [Rectangle]>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
        Some((
            RectanglesRequest {
                req_type: req_type,
                length: length,
                operation: operation,
                destination_kind: destination_kind,
                ordering: ordering,
                destination_window: destination_window,
                x_offset: x_offset,
                y_offset: y_offset,
                rectangles: rectangles,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.operation.size()
            + self.destination_kind.size()
            + self.ordering.size()
            + 1
            + self.destination_window.size()
            + self.x_offset.size()
            + self.y_offset.size()
            + {
                let block_len: usize = self.rectangles.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
                block_len + pad
            }
    }
}
impl<'a> Request for RectanglesRequest<'a> {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = Some("SHAPE");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum So {
    Set = 0,
    Union = 1,
    Intersect = 2,
    Subtract = 3,
    Invert = 4,
}
impl AsByteSequence for So {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Set, sz)),
            1 => Some((Self::Union, sz)),
            2 => Some((Self::Intersect, sz)),
            3 => Some((Self::Subtract, sz)),
            4 => Some((Self::Invert, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for So {
    #[inline]
    fn default() -> So {
        So::Set
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct MaskRequest {
    pub req_type: u8,
    pub length: u16,
    pub operation: So,
    pub destination_kind: Sk,
    pub destination_window: Window,
    pub x_offset: Int16,
    pub y_offset: Int16,
    pub source_bitmap: Pixmap,
}
impl MaskRequest {}
impl AsByteSequence for MaskRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.operation.as_bytes(&mut bytes[index..]);
        index += self.destination_kind.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.destination_window.as_bytes(&mut bytes[index..]);
        index += self.x_offset.as_bytes(&mut bytes[index..]);
        index += self.y_offset.as_bytes(&mut bytes[index..]);
        index += self.source_bitmap.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing MaskRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (operation, sz): (So, usize) = <So>::from_bytes(&bytes[index..])?;
        index += sz;
        let (destination_kind, sz): (Sk, usize) = <Sk>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (destination_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x_offset, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y_offset, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (source_bitmap, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            MaskRequest {
                req_type: req_type,
                length: length,
                operation: operation,
                destination_kind: destination_kind,
                destination_window: destination_window,
                x_offset: x_offset,
                y_offset: y_offset,
                source_bitmap: source_bitmap,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.operation.size()
            + self.destination_kind.size()
            + 2
            + self.destination_window.size()
            + self.x_offset.size()
            + self.y_offset.size()
            + self.source_bitmap.size()
    }
}
impl Request for MaskRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = Some("SHAPE");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CombineRequest {
    pub req_type: u8,
    pub length: u16,
    pub operation: So,
    pub destination_kind: Sk,
    pub source_kind: Sk,
    pub destination_window: Window,
    pub x_offset: Int16,
    pub y_offset: Int16,
    pub source_window: Window,
}
impl CombineRequest {}
impl AsByteSequence for CombineRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.operation.as_bytes(&mut bytes[index..]);
        index += self.destination_kind.as_bytes(&mut bytes[index..]);
        index += self.source_kind.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.destination_window.as_bytes(&mut bytes[index..]);
        index += self.x_offset.as_bytes(&mut bytes[index..]);
        index += self.y_offset.as_bytes(&mut bytes[index..]);
        index += self.source_window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CombineRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (operation, sz): (So, usize) = <So>::from_bytes(&bytes[index..])?;
        index += sz;
        let (destination_kind, sz): (Sk, usize) = <Sk>::from_bytes(&bytes[index..])?;
        index += sz;
        let (source_kind, sz): (Sk, usize) = <Sk>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (destination_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x_offset, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y_offset, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (source_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CombineRequest {
                req_type: req_type,
                length: length,
                operation: operation,
                destination_kind: destination_kind,
                source_kind: source_kind,
                destination_window: destination_window,
                x_offset: x_offset,
                y_offset: y_offset,
                source_window: source_window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.operation.size()
            + self.destination_kind.size()
            + self.source_kind.size()
            + 1
            + self.destination_window.size()
            + self.x_offset.size()
            + self.y_offset.size()
            + self.source_window.size()
    }
}
impl Request for CombineRequest {
    const OPCODE: u8 = 3;
    const EXTENSION: Option<&'static str> = Some("SHAPE");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct OffsetRequest {
    pub req_type: u8,
    pub length: u16,
    pub destination_kind: Sk,
    pub destination_window: Window,
    pub x_offset: Int16,
    pub y_offset: Int16,
}
impl OffsetRequest {}
impl AsByteSequence for OffsetRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.destination_kind.as_bytes(&mut bytes[index..]);
        index += 3;
        index += self.destination_window.as_bytes(&mut bytes[index..]);
        index += self.x_offset.as_bytes(&mut bytes[index..]);
        index += self.y_offset.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing OffsetRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (destination_kind, sz): (Sk, usize) = <Sk>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (destination_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x_offset, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y_offset, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            OffsetRequest {
                req_type: req_type,
                length: length,
                destination_kind: destination_kind,
                destination_window: destination_window,
                x_offset: x_offset,
                y_offset: y_offset,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.destination_kind.size()
            + 3
            + self.destination_window.size()
            + self.x_offset.size()
            + self.y_offset.size()
    }
}
impl Request for OffsetRequest {
    const OPCODE: u8 = 4;
    const EXTENSION: Option<&'static str> = Some("SHAPE");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryExtentsRequest {
    pub req_type: u8,
    pub length: u16,
    pub destination_window: Window,
}
impl QueryExtentsRequest {}
impl AsByteSequence for QueryExtentsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.destination_window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryExtentsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (destination_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryExtentsRequest {
                req_type: req_type,
                length: length,
                destination_window: destination_window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.destination_window.size()
    }
}
impl Request for QueryExtentsRequest {
    const OPCODE: u8 = 5;
    const EXTENSION: Option<&'static str> = Some("SHAPE");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryExtentsReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryExtentsReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub bounding_shaped: bool,
    pub clip_shaped: bool,
    pub bounding_shape_extents_x: Int16,
    pub bounding_shape_extents_y: Int16,
    pub bounding_shape_extents_width: Card16,
    pub bounding_shape_extents_height: Card16,
    pub clip_shape_extents_x: Int16,
    pub clip_shape_extents_y: Int16,
    pub clip_shape_extents_width: Card16,
    pub clip_shape_extents_height: Card16,
}
impl QueryExtentsReply {}
impl AsByteSequence for QueryExtentsReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.bounding_shaped.as_bytes(&mut bytes[index..]);
        index += self.clip_shaped.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.bounding_shape_extents_x.as_bytes(&mut bytes[index..]);
        index += self.bounding_shape_extents_y.as_bytes(&mut bytes[index..]);
        index += self
            .bounding_shape_extents_width
            .as_bytes(&mut bytes[index..]);
        index += self
            .bounding_shape_extents_height
            .as_bytes(&mut bytes[index..]);
        index += self.clip_shape_extents_x.as_bytes(&mut bytes[index..]);
        index += self.clip_shape_extents_y.as_bytes(&mut bytes[index..]);
        index += self.clip_shape_extents_width.as_bytes(&mut bytes[index..]);
        index += self.clip_shape_extents_height.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryExtentsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bounding_shaped, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (clip_shaped, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (bounding_shape_extents_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bounding_shape_extents_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bounding_shape_extents_width, sz): (Card16, usize) =
            <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bounding_shape_extents_height, sz): (Card16, usize) =
            <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (clip_shape_extents_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (clip_shape_extents_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (clip_shape_extents_width, sz): (Card16, usize) =
            <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (clip_shape_extents_height, sz): (Card16, usize) =
            <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryExtentsReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                bounding_shaped: bounding_shaped,
                clip_shaped: clip_shaped,
                bounding_shape_extents_x: bounding_shape_extents_x,
                bounding_shape_extents_y: bounding_shape_extents_y,
                bounding_shape_extents_width: bounding_shape_extents_width,
                bounding_shape_extents_height: bounding_shape_extents_height,
                clip_shape_extents_x: clip_shape_extents_x,
                clip_shape_extents_y: clip_shape_extents_y,
                clip_shape_extents_width: clip_shape_extents_width,
                clip_shape_extents_height: clip_shape_extents_height,
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
            + self.bounding_shaped.size()
            + self.clip_shaped.size()
            + 2
            + self.bounding_shape_extents_x.size()
            + self.bounding_shape_extents_y.size()
            + self.bounding_shape_extents_width.size()
            + self.bounding_shape_extents_height.size()
            + self.clip_shape_extents_x.size()
            + self.clip_shape_extents_y.size()
            + self.clip_shape_extents_width.size()
            + self.clip_shape_extents_height.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SelectInputRequest {
    pub req_type: u8,
    pub length: u16,
    pub destination_window: Window,
    pub enable: bool,
}
impl SelectInputRequest {}
impl AsByteSequence for SelectInputRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.destination_window.as_bytes(&mut bytes[index..]);
        index += self.enable.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SelectInputRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (destination_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (enable, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            SelectInputRequest {
                req_type: req_type,
                length: length,
                destination_window: destination_window,
                enable: enable,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.destination_window.size()
            + self.enable.size()
            + 3
    }
}
impl Request for SelectInputRequest {
    const OPCODE: u8 = 6;
    const EXTENSION: Option<&'static str> = Some("SHAPE");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct InputSelectedRequest {
    pub req_type: u8,
    pub length: u16,
    pub destination_window: Window,
}
impl InputSelectedRequest {}
impl AsByteSequence for InputSelectedRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.destination_window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing InputSelectedRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (destination_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            InputSelectedRequest {
                req_type: req_type,
                length: length,
                destination_window: destination_window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.destination_window.size()
    }
}
impl Request for InputSelectedRequest {
    const OPCODE: u8 = 7;
    const EXTENSION: Option<&'static str> = Some("SHAPE");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = InputSelectedReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct InputSelectedReply {
    pub reply_type: u8,
    pub enabled: bool,
    pub sequence: u16,
    pub length: u32,
}
impl InputSelectedReply {}
impl AsByteSequence for InputSelectedReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.enabled.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing InputSelectedReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (enabled, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            InputSelectedReply {
                reply_type: reply_type,
                enabled: enabled,
                sequence: sequence,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + self.enabled.size() + self.sequence.size() + self.length.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetRectanglesRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub source_kind: Sk,
}
impl GetRectanglesRequest {}
impl AsByteSequence for GetRectanglesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.source_kind.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetRectanglesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (source_kind, sz): (Sk, usize) = <Sk>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            GetRectanglesRequest {
                req_type: req_type,
                length: length,
                window: window,
                source_kind: source_kind,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.window.size()
            + self.source_kind.size()
            + 3
    }
}
impl Request for GetRectanglesRequest {
    const OPCODE: u8 = 8;
    const EXTENSION: Option<&'static str> = Some("SHAPE");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetRectanglesReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetRectanglesReply<'b> {
    pub reply_type: u8,
    pub ordering: ClipOrdering,
    pub sequence: u16,
    pub length: u32,
    pub rectangles: Cow<'b, [Rectangle]>,
}
impl<'b> GetRectanglesReply<'b> {}
impl<'b> AsByteSequence for GetRectanglesReply<'b> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.ordering.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.rectangles.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.rectangles, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetRectanglesReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ordering, sz): (ClipOrdering, usize) = <ClipOrdering>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        let (rectangles, block_len): (Cow<'_, [Rectangle]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
        Some((
            GetRectanglesReply {
                reply_type: reply_type,
                ordering: ordering,
                sequence: sequence,
                length: length,
                rectangles: rectangles,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.ordering.size()
            + self.sequence.size()
            + self.length.size()
            + ::core::mem::size_of::<Card32>()
            + 20
            + {
                let block_len: usize = self.rectangles.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct NotifyEvent {
    pub event_type: u8,
    pub shape_kind: Sk,
    pub sequence: u16,
    pub affected_window: Window,
    pub extents_x: Int16,
    pub extents_y: Int16,
    pub extents_width: Card16,
    pub extents_height: Card16,
    pub server_time: Timestamp,
    pub shaped: bool,
}
impl NotifyEvent {}
impl AsByteSequence for NotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.shape_kind.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.affected_window.as_bytes(&mut bytes[index..]);
        index += self.extents_x.as_bytes(&mut bytes[index..]);
        index += self.extents_y.as_bytes(&mut bytes[index..]);
        index += self.extents_width.as_bytes(&mut bytes[index..]);
        index += self.extents_height.as_bytes(&mut bytes[index..]);
        index += self.server_time.as_bytes(&mut bytes[index..]);
        index += self.shaped.as_bytes(&mut bytes[index..]);
        index += 11;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing NotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (shape_kind, sz): (Sk, usize) = <Sk>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (affected_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (extents_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (extents_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (extents_width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (extents_height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (server_time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (shaped, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 11;
        Some((
            NotifyEvent {
                event_type: event_type,
                shape_kind: shape_kind,
                sequence: sequence,
                affected_window: affected_window,
                extents_x: extents_x,
                extents_y: extents_y,
                extents_width: extents_width,
                extents_height: extents_height,
                server_time: server_time,
                shaped: shaped,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.shape_kind.size()
            + self.sequence.size()
            + self.affected_window.size()
            + self.extents_x.size()
            + self.extents_y.size()
            + self.extents_width.size()
            + self.extents_height.size()
            + self.server_time.size()
            + self.shaped.size()
            + 11
    }
}
impl crate::auto::Event for NotifyEvent {
    const OPCODE: u8 = 0;
}
