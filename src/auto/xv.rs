// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

use super::shm::*;
use super::xproto::*;
#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Port {
    pub xid: XID,
}
impl Port {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Port {
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
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Encoding {
    pub xid: XID,
}
impl Encoding {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Encoding {
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
pub struct Rational {
    pub numerator: Int32,
    pub denominator: Int32,
}
impl Rational {}
impl AsByteSequence for Rational {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.numerator.as_bytes(&mut bytes[index..]);
        index += self.denominator.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Rational from byte buffer");
        let (numerator, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (denominator, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Rational {
                numerator: numerator,
                denominator: denominator,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.numerator.size() + self.denominator.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct Format {
    pub visual: Visualid,
    pub depth: Card8,
}
impl Format {}
impl AsByteSequence for Format {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.visual.as_bytes(&mut bytes[index..]);
        index += self.depth.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Format from byte buffer");
        let (visual, sz): (Visualid, usize) = <Visualid>::from_bytes(&bytes[index..])?;
        index += sz;
        let (depth, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            Format {
                visual: visual,
                depth: depth,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.visual.size() + self.depth.size() + 3
    }
}
#[derive(Clone, Debug, Default)]
pub struct AdaptorInfo {
    pub base_id: Port,
    pub num_ports: Card16,
    pub ty: Type,
    pub name: String,
    pub formats: Vec<Format>,
}
impl AdaptorInfo {}
impl AsByteSequence for AdaptorInfo {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.base_id.as_bytes(&mut bytes[index..]);
        index += (self.name.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.num_ports.as_bytes(&mut bytes[index..]);
        index += (self.formats.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += 1;
        let block_len: usize = string_as_bytes(&self.name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, 4);
        let block_len: usize = vector_as_bytes(&self.formats, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Format>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AdaptorInfo from byte buffer");
        let (base_id, sz): (Port, usize) = <Port>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_ports, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Type, usize) = <Type>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (name, block_len): (String, usize) = string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, 4);
        let (formats, block_len): (Vec<Format>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Format>());
        Some((
            AdaptorInfo {
                base_id: base_id,
                num_ports: num_ports,
                ty: ty,
                name: name,
                formats: formats,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.base_id.size()
            + ::core::mem::size_of::<Card16>()
            + self.num_ports.size()
            + ::core::mem::size_of::<Card16>()
            + self.ty.size()
            + 1
            + {
                let block_len: usize = self.name.len();
                let pad: usize = buffer_pad(block_len, 4);
                block_len + pad
            }
            + {
                let block_len: usize = self.formats.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Format>());
                block_len + pad
            }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Type {
    pub inner: u8,
}
impl Type {
    #[inline]
    pub fn input_mask(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_input_mask(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn output_mask(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_output_mask(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn video_mask(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_video_mask(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn still_mask(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_still_mask(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn image_mask(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_image_mask(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn new(
        input_mask: bool,
        output_mask: bool,
        video_mask: bool,
        still_mask: bool,
        image_mask: bool,
    ) -> Self {
        let mut inner: u8 = 0;
        if input_mask {
            inner |= 1 << 0;
        }
        if output_mask {
            inner |= 1 << 1;
        }
        if video_mask {
            inner |= 1 << 2;
        }
        if still_mask {
            inner |= 1 << 3;
        }
        if image_mask {
            inner |= 1 << 4;
        }
        Type { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
}
impl AsByteSequence for Type {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        Some((Type { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for Type {
    type Output = Type;
    #[inline]
    fn not(self) -> Type {
        Type { inner: !self.inner }
    }
}
impl core::ops::BitAnd for Type {
    type Output = Type;
    #[inline]
    fn bitand(self, rhs: Type) -> Type {
        Type {
            inner: self.inner & rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct EncodingInfo {
    pub encoding: Encoding,
    pub width: Card16,
    pub height: Card16,
    pub rate: Rational,
    pub name: String,
}
impl EncodingInfo {}
impl AsByteSequence for EncodingInfo {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.encoding.as_bytes(&mut bytes[index..]);
        index += (self.name.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.rate.as_bytes(&mut bytes[index..]);
        let block_len: usize = string_as_bytes(&self.name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, 4);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing EncodingInfo from byte buffer");
        let (encoding, sz): (Encoding, usize) = <Encoding>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (rate, sz): (Rational, usize) = <Rational>::from_bytes(&bytes[index..])?;
        index += sz;
        let (name, block_len): (String, usize) = string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, 4);
        Some((
            EncodingInfo {
                encoding: encoding,
                width: width,
                height: height,
                rate: rate,
                name: name,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.encoding.size()
            + ::core::mem::size_of::<Card16>()
            + self.width.size()
            + self.height.size()
            + 2
            + self.rate.size()
            + {
                let block_len: usize = self.name.len();
                let pad: usize = buffer_pad(block_len, 4);
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct Image {
    pub id: Card32,
    pub width: Card16,
    pub height: Card16,
    pub num_planes: Card32,
    pub pitches: Vec<Card32>,
    pub offsets: Vec<Card32>,
    pub data: Vec<Card8>,
}
impl Image {}
impl AsByteSequence for Image {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.id.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.num_planes.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.pitches, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.offsets, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Image from byte buffer");
        let (id, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_planes, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pitches, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], (num_planes as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (offsets, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], (num_planes as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (data, block_len): (Vec<Card8>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        Some((
            Image {
                id: id,
                width: width,
                height: height,
                num_planes: num_planes,
                pitches: pitches,
                offsets: offsets,
                data: data,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.id.size()
            + self.width.size()
            + self.height.size()
            + ::core::mem::size_of::<Card32>()
            + self.num_planes.size()
            + {
                let block_len: usize = self.pitches.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.offsets.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct AttributeInfo {
    pub flags: AttributeFlag,
    pub min: Int32,
    pub max: Int32,
    pub name: String,
}
impl AttributeInfo {}
impl AsByteSequence for AttributeInfo {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.min.as_bytes(&mut bytes[index..]);
        index += self.max.as_bytes(&mut bytes[index..]);
        index += (self.name.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = string_as_bytes(&self.name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, 4);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AttributeInfo from byte buffer");
        let (flags, sz): (AttributeFlag, usize) = <AttributeFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (min, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (name, block_len): (String, usize) = string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, 4);
        Some((
            AttributeInfo {
                flags: flags,
                min: min,
                max: max,
                name: name,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.flags.size() + self.min.size() + self.max.size() + ::core::mem::size_of::<Card32>() + {
            let block_len: usize = self.name.len();
            let pad: usize = buffer_pad(block_len, 4);
            block_len + pad
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct AttributeFlag {
    pub inner: u32,
}
impl AttributeFlag {
    #[inline]
    pub fn gettable(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_gettable(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn settable(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_settable(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn new(gettable: bool, settable: bool) -> Self {
        let mut inner: u32 = 0;
        if gettable {
            inner |= 1 << 0;
        }
        if settable {
            inner |= 1 << 1;
        }
        AttributeFlag { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
}
impl AsByteSequence for AttributeFlag {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((AttributeFlag { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for AttributeFlag {
    type Output = AttributeFlag;
    #[inline]
    fn not(self) -> AttributeFlag {
        AttributeFlag { inner: !self.inner }
    }
}
impl core::ops::BitAnd for AttributeFlag {
    type Output = AttributeFlag;
    #[inline]
    fn bitand(self, rhs: AttributeFlag) -> AttributeFlag {
        AttributeFlag {
            inner: self.inner & rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct ImageFormatInfo {
    pub id: Card32,
    pub ty: ImageFormatInfoType,
    pub byte_order: ImageOrder,
    pub guid: [Card8; 16],
    pub bpp: Card8,
    pub num_planes: Card8,
    pub depth: Card8,
    pub red_mask: Card32,
    pub green_mask: Card32,
    pub blue_mask: Card32,
    pub format: ImageFormatInfoFormat,
    pub y_sample_bits: Card32,
    pub u_sample_bits: Card32,
    pub v_sample_bits: Card32,
    pub vhorz_y_period: Card32,
    pub vhorz_u_period: Card32,
    pub vhorz_v_period: Card32,
    pub vvert_y_period: Card32,
    pub vvert_u_period: Card32,
    pub vvert_v_period: Card32,
    pub vcomp_order: [Card8; 32],
    pub vscanline_order: ScanlineOrder,
}
impl ImageFormatInfo {}
impl AsByteSequence for ImageFormatInfo {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.id.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.byte_order.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.guid.as_bytes(&mut bytes[index..]);
        index += self.bpp.as_bytes(&mut bytes[index..]);
        index += self.num_planes.as_bytes(&mut bytes[index..]);
        index += self.depth.as_bytes(&mut bytes[index..]);
        index += 3;
        index += self.red_mask.as_bytes(&mut bytes[index..]);
        index += self.green_mask.as_bytes(&mut bytes[index..]);
        index += self.blue_mask.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index += self.y_sample_bits.as_bytes(&mut bytes[index..]);
        index += self.u_sample_bits.as_bytes(&mut bytes[index..]);
        index += self.v_sample_bits.as_bytes(&mut bytes[index..]);
        index += self.vhorz_y_period.as_bytes(&mut bytes[index..]);
        index += self.vhorz_u_period.as_bytes(&mut bytes[index..]);
        index += self.vhorz_v_period.as_bytes(&mut bytes[index..]);
        index += self.vvert_y_period.as_bytes(&mut bytes[index..]);
        index += self.vvert_u_period.as_bytes(&mut bytes[index..]);
        index += self.vvert_v_period.as_bytes(&mut bytes[index..]);
        index += self.vcomp_order.as_bytes(&mut bytes[index..]);
        index += self.vscanline_order.as_bytes(&mut bytes[index..]);
        index += 11;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ImageFormatInfo from byte buffer");
        let (id, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (ImageFormatInfoType, usize) =
            <ImageFormatInfoType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (byte_order, sz): (ImageOrder, usize) = <ImageOrder>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (guid, sz): ([Card8; 16], usize) = <[Card8; 16]>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bpp, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_planes, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (depth, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (red_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (green_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (blue_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (ImageFormatInfoFormat, usize) =
            <ImageFormatInfoFormat>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y_sample_bits, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (u_sample_bits, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (v_sample_bits, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vhorz_y_period, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vhorz_u_period, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vhorz_v_period, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vvert_y_period, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vvert_u_period, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vvert_v_period, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vcomp_order, sz): ([Card8; 32], usize) = <[Card8; 32]>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vscanline_order, sz): (ScanlineOrder, usize) =
            <ScanlineOrder>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 11;
        Some((
            ImageFormatInfo {
                id: id,
                ty: ty,
                byte_order: byte_order,
                guid: guid,
                bpp: bpp,
                num_planes: num_planes,
                depth: depth,
                red_mask: red_mask,
                green_mask: green_mask,
                blue_mask: blue_mask,
                format: format,
                y_sample_bits: y_sample_bits,
                u_sample_bits: u_sample_bits,
                v_sample_bits: v_sample_bits,
                vhorz_y_period: vhorz_y_period,
                vhorz_u_period: vhorz_u_period,
                vhorz_v_period: vhorz_v_period,
                vvert_y_period: vvert_y_period,
                vvert_u_period: vvert_u_period,
                vvert_v_period: vvert_v_period,
                vcomp_order: vcomp_order,
                vscanline_order: vscanline_order,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.id.size()
            + self.ty.size()
            + self.byte_order.size()
            + 2
            + self.guid.size()
            + self.bpp.size()
            + self.num_planes.size()
            + self.depth.size()
            + 3
            + self.red_mask.size()
            + self.green_mask.size()
            + self.blue_mask.size()
            + self.format.size()
            + self.y_sample_bits.size()
            + self.u_sample_bits.size()
            + self.v_sample_bits.size()
            + self.vhorz_y_period.size()
            + self.vhorz_u_period.size()
            + self.vhorz_v_period.size()
            + self.vvert_y_period.size()
            + self.vvert_u_period.size()
            + self.vvert_v_period.size()
            + self.vcomp_order.size()
            + self.vscanline_order.size()
            + 11
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ImageFormatInfoType {
    Rgb = 0,
    Yuv = 1,
}
impl AsByteSequence for ImageFormatInfoType {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Rgb, sz)),
            1 => Some((Self::Yuv, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for ImageFormatInfoType {
    #[inline]
    fn default() -> ImageFormatInfoType {
        ImageFormatInfoType::Rgb
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ImageFormatInfoFormat {
    Packed = 0,
    Planar = 1,
}
impl AsByteSequence for ImageFormatInfoFormat {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Packed, sz)),
            1 => Some((Self::Planar, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for ImageFormatInfoFormat {
    #[inline]
    fn default() -> ImageFormatInfoFormat {
        ImageFormatInfoFormat::Packed
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScanlineOrder {
    TopToBottom = 0,
    BottomToTop = 1,
}
impl AsByteSequence for ScanlineOrder {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::TopToBottom, sz)),
            1 => Some((Self::BottomToTop, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for ScanlineOrder {
    #[inline]
    fn default() -> ScanlineOrder {
        ScanlineOrder::TopToBottom
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum VideoNotifyReason {
    Started = 0,
    Stopped = 1,
    Busy = 2,
    Preempted = 3,
    HardError = 4,
}
impl AsByteSequence for VideoNotifyReason {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Started, sz)),
            1 => Some((Self::Stopped, sz)),
            2 => Some((Self::Busy, sz)),
            3 => Some((Self::Preempted, sz)),
            4 => Some((Self::HardError, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for VideoNotifyReason {
    #[inline]
    fn default() -> VideoNotifyReason {
        VideoNotifyReason::Started
    }
}
#[derive(Clone, Debug, Default)]
pub struct QueryExtensionRequest {
    pub req_type: u8,
    pub length: u16,
}
impl QueryExtensionRequest {}
impl AsByteSequence for QueryExtensionRequest {
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
        log::trace!("Deserializing QueryExtensionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryExtensionRequest {
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
impl Request for QueryExtensionRequest {
    const OPCODE: u8 = 0;
    const EXTENSION: Option<&'static str> = Some("XVideo");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryExtensionReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryExtensionReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major: Card16,
    pub minor: Card16,
}
impl QueryExtensionReply {}
impl AsByteSequence for QueryExtensionReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.major.as_bytes(&mut bytes[index..]);
        index += self.minor.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryExtensionReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryExtensionReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                major: major,
                minor: minor,
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
            + self.major.size()
            + self.minor.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct QueryAdaptorsRequest {
    pub req_type: u8,
    pub window: Window,
    pub length: u16,
}
impl QueryAdaptorsRequest {}
impl AsByteSequence for QueryAdaptorsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryAdaptorsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryAdaptorsRequest {
                req_type: req_type,
                window: window,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.window.size() + self.length.size()
    }
}
impl Request for QueryAdaptorsRequest {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = Some("XVideo");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryAdaptorsReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryAdaptorsReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub info: Vec<AdaptorInfo>,
}
impl QueryAdaptorsReply {}
impl AsByteSequence for QueryAdaptorsReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.info.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 22;
        let block_len: usize = vector_as_bytes(&self.info, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<AdaptorInfo>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryAdaptorsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 22;
        let (info, block_len): (Vec<AdaptorInfo>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<AdaptorInfo>());
        Some((
            QueryAdaptorsReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                info: info,
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
            + ::core::mem::size_of::<Card16>()
            + 22
            + {
                let block_len: usize = self.info.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<AdaptorInfo>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct QueryEncodingsRequest {
    pub req_type: u8,
    pub port: Port,
    pub length: u16,
}
impl QueryEncodingsRequest {}
impl AsByteSequence for QueryEncodingsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.port.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryEncodingsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (port, sz): (Port, usize) = <Port>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryEncodingsRequest {
                req_type: req_type,
                port: port,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.port.size() + self.length.size()
    }
}
impl Request for QueryEncodingsRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = Some("XVideo");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryEncodingsReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryEncodingsReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub info: Vec<EncodingInfo>,
}
impl QueryEncodingsReply {}
impl AsByteSequence for QueryEncodingsReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.info.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 22;
        let block_len: usize = vector_as_bytes(&self.info, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EncodingInfo>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryEncodingsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 22;
        let (info, block_len): (Vec<EncodingInfo>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EncodingInfo>());
        Some((
            QueryEncodingsReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                info: info,
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
            + ::core::mem::size_of::<Card16>()
            + 22
            + {
                let block_len: usize = self.info.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<EncodingInfo>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GrabPortRequest {
    pub req_type: u8,
    pub port: Port,
    pub length: u16,
    pub time: Timestamp,
}
impl GrabPortRequest {}
impl AsByteSequence for GrabPortRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.port.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GrabPortRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (port, sz): (Port, usize) = <Port>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GrabPortRequest {
                req_type: req_type,
                port: port,
                length: length,
                time: time,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.port.size() + self.length.size() + self.time.size()
    }
}
impl Request for GrabPortRequest {
    const OPCODE: u8 = 3;
    const EXTENSION: Option<&'static str> = Some("XVideo");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GrabPortReply;
}
#[derive(Clone, Debug, Default)]
pub struct GrabPortReply {
    pub reply_type: u8,
    pub result: GrabPortStatus,
    pub sequence: u16,
    pub length: u32,
}
impl GrabPortReply {}
impl AsByteSequence for GrabPortReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.result.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GrabPortReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (result, sz): (GrabPortStatus, usize) = <GrabPortStatus>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GrabPortReply {
                reply_type: reply_type,
                result: result,
                sequence: sequence,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + self.result.size() + self.sequence.size() + self.length.size()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GrabPortStatus {
    Success = 0,
    BadExtension = 1,
    AlreadyGrabbed = 2,
    InvalidTime = 3,
    BadReply = 4,
    BadAlloc = 5,
}
impl AsByteSequence for GrabPortStatus {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Success, sz)),
            1 => Some((Self::BadExtension, sz)),
            2 => Some((Self::AlreadyGrabbed, sz)),
            3 => Some((Self::InvalidTime, sz)),
            4 => Some((Self::BadReply, sz)),
            5 => Some((Self::BadAlloc, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for GrabPortStatus {
    #[inline]
    fn default() -> GrabPortStatus {
        GrabPortStatus::Success
    }
}
#[derive(Clone, Debug, Default)]
pub struct UngrabPortRequest {
    pub req_type: u8,
    pub port: Port,
    pub length: u16,
    pub time: Timestamp,
}
impl UngrabPortRequest {}
impl AsByteSequence for UngrabPortRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.port.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing UngrabPortRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (port, sz): (Port, usize) = <Port>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            UngrabPortRequest {
                req_type: req_type,
                port: port,
                length: length,
                time: time,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.port.size() + self.length.size() + self.time.size()
    }
}
impl Request for UngrabPortRequest {
    const OPCODE: u8 = 4;
    const EXTENSION: Option<&'static str> = Some("XVideo");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct PutVideoRequest {
    pub req_type: u8,
    pub port: Port,
    pub length: u16,
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub vid_x: Int16,
    pub vid_y: Int16,
    pub vid_w: Card16,
    pub vid_h: Card16,
    pub drw_x: Int16,
    pub drw_y: Int16,
    pub drw_w: Card16,
    pub drw_h: Card16,
}
impl PutVideoRequest {}
impl AsByteSequence for PutVideoRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.port.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        index += self.vid_x.as_bytes(&mut bytes[index..]);
        index += self.vid_y.as_bytes(&mut bytes[index..]);
        index += self.vid_w.as_bytes(&mut bytes[index..]);
        index += self.vid_h.as_bytes(&mut bytes[index..]);
        index += self.drw_x.as_bytes(&mut bytes[index..]);
        index += self.drw_y.as_bytes(&mut bytes[index..]);
        index += self.drw_w.as_bytes(&mut bytes[index..]);
        index += self.drw_h.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PutVideoRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (port, sz): (Port, usize) = <Port>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vid_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vid_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vid_w, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vid_h, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_w, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_h, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PutVideoRequest {
                req_type: req_type,
                port: port,
                length: length,
                drawable: drawable,
                gc: gc,
                vid_x: vid_x,
                vid_y: vid_y,
                vid_w: vid_w,
                vid_h: vid_h,
                drw_x: drw_x,
                drw_y: drw_y,
                drw_w: drw_w,
                drw_h: drw_h,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.port.size()
            + self.length.size()
            + self.drawable.size()
            + self.gc.size()
            + self.vid_x.size()
            + self.vid_y.size()
            + self.vid_w.size()
            + self.vid_h.size()
            + self.drw_x.size()
            + self.drw_y.size()
            + self.drw_w.size()
            + self.drw_h.size()
    }
}
impl Request for PutVideoRequest {
    const OPCODE: u8 = 5;
    const EXTENSION: Option<&'static str> = Some("XVideo");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct PutStillRequest {
    pub req_type: u8,
    pub port: Port,
    pub length: u16,
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub vid_x: Int16,
    pub vid_y: Int16,
    pub vid_w: Card16,
    pub vid_h: Card16,
    pub drw_x: Int16,
    pub drw_y: Int16,
    pub drw_w: Card16,
    pub drw_h: Card16,
}
impl PutStillRequest {}
impl AsByteSequence for PutStillRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.port.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        index += self.vid_x.as_bytes(&mut bytes[index..]);
        index += self.vid_y.as_bytes(&mut bytes[index..]);
        index += self.vid_w.as_bytes(&mut bytes[index..]);
        index += self.vid_h.as_bytes(&mut bytes[index..]);
        index += self.drw_x.as_bytes(&mut bytes[index..]);
        index += self.drw_y.as_bytes(&mut bytes[index..]);
        index += self.drw_w.as_bytes(&mut bytes[index..]);
        index += self.drw_h.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PutStillRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (port, sz): (Port, usize) = <Port>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vid_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vid_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vid_w, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vid_h, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_w, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_h, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PutStillRequest {
                req_type: req_type,
                port: port,
                length: length,
                drawable: drawable,
                gc: gc,
                vid_x: vid_x,
                vid_y: vid_y,
                vid_w: vid_w,
                vid_h: vid_h,
                drw_x: drw_x,
                drw_y: drw_y,
                drw_w: drw_w,
                drw_h: drw_h,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.port.size()
            + self.length.size()
            + self.drawable.size()
            + self.gc.size()
            + self.vid_x.size()
            + self.vid_y.size()
            + self.vid_w.size()
            + self.vid_h.size()
            + self.drw_x.size()
            + self.drw_y.size()
            + self.drw_w.size()
            + self.drw_h.size()
    }
}
impl Request for PutStillRequest {
    const OPCODE: u8 = 6;
    const EXTENSION: Option<&'static str> = Some("XVideo");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GetVideoRequest {
    pub req_type: u8,
    pub port: Port,
    pub length: u16,
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub vid_x: Int16,
    pub vid_y: Int16,
    pub vid_w: Card16,
    pub vid_h: Card16,
    pub drw_x: Int16,
    pub drw_y: Int16,
    pub drw_w: Card16,
    pub drw_h: Card16,
}
impl GetVideoRequest {}
impl AsByteSequence for GetVideoRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.port.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        index += self.vid_x.as_bytes(&mut bytes[index..]);
        index += self.vid_y.as_bytes(&mut bytes[index..]);
        index += self.vid_w.as_bytes(&mut bytes[index..]);
        index += self.vid_h.as_bytes(&mut bytes[index..]);
        index += self.drw_x.as_bytes(&mut bytes[index..]);
        index += self.drw_y.as_bytes(&mut bytes[index..]);
        index += self.drw_w.as_bytes(&mut bytes[index..]);
        index += self.drw_h.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetVideoRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (port, sz): (Port, usize) = <Port>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vid_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vid_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vid_w, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vid_h, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_w, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_h, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetVideoRequest {
                req_type: req_type,
                port: port,
                length: length,
                drawable: drawable,
                gc: gc,
                vid_x: vid_x,
                vid_y: vid_y,
                vid_w: vid_w,
                vid_h: vid_h,
                drw_x: drw_x,
                drw_y: drw_y,
                drw_w: drw_w,
                drw_h: drw_h,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.port.size()
            + self.length.size()
            + self.drawable.size()
            + self.gc.size()
            + self.vid_x.size()
            + self.vid_y.size()
            + self.vid_w.size()
            + self.vid_h.size()
            + self.drw_x.size()
            + self.drw_y.size()
            + self.drw_w.size()
            + self.drw_h.size()
    }
}
impl Request for GetVideoRequest {
    const OPCODE: u8 = 7;
    const EXTENSION: Option<&'static str> = Some("XVideo");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GetStillRequest {
    pub req_type: u8,
    pub port: Port,
    pub length: u16,
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub vid_x: Int16,
    pub vid_y: Int16,
    pub vid_w: Card16,
    pub vid_h: Card16,
    pub drw_x: Int16,
    pub drw_y: Int16,
    pub drw_w: Card16,
    pub drw_h: Card16,
}
impl GetStillRequest {}
impl AsByteSequence for GetStillRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.port.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        index += self.vid_x.as_bytes(&mut bytes[index..]);
        index += self.vid_y.as_bytes(&mut bytes[index..]);
        index += self.vid_w.as_bytes(&mut bytes[index..]);
        index += self.vid_h.as_bytes(&mut bytes[index..]);
        index += self.drw_x.as_bytes(&mut bytes[index..]);
        index += self.drw_y.as_bytes(&mut bytes[index..]);
        index += self.drw_w.as_bytes(&mut bytes[index..]);
        index += self.drw_h.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetStillRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (port, sz): (Port, usize) = <Port>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vid_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vid_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vid_w, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vid_h, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_w, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_h, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetStillRequest {
                req_type: req_type,
                port: port,
                length: length,
                drawable: drawable,
                gc: gc,
                vid_x: vid_x,
                vid_y: vid_y,
                vid_w: vid_w,
                vid_h: vid_h,
                drw_x: drw_x,
                drw_y: drw_y,
                drw_w: drw_w,
                drw_h: drw_h,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.port.size()
            + self.length.size()
            + self.drawable.size()
            + self.gc.size()
            + self.vid_x.size()
            + self.vid_y.size()
            + self.vid_w.size()
            + self.vid_h.size()
            + self.drw_x.size()
            + self.drw_y.size()
            + self.drw_w.size()
            + self.drw_h.size()
    }
}
impl Request for GetStillRequest {
    const OPCODE: u8 = 8;
    const EXTENSION: Option<&'static str> = Some("XVideo");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct StopVideoRequest {
    pub req_type: u8,
    pub port: Port,
    pub length: u16,
    pub drawable: Drawable,
}
impl StopVideoRequest {}
impl AsByteSequence for StopVideoRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.port.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing StopVideoRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (port, sz): (Port, usize) = <Port>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            StopVideoRequest {
                req_type: req_type,
                port: port,
                length: length,
                drawable: drawable,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.port.size() + self.length.size() + self.drawable.size()
    }
}
impl Request for StopVideoRequest {
    const OPCODE: u8 = 9;
    const EXTENSION: Option<&'static str> = Some("XVideo");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct SelectVideoNotifyRequest {
    pub req_type: u8,
    pub drawable: Drawable,
    pub length: u16,
    pub onoff: bool,
}
impl SelectVideoNotifyRequest {}
impl AsByteSequence for SelectVideoNotifyRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.onoff.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SelectVideoNotifyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (onoff, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            SelectVideoNotifyRequest {
                req_type: req_type,
                drawable: drawable,
                length: length,
                onoff: onoff,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.drawable.size() + self.length.size() + self.onoff.size() + 3
    }
}
impl Request for SelectVideoNotifyRequest {
    const OPCODE: u8 = 10;
    const EXTENSION: Option<&'static str> = Some("XVideo");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct SelectPortNotifyRequest {
    pub req_type: u8,
    pub port: Port,
    pub length: u16,
    pub onoff: bool,
}
impl SelectPortNotifyRequest {}
impl AsByteSequence for SelectPortNotifyRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.port.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.onoff.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SelectPortNotifyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (port, sz): (Port, usize) = <Port>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (onoff, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            SelectPortNotifyRequest {
                req_type: req_type,
                port: port,
                length: length,
                onoff: onoff,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.port.size() + self.length.size() + self.onoff.size() + 3
    }
}
impl Request for SelectPortNotifyRequest {
    const OPCODE: u8 = 11;
    const EXTENSION: Option<&'static str> = Some("XVideo");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct QueryBestSizeRequest {
    pub req_type: u8,
    pub port: Port,
    pub length: u16,
    pub vid_w: Card16,
    pub vid_h: Card16,
    pub drw_w: Card16,
    pub drw_h: Card16,
    pub motion: bool,
}
impl QueryBestSizeRequest {}
impl AsByteSequence for QueryBestSizeRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.port.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.vid_w.as_bytes(&mut bytes[index..]);
        index += self.vid_h.as_bytes(&mut bytes[index..]);
        index += self.drw_w.as_bytes(&mut bytes[index..]);
        index += self.drw_h.as_bytes(&mut bytes[index..]);
        index += self.motion.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryBestSizeRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (port, sz): (Port, usize) = <Port>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vid_w, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vid_h, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_w, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_h, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (motion, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            QueryBestSizeRequest {
                req_type: req_type,
                port: port,
                length: length,
                vid_w: vid_w,
                vid_h: vid_h,
                drw_w: drw_w,
                drw_h: drw_h,
                motion: motion,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.port.size()
            + self.length.size()
            + self.vid_w.size()
            + self.vid_h.size()
            + self.drw_w.size()
            + self.drw_h.size()
            + self.motion.size()
            + 3
    }
}
impl Request for QueryBestSizeRequest {
    const OPCODE: u8 = 12;
    const EXTENSION: Option<&'static str> = Some("XVideo");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryBestSizeReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryBestSizeReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub actual_width: Card16,
    pub actual_height: Card16,
}
impl QueryBestSizeReply {}
impl AsByteSequence for QueryBestSizeReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.actual_width.as_bytes(&mut bytes[index..]);
        index += self.actual_height.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryBestSizeReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (actual_width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (actual_height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryBestSizeReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                actual_width: actual_width,
                actual_height: actual_height,
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
            + self.actual_width.size()
            + self.actual_height.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct SetPortAttributeRequest {
    pub req_type: u8,
    pub port: Port,
    pub length: u16,
    pub attribute: Atom,
    pub value: Int32,
}
impl SetPortAttributeRequest {}
impl AsByteSequence for SetPortAttributeRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.port.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.attribute.as_bytes(&mut bytes[index..]);
        index += self.value.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetPortAttributeRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (port, sz): (Port, usize) = <Port>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (attribute, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetPortAttributeRequest {
                req_type: req_type,
                port: port,
                length: length,
                attribute: attribute,
                value: value,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.port.size()
            + self.length.size()
            + self.attribute.size()
            + self.value.size()
    }
}
impl Request for SetPortAttributeRequest {
    const OPCODE: u8 = 13;
    const EXTENSION: Option<&'static str> = Some("XVideo");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GetPortAttributeRequest {
    pub req_type: u8,
    pub port: Port,
    pub length: u16,
    pub attribute: Atom,
}
impl GetPortAttributeRequest {}
impl AsByteSequence for GetPortAttributeRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.port.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.attribute.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPortAttributeRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (port, sz): (Port, usize) = <Port>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (attribute, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetPortAttributeRequest {
                req_type: req_type,
                port: port,
                length: length,
                attribute: attribute,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.port.size() + self.length.size() + self.attribute.size()
    }
}
impl Request for GetPortAttributeRequest {
    const OPCODE: u8 = 14;
    const EXTENSION: Option<&'static str> = Some("XVideo");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetPortAttributeReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetPortAttributeReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub value: Int32,
}
impl GetPortAttributeReply {}
impl AsByteSequence for GetPortAttributeReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.value.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPortAttributeReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetPortAttributeReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                value: value,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + 1 + self.sequence.size() + self.length.size() + self.value.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct QueryPortAttributesRequest {
    pub req_type: u8,
    pub port: Port,
    pub length: u16,
}
impl QueryPortAttributesRequest {}
impl AsByteSequence for QueryPortAttributesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.port.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryPortAttributesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (port, sz): (Port, usize) = <Port>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryPortAttributesRequest {
                req_type: req_type,
                port: port,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.port.size() + self.length.size()
    }
}
impl Request for QueryPortAttributesRequest {
    const OPCODE: u8 = 15;
    const EXTENSION: Option<&'static str> = Some("XVideo");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryPortAttributesReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryPortAttributesReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub text_size: Card32,
    pub attributes: Vec<AttributeInfo>,
}
impl QueryPortAttributesReply {}
impl AsByteSequence for QueryPortAttributesReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.attributes.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.text_size.as_bytes(&mut bytes[index..]);
        index += 16;
        let block_len: usize = vector_as_bytes(&self.attributes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<AttributeInfo>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryPortAttributesReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (text_size, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 16;
        let (attributes, block_len): (Vec<AttributeInfo>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<AttributeInfo>());
        Some((
            QueryPortAttributesReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                text_size: text_size,
                attributes: attributes,
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
            + self.text_size.size()
            + 16
            + {
                let block_len: usize = self.attributes.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<AttributeInfo>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct ListImageFormatsRequest {
    pub req_type: u8,
    pub port: Port,
    pub length: u16,
}
impl ListImageFormatsRequest {}
impl AsByteSequence for ListImageFormatsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.port.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListImageFormatsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (port, sz): (Port, usize) = <Port>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ListImageFormatsRequest {
                req_type: req_type,
                port: port,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.port.size() + self.length.size()
    }
}
impl Request for ListImageFormatsRequest {
    const OPCODE: u8 = 16;
    const EXTENSION: Option<&'static str> = Some("XVideo");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ListImageFormatsReply;
}
#[derive(Clone, Debug, Default)]
pub struct ListImageFormatsReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub format: Vec<ImageFormatInfo>,
}
impl ListImageFormatsReply {}
impl AsByteSequence for ListImageFormatsReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.format.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.format, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ImageFormatInfo>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListImageFormatsReply from byte buffer");
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
        let (format, block_len): (Vec<ImageFormatInfo>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ImageFormatInfo>());
        Some((
            ListImageFormatsReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                format: format,
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
                let block_len: usize = self.format.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<ImageFormatInfo>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct QueryImageAttributesRequest {
    pub req_type: u8,
    pub port: Port,
    pub length: u16,
    pub id: Card32,
    pub width: Card16,
    pub height: Card16,
}
impl QueryImageAttributesRequest {}
impl AsByteSequence for QueryImageAttributesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.port.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.id.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryImageAttributesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (port, sz): (Port, usize) = <Port>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (id, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryImageAttributesRequest {
                req_type: req_type,
                port: port,
                length: length,
                id: id,
                width: width,
                height: height,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.port.size()
            + self.length.size()
            + self.id.size()
            + self.width.size()
            + self.height.size()
    }
}
impl Request for QueryImageAttributesRequest {
    const OPCODE: u8 = 17;
    const EXTENSION: Option<&'static str> = Some("XVideo");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryImageAttributesReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryImageAttributesReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_planes: Card32,
    pub data_size: Card32,
    pub width: Card16,
    pub height: Card16,
    pub pitches: Vec<Card32>,
    pub offsets: Vec<Card32>,
}
impl QueryImageAttributesReply {}
impl AsByteSequence for QueryImageAttributesReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.num_planes.as_bytes(&mut bytes[index..]);
        index += self.data_size.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.pitches, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.offsets, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryImageAttributesReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_planes, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (data_size, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (pitches, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], (num_planes as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (offsets, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], (num_planes as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            QueryImageAttributesReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                num_planes: num_planes,
                data_size: data_size,
                width: width,
                height: height,
                pitches: pitches,
                offsets: offsets,
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
            + self.num_planes.size()
            + self.data_size.size()
            + self.width.size()
            + self.height.size()
            + 12
            + {
                let block_len: usize = self.pitches.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.offsets.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct PutImageRequest {
    pub req_type: u8,
    pub port: Port,
    pub length: u16,
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub id: Card32,
    pub src_x: Int16,
    pub src_y: Int16,
    pub src_w: Card16,
    pub src_h: Card16,
    pub drw_x: Int16,
    pub drw_y: Int16,
    pub drw_w: Card16,
    pub drw_h: Card16,
    pub width: Card16,
    pub height: Card16,
    pub data: Vec<Card8>,
}
impl PutImageRequest {}
impl AsByteSequence for PutImageRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.port.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        index += self.id.as_bytes(&mut bytes[index..]);
        index += self.src_x.as_bytes(&mut bytes[index..]);
        index += self.src_y.as_bytes(&mut bytes[index..]);
        index += self.src_w.as_bytes(&mut bytes[index..]);
        index += self.src_h.as_bytes(&mut bytes[index..]);
        index += self.drw_x.as_bytes(&mut bytes[index..]);
        index += self.drw_y.as_bytes(&mut bytes[index..]);
        index += self.drw_w.as_bytes(&mut bytes[index..]);
        index += self.drw_h.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PutImageRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (port, sz): (Port, usize) = <Port>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (id, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_w, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_h, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_w, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_h, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (data, block_len): (Vec<Card8>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        Some((
            PutImageRequest {
                req_type: req_type,
                port: port,
                length: length,
                drawable: drawable,
                gc: gc,
                id: id,
                src_x: src_x,
                src_y: src_y,
                src_w: src_w,
                src_h: src_h,
                drw_x: drw_x,
                drw_y: drw_y,
                drw_w: drw_w,
                drw_h: drw_h,
                width: width,
                height: height,
                data: data,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.port.size()
            + self.length.size()
            + self.drawable.size()
            + self.gc.size()
            + self.id.size()
            + self.src_x.size()
            + self.src_y.size()
            + self.src_w.size()
            + self.src_h.size()
            + self.drw_x.size()
            + self.drw_y.size()
            + self.drw_w.size()
            + self.drw_h.size()
            + self.width.size()
            + self.height.size()
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
    }
}
impl Request for PutImageRequest {
    const OPCODE: u8 = 18;
    const EXTENSION: Option<&'static str> = Some("XVideo");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct ShmPutImageRequest {
    pub req_type: u8,
    pub port: Port,
    pub length: u16,
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub shmseg: Seg,
    pub id: Card32,
    pub offset: Card32,
    pub src_x: Int16,
    pub src_y: Int16,
    pub src_w: Card16,
    pub src_h: Card16,
    pub drw_x: Int16,
    pub drw_y: Int16,
    pub drw_w: Card16,
    pub drw_h: Card16,
    pub width: Card16,
    pub height: Card16,
    pub send_event: Card8,
}
impl ShmPutImageRequest {}
impl AsByteSequence for ShmPutImageRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.port.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        index += self.shmseg.as_bytes(&mut bytes[index..]);
        index += self.id.as_bytes(&mut bytes[index..]);
        index += self.offset.as_bytes(&mut bytes[index..]);
        index += self.src_x.as_bytes(&mut bytes[index..]);
        index += self.src_y.as_bytes(&mut bytes[index..]);
        index += self.src_w.as_bytes(&mut bytes[index..]);
        index += self.src_h.as_bytes(&mut bytes[index..]);
        index += self.drw_x.as_bytes(&mut bytes[index..]);
        index += self.drw_y.as_bytes(&mut bytes[index..]);
        index += self.drw_w.as_bytes(&mut bytes[index..]);
        index += self.drw_h.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.send_event.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ShmPutImageRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (port, sz): (Port, usize) = <Port>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (shmseg, sz): (Seg, usize) = <Seg>::from_bytes(&bytes[index..])?;
        index += sz;
        let (id, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (offset, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_w, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_h, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_w, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drw_h, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (send_event, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            ShmPutImageRequest {
                req_type: req_type,
                port: port,
                length: length,
                drawable: drawable,
                gc: gc,
                shmseg: shmseg,
                id: id,
                offset: offset,
                src_x: src_x,
                src_y: src_y,
                src_w: src_w,
                src_h: src_h,
                drw_x: drw_x,
                drw_y: drw_y,
                drw_w: drw_w,
                drw_h: drw_h,
                width: width,
                height: height,
                send_event: send_event,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.port.size()
            + self.length.size()
            + self.drawable.size()
            + self.gc.size()
            + self.shmseg.size()
            + self.id.size()
            + self.offset.size()
            + self.src_x.size()
            + self.src_y.size()
            + self.src_w.size()
            + self.src_h.size()
            + self.drw_x.size()
            + self.drw_y.size()
            + self.drw_w.size()
            + self.drw_h.size()
            + self.width.size()
            + self.height.size()
            + self.send_event.size()
            + 3
    }
}
impl Request for ShmPutImageRequest {
    const OPCODE: u8 = 19;
    const EXTENSION: Option<&'static str> = Some("XVideo");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct PortNotifyEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub time: Timestamp,
    pub port: Port,
    pub attribute: Atom,
    pub value: Int32,
}
impl PortNotifyEvent {}
impl AsByteSequence for PortNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.port.as_bytes(&mut bytes[index..]);
        index += self.attribute.as_bytes(&mut bytes[index..]);
        index += self.value.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PortNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (port, sz): (Port, usize) = <Port>::from_bytes(&bytes[index..])?;
        index += sz;
        let (attribute, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PortNotifyEvent {
                event_type: event_type,
                sequence: sequence,
                time: time,
                port: port,
                attribute: attribute,
                value: value,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + 1
            + self.sequence.size()
            + self.time.size()
            + self.port.size()
            + self.attribute.size()
            + self.value.size()
    }
}
impl crate::auto::Event for PortNotifyEvent {
    const OPCODE: u8 = 1;
}
#[derive(Clone, Debug, Default)]
pub struct VideoNotifyEvent {
    pub event_type: u8,
    pub reason: VideoNotifyReason,
    pub sequence: u16,
    pub time: Timestamp,
    pub drawable: Drawable,
    pub port: Port,
}
impl VideoNotifyEvent {}
impl AsByteSequence for VideoNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.reason.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.port.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing VideoNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (reason, sz): (VideoNotifyReason, usize) =
            <VideoNotifyReason>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (port, sz): (Port, usize) = <Port>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            VideoNotifyEvent {
                event_type: event_type,
                reason: reason,
                sequence: sequence,
                time: time,
                drawable: drawable,
                port: port,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.reason.size()
            + self.sequence.size()
            + self.time.size()
            + self.drawable.size()
            + self.port.size()
    }
}
impl crate::auto::Event for VideoNotifyEvent {
    const OPCODE: u8 = 0;
}
