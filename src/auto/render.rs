// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

use super::xproto::*;
pub type Glyph = Card32;
#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Glyphset {
    pub xid: XID,
}
impl Glyphset {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Glyphset {
    #[inline]
    fn xid(&self) -> XID {
        self.xid
    }
    #[inline]
    fn from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Picture {
    pub xid: XID,
}
impl Picture {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Picture {
    #[inline]
    fn xid(&self) -> XID {
        self.xid
    }
    #[inline]
    fn from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pictformat {
    pub xid: XID,
}
impl Pictformat {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Pictformat {
    #[inline]
    fn xid(&self) -> XID {
        self.xid
    }
    #[inline]
    fn from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
pub type Fixed = Int32;
#[derive(Clone, Debug, Default)]
pub struct Directformat {
    pub red_shift: Card16,
    pub red_mask: Card16,
    pub green_shift: Card16,
    pub green_mask: Card16,
    pub blue_shift: Card16,
    pub blue_mask: Card16,
    pub alpha_shift: Card16,
    pub alpha_mask: Card16,
}
impl Directformat {}
impl AsByteSequence for Directformat {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.red_shift.as_bytes(&mut bytes[index..]);
        index += self.red_mask.as_bytes(&mut bytes[index..]);
        index += self.green_shift.as_bytes(&mut bytes[index..]);
        index += self.green_mask.as_bytes(&mut bytes[index..]);
        index += self.blue_shift.as_bytes(&mut bytes[index..]);
        index += self.blue_mask.as_bytes(&mut bytes[index..]);
        index += self.alpha_shift.as_bytes(&mut bytes[index..]);
        index += self.alpha_mask.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Directformat from byte buffer");
        let (red_shift, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (red_mask, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (green_shift, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (green_mask, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (blue_shift, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (blue_mask, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (alpha_shift, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (alpha_mask, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Directformat {
                red_shift: red_shift,
                red_mask: red_mask,
                green_shift: green_shift,
                green_mask: green_mask,
                blue_shift: blue_shift,
                blue_mask: blue_mask,
                alpha_shift: alpha_shift,
                alpha_mask: alpha_mask,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.red_shift.size()
            + self.red_mask.size()
            + self.green_shift.size()
            + self.green_mask.size()
            + self.blue_shift.size()
            + self.blue_mask.size()
            + self.alpha_shift.size()
            + self.alpha_mask.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct Pictforminfo {
    pub id: Pictformat,
    pub ty: PictType,
    pub depth: Card8,
    pub direct: Directformat,
    pub colormap: Colormap,
}
impl Pictforminfo {}
impl AsByteSequence for Pictforminfo {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.id.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.depth.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.direct.as_bytes(&mut bytes[index..]);
        index += self.colormap.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Pictforminfo from byte buffer");
        let (id, sz): (Pictformat, usize) = <Pictformat>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (PictType, usize) = <PictType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (depth, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (direct, sz): (Directformat, usize) = <Directformat>::from_bytes(&bytes[index..])?;
        index += sz;
        let (colormap, sz): (Colormap, usize) = <Colormap>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Pictforminfo {
                id: id,
                ty: ty,
                depth: depth,
                direct: direct,
                colormap: colormap,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.id.size()
            + self.ty.size()
            + self.depth.size()
            + 2
            + self.direct.size()
            + self.colormap.size()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PictType {
    Indexed = 0,
    Direct = 1,
}
impl AsByteSequence for PictType {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Indexed, sz)),
            1 => Some((Self::Direct, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for PictType {
    #[inline]
    fn default() -> PictType {
        PictType::Indexed
    }
}
#[derive(Clone, Debug, Default)]
pub struct Pictvisual {
    pub visual: Visualid,
    pub format: Pictformat,
}
impl Pictvisual {}
impl AsByteSequence for Pictvisual {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.visual.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Pictvisual from byte buffer");
        let (visual, sz): (Visualid, usize) = <Visualid>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (Pictformat, usize) = <Pictformat>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Pictvisual {
                visual: visual,
                format: format,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.visual.size() + self.format.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct Pictdepth {
    pub depth: Card8,
    pub visuals: Vec<Pictvisual>,
}
impl Pictdepth {}
impl AsByteSequence for Pictdepth {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.depth.as_bytes(&mut bytes[index..]);
        index += 1;
        index += (self.visuals.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 4;
        let block_len: usize = vector_as_bytes(&self.visuals, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Pictvisual>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Pictdepth from byte buffer");
        let (depth, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (visuals, block_len): (Vec<Pictvisual>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Pictvisual>());
        Some((
            Pictdepth {
                depth: depth,
                visuals: visuals,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.depth.size() + 1 + ::core::mem::size_of::<Card16>() + 4 + {
            let block_len: usize = self.visuals.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Pictvisual>());
            block_len + pad
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct Pictscreen {
    pub fallback: Pictformat,
    pub depths: Vec<Pictdepth>,
}
impl Pictscreen {}
impl AsByteSequence for Pictscreen {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += (self.depths.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.fallback.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.depths, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Pictdepth>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Pictscreen from byte buffer");
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (fallback, sz): (Pictformat, usize) = <Pictformat>::from_bytes(&bytes[index..])?;
        index += sz;
        let (depths, block_len): (Vec<Pictdepth>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Pictdepth>());
        Some((
            Pictscreen {
                fallback: fallback,
                depths: depths,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<Card32>() + self.fallback.size() + {
            let block_len: usize = self.depths.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Pictdepth>());
            block_len + pad
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct Indexvalue {
    pub pixel: Card32,
    pub red: Card16,
    pub green: Card16,
    pub blue: Card16,
    pub alpha: Card16,
}
impl Indexvalue {}
impl AsByteSequence for Indexvalue {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.pixel.as_bytes(&mut bytes[index..]);
        index += self.red.as_bytes(&mut bytes[index..]);
        index += self.green.as_bytes(&mut bytes[index..]);
        index += self.blue.as_bytes(&mut bytes[index..]);
        index += self.alpha.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Indexvalue from byte buffer");
        let (pixel, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (red, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (green, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (blue, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (alpha, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Indexvalue {
                pixel: pixel,
                red: red,
                green: green,
                blue: blue,
                alpha: alpha,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.pixel.size()
            + self.red.size()
            + self.green.size()
            + self.blue.size()
            + self.alpha.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct Color {
    pub red: Card16,
    pub green: Card16,
    pub blue: Card16,
    pub alpha: Card16,
}
impl Color {}
impl AsByteSequence for Color {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.red.as_bytes(&mut bytes[index..]);
        index += self.green.as_bytes(&mut bytes[index..]);
        index += self.blue.as_bytes(&mut bytes[index..]);
        index += self.alpha.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Color from byte buffer");
        let (red, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (green, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (blue, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (alpha, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Color {
                red: red,
                green: green,
                blue: blue,
                alpha: alpha,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.red.size() + self.green.size() + self.blue.size() + self.alpha.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct Pointfix {
    pub x: Fixed,
    pub y: Fixed,
}
impl Pointfix {}
impl AsByteSequence for Pointfix {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Pointfix from byte buffer");
        let (x, sz): (Fixed, usize) = <Fixed>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Fixed, usize) = <Fixed>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((Pointfix { x: x, y: y }, index))
    }
    #[inline]
    fn size(&self) -> usize {
        self.x.size() + self.y.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct Linefix {
    pub p1: Pointfix,
    pub p2: Pointfix,
}
impl Linefix {}
impl AsByteSequence for Linefix {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.p1.as_bytes(&mut bytes[index..]);
        index += self.p2.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Linefix from byte buffer");
        let (p1, sz): (Pointfix, usize) = <Pointfix>::from_bytes(&bytes[index..])?;
        index += sz;
        let (p2, sz): (Pointfix, usize) = <Pointfix>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((Linefix { p1: p1, p2: p2 }, index))
    }
    #[inline]
    fn size(&self) -> usize {
        self.p1.size() + self.p2.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct Triangle {
    pub p1: Pointfix,
    pub p2: Pointfix,
    pub p3: Pointfix,
}
impl Triangle {}
impl AsByteSequence for Triangle {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.p1.as_bytes(&mut bytes[index..]);
        index += self.p2.as_bytes(&mut bytes[index..]);
        index += self.p3.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Triangle from byte buffer");
        let (p1, sz): (Pointfix, usize) = <Pointfix>::from_bytes(&bytes[index..])?;
        index += sz;
        let (p2, sz): (Pointfix, usize) = <Pointfix>::from_bytes(&bytes[index..])?;
        index += sz;
        let (p3, sz): (Pointfix, usize) = <Pointfix>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Triangle {
                p1: p1,
                p2: p2,
                p3: p3,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.p1.size() + self.p2.size() + self.p3.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct Trapezoid {
    pub top: Fixed,
    pub bottom: Fixed,
    pub left: Linefix,
    pub right: Linefix,
}
impl Trapezoid {}
impl AsByteSequence for Trapezoid {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.top.as_bytes(&mut bytes[index..]);
        index += self.bottom.as_bytes(&mut bytes[index..]);
        index += self.left.as_bytes(&mut bytes[index..]);
        index += self.right.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Trapezoid from byte buffer");
        let (top, sz): (Fixed, usize) = <Fixed>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bottom, sz): (Fixed, usize) = <Fixed>::from_bytes(&bytes[index..])?;
        index += sz;
        let (left, sz): (Linefix, usize) = <Linefix>::from_bytes(&bytes[index..])?;
        index += sz;
        let (right, sz): (Linefix, usize) = <Linefix>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Trapezoid {
                top: top,
                bottom: bottom,
                left: left,
                right: right,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.top.size() + self.bottom.size() + self.left.size() + self.right.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct Glyphinfo {
    pub width: Card16,
    pub height: Card16,
    pub x: Int16,
    pub y: Int16,
    pub x_off: Int16,
    pub y_off: Int16,
}
impl Glyphinfo {}
impl AsByteSequence for Glyphinfo {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index += self.x_off.as_bytes(&mut bytes[index..]);
        index += self.y_off.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Glyphinfo from byte buffer");
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x_off, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y_off, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Glyphinfo {
                width: width,
                height: height,
                x: x,
                y: y,
                x_off: x_off,
                y_off: y_off,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.width.size()
            + self.height.size()
            + self.x.size()
            + self.y.size()
            + self.x_off.size()
            + self.y_off.size()
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
    const EXTENSION: Option<&'static str> = Some("RENDER");
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
pub struct QueryPictFormatsRequest {
    pub req_type: u8,
    pub length: u16,
}
impl QueryPictFormatsRequest {}
impl AsByteSequence for QueryPictFormatsRequest {
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
        log::trace!("Deserializing QueryPictFormatsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryPictFormatsRequest {
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
impl Request for QueryPictFormatsRequest {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryPictFormatsReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryPictFormatsReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_depths: Card32,
    pub num_visuals: Card32,
    pub formats: Vec<Pictforminfo>,
    pub screens: Vec<Pictscreen>,
    pub subpixels: Vec<Card32>,
}
impl QueryPictFormatsReply {}
impl AsByteSequence for QueryPictFormatsReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.formats.len() as Card32).as_bytes(&mut bytes[index..]);
        index += (self.screens.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.num_depths.as_bytes(&mut bytes[index..]);
        index += self.num_visuals.as_bytes(&mut bytes[index..]);
        index += (self.subpixels.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 4;
        let block_len: usize = vector_as_bytes(&self.formats, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Pictforminfo>());
        let block_len: usize = vector_as_bytes(&self.screens, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Pictscreen>());
        let block_len: usize = vector_as_bytes(&self.subpixels, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryPictFormatsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_depths, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_visuals, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len2, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (formats, block_len): (Vec<Pictforminfo>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Pictforminfo>());
        let (screens, block_len): (Vec<Pictscreen>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Pictscreen>());
        let (subpixels, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], len2 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            QueryPictFormatsReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                num_depths: num_depths,
                num_visuals: num_visuals,
                formats: formats,
                screens: screens,
                subpixels: subpixels,
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
            + ::core::mem::size_of::<Card32>()
            + self.num_depths.size()
            + self.num_visuals.size()
            + ::core::mem::size_of::<Card32>()
            + 4
            + {
                let block_len: usize = self.formats.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Pictforminfo>());
                block_len + pad
            }
            + {
                let block_len: usize = self.screens.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Pictscreen>());
                block_len + pad
            }
            + {
                let block_len: usize = self.subpixels.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct QueryPictIndexValuesRequest {
    pub req_type: u8,
    pub format: Pictformat,
    pub length: u16,
}
impl QueryPictIndexValuesRequest {}
impl AsByteSequence for QueryPictIndexValuesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryPictIndexValuesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (Pictformat, usize) = <Pictformat>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryPictIndexValuesRequest {
                req_type: req_type,
                format: format,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.format.size() + self.length.size()
    }
}
impl Request for QueryPictIndexValuesRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryPictIndexValuesReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryPictIndexValuesReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub values: Vec<Indexvalue>,
}
impl QueryPictIndexValuesReply {}
impl AsByteSequence for QueryPictIndexValuesReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.values.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.values, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Indexvalue>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryPictIndexValuesReply from byte buffer");
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
        let (values, block_len): (Vec<Indexvalue>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Indexvalue>());
        Some((
            QueryPictIndexValuesReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                values: values,
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
                let block_len: usize = self.values.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Indexvalue>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct CreatePictureRequest {
    pub req_type: u8,
    pub pid: Picture,
    pub length: u16,
    pub drawable: Drawable,
    pub format: Pictformat,
    pub value_mask: Cp,
    pub repeat: Repeat,
    pub alphamap: Picture,
    pub alphaxorigin: Int32,
    pub alphayorigin: Int32,
    pub clipxorigin: Int32,
    pub clipyorigin: Int32,
    pub clipmask: Pixmap,
    pub graphicsexposure: Card32,
    pub subwindowmode: SubwindowMode,
    pub polyedge: PolyEdge,
    pub polymode: PolyMode,
    pub dither: Atom,
    pub componentalpha: Card32,
}
impl CreatePictureRequest {}
impl AsByteSequence for CreatePictureRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.pid.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index += self.value_mask.as_bytes(&mut bytes[index..]);
        let cond0 = (self.value_mask);
        if cond0.repeat() {
            index += self.repeat.as_bytes(&mut bytes[index..]);
        }
        if cond0.alpha_map() {
            index += self.alphamap.as_bytes(&mut bytes[index..]);
        }
        if cond0.alpha_x_origin() {
            index += self.alphaxorigin.as_bytes(&mut bytes[index..]);
        }
        if cond0.alpha_y_origin() {
            index += self.alphayorigin.as_bytes(&mut bytes[index..]);
        }
        if cond0.clip_x_origin() {
            index += self.clipxorigin.as_bytes(&mut bytes[index..]);
        }
        if cond0.clip_y_origin() {
            index += self.clipyorigin.as_bytes(&mut bytes[index..]);
        }
        if cond0.clip_mask() {
            index += self.clipmask.as_bytes(&mut bytes[index..]);
        }
        if cond0.graphics_exposure() {
            index += self.graphicsexposure.as_bytes(&mut bytes[index..]);
        }
        if cond0.subwindow_mode() {
            index += self.subwindowmode.as_bytes(&mut bytes[index..]);
        }
        if cond0.poly_edge() {
            index += self.polyedge.as_bytes(&mut bytes[index..]);
        }
        if cond0.poly_mode() {
            index += self.polymode.as_bytes(&mut bytes[index..]);
        }
        if cond0.dither() {
            index += self.dither.as_bytes(&mut bytes[index..]);
        }
        if cond0.component_alpha() {
            index += self.componentalpha.as_bytes(&mut bytes[index..]);
        }
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreatePictureRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pid, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (Pictformat, usize) = <Pictformat>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value_mask, sz): (Cp, usize) = <Cp>::from_bytes(&bytes[index..])?;
        index += sz;
        let cond0 = (value_mask);
        let repeat: Repeat = if cond0.repeat() {
            let (repeat, sz): (Repeat, usize) = <Repeat>::from_bytes(&bytes[index..])?;
            index += sz;
            repeat
        } else {
            Default::default()
        };
        let alphamap: Picture = if cond0.alpha_map() {
            let (alphamap, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
            index += sz;
            alphamap
        } else {
            Default::default()
        };
        let alphaxorigin: Int32 = if cond0.alpha_x_origin() {
            let (alphaxorigin, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            alphaxorigin
        } else {
            Default::default()
        };
        let alphayorigin: Int32 = if cond0.alpha_y_origin() {
            let (alphayorigin, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            alphayorigin
        } else {
            Default::default()
        };
        let clipxorigin: Int32 = if cond0.clip_x_origin() {
            let (clipxorigin, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            clipxorigin
        } else {
            Default::default()
        };
        let clipyorigin: Int32 = if cond0.clip_y_origin() {
            let (clipyorigin, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            clipyorigin
        } else {
            Default::default()
        };
        let clipmask: Pixmap = if cond0.clip_mask() {
            let (clipmask, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
            index += sz;
            clipmask
        } else {
            Default::default()
        };
        let graphicsexposure: Card32 = if cond0.graphics_exposure() {
            let (graphicsexposure, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            graphicsexposure
        } else {
            Default::default()
        };
        let subwindowmode: SubwindowMode = if cond0.subwindow_mode() {
            let (subwindowmode, sz): (SubwindowMode, usize) =
                <SubwindowMode>::from_bytes(&bytes[index..])?;
            index += sz;
            subwindowmode
        } else {
            Default::default()
        };
        let polyedge: PolyEdge = if cond0.poly_edge() {
            let (polyedge, sz): (PolyEdge, usize) = <PolyEdge>::from_bytes(&bytes[index..])?;
            index += sz;
            polyedge
        } else {
            Default::default()
        };
        let polymode: PolyMode = if cond0.poly_mode() {
            let (polymode, sz): (PolyMode, usize) = <PolyMode>::from_bytes(&bytes[index..])?;
            index += sz;
            polymode
        } else {
            Default::default()
        };
        let dither: Atom = if cond0.dither() {
            let (dither, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
            index += sz;
            dither
        } else {
            Default::default()
        };
        let componentalpha: Card32 = if cond0.component_alpha() {
            let (componentalpha, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            componentalpha
        } else {
            Default::default()
        };
        Some((
            CreatePictureRequest {
                req_type: req_type,
                pid: pid,
                length: length,
                drawable: drawable,
                format: format,
                value_mask: value_mask,
                repeat: repeat,
                alphamap: alphamap,
                alphaxorigin: alphaxorigin,
                alphayorigin: alphayorigin,
                clipxorigin: clipxorigin,
                clipyorigin: clipyorigin,
                clipmask: clipmask,
                graphicsexposure: graphicsexposure,
                subwindowmode: subwindowmode,
                polyedge: polyedge,
                polymode: polymode,
                dither: dither,
                componentalpha: componentalpha,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.pid.size()
            + self.length.size()
            + self.drawable.size()
            + self.format.size()
            + self.value_mask.size()
            + self.repeat.size()
            + self.alphamap.size()
            + self.alphaxorigin.size()
            + self.alphayorigin.size()
            + self.clipxorigin.size()
            + self.clipyorigin.size()
            + self.clipmask.size()
            + self.graphicsexposure.size()
            + self.subwindowmode.size()
            + self.polyedge.size()
            + self.polymode.size()
            + self.dither.size()
            + self.componentalpha.size()
    }
}
impl Request for CreatePictureRequest {
    const OPCODE: u8 = 4;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cp {
    pub inner: u32,
}
impl Cp {
    #[inline]
    pub fn repeat(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_repeat(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn alpha_map(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_alpha_map(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn alpha_x_origin(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_alpha_x_origin(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn alpha_y_origin(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_alpha_y_origin(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn clip_x_origin(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_clip_x_origin(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn clip_y_origin(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_clip_y_origin(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn clip_mask(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_clip_mask(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn graphics_exposure(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_graphics_exposure(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn subwindow_mode(&self) -> bool {
        self.inner & (1 << 8) != 0
    }
    #[inline]
    pub fn set_subwindow_mode(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 8;
        } else {
            self.inner &= !(1 << 8);
        }
        self
    }
    #[inline]
    pub fn poly_edge(&self) -> bool {
        self.inner & (1 << 9) != 0
    }
    #[inline]
    pub fn set_poly_edge(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 9;
        } else {
            self.inner &= !(1 << 9);
        }
        self
    }
    #[inline]
    pub fn poly_mode(&self) -> bool {
        self.inner & (1 << 10) != 0
    }
    #[inline]
    pub fn set_poly_mode(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 10;
        } else {
            self.inner &= !(1 << 10);
        }
        self
    }
    #[inline]
    pub fn dither(&self) -> bool {
        self.inner & (1 << 11) != 0
    }
    #[inline]
    pub fn set_dither(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 11;
        } else {
            self.inner &= !(1 << 11);
        }
        self
    }
    #[inline]
    pub fn component_alpha(&self) -> bool {
        self.inner & (1 << 12) != 0
    }
    #[inline]
    pub fn set_component_alpha(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 12;
        } else {
            self.inner &= !(1 << 12);
        }
        self
    }
    #[inline]
    pub fn new(
        repeat: bool,
        alpha_map: bool,
        alpha_x_origin: bool,
        alpha_y_origin: bool,
        clip_x_origin: bool,
        clip_y_origin: bool,
        clip_mask: bool,
        graphics_exposure: bool,
        subwindow_mode: bool,
        poly_edge: bool,
        poly_mode: bool,
        dither: bool,
        component_alpha: bool,
    ) -> Self {
        let mut inner: u32 = 0;
        if repeat {
            inner |= 1 << 0;
        }
        if alpha_map {
            inner |= 1 << 1;
        }
        if alpha_x_origin {
            inner |= 1 << 2;
        }
        if alpha_y_origin {
            inner |= 1 << 3;
        }
        if clip_x_origin {
            inner |= 1 << 4;
        }
        if clip_y_origin {
            inner |= 1 << 5;
        }
        if clip_mask {
            inner |= 1 << 6;
        }
        if graphics_exposure {
            inner |= 1 << 7;
        }
        if subwindow_mode {
            inner |= 1 << 8;
        }
        if poly_edge {
            inner |= 1 << 9;
        }
        if poly_mode {
            inner |= 1 << 10;
        }
        if dither {
            inner |= 1 << 11;
        }
        if component_alpha {
            inner |= 1 << 12;
        }
        Cp { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const REPEAT: Self = Self { inner: 1 };
    pub const ALPHA_MAP: Self = Self { inner: 2 };
    pub const ALPHA_X_ORIGIN: Self = Self { inner: 4 };
    pub const ALPHA_Y_ORIGIN: Self = Self { inner: 8 };
    pub const CLIP_X_ORIGIN: Self = Self { inner: 16 };
    pub const CLIP_Y_ORIGIN: Self = Self { inner: 32 };
    pub const CLIP_MASK: Self = Self { inner: 64 };
    pub const GRAPHICS_EXPOSURE: Self = Self { inner: 128 };
    pub const SUBWINDOW_MODE: Self = Self { inner: 256 };
    pub const POLY_EDGE: Self = Self { inner: 512 };
    pub const POLY_MODE: Self = Self { inner: 1024 };
    pub const DITHER: Self = Self { inner: 2048 };
    pub const COMPONENT_ALPHA: Self = Self { inner: 4096 };
    pub const COMPLETE: Self = Self { inner: 8191 };
}
impl AsByteSequence for Cp {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((Cp { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for Cp {
    type Output = Cp;
    #[inline]
    fn not(self) -> Cp {
        Cp { inner: !self.inner }
    }
}
impl core::ops::BitAnd for Cp {
    type Output = Cp;
    #[inline]
    fn bitand(self, rhs: Cp) -> Cp {
        Cp {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for Cp {
    type Output = Cp;
    #[inline]
    fn bitor(self, rhs: Cp) -> Cp {
        Cp {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for Cp {
    type Output = Cp;
    #[inline]
    fn bitxor(self, rhs: Cp) -> Cp {
        Cp {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct ChangePictureRequest {
    pub req_type: u8,
    pub picture: Picture,
    pub length: u16,
    pub value_mask: Cp,
    pub repeat: Repeat,
    pub alphamap: Picture,
    pub alphaxorigin: Int32,
    pub alphayorigin: Int32,
    pub clipxorigin: Int32,
    pub clipyorigin: Int32,
    pub clipmask: Pixmap,
    pub graphicsexposure: Card32,
    pub subwindowmode: SubwindowMode,
    pub polyedge: PolyEdge,
    pub polymode: PolyMode,
    pub dither: Atom,
    pub componentalpha: Card32,
}
impl ChangePictureRequest {}
impl AsByteSequence for ChangePictureRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.picture.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.value_mask.as_bytes(&mut bytes[index..]);
        let cond0 = (self.value_mask);
        if cond0.repeat() {
            index += self.repeat.as_bytes(&mut bytes[index..]);
        }
        if cond0.alpha_map() {
            index += self.alphamap.as_bytes(&mut bytes[index..]);
        }
        if cond0.alpha_x_origin() {
            index += self.alphaxorigin.as_bytes(&mut bytes[index..]);
        }
        if cond0.alpha_y_origin() {
            index += self.alphayorigin.as_bytes(&mut bytes[index..]);
        }
        if cond0.clip_x_origin() {
            index += self.clipxorigin.as_bytes(&mut bytes[index..]);
        }
        if cond0.clip_y_origin() {
            index += self.clipyorigin.as_bytes(&mut bytes[index..]);
        }
        if cond0.clip_mask() {
            index += self.clipmask.as_bytes(&mut bytes[index..]);
        }
        if cond0.graphics_exposure() {
            index += self.graphicsexposure.as_bytes(&mut bytes[index..]);
        }
        if cond0.subwindow_mode() {
            index += self.subwindowmode.as_bytes(&mut bytes[index..]);
        }
        if cond0.poly_edge() {
            index += self.polyedge.as_bytes(&mut bytes[index..]);
        }
        if cond0.poly_mode() {
            index += self.polymode.as_bytes(&mut bytes[index..]);
        }
        if cond0.dither() {
            index += self.dither.as_bytes(&mut bytes[index..]);
        }
        if cond0.component_alpha() {
            index += self.componentalpha.as_bytes(&mut bytes[index..]);
        }
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangePictureRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (picture, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value_mask, sz): (Cp, usize) = <Cp>::from_bytes(&bytes[index..])?;
        index += sz;
        let cond0 = (value_mask);
        let repeat: Repeat = if cond0.repeat() {
            let (repeat, sz): (Repeat, usize) = <Repeat>::from_bytes(&bytes[index..])?;
            index += sz;
            repeat
        } else {
            Default::default()
        };
        let alphamap: Picture = if cond0.alpha_map() {
            let (alphamap, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
            index += sz;
            alphamap
        } else {
            Default::default()
        };
        let alphaxorigin: Int32 = if cond0.alpha_x_origin() {
            let (alphaxorigin, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            alphaxorigin
        } else {
            Default::default()
        };
        let alphayorigin: Int32 = if cond0.alpha_y_origin() {
            let (alphayorigin, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            alphayorigin
        } else {
            Default::default()
        };
        let clipxorigin: Int32 = if cond0.clip_x_origin() {
            let (clipxorigin, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            clipxorigin
        } else {
            Default::default()
        };
        let clipyorigin: Int32 = if cond0.clip_y_origin() {
            let (clipyorigin, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            clipyorigin
        } else {
            Default::default()
        };
        let clipmask: Pixmap = if cond0.clip_mask() {
            let (clipmask, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
            index += sz;
            clipmask
        } else {
            Default::default()
        };
        let graphicsexposure: Card32 = if cond0.graphics_exposure() {
            let (graphicsexposure, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            graphicsexposure
        } else {
            Default::default()
        };
        let subwindowmode: SubwindowMode = if cond0.subwindow_mode() {
            let (subwindowmode, sz): (SubwindowMode, usize) =
                <SubwindowMode>::from_bytes(&bytes[index..])?;
            index += sz;
            subwindowmode
        } else {
            Default::default()
        };
        let polyedge: PolyEdge = if cond0.poly_edge() {
            let (polyedge, sz): (PolyEdge, usize) = <PolyEdge>::from_bytes(&bytes[index..])?;
            index += sz;
            polyedge
        } else {
            Default::default()
        };
        let polymode: PolyMode = if cond0.poly_mode() {
            let (polymode, sz): (PolyMode, usize) = <PolyMode>::from_bytes(&bytes[index..])?;
            index += sz;
            polymode
        } else {
            Default::default()
        };
        let dither: Atom = if cond0.dither() {
            let (dither, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
            index += sz;
            dither
        } else {
            Default::default()
        };
        let componentalpha: Card32 = if cond0.component_alpha() {
            let (componentalpha, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            componentalpha
        } else {
            Default::default()
        };
        Some((
            ChangePictureRequest {
                req_type: req_type,
                picture: picture,
                length: length,
                value_mask: value_mask,
                repeat: repeat,
                alphamap: alphamap,
                alphaxorigin: alphaxorigin,
                alphayorigin: alphayorigin,
                clipxorigin: clipxorigin,
                clipyorigin: clipyorigin,
                clipmask: clipmask,
                graphicsexposure: graphicsexposure,
                subwindowmode: subwindowmode,
                polyedge: polyedge,
                polymode: polymode,
                dither: dither,
                componentalpha: componentalpha,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.picture.size()
            + self.length.size()
            + self.value_mask.size()
            + self.repeat.size()
            + self.alphamap.size()
            + self.alphaxorigin.size()
            + self.alphayorigin.size()
            + self.clipxorigin.size()
            + self.clipyorigin.size()
            + self.clipmask.size()
            + self.graphicsexposure.size()
            + self.subwindowmode.size()
            + self.polyedge.size()
            + self.polymode.size()
            + self.dither.size()
            + self.componentalpha.size()
    }
}
impl Request for ChangePictureRequest {
    const OPCODE: u8 = 5;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct SetPictureClipRectanglesRequest {
    pub req_type: u8,
    pub picture: Picture,
    pub length: u16,
    pub clip_x_origin: Int16,
    pub clip_y_origin: Int16,
    pub rectangles: Vec<Rectangle>,
}
impl SetPictureClipRectanglesRequest {}
impl AsByteSequence for SetPictureClipRectanglesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.picture.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.clip_x_origin.as_bytes(&mut bytes[index..]);
        index += self.clip_y_origin.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.rectangles, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetPictureClipRectanglesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (picture, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (clip_x_origin, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (clip_y_origin, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (rectangles, block_len): (Vec<Rectangle>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
        Some((
            SetPictureClipRectanglesRequest {
                req_type: req_type,
                picture: picture,
                length: length,
                clip_x_origin: clip_x_origin,
                clip_y_origin: clip_y_origin,
                rectangles: rectangles,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.picture.size()
            + self.length.size()
            + self.clip_x_origin.size()
            + self.clip_y_origin.size()
            + {
                let block_len: usize = self.rectangles.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
                block_len + pad
            }
    }
}
impl Request for SetPictureClipRectanglesRequest {
    const OPCODE: u8 = 6;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct FreePictureRequest {
    pub req_type: u8,
    pub picture: Picture,
    pub length: u16,
}
impl FreePictureRequest {}
impl AsByteSequence for FreePictureRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.picture.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FreePictureRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (picture, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            FreePictureRequest {
                req_type: req_type,
                picture: picture,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.picture.size() + self.length.size()
    }
}
impl Request for FreePictureRequest {
    const OPCODE: u8 = 7;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CompositeRequest {
    pub req_type: u8,
    pub op: PictOp,
    pub length: u16,
    pub src: Picture,
    pub mask: Picture,
    pub dst: Picture,
    pub src_x: Int16,
    pub src_y: Int16,
    pub mask_x: Int16,
    pub mask_y: Int16,
    pub dst_x: Int16,
    pub dst_y: Int16,
    pub width: Card16,
    pub height: Card16,
}
impl CompositeRequest {}
impl AsByteSequence for CompositeRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.op.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 3;
        index += self.src.as_bytes(&mut bytes[index..]);
        index += self.mask.as_bytes(&mut bytes[index..]);
        index += self.dst.as_bytes(&mut bytes[index..]);
        index += self.src_x.as_bytes(&mut bytes[index..]);
        index += self.src_y.as_bytes(&mut bytes[index..]);
        index += self.mask_x.as_bytes(&mut bytes[index..]);
        index += self.mask_y.as_bytes(&mut bytes[index..]);
        index += self.dst_x.as_bytes(&mut bytes[index..]);
        index += self.dst_y.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CompositeRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (op, sz): (PictOp, usize) = <PictOp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (src, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mask, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mask_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mask_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CompositeRequest {
                req_type: req_type,
                op: op,
                length: length,
                src: src,
                mask: mask,
                dst: dst,
                src_x: src_x,
                src_y: src_y,
                mask_x: mask_x,
                mask_y: mask_y,
                dst_x: dst_x,
                dst_y: dst_y,
                width: width,
                height: height,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.op.size()
            + self.length.size()
            + 3
            + self.src.size()
            + self.mask.size()
            + self.dst.size()
            + self.src_x.size()
            + self.src_y.size()
            + self.mask_x.size()
            + self.mask_y.size()
            + self.dst_x.size()
            + self.dst_y.size()
            + self.width.size()
            + self.height.size()
    }
}
impl Request for CompositeRequest {
    const OPCODE: u8 = 8;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PictOp {
    Clear = 0,
    Src = 1,
    Dst = 2,
    Over = 3,
    OverReverse = 4,
    In = 5,
    InReverse = 6,
    Out = 7,
    OutReverse = 8,
    Atop = 9,
    AtopReverse = 10,
    Xor = 11,
    Add = 12,
    Saturate = 13,
    DisjointClear = 16,
    DisjointSrc = 17,
    DisjointDst = 18,
    DisjointOver = 19,
    DisjointOverReverse = 20,
    DisjointIn = 21,
    DisjointInReverse = 22,
    DisjointOut = 23,
    DisjointOutReverse = 24,
    DisjointAtop = 25,
    DisjointAtopReverse = 26,
    DisjointXor = 27,
    ConjointClear = 32,
    ConjointSrc = 33,
    ConjointDst = 34,
    ConjointOver = 35,
    ConjointOverReverse = 36,
    ConjointIn = 37,
    ConjointInReverse = 38,
    ConjointOut = 39,
    ConjointOutReverse = 40,
    ConjointAtop = 41,
    ConjointAtopReverse = 42,
    ConjointXor = 43,
    Multiply = 48,
    Screen = 49,
    Overlay = 50,
    Darken = 51,
    Lighten = 52,
    ColorDodge = 53,
    ColorBurn = 54,
    HardLight = 55,
    SoftLight = 56,
    Difference = 57,
    Exclusion = 58,
    HslHue = 59,
    HslSaturation = 60,
    HslColor = 61,
    HslLuminosity = 62,
}
impl AsByteSequence for PictOp {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Clear, sz)),
            1 => Some((Self::Src, sz)),
            2 => Some((Self::Dst, sz)),
            3 => Some((Self::Over, sz)),
            4 => Some((Self::OverReverse, sz)),
            5 => Some((Self::In, sz)),
            6 => Some((Self::InReverse, sz)),
            7 => Some((Self::Out, sz)),
            8 => Some((Self::OutReverse, sz)),
            9 => Some((Self::Atop, sz)),
            10 => Some((Self::AtopReverse, sz)),
            11 => Some((Self::Xor, sz)),
            12 => Some((Self::Add, sz)),
            13 => Some((Self::Saturate, sz)),
            16 => Some((Self::DisjointClear, sz)),
            17 => Some((Self::DisjointSrc, sz)),
            18 => Some((Self::DisjointDst, sz)),
            19 => Some((Self::DisjointOver, sz)),
            20 => Some((Self::DisjointOverReverse, sz)),
            21 => Some((Self::DisjointIn, sz)),
            22 => Some((Self::DisjointInReverse, sz)),
            23 => Some((Self::DisjointOut, sz)),
            24 => Some((Self::DisjointOutReverse, sz)),
            25 => Some((Self::DisjointAtop, sz)),
            26 => Some((Self::DisjointAtopReverse, sz)),
            27 => Some((Self::DisjointXor, sz)),
            32 => Some((Self::ConjointClear, sz)),
            33 => Some((Self::ConjointSrc, sz)),
            34 => Some((Self::ConjointDst, sz)),
            35 => Some((Self::ConjointOver, sz)),
            36 => Some((Self::ConjointOverReverse, sz)),
            37 => Some((Self::ConjointIn, sz)),
            38 => Some((Self::ConjointInReverse, sz)),
            39 => Some((Self::ConjointOut, sz)),
            40 => Some((Self::ConjointOutReverse, sz)),
            41 => Some((Self::ConjointAtop, sz)),
            42 => Some((Self::ConjointAtopReverse, sz)),
            43 => Some((Self::ConjointXor, sz)),
            48 => Some((Self::Multiply, sz)),
            49 => Some((Self::Screen, sz)),
            50 => Some((Self::Overlay, sz)),
            51 => Some((Self::Darken, sz)),
            52 => Some((Self::Lighten, sz)),
            53 => Some((Self::ColorDodge, sz)),
            54 => Some((Self::ColorBurn, sz)),
            55 => Some((Self::HardLight, sz)),
            56 => Some((Self::SoftLight, sz)),
            57 => Some((Self::Difference, sz)),
            58 => Some((Self::Exclusion, sz)),
            59 => Some((Self::HslHue, sz)),
            60 => Some((Self::HslSaturation, sz)),
            61 => Some((Self::HslColor, sz)),
            62 => Some((Self::HslLuminosity, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for PictOp {
    #[inline]
    fn default() -> PictOp {
        PictOp::Clear
    }
}
pub const PICTURE_NONE: Picture = <Picture>::const_from_xid(0);
#[derive(Clone, Debug, Default)]
pub struct TrapezoidsRequest {
    pub req_type: u8,
    pub op: PictOp,
    pub length: u16,
    pub src: Picture,
    pub dst: Picture,
    pub mask_format: Pictformat,
    pub src_x: Int16,
    pub src_y: Int16,
    pub traps: Vec<Trapezoid>,
}
impl TrapezoidsRequest {}
impl AsByteSequence for TrapezoidsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.op.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 3;
        index += self.src.as_bytes(&mut bytes[index..]);
        index += self.dst.as_bytes(&mut bytes[index..]);
        index += self.mask_format.as_bytes(&mut bytes[index..]);
        index += self.src_x.as_bytes(&mut bytes[index..]);
        index += self.src_y.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.traps, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Trapezoid>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing TrapezoidsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (op, sz): (PictOp, usize) = <PictOp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (src, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mask_format, sz): (Pictformat, usize) = <Pictformat>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (traps, block_len): (Vec<Trapezoid>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Trapezoid>());
        Some((
            TrapezoidsRequest {
                req_type: req_type,
                op: op,
                length: length,
                src: src,
                dst: dst,
                mask_format: mask_format,
                src_x: src_x,
                src_y: src_y,
                traps: traps,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.op.size()
            + self.length.size()
            + 3
            + self.src.size()
            + self.dst.size()
            + self.mask_format.size()
            + self.src_x.size()
            + self.src_y.size()
            + {
                let block_len: usize = self.traps.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Trapezoid>());
                block_len + pad
            }
    }
}
impl Request for TrapezoidsRequest {
    const OPCODE: u8 = 10;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct TrianglesRequest {
    pub req_type: u8,
    pub op: PictOp,
    pub length: u16,
    pub src: Picture,
    pub dst: Picture,
    pub mask_format: Pictformat,
    pub src_x: Int16,
    pub src_y: Int16,
    pub triangles: Vec<Triangle>,
}
impl TrianglesRequest {}
impl AsByteSequence for TrianglesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.op.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 3;
        index += self.src.as_bytes(&mut bytes[index..]);
        index += self.dst.as_bytes(&mut bytes[index..]);
        index += self.mask_format.as_bytes(&mut bytes[index..]);
        index += self.src_x.as_bytes(&mut bytes[index..]);
        index += self.src_y.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.triangles, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Triangle>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing TrianglesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (op, sz): (PictOp, usize) = <PictOp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (src, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mask_format, sz): (Pictformat, usize) = <Pictformat>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (triangles, block_len): (Vec<Triangle>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Triangle>());
        Some((
            TrianglesRequest {
                req_type: req_type,
                op: op,
                length: length,
                src: src,
                dst: dst,
                mask_format: mask_format,
                src_x: src_x,
                src_y: src_y,
                triangles: triangles,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.op.size()
            + self.length.size()
            + 3
            + self.src.size()
            + self.dst.size()
            + self.mask_format.size()
            + self.src_x.size()
            + self.src_y.size()
            + {
                let block_len: usize = self.triangles.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Triangle>());
                block_len + pad
            }
    }
}
impl Request for TrianglesRequest {
    const OPCODE: u8 = 11;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct TriStripRequest {
    pub req_type: u8,
    pub op: PictOp,
    pub length: u16,
    pub src: Picture,
    pub dst: Picture,
    pub mask_format: Pictformat,
    pub src_x: Int16,
    pub src_y: Int16,
    pub points: Vec<Pointfix>,
}
impl TriStripRequest {}
impl AsByteSequence for TriStripRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.op.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 3;
        index += self.src.as_bytes(&mut bytes[index..]);
        index += self.dst.as_bytes(&mut bytes[index..]);
        index += self.mask_format.as_bytes(&mut bytes[index..]);
        index += self.src_x.as_bytes(&mut bytes[index..]);
        index += self.src_y.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.points, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Pointfix>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing TriStripRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (op, sz): (PictOp, usize) = <PictOp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (src, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mask_format, sz): (Pictformat, usize) = <Pictformat>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (points, block_len): (Vec<Pointfix>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Pointfix>());
        Some((
            TriStripRequest {
                req_type: req_type,
                op: op,
                length: length,
                src: src,
                dst: dst,
                mask_format: mask_format,
                src_x: src_x,
                src_y: src_y,
                points: points,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.op.size()
            + self.length.size()
            + 3
            + self.src.size()
            + self.dst.size()
            + self.mask_format.size()
            + self.src_x.size()
            + self.src_y.size()
            + {
                let block_len: usize = self.points.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Pointfix>());
                block_len + pad
            }
    }
}
impl Request for TriStripRequest {
    const OPCODE: u8 = 12;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct TriFanRequest {
    pub req_type: u8,
    pub op: PictOp,
    pub length: u16,
    pub src: Picture,
    pub dst: Picture,
    pub mask_format: Pictformat,
    pub src_x: Int16,
    pub src_y: Int16,
    pub points: Vec<Pointfix>,
}
impl TriFanRequest {}
impl AsByteSequence for TriFanRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.op.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 3;
        index += self.src.as_bytes(&mut bytes[index..]);
        index += self.dst.as_bytes(&mut bytes[index..]);
        index += self.mask_format.as_bytes(&mut bytes[index..]);
        index += self.src_x.as_bytes(&mut bytes[index..]);
        index += self.src_y.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.points, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Pointfix>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing TriFanRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (op, sz): (PictOp, usize) = <PictOp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (src, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mask_format, sz): (Pictformat, usize) = <Pictformat>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (points, block_len): (Vec<Pointfix>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Pointfix>());
        Some((
            TriFanRequest {
                req_type: req_type,
                op: op,
                length: length,
                src: src,
                dst: dst,
                mask_format: mask_format,
                src_x: src_x,
                src_y: src_y,
                points: points,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.op.size()
            + self.length.size()
            + 3
            + self.src.size()
            + self.dst.size()
            + self.mask_format.size()
            + self.src_x.size()
            + self.src_y.size()
            + {
                let block_len: usize = self.points.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Pointfix>());
                block_len + pad
            }
    }
}
impl Request for TriFanRequest {
    const OPCODE: u8 = 13;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CreateGlyphSetRequest {
    pub req_type: u8,
    pub gsid: Glyphset,
    pub length: u16,
    pub format: Pictformat,
}
impl CreateGlyphSetRequest {}
impl AsByteSequence for CreateGlyphSetRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.gsid.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateGlyphSetRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gsid, sz): (Glyphset, usize) = <Glyphset>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (Pictformat, usize) = <Pictformat>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CreateGlyphSetRequest {
                req_type: req_type,
                gsid: gsid,
                length: length,
                format: format,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.gsid.size() + self.length.size() + self.format.size()
    }
}
impl Request for CreateGlyphSetRequest {
    const OPCODE: u8 = 17;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct ReferenceGlyphSetRequest {
    pub req_type: u8,
    pub gsid: Glyphset,
    pub length: u16,
    pub existing: Glyphset,
}
impl ReferenceGlyphSetRequest {}
impl AsByteSequence for ReferenceGlyphSetRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.gsid.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.existing.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ReferenceGlyphSetRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gsid, sz): (Glyphset, usize) = <Glyphset>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (existing, sz): (Glyphset, usize) = <Glyphset>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ReferenceGlyphSetRequest {
                req_type: req_type,
                gsid: gsid,
                length: length,
                existing: existing,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.gsid.size() + self.length.size() + self.existing.size()
    }
}
impl Request for ReferenceGlyphSetRequest {
    const OPCODE: u8 = 18;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct FreeGlyphSetRequest {
    pub req_type: u8,
    pub glyphset: Glyphset,
    pub length: u16,
}
impl FreeGlyphSetRequest {}
impl AsByteSequence for FreeGlyphSetRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.glyphset.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FreeGlyphSetRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (glyphset, sz): (Glyphset, usize) = <Glyphset>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            FreeGlyphSetRequest {
                req_type: req_type,
                glyphset: glyphset,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.glyphset.size() + self.length.size()
    }
}
impl Request for FreeGlyphSetRequest {
    const OPCODE: u8 = 19;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct AddGlyphsRequest {
    pub req_type: u8,
    pub glyphset: Glyphset,
    pub length: u16,
    pub glyphs_len: Card32,
    pub glyphids: Vec<Card32>,
    pub glyphs: Vec<Glyphinfo>,
    pub data: Vec<Byte>,
}
impl AddGlyphsRequest {}
impl AsByteSequence for AddGlyphsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.glyphset.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.glyphs_len.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.glyphids, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.glyphs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Glyphinfo>());
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AddGlyphsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (glyphset, sz): (Glyphset, usize) = <Glyphset>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (glyphs_len, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (glyphids, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], (glyphs_len as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (glyphs, block_len): (Vec<Glyphinfo>, usize) =
            vector_from_bytes(&bytes[index..], (glyphs_len as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Glyphinfo>());
        let (data, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            AddGlyphsRequest {
                req_type: req_type,
                glyphset: glyphset,
                length: length,
                glyphs_len: glyphs_len,
                glyphids: glyphids,
                glyphs: glyphs,
                data: data,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.glyphset.size()
            + self.length.size()
            + self.glyphs_len.size()
            + {
                let block_len: usize = self.glyphids.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.glyphs.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Glyphinfo>());
                block_len + pad
            }
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
    }
}
impl Request for AddGlyphsRequest {
    const OPCODE: u8 = 20;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct FreeGlyphsRequest {
    pub req_type: u8,
    pub glyphset: Glyphset,
    pub length: u16,
    pub glyphs: Vec<Glyph>,
}
impl FreeGlyphsRequest {}
impl AsByteSequence for FreeGlyphsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.glyphset.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.glyphs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Glyph>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FreeGlyphsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (glyphset, sz): (Glyphset, usize) = <Glyphset>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (glyphs, block_len): (Vec<Glyph>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Glyph>());
        Some((
            FreeGlyphsRequest {
                req_type: req_type,
                glyphset: glyphset,
                length: length,
                glyphs: glyphs,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.glyphset.size() + self.length.size() + {
            let block_len: usize = self.glyphs.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Glyph>());
            block_len + pad
        }
    }
}
impl Request for FreeGlyphsRequest {
    const OPCODE: u8 = 22;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CompositeGlyphs8Request {
    pub req_type: u8,
    pub op: PictOp,
    pub length: u16,
    pub src: Picture,
    pub dst: Picture,
    pub mask_format: Pictformat,
    pub glyphset: Glyphset,
    pub src_x: Int16,
    pub src_y: Int16,
    pub glyphcmds: Vec<Byte>,
}
impl CompositeGlyphs8Request {}
impl AsByteSequence for CompositeGlyphs8Request {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.op.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 3;
        index += self.src.as_bytes(&mut bytes[index..]);
        index += self.dst.as_bytes(&mut bytes[index..]);
        index += self.mask_format.as_bytes(&mut bytes[index..]);
        index += self.glyphset.as_bytes(&mut bytes[index..]);
        index += self.src_x.as_bytes(&mut bytes[index..]);
        index += self.src_y.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.glyphcmds, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CompositeGlyphs8Request from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (op, sz): (PictOp, usize) = <PictOp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (src, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mask_format, sz): (Pictformat, usize) = <Pictformat>::from_bytes(&bytes[index..])?;
        index += sz;
        let (glyphset, sz): (Glyphset, usize) = <Glyphset>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (glyphcmds, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            CompositeGlyphs8Request {
                req_type: req_type,
                op: op,
                length: length,
                src: src,
                dst: dst,
                mask_format: mask_format,
                glyphset: glyphset,
                src_x: src_x,
                src_y: src_y,
                glyphcmds: glyphcmds,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.op.size()
            + self.length.size()
            + 3
            + self.src.size()
            + self.dst.size()
            + self.mask_format.size()
            + self.glyphset.size()
            + self.src_x.size()
            + self.src_y.size()
            + {
                let block_len: usize = self.glyphcmds.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
    }
}
impl Request for CompositeGlyphs8Request {
    const OPCODE: u8 = 23;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CompositeGlyphs16Request {
    pub req_type: u8,
    pub op: PictOp,
    pub length: u16,
    pub src: Picture,
    pub dst: Picture,
    pub mask_format: Pictformat,
    pub glyphset: Glyphset,
    pub src_x: Int16,
    pub src_y: Int16,
    pub glyphcmds: Vec<Byte>,
}
impl CompositeGlyphs16Request {}
impl AsByteSequence for CompositeGlyphs16Request {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.op.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 3;
        index += self.src.as_bytes(&mut bytes[index..]);
        index += self.dst.as_bytes(&mut bytes[index..]);
        index += self.mask_format.as_bytes(&mut bytes[index..]);
        index += self.glyphset.as_bytes(&mut bytes[index..]);
        index += self.src_x.as_bytes(&mut bytes[index..]);
        index += self.src_y.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.glyphcmds, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CompositeGlyphs16Request from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (op, sz): (PictOp, usize) = <PictOp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (src, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mask_format, sz): (Pictformat, usize) = <Pictformat>::from_bytes(&bytes[index..])?;
        index += sz;
        let (glyphset, sz): (Glyphset, usize) = <Glyphset>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (glyphcmds, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            CompositeGlyphs16Request {
                req_type: req_type,
                op: op,
                length: length,
                src: src,
                dst: dst,
                mask_format: mask_format,
                glyphset: glyphset,
                src_x: src_x,
                src_y: src_y,
                glyphcmds: glyphcmds,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.op.size()
            + self.length.size()
            + 3
            + self.src.size()
            + self.dst.size()
            + self.mask_format.size()
            + self.glyphset.size()
            + self.src_x.size()
            + self.src_y.size()
            + {
                let block_len: usize = self.glyphcmds.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
    }
}
impl Request for CompositeGlyphs16Request {
    const OPCODE: u8 = 24;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CompositeGlyphs32Request {
    pub req_type: u8,
    pub op: PictOp,
    pub length: u16,
    pub src: Picture,
    pub dst: Picture,
    pub mask_format: Pictformat,
    pub glyphset: Glyphset,
    pub src_x: Int16,
    pub src_y: Int16,
    pub glyphcmds: Vec<Byte>,
}
impl CompositeGlyphs32Request {}
impl AsByteSequence for CompositeGlyphs32Request {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.op.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 3;
        index += self.src.as_bytes(&mut bytes[index..]);
        index += self.dst.as_bytes(&mut bytes[index..]);
        index += self.mask_format.as_bytes(&mut bytes[index..]);
        index += self.glyphset.as_bytes(&mut bytes[index..]);
        index += self.src_x.as_bytes(&mut bytes[index..]);
        index += self.src_y.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.glyphcmds, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CompositeGlyphs32Request from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (op, sz): (PictOp, usize) = <PictOp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (src, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mask_format, sz): (Pictformat, usize) = <Pictformat>::from_bytes(&bytes[index..])?;
        index += sz;
        let (glyphset, sz): (Glyphset, usize) = <Glyphset>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (glyphcmds, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            CompositeGlyphs32Request {
                req_type: req_type,
                op: op,
                length: length,
                src: src,
                dst: dst,
                mask_format: mask_format,
                glyphset: glyphset,
                src_x: src_x,
                src_y: src_y,
                glyphcmds: glyphcmds,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.op.size()
            + self.length.size()
            + 3
            + self.src.size()
            + self.dst.size()
            + self.mask_format.size()
            + self.glyphset.size()
            + self.src_x.size()
            + self.src_y.size()
            + {
                let block_len: usize = self.glyphcmds.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
    }
}
impl Request for CompositeGlyphs32Request {
    const OPCODE: u8 = 25;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct FillRectanglesRequest {
    pub req_type: u8,
    pub op: PictOp,
    pub length: u16,
    pub dst: Picture,
    pub color: Color,
    pub rects: Vec<Rectangle>,
}
impl FillRectanglesRequest {}
impl AsByteSequence for FillRectanglesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.op.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 3;
        index += self.dst.as_bytes(&mut bytes[index..]);
        index += self.color.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.rects, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FillRectanglesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (op, sz): (PictOp, usize) = <PictOp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (dst, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (color, sz): (Color, usize) = <Color>::from_bytes(&bytes[index..])?;
        index += sz;
        let (rects, block_len): (Vec<Rectangle>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
        Some((
            FillRectanglesRequest {
                req_type: req_type,
                op: op,
                length: length,
                dst: dst,
                color: color,
                rects: rects,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.op.size()
            + self.length.size()
            + 3
            + self.dst.size()
            + self.color.size()
            + {
                let block_len: usize = self.rects.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
                block_len + pad
            }
    }
}
impl Request for FillRectanglesRequest {
    const OPCODE: u8 = 26;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CreateCursorRequest {
    pub req_type: u8,
    pub cid: Cursor,
    pub length: u16,
    pub source: Picture,
    pub x: Card16,
    pub y: Card16,
}
impl CreateCursorRequest {}
impl AsByteSequence for CreateCursorRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.cid.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.source.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateCursorRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cid, sz): (Cursor, usize) = <Cursor>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (source, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CreateCursorRequest {
                req_type: req_type,
                cid: cid,
                length: length,
                source: source,
                x: x,
                y: y,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.cid.size()
            + self.length.size()
            + self.source.size()
            + self.x.size()
            + self.y.size()
    }
}
impl Request for CreateCursorRequest {
    const OPCODE: u8 = 27;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct Transform {
    pub matrix11: Fixed,
    pub matrix12: Fixed,
    pub matrix13: Fixed,
    pub matrix21: Fixed,
    pub matrix22: Fixed,
    pub matrix23: Fixed,
    pub matrix31: Fixed,
    pub matrix32: Fixed,
    pub matrix33: Fixed,
}
impl Transform {}
impl AsByteSequence for Transform {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.matrix11.as_bytes(&mut bytes[index..]);
        index += self.matrix12.as_bytes(&mut bytes[index..]);
        index += self.matrix13.as_bytes(&mut bytes[index..]);
        index += self.matrix21.as_bytes(&mut bytes[index..]);
        index += self.matrix22.as_bytes(&mut bytes[index..]);
        index += self.matrix23.as_bytes(&mut bytes[index..]);
        index += self.matrix31.as_bytes(&mut bytes[index..]);
        index += self.matrix32.as_bytes(&mut bytes[index..]);
        index += self.matrix33.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Transform from byte buffer");
        let (matrix11, sz): (Fixed, usize) = <Fixed>::from_bytes(&bytes[index..])?;
        index += sz;
        let (matrix12, sz): (Fixed, usize) = <Fixed>::from_bytes(&bytes[index..])?;
        index += sz;
        let (matrix13, sz): (Fixed, usize) = <Fixed>::from_bytes(&bytes[index..])?;
        index += sz;
        let (matrix21, sz): (Fixed, usize) = <Fixed>::from_bytes(&bytes[index..])?;
        index += sz;
        let (matrix22, sz): (Fixed, usize) = <Fixed>::from_bytes(&bytes[index..])?;
        index += sz;
        let (matrix23, sz): (Fixed, usize) = <Fixed>::from_bytes(&bytes[index..])?;
        index += sz;
        let (matrix31, sz): (Fixed, usize) = <Fixed>::from_bytes(&bytes[index..])?;
        index += sz;
        let (matrix32, sz): (Fixed, usize) = <Fixed>::from_bytes(&bytes[index..])?;
        index += sz;
        let (matrix33, sz): (Fixed, usize) = <Fixed>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Transform {
                matrix11: matrix11,
                matrix12: matrix12,
                matrix13: matrix13,
                matrix21: matrix21,
                matrix22: matrix22,
                matrix23: matrix23,
                matrix31: matrix31,
                matrix32: matrix32,
                matrix33: matrix33,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.matrix11.size()
            + self.matrix12.size()
            + self.matrix13.size()
            + self.matrix21.size()
            + self.matrix22.size()
            + self.matrix23.size()
            + self.matrix31.size()
            + self.matrix32.size()
            + self.matrix33.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct SetPictureTransformRequest {
    pub req_type: u8,
    pub picture: Picture,
    pub length: u16,
    pub transform: Transform,
}
impl SetPictureTransformRequest {}
impl AsByteSequence for SetPictureTransformRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.picture.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.transform.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetPictureTransformRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (picture, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (transform, sz): (Transform, usize) = <Transform>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetPictureTransformRequest {
                req_type: req_type,
                picture: picture,
                length: length,
                transform: transform,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.picture.size() + self.length.size() + self.transform.size()
    }
}
impl Request for SetPictureTransformRequest {
    const OPCODE: u8 = 28;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct QueryFiltersRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
}
impl QueryFiltersRequest {}
impl AsByteSequence for QueryFiltersRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryFiltersRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryFiltersRequest {
                req_type: req_type,
                length: length,
                drawable: drawable,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.drawable.size()
    }
}
impl Request for QueryFiltersRequest {
    const OPCODE: u8 = 29;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryFiltersReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryFiltersReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub aliases: Vec<Card16>,
    pub filters: Vec<Str>,
}
impl QueryFiltersReply {}
impl AsByteSequence for QueryFiltersReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.aliases.len() as Card32).as_bytes(&mut bytes[index..]);
        index += (self.filters.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 16;
        let block_len: usize = vector_as_bytes(&self.aliases, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        let block_len: usize = vector_as_bytes(&self.filters, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Str>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryFiltersReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 16;
        let (aliases, block_len): (Vec<Card16>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        let (filters, block_len): (Vec<Str>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Str>());
        Some((
            QueryFiltersReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                aliases: aliases,
                filters: filters,
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
            + ::core::mem::size_of::<Card32>()
            + 16
            + {
                let block_len: usize = self.aliases.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card16>());
                block_len + pad
            }
            + {
                let block_len: usize = self.filters.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Str>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct SetPictureFilterRequest {
    pub req_type: u8,
    pub picture: Picture,
    pub length: u16,
    pub filter: String,
    pub values: Vec<Fixed>,
}
impl SetPictureFilterRequest {}
impl AsByteSequence for SetPictureFilterRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.picture.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.filter.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = string_as_bytes(&self.filter, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, 4);
        let block_len: usize = vector_as_bytes(&self.values, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fixed>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetPictureFilterRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (picture, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (filter, block_len): (String, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, 4);
        let (values, block_len): (Vec<Fixed>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fixed>());
        Some((
            SetPictureFilterRequest {
                req_type: req_type,
                picture: picture,
                length: length,
                filter: filter,
                values: values,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.picture.size()
            + self.length.size()
            + ::core::mem::size_of::<Card16>()
            + 2
            + {
                let block_len: usize = self.filter.len();
                let pad: usize = buffer_pad(block_len, 4);
                block_len + pad
            }
            + {
                let block_len: usize = self.values.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fixed>());
                block_len + pad
            }
    }
}
impl Request for SetPictureFilterRequest {
    const OPCODE: u8 = 30;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct Animcursorelt {
    pub cursor: Cursor,
    pub delay: Card32,
}
impl Animcursorelt {}
impl AsByteSequence for Animcursorelt {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.cursor.as_bytes(&mut bytes[index..]);
        index += self.delay.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Animcursorelt from byte buffer");
        let (cursor, sz): (Cursor, usize) = <Cursor>::from_bytes(&bytes[index..])?;
        index += sz;
        let (delay, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Animcursorelt {
                cursor: cursor,
                delay: delay,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.cursor.size() + self.delay.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct CreateAnimCursorRequest {
    pub req_type: u8,
    pub cid: Cursor,
    pub length: u16,
    pub cursors: Vec<Animcursorelt>,
}
impl CreateAnimCursorRequest {}
impl AsByteSequence for CreateAnimCursorRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.cid.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.cursors, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Animcursorelt>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateAnimCursorRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cid, sz): (Cursor, usize) = <Cursor>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cursors, block_len): (Vec<Animcursorelt>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Animcursorelt>());
        Some((
            CreateAnimCursorRequest {
                req_type: req_type,
                cid: cid,
                length: length,
                cursors: cursors,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.cid.size() + self.length.size() + {
            let block_len: usize = self.cursors.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Animcursorelt>());
            block_len + pad
        }
    }
}
impl Request for CreateAnimCursorRequest {
    const OPCODE: u8 = 31;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct Spanfix {
    pub l: Fixed,
    pub r: Fixed,
    pub y: Fixed,
}
impl Spanfix {}
impl AsByteSequence for Spanfix {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.l.as_bytes(&mut bytes[index..]);
        index += self.r.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Spanfix from byte buffer");
        let (l, sz): (Fixed, usize) = <Fixed>::from_bytes(&bytes[index..])?;
        index += sz;
        let (r, sz): (Fixed, usize) = <Fixed>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Fixed, usize) = <Fixed>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((Spanfix { l: l, r: r, y: y }, index))
    }
    #[inline]
    fn size(&self) -> usize {
        self.l.size() + self.r.size() + self.y.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct Trap {
    pub top: Spanfix,
    pub bot: Spanfix,
}
impl Trap {}
impl AsByteSequence for Trap {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.top.as_bytes(&mut bytes[index..]);
        index += self.bot.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Trap from byte buffer");
        let (top, sz): (Spanfix, usize) = <Spanfix>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bot, sz): (Spanfix, usize) = <Spanfix>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((Trap { top: top, bot: bot }, index))
    }
    #[inline]
    fn size(&self) -> usize {
        self.top.size() + self.bot.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct AddTrapsRequest {
    pub req_type: u8,
    pub picture: Picture,
    pub length: u16,
    pub x_off: Int16,
    pub y_off: Int16,
    pub traps: Vec<Trap>,
}
impl AddTrapsRequest {}
impl AsByteSequence for AddTrapsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.picture.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.x_off.as_bytes(&mut bytes[index..]);
        index += self.y_off.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.traps, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Trap>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AddTrapsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (picture, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x_off, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y_off, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (traps, block_len): (Vec<Trap>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Trap>());
        Some((
            AddTrapsRequest {
                req_type: req_type,
                picture: picture,
                length: length,
                x_off: x_off,
                y_off: y_off,
                traps: traps,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.picture.size()
            + self.length.size()
            + self.x_off.size()
            + self.y_off.size()
            + {
                let block_len: usize = self.traps.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Trap>());
                block_len + pad
            }
    }
}
impl Request for AddTrapsRequest {
    const OPCODE: u8 = 32;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CreateSolidFillRequest {
    pub req_type: u8,
    pub picture: Picture,
    pub length: u16,
    pub color: Color,
}
impl CreateSolidFillRequest {}
impl AsByteSequence for CreateSolidFillRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.picture.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.color.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateSolidFillRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (picture, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (color, sz): (Color, usize) = <Color>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CreateSolidFillRequest {
                req_type: req_type,
                picture: picture,
                length: length,
                color: color,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.picture.size() + self.length.size() + self.color.size()
    }
}
impl Request for CreateSolidFillRequest {
    const OPCODE: u8 = 33;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CreateLinearGradientRequest {
    pub req_type: u8,
    pub picture: Picture,
    pub length: u16,
    pub p1: Pointfix,
    pub p2: Pointfix,
    pub num_stops: Card32,
    pub stops: Vec<Fixed>,
    pub colors: Vec<Color>,
}
impl CreateLinearGradientRequest {}
impl AsByteSequence for CreateLinearGradientRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.picture.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.p1.as_bytes(&mut bytes[index..]);
        index += self.p2.as_bytes(&mut bytes[index..]);
        index += self.num_stops.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.stops, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fixed>());
        let block_len: usize = vector_as_bytes(&self.colors, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Color>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateLinearGradientRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (picture, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (p1, sz): (Pointfix, usize) = <Pointfix>::from_bytes(&bytes[index..])?;
        index += sz;
        let (p2, sz): (Pointfix, usize) = <Pointfix>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_stops, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (stops, block_len): (Vec<Fixed>, usize) =
            vector_from_bytes(&bytes[index..], (num_stops as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fixed>());
        let (colors, block_len): (Vec<Color>, usize) =
            vector_from_bytes(&bytes[index..], (num_stops as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Color>());
        Some((
            CreateLinearGradientRequest {
                req_type: req_type,
                picture: picture,
                length: length,
                p1: p1,
                p2: p2,
                num_stops: num_stops,
                stops: stops,
                colors: colors,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.picture.size()
            + self.length.size()
            + self.p1.size()
            + self.p2.size()
            + self.num_stops.size()
            + {
                let block_len: usize = self.stops.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fixed>());
                block_len + pad
            }
            + {
                let block_len: usize = self.colors.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Color>());
                block_len + pad
            }
    }
}
impl Request for CreateLinearGradientRequest {
    const OPCODE: u8 = 34;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CreateRadialGradientRequest {
    pub req_type: u8,
    pub picture: Picture,
    pub length: u16,
    pub inner: Pointfix,
    pub outer: Pointfix,
    pub inner_radius: Fixed,
    pub outer_radius: Fixed,
    pub num_stops: Card32,
    pub stops: Vec<Fixed>,
    pub colors: Vec<Color>,
}
impl CreateRadialGradientRequest {}
impl AsByteSequence for CreateRadialGradientRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.picture.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.inner.as_bytes(&mut bytes[index..]);
        index += self.outer.as_bytes(&mut bytes[index..]);
        index += self.inner_radius.as_bytes(&mut bytes[index..]);
        index += self.outer_radius.as_bytes(&mut bytes[index..]);
        index += self.num_stops.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.stops, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fixed>());
        let block_len: usize = vector_as_bytes(&self.colors, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Color>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateRadialGradientRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (picture, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (inner, sz): (Pointfix, usize) = <Pointfix>::from_bytes(&bytes[index..])?;
        index += sz;
        let (outer, sz): (Pointfix, usize) = <Pointfix>::from_bytes(&bytes[index..])?;
        index += sz;
        let (inner_radius, sz): (Fixed, usize) = <Fixed>::from_bytes(&bytes[index..])?;
        index += sz;
        let (outer_radius, sz): (Fixed, usize) = <Fixed>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_stops, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (stops, block_len): (Vec<Fixed>, usize) =
            vector_from_bytes(&bytes[index..], (num_stops as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fixed>());
        let (colors, block_len): (Vec<Color>, usize) =
            vector_from_bytes(&bytes[index..], (num_stops as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Color>());
        Some((
            CreateRadialGradientRequest {
                req_type: req_type,
                picture: picture,
                length: length,
                inner: inner,
                outer: outer,
                inner_radius: inner_radius,
                outer_radius: outer_radius,
                num_stops: num_stops,
                stops: stops,
                colors: colors,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.picture.size()
            + self.length.size()
            + self.inner.size()
            + self.outer.size()
            + self.inner_radius.size()
            + self.outer_radius.size()
            + self.num_stops.size()
            + {
                let block_len: usize = self.stops.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fixed>());
                block_len + pad
            }
            + {
                let block_len: usize = self.colors.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Color>());
                block_len + pad
            }
    }
}
impl Request for CreateRadialGradientRequest {
    const OPCODE: u8 = 35;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CreateConicalGradientRequest {
    pub req_type: u8,
    pub picture: Picture,
    pub length: u16,
    pub center: Pointfix,
    pub angle: Fixed,
    pub num_stops: Card32,
    pub stops: Vec<Fixed>,
    pub colors: Vec<Color>,
}
impl CreateConicalGradientRequest {}
impl AsByteSequence for CreateConicalGradientRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.picture.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.center.as_bytes(&mut bytes[index..]);
        index += self.angle.as_bytes(&mut bytes[index..]);
        index += self.num_stops.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.stops, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fixed>());
        let block_len: usize = vector_as_bytes(&self.colors, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Color>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateConicalGradientRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (picture, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (center, sz): (Pointfix, usize) = <Pointfix>::from_bytes(&bytes[index..])?;
        index += sz;
        let (angle, sz): (Fixed, usize) = <Fixed>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_stops, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (stops, block_len): (Vec<Fixed>, usize) =
            vector_from_bytes(&bytes[index..], (num_stops as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fixed>());
        let (colors, block_len): (Vec<Color>, usize) =
            vector_from_bytes(&bytes[index..], (num_stops as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Color>());
        Some((
            CreateConicalGradientRequest {
                req_type: req_type,
                picture: picture,
                length: length,
                center: center,
                angle: angle,
                num_stops: num_stops,
                stops: stops,
                colors: colors,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.picture.size()
            + self.length.size()
            + self.center.size()
            + self.angle.size()
            + self.num_stops.size()
            + {
                let block_len: usize = self.stops.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fixed>());
                block_len + pad
            }
            + {
                let block_len: usize = self.colors.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Color>());
                block_len + pad
            }
    }
}
impl Request for CreateConicalGradientRequest {
    const OPCODE: u8 = 36;
    const EXTENSION: Option<&'static str> = Some("RENDER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Repeat {
    None = 0,
    Normal = 1,
    Pad = 2,
    Reflect = 3,
}
impl AsByteSequence for Repeat {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::None, sz)),
            1 => Some((Self::Normal, sz)),
            2 => Some((Self::Pad, sz)),
            3 => Some((Self::Reflect, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for Repeat {
    #[inline]
    fn default() -> Repeat {
        Repeat::None
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PolyMode {
    Precise = 0,
    Imprecise = 1,
}
impl AsByteSequence for PolyMode {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Precise, sz)),
            1 => Some((Self::Imprecise, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for PolyMode {
    #[inline]
    fn default() -> PolyMode {
        PolyMode::Precise
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SubPixel {
    Unknown = 0,
    HorizontalRgb = 1,
    HorizontalBgr = 2,
    VerticalRgb = 3,
    VerticalBgr = 4,
    None = 5,
}
impl AsByteSequence for SubPixel {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Unknown, sz)),
            1 => Some((Self::HorizontalRgb, sz)),
            2 => Some((Self::HorizontalBgr, sz)),
            3 => Some((Self::VerticalRgb, sz)),
            4 => Some((Self::VerticalBgr, sz)),
            5 => Some((Self::None, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for SubPixel {
    #[inline]
    fn default() -> SubPixel {
        SubPixel::Unknown
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PolyEdge {
    Sharp = 0,
    Smooth = 1,
}
impl AsByteSequence for PolyEdge {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Sharp, sz)),
            1 => Some((Self::Smooth, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for PolyEdge {
    #[inline]
    fn default() -> PolyEdge {
        PolyEdge::Sharp
    }
}
