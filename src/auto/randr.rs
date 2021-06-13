// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

use super::render::*;
use super::xproto::*;
#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mode {
    pub xid: XID,
}
impl Mode {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Mode {
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
pub struct Crtc {
    pub xid: XID,
}
impl Crtc {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Crtc {
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
pub struct Output {
    pub xid: XID,
}
impl Output {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Output {
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
pub struct Provider {
    pub xid: XID,
}
impl Provider {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Provider {
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
pub struct Lease {
    pub xid: XID,
}
impl Lease {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Lease {
    #[inline]
    fn xid(&self) -> XID {
        self.xid
    }
    #[inline]
    fn from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ScreenSize {
    pub width: Card16,
    pub height: Card16,
    pub mwidth: Card16,
    pub mheight: Card16,
}
impl ScreenSize {}
impl AsByteSequence for ScreenSize {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.mwidth.as_bytes(&mut bytes[index..]);
        index += self.mheight.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ScreenSize from byte buffer");
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mwidth, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mheight, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ScreenSize {
                width: width,
                height: height,
                mwidth: mwidth,
                mheight: mheight,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.width.size() + self.height.size() + self.mwidth.size() + self.mheight.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct RefreshRates<'a> {
    pub rates: Cow<'a, [Card16]>,
}
impl<'a> RefreshRates<'a> {}
impl<'a> AsByteSequence for RefreshRates<'a> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += (self.rates.len() as Card16).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.rates, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing RefreshRates from byte buffer");
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (rates, block_len): (Cow<'_, [Card16]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        Some((RefreshRates { rates: rates }, index))
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<Card16>() + {
            let block_len: usize = self.rates.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card16>());
            block_len + pad
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryVersionRequest {
    pub req_type: u8,
    pub length: u16,
    pub major_version: Card32,
    pub minor_version: Card32,
}
impl QueryVersionRequest {}
impl AsByteSequence for QueryVersionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.major_version.as_bytes(&mut bytes[index..]);
        index += self.minor_version.as_bytes(&mut bytes[index..]);
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
        let (major_version, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_version, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryVersionRequest {
                req_type: req_type,
                length: length,
                major_version: major_version,
                minor_version: minor_version,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.major_version.size()
            + self.minor_version.size()
    }
}
impl Request for QueryVersionRequest {
    const OPCODE: u8 = 0;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryVersionReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
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
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetScreenConfigRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub timestamp: Timestamp,
    pub config_timestamp: Timestamp,
    pub size_id: Card16,
    pub rotation: Rotation,
    pub rate: Card16,
}
impl SetScreenConfigRequest {}
impl AsByteSequence for SetScreenConfigRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += self.config_timestamp.as_bytes(&mut bytes[index..]);
        index += self.size_id.as_bytes(&mut bytes[index..]);
        index += self.rotation.as_bytes(&mut bytes[index..]);
        index += self.rate.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetScreenConfigRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (config_timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (size_id, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (rotation, sz): (Rotation, usize) = <Rotation>::from_bytes(&bytes[index..])?;
        index += sz;
        let (rate, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            SetScreenConfigRequest {
                req_type: req_type,
                length: length,
                window: window,
                timestamp: timestamp,
                config_timestamp: config_timestamp,
                size_id: size_id,
                rotation: rotation,
                rate: rate,
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
            + self.timestamp.size()
            + self.config_timestamp.size()
            + self.size_id.size()
            + self.rotation.size()
            + self.rate.size()
            + 2
    }
}
impl Request for SetScreenConfigRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = SetScreenConfigReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetScreenConfigReply {
    pub reply_type: u8,
    pub status: SetConfig,
    pub sequence: u16,
    pub length: u32,
    pub new_timestamp: Timestamp,
    pub config_timestamp: Timestamp,
    pub root: Window,
    pub subpixel_order: SubPixel,
}
impl SetScreenConfigReply {}
impl AsByteSequence for SetScreenConfigReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.new_timestamp.as_bytes(&mut bytes[index..]);
        index += self.config_timestamp.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.subpixel_order.as_bytes(&mut bytes[index..]);
        index += 10;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetScreenConfigReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (SetConfig, usize) = <SetConfig>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (new_timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (config_timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (subpixel_order, sz): (SubPixel, usize) = <SubPixel>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 10;
        Some((
            SetScreenConfigReply {
                reply_type: reply_type,
                status: status,
                sequence: sequence,
                length: length,
                new_timestamp: new_timestamp,
                config_timestamp: config_timestamp,
                root: root,
                subpixel_order: subpixel_order,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.status.size()
            + self.sequence.size()
            + self.length.size()
            + self.new_timestamp.size()
            + self.config_timestamp.size()
            + self.root.size()
            + self.subpixel_order.size()
            + 10
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rotation {
    pub inner: u16,
}
impl Rotation {
    #[inline]
    pub fn rotate_0(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_rotate_0(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn rotate_90(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_rotate_90(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn rotate_180(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_rotate_180(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn rotate_270(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_rotate_270(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn reflect_x(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_reflect_x(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn reflect_y(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_reflect_y(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn new(
        rotate_0: bool,
        rotate_90: bool,
        rotate_180: bool,
        rotate_270: bool,
        reflect_x: bool,
        reflect_y: bool,
    ) -> Self {
        let mut inner: u16 = 0;
        if rotate_0 {
            inner |= 1 << 0;
        }
        if rotate_90 {
            inner |= 1 << 1;
        }
        if rotate_180 {
            inner |= 1 << 2;
        }
        if rotate_270 {
            inner |= 1 << 3;
        }
        if reflect_x {
            inner |= 1 << 4;
        }
        if reflect_y {
            inner |= 1 << 5;
        }
        Rotation { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const ROTATE_0: Self = Self { inner: 1 };
    pub const ROTATE_90: Self = Self { inner: 2 };
    pub const ROTATE_180: Self = Self { inner: 4 };
    pub const ROTATE_270: Self = Self { inner: 8 };
    pub const REFLECT_X: Self = Self { inner: 16 };
    pub const REFLECT_Y: Self = Self { inner: 32 };
    pub const COMPLETE: Self = Self { inner: 63 };
}
impl AsByteSequence for Rotation {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        Some((Rotation { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for Rotation {
    type Output = Rotation;
    #[inline]
    fn not(self) -> Rotation {
        Rotation { inner: !self.inner }
    }
}
impl core::ops::BitAnd for Rotation {
    type Output = Rotation;
    #[inline]
    fn bitand(self, rhs: Rotation) -> Rotation {
        Rotation {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for Rotation {
    type Output = Rotation;
    #[inline]
    fn bitor(self, rhs: Rotation) -> Rotation {
        Rotation {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for Rotation {
    type Output = Rotation;
    #[inline]
    fn bitxor(self, rhs: Rotation) -> Rotation {
        Rotation {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum SetConfig {
    Success = 0,
    InvalidConfigTime = 1,
    InvalidTime = 2,
    Failed = 3,
}
impl AsByteSequence for SetConfig {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Success, sz)),
            1 => Some((Self::InvalidConfigTime, sz)),
            2 => Some((Self::InvalidTime, sz)),
            3 => Some((Self::Failed, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for SetConfig {
    #[inline]
    fn default() -> SetConfig {
        SetConfig::Success
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SelectInputRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub enable: NotifyMask,
}
impl SelectInputRequest {}
impl AsByteSequence for SelectInputRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.enable.as_bytes(&mut bytes[index..]);
        index += 2;
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
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (enable, sz): (NotifyMask, usize) = <NotifyMask>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            SelectInputRequest {
                req_type: req_type,
                length: length,
                window: window,
                enable: enable,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size() + self.enable.size() + 2
    }
}
impl Request for SelectInputRequest {
    const OPCODE: u8 = 4;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NotifyMask {
    pub inner: u16,
}
impl NotifyMask {
    #[inline]
    pub fn screen_change(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_screen_change(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn crtc_change(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_crtc_change(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn output_change(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_output_change(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn output_property(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_output_property(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn provider_change(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_provider_change(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn provider_property(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_provider_property(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn resource_change(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_resource_change(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn lease(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_lease(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn new(
        screen_change: bool,
        crtc_change: bool,
        output_change: bool,
        output_property: bool,
        provider_change: bool,
        provider_property: bool,
        resource_change: bool,
        lease: bool,
    ) -> Self {
        let mut inner: u16 = 0;
        if screen_change {
            inner |= 1 << 0;
        }
        if crtc_change {
            inner |= 1 << 1;
        }
        if output_change {
            inner |= 1 << 2;
        }
        if output_property {
            inner |= 1 << 3;
        }
        if provider_change {
            inner |= 1 << 4;
        }
        if provider_property {
            inner |= 1 << 5;
        }
        if resource_change {
            inner |= 1 << 6;
        }
        if lease {
            inner |= 1 << 7;
        }
        NotifyMask { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const SCREEN_CHANGE: Self = Self { inner: 1 };
    pub const CRTC_CHANGE: Self = Self { inner: 2 };
    pub const OUTPUT_CHANGE: Self = Self { inner: 4 };
    pub const OUTPUT_PROPERTY: Self = Self { inner: 8 };
    pub const PROVIDER_CHANGE: Self = Self { inner: 16 };
    pub const PROVIDER_PROPERTY: Self = Self { inner: 32 };
    pub const RESOURCE_CHANGE: Self = Self { inner: 64 };
    pub const LEASE: Self = Self { inner: 128 };
    pub const COMPLETE: Self = Self { inner: 255 };
}
impl AsByteSequence for NotifyMask {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        Some((NotifyMask { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for NotifyMask {
    type Output = NotifyMask;
    #[inline]
    fn not(self) -> NotifyMask {
        NotifyMask { inner: !self.inner }
    }
}
impl core::ops::BitAnd for NotifyMask {
    type Output = NotifyMask;
    #[inline]
    fn bitand(self, rhs: NotifyMask) -> NotifyMask {
        NotifyMask {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for NotifyMask {
    type Output = NotifyMask;
    #[inline]
    fn bitor(self, rhs: NotifyMask) -> NotifyMask {
        NotifyMask {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for NotifyMask {
    type Output = NotifyMask;
    #[inline]
    fn bitxor(self, rhs: NotifyMask) -> NotifyMask {
        NotifyMask {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetScreenInfoRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
}
impl GetScreenInfoRequest {}
impl AsByteSequence for GetScreenInfoRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetScreenInfoRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetScreenInfoRequest {
                req_type: req_type,
                length: length,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size()
    }
}
impl Request for GetScreenInfoRequest {
    const OPCODE: u8 = 5;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetScreenInfoReply<'static, 'static, 'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetScreenInfoReply<'b, 'd, 'c> {
    pub reply_type: u8,
    pub rotations: Rotation,
    pub sequence: u16,
    pub length: u32,
    pub root: Window,
    pub timestamp: Timestamp,
    pub config_timestamp: Timestamp,
    pub n_sizes: Card16,
    pub size_id: Card16,
    pub rotation: Rotation,
    pub rate: Card16,
    pub n_info: Card16,
    pub sizes: Cow<'b, [ScreenSize]>,
    pub rates: Cow<'d, [RefreshRates<'c>]>,
}
impl<'b, 'd, 'c> GetScreenInfoReply<'b, 'd, 'c> {}
impl<'b, 'd, 'c> AsByteSequence for GetScreenInfoReply<'b, 'd, 'c> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.rotations.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += self.config_timestamp.as_bytes(&mut bytes[index..]);
        index += self.n_sizes.as_bytes(&mut bytes[index..]);
        index += self.size_id.as_bytes(&mut bytes[index..]);
        index += self.rotation.as_bytes(&mut bytes[index..]);
        index += self.rate.as_bytes(&mut bytes[index..]);
        index += self.n_info.as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = vector_as_bytes(&self.sizes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ScreenSize>());
        let block_len: usize = vector_as_bytes(&self.rates, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<RefreshRates<'c>>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetScreenInfoReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (rotations, sz): (Rotation, usize) = <Rotation>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (config_timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_sizes, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (size_id, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (rotation, sz): (Rotation, usize) = <Rotation>::from_bytes(&bytes[index..])?;
        index += sz;
        let (rate, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_info, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (sizes, block_len): (Cow<'_, [ScreenSize]>, usize) =
            vector_from_bytes(&bytes[index..], (n_sizes as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ScreenSize>());
        let (rates, block_len): (Cow<'_, [RefreshRates<'_>]>, usize) = vector_from_bytes(
            &bytes[index..],
            ((n_info as usize) - (n_sizes as usize)) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<RefreshRates<'c>>());
        Some((
            GetScreenInfoReply {
                reply_type: reply_type,
                rotations: rotations,
                sequence: sequence,
                length: length,
                root: root,
                timestamp: timestamp,
                config_timestamp: config_timestamp,
                n_sizes: n_sizes,
                size_id: size_id,
                rotation: rotation,
                rate: rate,
                n_info: n_info,
                sizes: sizes,
                rates: rates,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.rotations.size()
            + self.sequence.size()
            + self.length.size()
            + self.root.size()
            + self.timestamp.size()
            + self.config_timestamp.size()
            + self.n_sizes.size()
            + self.size_id.size()
            + self.rotation.size()
            + self.rate.size()
            + self.n_info.size()
            + 2
            + {
                let block_len: usize = self.sizes.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<ScreenSize>());
                block_len + pad
            }
            + {
                let block_len: usize = self.rates.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<RefreshRates<'c>>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetScreenSizeRangeRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
}
impl GetScreenSizeRangeRequest {}
impl AsByteSequence for GetScreenSizeRangeRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetScreenSizeRangeRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetScreenSizeRangeRequest {
                req_type: req_type,
                length: length,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size()
    }
}
impl Request for GetScreenSizeRangeRequest {
    const OPCODE: u8 = 6;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetScreenSizeRangeReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetScreenSizeRangeReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub min_width: Card16,
    pub min_height: Card16,
    pub max_width: Card16,
    pub max_height: Card16,
}
impl GetScreenSizeRangeReply {}
impl AsByteSequence for GetScreenSizeRangeReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.min_width.as_bytes(&mut bytes[index..]);
        index += self.min_height.as_bytes(&mut bytes[index..]);
        index += self.max_width.as_bytes(&mut bytes[index..]);
        index += self.max_height.as_bytes(&mut bytes[index..]);
        index += 16;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetScreenSizeRangeReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (min_width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (min_height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 16;
        Some((
            GetScreenSizeRangeReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                min_width: min_width,
                min_height: min_height,
                max_width: max_width,
                max_height: max_height,
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
            + self.min_width.size()
            + self.min_height.size()
            + self.max_width.size()
            + self.max_height.size()
            + 16
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetScreenSizeRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub width: Card16,
    pub height: Card16,
    pub mm_width: Card32,
    pub mm_height: Card32,
}
impl SetScreenSizeRequest {}
impl AsByteSequence for SetScreenSizeRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.mm_width.as_bytes(&mut bytes[index..]);
        index += self.mm_height.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetScreenSizeRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mm_width, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mm_height, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetScreenSizeRequest {
                req_type: req_type,
                length: length,
                window: window,
                width: width,
                height: height,
                mm_width: mm_width,
                mm_height: mm_height,
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
            + self.width.size()
            + self.height.size()
            + self.mm_width.size()
            + self.mm_height.size()
    }
}
impl Request for SetScreenSizeRequest {
    const OPCODE: u8 = 7;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ModeInfo {
    pub id: Card32,
    pub width: Card16,
    pub height: Card16,
    pub dot_clock: Card32,
    pub hsync_start: Card16,
    pub hsync_end: Card16,
    pub htotal: Card16,
    pub hskew: Card16,
    pub vsync_start: Card16,
    pub vsync_end: Card16,
    pub vtotal: Card16,
    pub name_len: Card16,
    pub mode_flags: ModeFlag,
}
impl ModeInfo {}
impl AsByteSequence for ModeInfo {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.id.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.dot_clock.as_bytes(&mut bytes[index..]);
        index += self.hsync_start.as_bytes(&mut bytes[index..]);
        index += self.hsync_end.as_bytes(&mut bytes[index..]);
        index += self.htotal.as_bytes(&mut bytes[index..]);
        index += self.hskew.as_bytes(&mut bytes[index..]);
        index += self.vsync_start.as_bytes(&mut bytes[index..]);
        index += self.vsync_end.as_bytes(&mut bytes[index..]);
        index += self.vtotal.as_bytes(&mut bytes[index..]);
        index += self.name_len.as_bytes(&mut bytes[index..]);
        index += self.mode_flags.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ModeInfo from byte buffer");
        let (id, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dot_clock, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hsync_start, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hsync_end, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (htotal, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hskew, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vsync_start, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vsync_end, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vtotal, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (name_len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode_flags, sz): (ModeFlag, usize) = <ModeFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ModeInfo {
                id: id,
                width: width,
                height: height,
                dot_clock: dot_clock,
                hsync_start: hsync_start,
                hsync_end: hsync_end,
                htotal: htotal,
                hskew: hskew,
                vsync_start: vsync_start,
                vsync_end: vsync_end,
                vtotal: vtotal,
                name_len: name_len,
                mode_flags: mode_flags,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.id.size()
            + self.width.size()
            + self.height.size()
            + self.dot_clock.size()
            + self.hsync_start.size()
            + self.hsync_end.size()
            + self.htotal.size()
            + self.hskew.size()
            + self.vsync_start.size()
            + self.vsync_end.size()
            + self.vtotal.size()
            + self.name_len.size()
            + self.mode_flags.size()
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModeFlag {
    pub inner: u32,
}
impl ModeFlag {
    #[inline]
    pub fn hsync_positive(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_hsync_positive(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn hsync_negative(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_hsync_negative(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn vsync_positive(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_vsync_positive(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn vsync_negative(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_vsync_negative(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn interlace(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_interlace(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn double_scan(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_double_scan(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn csync(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_csync(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn csync_positive(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_csync_positive(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn csync_negative(&self) -> bool {
        self.inner & (1 << 8) != 0
    }
    #[inline]
    pub fn set_csync_negative(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 8;
        } else {
            self.inner &= !(1 << 8);
        }
        self
    }
    #[inline]
    pub fn hskew_present(&self) -> bool {
        self.inner & (1 << 9) != 0
    }
    #[inline]
    pub fn set_hskew_present(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 9;
        } else {
            self.inner &= !(1 << 9);
        }
        self
    }
    #[inline]
    pub fn bcast(&self) -> bool {
        self.inner & (1 << 10) != 0
    }
    #[inline]
    pub fn set_bcast(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 10;
        } else {
            self.inner &= !(1 << 10);
        }
        self
    }
    #[inline]
    pub fn pixel_multiplex(&self) -> bool {
        self.inner & (1 << 11) != 0
    }
    #[inline]
    pub fn set_pixel_multiplex(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 11;
        } else {
            self.inner &= !(1 << 11);
        }
        self
    }
    #[inline]
    pub fn double_clock(&self) -> bool {
        self.inner & (1 << 12) != 0
    }
    #[inline]
    pub fn set_double_clock(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 12;
        } else {
            self.inner &= !(1 << 12);
        }
        self
    }
    #[inline]
    pub fn halve_clock(&self) -> bool {
        self.inner & (1 << 13) != 0
    }
    #[inline]
    pub fn set_halve_clock(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 13;
        } else {
            self.inner &= !(1 << 13);
        }
        self
    }
    #[inline]
    pub fn new(
        hsync_positive: bool,
        hsync_negative: bool,
        vsync_positive: bool,
        vsync_negative: bool,
        interlace: bool,
        double_scan: bool,
        csync: bool,
        csync_positive: bool,
        csync_negative: bool,
        hskew_present: bool,
        bcast: bool,
        pixel_multiplex: bool,
        double_clock: bool,
        halve_clock: bool,
    ) -> Self {
        let mut inner: u32 = 0;
        if hsync_positive {
            inner |= 1 << 0;
        }
        if hsync_negative {
            inner |= 1 << 1;
        }
        if vsync_positive {
            inner |= 1 << 2;
        }
        if vsync_negative {
            inner |= 1 << 3;
        }
        if interlace {
            inner |= 1 << 4;
        }
        if double_scan {
            inner |= 1 << 5;
        }
        if csync {
            inner |= 1 << 6;
        }
        if csync_positive {
            inner |= 1 << 7;
        }
        if csync_negative {
            inner |= 1 << 8;
        }
        if hskew_present {
            inner |= 1 << 9;
        }
        if bcast {
            inner |= 1 << 10;
        }
        if pixel_multiplex {
            inner |= 1 << 11;
        }
        if double_clock {
            inner |= 1 << 12;
        }
        if halve_clock {
            inner |= 1 << 13;
        }
        ModeFlag { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const HSYNC_POSITIVE: Self = Self { inner: 1 };
    pub const HSYNC_NEGATIVE: Self = Self { inner: 2 };
    pub const VSYNC_POSITIVE: Self = Self { inner: 4 };
    pub const VSYNC_NEGATIVE: Self = Self { inner: 8 };
    pub const INTERLACE: Self = Self { inner: 16 };
    pub const DOUBLE_SCAN: Self = Self { inner: 32 };
    pub const CSYNC: Self = Self { inner: 64 };
    pub const CSYNC_POSITIVE: Self = Self { inner: 128 };
    pub const CSYNC_NEGATIVE: Self = Self { inner: 256 };
    pub const HSKEW_PRESENT: Self = Self { inner: 512 };
    pub const BCAST: Self = Self { inner: 1024 };
    pub const PIXEL_MULTIPLEX: Self = Self { inner: 2048 };
    pub const DOUBLE_CLOCK: Self = Self { inner: 4096 };
    pub const HALVE_CLOCK: Self = Self { inner: 8192 };
    pub const COMPLETE: Self = Self { inner: 16383 };
}
impl AsByteSequence for ModeFlag {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((ModeFlag { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for ModeFlag {
    type Output = ModeFlag;
    #[inline]
    fn not(self) -> ModeFlag {
        ModeFlag { inner: !self.inner }
    }
}
impl core::ops::BitAnd for ModeFlag {
    type Output = ModeFlag;
    #[inline]
    fn bitand(self, rhs: ModeFlag) -> ModeFlag {
        ModeFlag {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for ModeFlag {
    type Output = ModeFlag;
    #[inline]
    fn bitor(self, rhs: ModeFlag) -> ModeFlag {
        ModeFlag {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for ModeFlag {
    type Output = ModeFlag;
    #[inline]
    fn bitxor(self, rhs: ModeFlag) -> ModeFlag {
        ModeFlag {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetScreenResourcesRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
}
impl GetScreenResourcesRequest {}
impl AsByteSequence for GetScreenResourcesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetScreenResourcesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetScreenResourcesRequest {
                req_type: req_type,
                length: length,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size()
    }
}
impl Request for GetScreenResourcesRequest {
    const OPCODE: u8 = 8;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetScreenResourcesReply<'static, 'static, 'static, 'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetScreenResourcesReply<'e, 'f, 'g, 'h> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: Timestamp,
    pub config_timestamp: Timestamp,
    pub crtcs: Cow<'e, [Crtc]>,
    pub outputs: Cow<'f, [Output]>,
    pub modes: Cow<'g, [ModeInfo]>,
    pub names: Cow<'h, [Byte]>,
}
impl<'e, 'f, 'g, 'h> GetScreenResourcesReply<'e, 'f, 'g, 'h> {}
impl<'e, 'f, 'g, 'h> AsByteSequence for GetScreenResourcesReply<'e, 'f, 'g, 'h> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += self.config_timestamp.as_bytes(&mut bytes[index..]);
        index += (self.crtcs.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.outputs.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.modes.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.names.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 8;
        let block_len: usize = vector_as_bytes(&self.crtcs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Crtc>());
        let block_len: usize = vector_as_bytes(&self.outputs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Output>());
        let block_len: usize = vector_as_bytes(&self.modes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ModeInfo>());
        let block_len: usize = vector_as_bytes(&self.names, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetScreenResourcesReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (config_timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len2, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len3, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 8;
        let (crtcs, block_len): (Cow<'_, [Crtc]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Crtc>());
        let (outputs, block_len): (Cow<'_, [Output]>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Output>());
        let (modes, block_len): (Cow<'_, [ModeInfo]>, usize) =
            vector_from_bytes(&bytes[index..], len2 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ModeInfo>());
        let (names, block_len): (Cow<'_, [Byte]>, usize) =
            vector_from_bytes(&bytes[index..], len3 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            GetScreenResourcesReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                timestamp: timestamp,
                config_timestamp: config_timestamp,
                crtcs: crtcs,
                outputs: outputs,
                modes: modes,
                names: names,
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
            + self.timestamp.size()
            + self.config_timestamp.size()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + 8
            + {
                let block_len: usize = self.crtcs.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Crtc>());
                block_len + pad
            }
            + {
                let block_len: usize = self.outputs.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Output>());
                block_len + pad
            }
            + {
                let block_len: usize = self.modes.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<ModeInfo>());
                block_len + pad
            }
            + {
                let block_len: usize = self.names.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetOutputInfoRequest {
    pub req_type: u8,
    pub length: u16,
    pub output: Output,
    pub config_timestamp: Timestamp,
}
impl GetOutputInfoRequest {}
impl AsByteSequence for GetOutputInfoRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.output.as_bytes(&mut bytes[index..]);
        index += self.config_timestamp.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetOutputInfoRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (output, sz): (Output, usize) = <Output>::from_bytes(&bytes[index..])?;
        index += sz;
        let (config_timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetOutputInfoRequest {
                req_type: req_type,
                length: length,
                output: output,
                config_timestamp: config_timestamp,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.output.size()
            + self.config_timestamp.size()
    }
}
impl Request for GetOutputInfoRequest {
    const OPCODE: u8 = 9;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetOutputInfoReply<'static, 'static, 'static, 'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetOutputInfoReply<'i, 'j, 'k, 'l> {
    pub reply_type: u8,
    pub status: SetConfig,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: Timestamp,
    pub crtc: Crtc,
    pub mm_width: Card32,
    pub mm_height: Card32,
    pub connection: Connection,
    pub subpixel_order: SubPixel,
    pub num_preferred: Card16,
    pub crtcs: Cow<'i, [Crtc]>,
    pub modes: Cow<'j, [Mode]>,
    pub clones: Cow<'k, [Output]>,
    pub name: Cow<'l, [Byte]>,
}
impl<'i, 'j, 'k, 'l> GetOutputInfoReply<'i, 'j, 'k, 'l> {}
impl<'i, 'j, 'k, 'l> AsByteSequence for GetOutputInfoReply<'i, 'j, 'k, 'l> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += self.crtc.as_bytes(&mut bytes[index..]);
        index += self.mm_width.as_bytes(&mut bytes[index..]);
        index += self.mm_height.as_bytes(&mut bytes[index..]);
        index += self.connection.as_bytes(&mut bytes[index..]);
        index += self.subpixel_order.as_bytes(&mut bytes[index..]);
        index += (self.crtcs.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.modes.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.num_preferred.as_bytes(&mut bytes[index..]);
        index += (self.clones.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.name.len() as Card16).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.crtcs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Crtc>());
        let block_len: usize = vector_as_bytes(&self.modes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Mode>());
        let block_len: usize = vector_as_bytes(&self.clones, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Output>());
        let block_len: usize = vector_as_bytes(&self.name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetOutputInfoReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (SetConfig, usize) = <SetConfig>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (crtc, sz): (Crtc, usize) = <Crtc>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mm_width, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mm_height, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (connection, sz): (Connection, usize) = <Connection>::from_bytes(&bytes[index..])?;
        index += sz;
        let (subpixel_order, sz): (SubPixel, usize) = <SubPixel>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_preferred, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len2, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len3, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (crtcs, block_len): (Cow<'_, [Crtc]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Crtc>());
        let (modes, block_len): (Cow<'_, [Mode]>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Mode>());
        let (clones, block_len): (Cow<'_, [Output]>, usize) =
            vector_from_bytes(&bytes[index..], len2 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Output>());
        let (name, block_len): (Cow<'_, [Byte]>, usize) =
            vector_from_bytes(&bytes[index..], len3 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            GetOutputInfoReply {
                reply_type: reply_type,
                status: status,
                sequence: sequence,
                length: length,
                timestamp: timestamp,
                crtc: crtc,
                mm_width: mm_width,
                mm_height: mm_height,
                connection: connection,
                subpixel_order: subpixel_order,
                num_preferred: num_preferred,
                crtcs: crtcs,
                modes: modes,
                clones: clones,
                name: name,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.status.size()
            + self.sequence.size()
            + self.length.size()
            + self.timestamp.size()
            + self.crtc.size()
            + self.mm_width.size()
            + self.mm_height.size()
            + self.connection.size()
            + self.subpixel_order.size()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + self.num_preferred.size()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + {
                let block_len: usize = self.crtcs.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Crtc>());
                block_len + pad
            }
            + {
                let block_len: usize = self.modes.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Mode>());
                block_len + pad
            }
            + {
                let block_len: usize = self.clones.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Output>());
                block_len + pad
            }
            + {
                let block_len: usize = self.name.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Connection {
    Connected = 0,
    Disconnected = 1,
    Unknown = 2,
}
impl AsByteSequence for Connection {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Connected, sz)),
            1 => Some((Self::Disconnected, sz)),
            2 => Some((Self::Unknown, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for Connection {
    #[inline]
    fn default() -> Connection {
        Connection::Connected
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ListOutputPropertiesRequest {
    pub req_type: u8,
    pub length: u16,
    pub output: Output,
}
impl ListOutputPropertiesRequest {}
impl AsByteSequence for ListOutputPropertiesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.output.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListOutputPropertiesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (output, sz): (Output, usize) = <Output>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ListOutputPropertiesRequest {
                req_type: req_type,
                length: length,
                output: output,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.output.size()
    }
}
impl Request for ListOutputPropertiesRequest {
    const OPCODE: u8 = 10;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ListOutputPropertiesReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ListOutputPropertiesReply<'m> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub atoms: Cow<'m, [Atom]>,
}
impl<'m> ListOutputPropertiesReply<'m> {}
impl<'m> AsByteSequence for ListOutputPropertiesReply<'m> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.atoms.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 22;
        let block_len: usize = vector_as_bytes(&self.atoms, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListOutputPropertiesReply from byte buffer");
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
        let (atoms, block_len): (Cow<'_, [Atom]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        Some((
            ListOutputPropertiesReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                atoms: atoms,
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
                let block_len: usize = self.atoms.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryOutputPropertyRequest {
    pub req_type: u8,
    pub length: u16,
    pub output: Output,
    pub property: Atom,
}
impl QueryOutputPropertyRequest {}
impl AsByteSequence for QueryOutputPropertyRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.output.as_bytes(&mut bytes[index..]);
        index += self.property.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryOutputPropertyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (output, sz): (Output, usize) = <Output>::from_bytes(&bytes[index..])?;
        index += sz;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryOutputPropertyRequest {
                req_type: req_type,
                length: length,
                output: output,
                property: property,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.output.size() + self.property.size()
    }
}
impl Request for QueryOutputPropertyRequest {
    const OPCODE: u8 = 11;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryOutputPropertyReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryOutputPropertyReply<'n> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub pending: bool,
    pub range: bool,
    pub immutable: bool,
    pub valid_values: Cow<'n, [Int32]>,
}
impl<'n> QueryOutputPropertyReply<'n> {}
impl<'n> AsByteSequence for QueryOutputPropertyReply<'n> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.pending.as_bytes(&mut bytes[index..]);
        index += self.range.as_bytes(&mut bytes[index..]);
        index += self.immutable.as_bytes(&mut bytes[index..]);
        index += 21;
        let block_len: usize = vector_as_bytes(&self.valid_values, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryOutputPropertyReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pending, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (range, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (immutable, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 21;
        let (valid_values, block_len): (Cow<'_, [Int32]>, usize) =
            vector_from_bytes(&bytes[index..], (length as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        Some((
            QueryOutputPropertyReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                pending: pending,
                range: range,
                immutable: immutable,
                valid_values: valid_values,
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
            + self.pending.size()
            + self.range.size()
            + self.immutable.size()
            + 21
            + {
                let block_len: usize = self.valid_values.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Int32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ConfigureOutputPropertyRequest<'o> {
    pub req_type: u8,
    pub length: u16,
    pub output: Output,
    pub property: Atom,
    pub pending: bool,
    pub range: bool,
    pub values: Cow<'o, [Int32]>,
}
impl<'o> ConfigureOutputPropertyRequest<'o> {}
impl<'o> AsByteSequence for ConfigureOutputPropertyRequest<'o> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.output.as_bytes(&mut bytes[index..]);
        index += self.property.as_bytes(&mut bytes[index..]);
        index += self.pending.as_bytes(&mut bytes[index..]);
        index += self.range.as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = vector_as_bytes(&self.values, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ConfigureOutputPropertyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (output, sz): (Output, usize) = <Output>::from_bytes(&bytes[index..])?;
        index += sz;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pending, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (range, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (values, block_len): (Cow<'_, [Int32]>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        Some((
            ConfigureOutputPropertyRequest {
                req_type: req_type,
                length: length,
                output: output,
                property: property,
                pending: pending,
                range: range,
                values: values,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.output.size()
            + self.property.size()
            + self.pending.size()
            + self.range.size()
            + 2
            + {
                let block_len: usize = self.values.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Int32>());
                block_len + pad
            }
    }
}
impl<'o> Request for ConfigureOutputPropertyRequest<'o> {
    const OPCODE: u8 = 12;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ChangeOutputPropertyRequest<'p> {
    pub req_type: u8,
    pub length: u16,
    pub output: Output,
    pub property: Atom,
    pub ty: Atom,
    pub format: Card8,
    pub mode: PropMode,
    pub num_units: Card32,
    pub data: Cow<'p, [Void]>,
}
impl<'p> ChangeOutputPropertyRequest<'p> {}
impl<'p> AsByteSequence for ChangeOutputPropertyRequest<'p> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.output.as_bytes(&mut bytes[index..]);
        index += self.property.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.num_units.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Void>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangeOutputPropertyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (output, sz): (Output, usize) = <Output>::from_bytes(&bytes[index..])?;
        index += sz;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (PropMode, usize) = <PropMode>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (num_units, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (data, block_len): (Cow<'_, [Void]>, usize) = vector_from_bytes(
            &bytes[index..],
            (((num_units as usize) * (format as usize)) / (8)) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Void>());
        Some((
            ChangeOutputPropertyRequest {
                req_type: req_type,
                length: length,
                output: output,
                property: property,
                ty: ty,
                format: format,
                mode: mode,
                num_units: num_units,
                data: data,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.output.size()
            + self.property.size()
            + self.ty.size()
            + self.format.size()
            + self.mode.size()
            + 2
            + self.num_units.size()
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Void>());
                block_len + pad
            }
    }
}
impl<'p> Request for ChangeOutputPropertyRequest<'p> {
    const OPCODE: u8 = 13;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeleteOutputPropertyRequest {
    pub req_type: u8,
    pub length: u16,
    pub output: Output,
    pub property: Atom,
}
impl DeleteOutputPropertyRequest {}
impl AsByteSequence for DeleteOutputPropertyRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.output.as_bytes(&mut bytes[index..]);
        index += self.property.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeleteOutputPropertyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (output, sz): (Output, usize) = <Output>::from_bytes(&bytes[index..])?;
        index += sz;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DeleteOutputPropertyRequest {
                req_type: req_type,
                length: length,
                output: output,
                property: property,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.output.size() + self.property.size()
    }
}
impl Request for DeleteOutputPropertyRequest {
    const OPCODE: u8 = 14;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetOutputPropertyRequest {
    pub req_type: u8,
    pub length: u16,
    pub output: Output,
    pub property: Atom,
    pub ty: Atom,
    pub long_offset: Card32,
    pub long_length: Card32,
    pub delete: bool,
    pub pending: bool,
}
impl GetOutputPropertyRequest {}
impl AsByteSequence for GetOutputPropertyRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.output.as_bytes(&mut bytes[index..]);
        index += self.property.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.long_offset.as_bytes(&mut bytes[index..]);
        index += self.long_length.as_bytes(&mut bytes[index..]);
        index += self.delete.as_bytes(&mut bytes[index..]);
        index += self.pending.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetOutputPropertyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (output, sz): (Output, usize) = <Output>::from_bytes(&bytes[index..])?;
        index += sz;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (long_offset, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (long_length, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (delete, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pending, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            GetOutputPropertyRequest {
                req_type: req_type,
                length: length,
                output: output,
                property: property,
                ty: ty,
                long_offset: long_offset,
                long_length: long_length,
                delete: delete,
                pending: pending,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.output.size()
            + self.property.size()
            + self.ty.size()
            + self.long_offset.size()
            + self.long_length.size()
            + self.delete.size()
            + self.pending.size()
            + 2
    }
}
impl Request for GetOutputPropertyRequest {
    const OPCODE: u8 = 15;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetOutputPropertyReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetOutputPropertyReply<'q> {
    pub reply_type: u8,
    pub format: Card8,
    pub sequence: u16,
    pub length: u32,
    pub ty: Atom,
    pub bytes_after: Card32,
    pub num_items: Card32,
    pub data: Cow<'q, [Byte]>,
}
impl<'q> GetOutputPropertyReply<'q> {}
impl<'q> AsByteSequence for GetOutputPropertyReply<'q> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.bytes_after.as_bytes(&mut bytes[index..]);
        index += self.num_items.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetOutputPropertyReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bytes_after, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_items, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Cow<'_, [Byte]>, usize) = vector_from_bytes(
            &bytes[index..],
            ((num_items as usize) * ((format as usize) / (8))) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            GetOutputPropertyReply {
                reply_type: reply_type,
                format: format,
                sequence: sequence,
                length: length,
                ty: ty,
                bytes_after: bytes_after,
                num_items: num_items,
                data: data,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.format.size()
            + self.sequence.size()
            + self.length.size()
            + self.ty.size()
            + self.bytes_after.size()
            + self.num_items.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CreateModeRequest<'r> {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub mode_info: ModeInfo,
    pub name: Cow<'r, str>,
}
impl<'r> CreateModeRequest<'r> {}
impl<'r> AsByteSequence for CreateModeRequest<'r> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.mode_info.as_bytes(&mut bytes[index..]);
        let block_len: usize = string_as_bytes(&self.name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateModeRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode_info, sz): (ModeInfo, usize) = <ModeInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (name, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            CreateModeRequest {
                req_type: req_type,
                length: length,
                window: window,
                mode_info: mode_info,
                name: name,
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
            + self.mode_info.size()
            + {
                let block_len: usize = self.name.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
impl<'r> Request for CreateModeRequest<'r> {
    const OPCODE: u8 = 16;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = CreateModeReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CreateModeReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub mode: Mode,
}
impl CreateModeReply {}
impl AsByteSequence for CreateModeReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += 20;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateModeReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (Mode, usize) = <Mode>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        Some((
            CreateModeReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                mode: mode,
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
            + self.mode.size()
            + 20
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DestroyModeRequest {
    pub req_type: u8,
    pub length: u16,
    pub mode: Mode,
}
impl DestroyModeRequest {}
impl AsByteSequence for DestroyModeRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DestroyModeRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (Mode, usize) = <Mode>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DestroyModeRequest {
                req_type: req_type,
                length: length,
                mode: mode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.mode.size()
    }
}
impl Request for DestroyModeRequest {
    const OPCODE: u8 = 17;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct AddOutputModeRequest {
    pub req_type: u8,
    pub length: u16,
    pub output: Output,
    pub mode: Mode,
}
impl AddOutputModeRequest {}
impl AsByteSequence for AddOutputModeRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.output.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AddOutputModeRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (output, sz): (Output, usize) = <Output>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (Mode, usize) = <Mode>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            AddOutputModeRequest {
                req_type: req_type,
                length: length,
                output: output,
                mode: mode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.output.size() + self.mode.size()
    }
}
impl Request for AddOutputModeRequest {
    const OPCODE: u8 = 18;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeleteOutputModeRequest {
    pub req_type: u8,
    pub length: u16,
    pub output: Output,
    pub mode: Mode,
}
impl DeleteOutputModeRequest {}
impl AsByteSequence for DeleteOutputModeRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.output.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeleteOutputModeRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (output, sz): (Output, usize) = <Output>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (Mode, usize) = <Mode>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DeleteOutputModeRequest {
                req_type: req_type,
                length: length,
                output: output,
                mode: mode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.output.size() + self.mode.size()
    }
}
impl Request for DeleteOutputModeRequest {
    const OPCODE: u8 = 19;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetCrtcInfoRequest {
    pub req_type: u8,
    pub length: u16,
    pub crtc: Crtc,
    pub config_timestamp: Timestamp,
}
impl GetCrtcInfoRequest {}
impl AsByteSequence for GetCrtcInfoRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.crtc.as_bytes(&mut bytes[index..]);
        index += self.config_timestamp.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetCrtcInfoRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (crtc, sz): (Crtc, usize) = <Crtc>::from_bytes(&bytes[index..])?;
        index += sz;
        let (config_timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetCrtcInfoRequest {
                req_type: req_type,
                length: length,
                crtc: crtc,
                config_timestamp: config_timestamp,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.crtc.size()
            + self.config_timestamp.size()
    }
}
impl Request for GetCrtcInfoRequest {
    const OPCODE: u8 = 20;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetCrtcInfoReply<'static, 'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetCrtcInfoReply<'s, 't> {
    pub reply_type: u8,
    pub status: SetConfig,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: Timestamp,
    pub x: Int16,
    pub y: Int16,
    pub width: Card16,
    pub height: Card16,
    pub mode: Mode,
    pub rotation: Rotation,
    pub rotations: Rotation,
    pub outputs: Cow<'s, [Output]>,
    pub possible: Cow<'t, [Output]>,
}
impl<'s, 't> GetCrtcInfoReply<'s, 't> {}
impl<'s, 't> AsByteSequence for GetCrtcInfoReply<'s, 't> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.rotation.as_bytes(&mut bytes[index..]);
        index += self.rotations.as_bytes(&mut bytes[index..]);
        index += (self.outputs.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.possible.len() as Card16).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.outputs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Output>());
        let block_len: usize = vector_as_bytes(&self.possible, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Output>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetCrtcInfoReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (SetConfig, usize) = <SetConfig>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (Mode, usize) = <Mode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (rotation, sz): (Rotation, usize) = <Rotation>::from_bytes(&bytes[index..])?;
        index += sz;
        let (rotations, sz): (Rotation, usize) = <Rotation>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (outputs, block_len): (Cow<'_, [Output]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Output>());
        let (possible, block_len): (Cow<'_, [Output]>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Output>());
        Some((
            GetCrtcInfoReply {
                reply_type: reply_type,
                status: status,
                sequence: sequence,
                length: length,
                timestamp: timestamp,
                x: x,
                y: y,
                width: width,
                height: height,
                mode: mode,
                rotation: rotation,
                rotations: rotations,
                outputs: outputs,
                possible: possible,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.status.size()
            + self.sequence.size()
            + self.length.size()
            + self.timestamp.size()
            + self.x.size()
            + self.y.size()
            + self.width.size()
            + self.height.size()
            + self.mode.size()
            + self.rotation.size()
            + self.rotations.size()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + {
                let block_len: usize = self.outputs.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Output>());
                block_len + pad
            }
            + {
                let block_len: usize = self.possible.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Output>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetCrtcConfigRequest<'u> {
    pub req_type: u8,
    pub length: u16,
    pub crtc: Crtc,
    pub timestamp: Timestamp,
    pub config_timestamp: Timestamp,
    pub x: Int16,
    pub y: Int16,
    pub mode: Mode,
    pub rotation: Rotation,
    pub outputs: Cow<'u, [Output]>,
}
impl<'u> SetCrtcConfigRequest<'u> {}
impl<'u> AsByteSequence for SetCrtcConfigRequest<'u> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.crtc.as_bytes(&mut bytes[index..]);
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += self.config_timestamp.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.rotation.as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = vector_as_bytes(&self.outputs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Output>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetCrtcConfigRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (crtc, sz): (Crtc, usize) = <Crtc>::from_bytes(&bytes[index..])?;
        index += sz;
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (config_timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (Mode, usize) = <Mode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (rotation, sz): (Rotation, usize) = <Rotation>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (outputs, block_len): (Cow<'_, [Output]>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Output>());
        Some((
            SetCrtcConfigRequest {
                req_type: req_type,
                length: length,
                crtc: crtc,
                timestamp: timestamp,
                config_timestamp: config_timestamp,
                x: x,
                y: y,
                mode: mode,
                rotation: rotation,
                outputs: outputs,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.crtc.size()
            + self.timestamp.size()
            + self.config_timestamp.size()
            + self.x.size()
            + self.y.size()
            + self.mode.size()
            + self.rotation.size()
            + 2
            + {
                let block_len: usize = self.outputs.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Output>());
                block_len + pad
            }
    }
}
impl<'u> Request for SetCrtcConfigRequest<'u> {
    const OPCODE: u8 = 21;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = SetCrtcConfigReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetCrtcConfigReply {
    pub reply_type: u8,
    pub status: SetConfig,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: Timestamp,
}
impl SetCrtcConfigReply {}
impl AsByteSequence for SetCrtcConfigReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += 20;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetCrtcConfigReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (SetConfig, usize) = <SetConfig>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        Some((
            SetCrtcConfigReply {
                reply_type: reply_type,
                status: status,
                sequence: sequence,
                length: length,
                timestamp: timestamp,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.status.size()
            + self.sequence.size()
            + self.length.size()
            + self.timestamp.size()
            + 20
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetCrtcGammaSizeRequest {
    pub req_type: u8,
    pub length: u16,
    pub crtc: Crtc,
}
impl GetCrtcGammaSizeRequest {}
impl AsByteSequence for GetCrtcGammaSizeRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.crtc.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetCrtcGammaSizeRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (crtc, sz): (Crtc, usize) = <Crtc>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetCrtcGammaSizeRequest {
                req_type: req_type,
                length: length,
                crtc: crtc,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.crtc.size()
    }
}
impl Request for GetCrtcGammaSizeRequest {
    const OPCODE: u8 = 22;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetCrtcGammaSizeReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetCrtcGammaSizeReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub size: Card16,
}
impl GetCrtcGammaSizeReply {}
impl AsByteSequence for GetCrtcGammaSizeReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.size.as_bytes(&mut bytes[index..]);
        index += 22;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetCrtcGammaSizeReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (size, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 22;
        Some((
            GetCrtcGammaSizeReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                size: size,
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
            + self.size.size()
            + 22
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetCrtcGammaRequest {
    pub req_type: u8,
    pub length: u16,
    pub crtc: Crtc,
}
impl GetCrtcGammaRequest {}
impl AsByteSequence for GetCrtcGammaRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.crtc.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetCrtcGammaRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (crtc, sz): (Crtc, usize) = <Crtc>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetCrtcGammaRequest {
                req_type: req_type,
                length: length,
                crtc: crtc,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.crtc.size()
    }
}
impl Request for GetCrtcGammaRequest {
    const OPCODE: u8 = 23;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetCrtcGammaReply<'static, 'static, 'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetCrtcGammaReply<'v, 'w, 'x> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub size: Card16,
    pub red: Cow<'v, [Card16]>,
    pub green: Cow<'w, [Card16]>,
    pub blue: Cow<'x, [Card16]>,
}
impl<'v, 'w, 'x> GetCrtcGammaReply<'v, 'w, 'x> {}
impl<'v, 'w, 'x> AsByteSequence for GetCrtcGammaReply<'v, 'w, 'x> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.size.as_bytes(&mut bytes[index..]);
        index += 22;
        let block_len: usize = vector_as_bytes(&self.red, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        let block_len: usize = vector_as_bytes(&self.green, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        let block_len: usize = vector_as_bytes(&self.blue, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetCrtcGammaReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (size, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 22;
        let (red, block_len): (Cow<'_, [Card16]>, usize) =
            vector_from_bytes(&bytes[index..], (size as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        let (green, block_len): (Cow<'_, [Card16]>, usize) =
            vector_from_bytes(&bytes[index..], (size as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        let (blue, block_len): (Cow<'_, [Card16]>, usize) =
            vector_from_bytes(&bytes[index..], (size as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        Some((
            GetCrtcGammaReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                size: size,
                red: red,
                green: green,
                blue: blue,
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
            + self.size.size()
            + 22
            + {
                let block_len: usize = self.red.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card16>());
                block_len + pad
            }
            + {
                let block_len: usize = self.green.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card16>());
                block_len + pad
            }
            + {
                let block_len: usize = self.blue.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card16>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetCrtcGammaRequest<'y, 'z, 'ab> {
    pub req_type: u8,
    pub length: u16,
    pub crtc: Crtc,
    pub size: Card16,
    pub red: Cow<'y, [Card16]>,
    pub green: Cow<'z, [Card16]>,
    pub blue: Cow<'ab, [Card16]>,
}
impl<'y, 'z, 'ab> SetCrtcGammaRequest<'y, 'z, 'ab> {}
impl<'y, 'z, 'ab> AsByteSequence for SetCrtcGammaRequest<'y, 'z, 'ab> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.crtc.as_bytes(&mut bytes[index..]);
        index += self.size.as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = vector_as_bytes(&self.red, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        let block_len: usize = vector_as_bytes(&self.green, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        let block_len: usize = vector_as_bytes(&self.blue, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetCrtcGammaRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (crtc, sz): (Crtc, usize) = <Crtc>::from_bytes(&bytes[index..])?;
        index += sz;
        let (size, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (red, block_len): (Cow<'_, [Card16]>, usize) =
            vector_from_bytes(&bytes[index..], (size as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        let (green, block_len): (Cow<'_, [Card16]>, usize) =
            vector_from_bytes(&bytes[index..], (size as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        let (blue, block_len): (Cow<'_, [Card16]>, usize) =
            vector_from_bytes(&bytes[index..], (size as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        Some((
            SetCrtcGammaRequest {
                req_type: req_type,
                length: length,
                crtc: crtc,
                size: size,
                red: red,
                green: green,
                blue: blue,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.crtc.size()
            + self.size.size()
            + 2
            + {
                let block_len: usize = self.red.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card16>());
                block_len + pad
            }
            + {
                let block_len: usize = self.green.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card16>());
                block_len + pad
            }
            + {
                let block_len: usize = self.blue.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card16>());
                block_len + pad
            }
    }
}
impl<'y, 'z, 'ab> Request for SetCrtcGammaRequest<'y, 'z, 'ab> {
    const OPCODE: u8 = 24;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetScreenResourcesCurrentRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
}
impl GetScreenResourcesCurrentRequest {}
impl AsByteSequence for GetScreenResourcesCurrentRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetScreenResourcesCurrentRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetScreenResourcesCurrentRequest {
                req_type: req_type,
                length: length,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size()
    }
}
impl Request for GetScreenResourcesCurrentRequest {
    const OPCODE: u8 = 25;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetScreenResourcesCurrentReply<'static, 'static, 'static, 'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetScreenResourcesCurrentReply<'bb, 'cb, 'db, 'eb> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: Timestamp,
    pub config_timestamp: Timestamp,
    pub crtcs: Cow<'bb, [Crtc]>,
    pub outputs: Cow<'cb, [Output]>,
    pub modes: Cow<'db, [ModeInfo]>,
    pub names: Cow<'eb, [Byte]>,
}
impl<'bb, 'cb, 'db, 'eb> GetScreenResourcesCurrentReply<'bb, 'cb, 'db, 'eb> {}
impl<'bb, 'cb, 'db, 'eb> AsByteSequence for GetScreenResourcesCurrentReply<'bb, 'cb, 'db, 'eb> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += self.config_timestamp.as_bytes(&mut bytes[index..]);
        index += (self.crtcs.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.outputs.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.modes.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.names.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 8;
        let block_len: usize = vector_as_bytes(&self.crtcs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Crtc>());
        let block_len: usize = vector_as_bytes(&self.outputs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Output>());
        let block_len: usize = vector_as_bytes(&self.modes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ModeInfo>());
        let block_len: usize = vector_as_bytes(&self.names, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetScreenResourcesCurrentReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (config_timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len2, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len3, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 8;
        let (crtcs, block_len): (Cow<'_, [Crtc]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Crtc>());
        let (outputs, block_len): (Cow<'_, [Output]>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Output>());
        let (modes, block_len): (Cow<'_, [ModeInfo]>, usize) =
            vector_from_bytes(&bytes[index..], len2 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ModeInfo>());
        let (names, block_len): (Cow<'_, [Byte]>, usize) =
            vector_from_bytes(&bytes[index..], len3 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            GetScreenResourcesCurrentReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                timestamp: timestamp,
                config_timestamp: config_timestamp,
                crtcs: crtcs,
                outputs: outputs,
                modes: modes,
                names: names,
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
            + self.timestamp.size()
            + self.config_timestamp.size()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + 8
            + {
                let block_len: usize = self.crtcs.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Crtc>());
                block_len + pad
            }
            + {
                let block_len: usize = self.outputs.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Output>());
                block_len + pad
            }
            + {
                let block_len: usize = self.modes.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<ModeInfo>());
                block_len + pad
            }
            + {
                let block_len: usize = self.names.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetCrtcTransformRequest<'fb, 'gb> {
    pub req_type: u8,
    pub length: u16,
    pub crtc: Crtc,
    pub transform: Transform,
    pub filter_name: Cow<'fb, str>,
    pub filter_params: Cow<'gb, [Fixed]>,
}
impl<'fb, 'gb> SetCrtcTransformRequest<'fb, 'gb> {}
impl<'fb, 'gb> AsByteSequence for SetCrtcTransformRequest<'fb, 'gb> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.crtc.as_bytes(&mut bytes[index..]);
        index += self.transform.as_bytes(&mut bytes[index..]);
        index += (self.filter_name.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = string_as_bytes(&self.filter_name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, 4);
        let block_len: usize = vector_as_bytes(&self.filter_params, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fixed>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetCrtcTransformRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (crtc, sz): (Crtc, usize) = <Crtc>::from_bytes(&bytes[index..])?;
        index += sz;
        let (transform, sz): (Transform, usize) = <Transform>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (filter_name, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, 4);
        let (filter_params, block_len): (Cow<'_, [Fixed]>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fixed>());
        Some((
            SetCrtcTransformRequest {
                req_type: req_type,
                length: length,
                crtc: crtc,
                transform: transform,
                filter_name: filter_name,
                filter_params: filter_params,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.crtc.size()
            + self.transform.size()
            + ::core::mem::size_of::<Card16>()
            + 2
            + {
                let block_len: usize = self.filter_name.len();
                let pad: usize = buffer_pad(block_len, 4);
                block_len + pad
            }
            + {
                let block_len: usize = self.filter_params.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fixed>());
                block_len + pad
            }
    }
}
impl<'fb, 'gb> Request for SetCrtcTransformRequest<'fb, 'gb> {
    const OPCODE: u8 = 26;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetCrtcTransformRequest {
    pub req_type: u8,
    pub length: u16,
    pub crtc: Crtc,
}
impl GetCrtcTransformRequest {}
impl AsByteSequence for GetCrtcTransformRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.crtc.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetCrtcTransformRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (crtc, sz): (Crtc, usize) = <Crtc>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetCrtcTransformRequest {
                req_type: req_type,
                length: length,
                crtc: crtc,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.crtc.size()
    }
}
impl Request for GetCrtcTransformRequest {
    const OPCODE: u8 = 27;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetCrtcTransformReply<'static, 'static, 'static, 'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetCrtcTransformReply<'hb, 'ib, 'jb, 'kb> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub pending_transform: Transform,
    pub has_transforms: bool,
    pub current_transform: Transform,
    pub pending_filter_name: Cow<'hb, str>,
    pub pending_params: Cow<'ib, [Fixed]>,
    pub current_filter_name: Cow<'jb, str>,
    pub current_params: Cow<'kb, [Fixed]>,
}
impl<'hb, 'ib, 'jb, 'kb> GetCrtcTransformReply<'hb, 'ib, 'jb, 'kb> {}
impl<'hb, 'ib, 'jb, 'kb> AsByteSequence for GetCrtcTransformReply<'hb, 'ib, 'jb, 'kb> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.pending_transform.as_bytes(&mut bytes[index..]);
        index += self.has_transforms.as_bytes(&mut bytes[index..]);
        index += 3;
        index += self.current_transform.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.pending_filter_name.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.pending_params.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.current_filter_name.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.current_params.len() as Card16).as_bytes(&mut bytes[index..]);
        let block_len: usize = string_as_bytes(&self.pending_filter_name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, 4);
        let block_len: usize = vector_as_bytes(&self.pending_params, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fixed>());
        let block_len: usize = string_as_bytes(&self.current_filter_name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, 4);
        let block_len: usize = vector_as_bytes(&self.current_params, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fixed>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetCrtcTransformReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pending_transform, sz): (Transform, usize) = <Transform>::from_bytes(&bytes[index..])?;
        index += sz;
        let (has_transforms, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (current_transform, sz): (Transform, usize) = <Transform>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len2, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len3, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pending_filter_name, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, 4);
        let (pending_params, block_len): (Cow<'_, [Fixed]>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fixed>());
        let (current_filter_name, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len2 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, 4);
        let (current_params, block_len): (Cow<'_, [Fixed]>, usize) =
            vector_from_bytes(&bytes[index..], len3 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fixed>());
        Some((
            GetCrtcTransformReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                pending_transform: pending_transform,
                has_transforms: has_transforms,
                current_transform: current_transform,
                pending_filter_name: pending_filter_name,
                pending_params: pending_params,
                current_filter_name: current_filter_name,
                current_params: current_params,
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
            + self.pending_transform.size()
            + self.has_transforms.size()
            + 3
            + self.current_transform.size()
            + 4
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + {
                let block_len: usize = self.pending_filter_name.len();
                let pad: usize = buffer_pad(block_len, 4);
                block_len + pad
            }
            + {
                let block_len: usize = self.pending_params.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fixed>());
                block_len + pad
            }
            + {
                let block_len: usize = self.current_filter_name.len();
                let pad: usize = buffer_pad(block_len, 4);
                block_len + pad
            }
            + {
                let block_len: usize = self.current_params.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fixed>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetPanningRequest {
    pub req_type: u8,
    pub length: u16,
    pub crtc: Crtc,
}
impl GetPanningRequest {}
impl AsByteSequence for GetPanningRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.crtc.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPanningRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (crtc, sz): (Crtc, usize) = <Crtc>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetPanningRequest {
                req_type: req_type,
                length: length,
                crtc: crtc,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.crtc.size()
    }
}
impl Request for GetPanningRequest {
    const OPCODE: u8 = 28;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetPanningReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetPanningReply {
    pub reply_type: u8,
    pub status: SetConfig,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: Timestamp,
    pub left: Card16,
    pub top: Card16,
    pub width: Card16,
    pub height: Card16,
    pub track_left: Card16,
    pub track_top: Card16,
    pub track_width: Card16,
    pub track_height: Card16,
    pub border_left: Int16,
    pub border_top: Int16,
    pub border_right: Int16,
    pub border_bottom: Int16,
}
impl GetPanningReply {}
impl AsByteSequence for GetPanningReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += self.left.as_bytes(&mut bytes[index..]);
        index += self.top.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.track_left.as_bytes(&mut bytes[index..]);
        index += self.track_top.as_bytes(&mut bytes[index..]);
        index += self.track_width.as_bytes(&mut bytes[index..]);
        index += self.track_height.as_bytes(&mut bytes[index..]);
        index += self.border_left.as_bytes(&mut bytes[index..]);
        index += self.border_top.as_bytes(&mut bytes[index..]);
        index += self.border_right.as_bytes(&mut bytes[index..]);
        index += self.border_bottom.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPanningReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (SetConfig, usize) = <SetConfig>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (left, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (top, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (track_left, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (track_top, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (track_width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (track_height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (border_left, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (border_top, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (border_right, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (border_bottom, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetPanningReply {
                reply_type: reply_type,
                status: status,
                sequence: sequence,
                length: length,
                timestamp: timestamp,
                left: left,
                top: top,
                width: width,
                height: height,
                track_left: track_left,
                track_top: track_top,
                track_width: track_width,
                track_height: track_height,
                border_left: border_left,
                border_top: border_top,
                border_right: border_right,
                border_bottom: border_bottom,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.status.size()
            + self.sequence.size()
            + self.length.size()
            + self.timestamp.size()
            + self.left.size()
            + self.top.size()
            + self.width.size()
            + self.height.size()
            + self.track_left.size()
            + self.track_top.size()
            + self.track_width.size()
            + self.track_height.size()
            + self.border_left.size()
            + self.border_top.size()
            + self.border_right.size()
            + self.border_bottom.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetPanningRequest {
    pub req_type: u8,
    pub length: u16,
    pub crtc: Crtc,
    pub timestamp: Timestamp,
    pub left: Card16,
    pub top: Card16,
    pub width: Card16,
    pub height: Card16,
    pub track_left: Card16,
    pub track_top: Card16,
    pub track_width: Card16,
    pub track_height: Card16,
    pub border_left: Int16,
    pub border_top: Int16,
    pub border_right: Int16,
    pub border_bottom: Int16,
}
impl SetPanningRequest {}
impl AsByteSequence for SetPanningRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.crtc.as_bytes(&mut bytes[index..]);
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += self.left.as_bytes(&mut bytes[index..]);
        index += self.top.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.track_left.as_bytes(&mut bytes[index..]);
        index += self.track_top.as_bytes(&mut bytes[index..]);
        index += self.track_width.as_bytes(&mut bytes[index..]);
        index += self.track_height.as_bytes(&mut bytes[index..]);
        index += self.border_left.as_bytes(&mut bytes[index..]);
        index += self.border_top.as_bytes(&mut bytes[index..]);
        index += self.border_right.as_bytes(&mut bytes[index..]);
        index += self.border_bottom.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetPanningRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (crtc, sz): (Crtc, usize) = <Crtc>::from_bytes(&bytes[index..])?;
        index += sz;
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (left, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (top, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (track_left, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (track_top, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (track_width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (track_height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (border_left, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (border_top, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (border_right, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (border_bottom, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetPanningRequest {
                req_type: req_type,
                length: length,
                crtc: crtc,
                timestamp: timestamp,
                left: left,
                top: top,
                width: width,
                height: height,
                track_left: track_left,
                track_top: track_top,
                track_width: track_width,
                track_height: track_height,
                border_left: border_left,
                border_top: border_top,
                border_right: border_right,
                border_bottom: border_bottom,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.crtc.size()
            + self.timestamp.size()
            + self.left.size()
            + self.top.size()
            + self.width.size()
            + self.height.size()
            + self.track_left.size()
            + self.track_top.size()
            + self.track_width.size()
            + self.track_height.size()
            + self.border_left.size()
            + self.border_top.size()
            + self.border_right.size()
            + self.border_bottom.size()
    }
}
impl Request for SetPanningRequest {
    const OPCODE: u8 = 29;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = SetPanningReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetPanningReply {
    pub reply_type: u8,
    pub status: SetConfig,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: Timestamp,
}
impl SetPanningReply {}
impl AsByteSequence for SetPanningReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetPanningReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (SetConfig, usize) = <SetConfig>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetPanningReply {
                reply_type: reply_type,
                status: status,
                sequence: sequence,
                length: length,
                timestamp: timestamp,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.status.size()
            + self.sequence.size()
            + self.length.size()
            + self.timestamp.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetOutputPrimaryRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub output: Output,
}
impl SetOutputPrimaryRequest {}
impl AsByteSequence for SetOutputPrimaryRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.output.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetOutputPrimaryRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (output, sz): (Output, usize) = <Output>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetOutputPrimaryRequest {
                req_type: req_type,
                length: length,
                window: window,
                output: output,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size() + self.output.size()
    }
}
impl Request for SetOutputPrimaryRequest {
    const OPCODE: u8 = 30;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetOutputPrimaryRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
}
impl GetOutputPrimaryRequest {}
impl AsByteSequence for GetOutputPrimaryRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetOutputPrimaryRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetOutputPrimaryRequest {
                req_type: req_type,
                length: length,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size()
    }
}
impl Request for GetOutputPrimaryRequest {
    const OPCODE: u8 = 31;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetOutputPrimaryReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetOutputPrimaryReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub output: Output,
}
impl GetOutputPrimaryReply {}
impl AsByteSequence for GetOutputPrimaryReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.output.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetOutputPrimaryReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (output, sz): (Output, usize) = <Output>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetOutputPrimaryReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                output: output,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + 1 + self.sequence.size() + self.length.size() + self.output.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetProvidersRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
}
impl GetProvidersRequest {}
impl AsByteSequence for GetProvidersRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetProvidersRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetProvidersRequest {
                req_type: req_type,
                length: length,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size()
    }
}
impl Request for GetProvidersRequest {
    const OPCODE: u8 = 32;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetProvidersReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetProvidersReply<'lb> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: Timestamp,
    pub providers: Cow<'lb, [Provider]>,
}
impl<'lb> GetProvidersReply<'lb> {}
impl<'lb> AsByteSequence for GetProvidersReply<'lb> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += (self.providers.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 18;
        let block_len: usize = vector_as_bytes(&self.providers, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Provider>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetProvidersReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 18;
        let (providers, block_len): (Cow<'_, [Provider]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Provider>());
        Some((
            GetProvidersReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                timestamp: timestamp,
                providers: providers,
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
            + self.timestamp.size()
            + ::core::mem::size_of::<Card16>()
            + 18
            + {
                let block_len: usize = self.providers.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Provider>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetProviderInfoRequest {
    pub req_type: u8,
    pub length: u16,
    pub provider: Provider,
    pub config_timestamp: Timestamp,
}
impl GetProviderInfoRequest {}
impl AsByteSequence for GetProviderInfoRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.provider.as_bytes(&mut bytes[index..]);
        index += self.config_timestamp.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetProviderInfoRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (provider, sz): (Provider, usize) = <Provider>::from_bytes(&bytes[index..])?;
        index += sz;
        let (config_timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetProviderInfoRequest {
                req_type: req_type,
                length: length,
                provider: provider,
                config_timestamp: config_timestamp,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.provider.size()
            + self.config_timestamp.size()
    }
}
impl Request for GetProviderInfoRequest {
    const OPCODE: u8 = 33;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetProviderInfoReply<'static, 'static, 'static, 'static, 'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetProviderInfoReply<'mb, 'nb, 'ob, 'pb, 'qb> {
    pub reply_type: u8,
    pub status: Card8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: Timestamp,
    pub capabilities: ProviderCapability,
    pub num_associated_providers: Card16,
    pub crtcs: Cow<'mb, [Crtc]>,
    pub outputs: Cow<'nb, [Output]>,
    pub associated_providers: Cow<'ob, [Provider]>,
    pub associated_capability: Cow<'pb, [Card32]>,
    pub name: Cow<'qb, str>,
}
impl<'mb, 'nb, 'ob, 'pb, 'qb> GetProviderInfoReply<'mb, 'nb, 'ob, 'pb, 'qb> {}
impl<'mb, 'nb, 'ob, 'pb, 'qb> AsByteSequence for GetProviderInfoReply<'mb, 'nb, 'ob, 'pb, 'qb> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += self.capabilities.as_bytes(&mut bytes[index..]);
        index += (self.crtcs.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.outputs.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.num_associated_providers.as_bytes(&mut bytes[index..]);
        index += (self.name.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 8;
        let block_len: usize = vector_as_bytes(&self.crtcs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Crtc>());
        let block_len: usize = vector_as_bytes(&self.outputs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Output>());
        let block_len: usize = vector_as_bytes(&self.associated_providers, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Provider>());
        let block_len: usize = vector_as_bytes(&self.associated_capability, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = string_as_bytes(&self.name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetProviderInfoReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (capabilities, sz): (ProviderCapability, usize) =
            <ProviderCapability>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_associated_providers, sz): (Card16, usize) =
            <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len2, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 8;
        let (crtcs, block_len): (Cow<'_, [Crtc]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Crtc>());
        let (outputs, block_len): (Cow<'_, [Output]>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Output>());
        let (associated_providers, block_len): (Cow<'_, [Provider]>, usize) = vector_from_bytes(
            &bytes[index..],
            (num_associated_providers as usize) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Provider>());
        let (associated_capability, block_len): (Cow<'_, [Card32]>, usize) = vector_from_bytes(
            &bytes[index..],
            (num_associated_providers as usize) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (name, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len2 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            GetProviderInfoReply {
                reply_type: reply_type,
                status: status,
                sequence: sequence,
                length: length,
                timestamp: timestamp,
                capabilities: capabilities,
                num_associated_providers: num_associated_providers,
                crtcs: crtcs,
                outputs: outputs,
                associated_providers: associated_providers,
                associated_capability: associated_capability,
                name: name,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.status.size()
            + self.sequence.size()
            + self.length.size()
            + self.timestamp.size()
            + self.capabilities.size()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + self.num_associated_providers.size()
            + ::core::mem::size_of::<Card16>()
            + 8
            + {
                let block_len: usize = self.crtcs.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Crtc>());
                block_len + pad
            }
            + {
                let block_len: usize = self.outputs.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Output>());
                block_len + pad
            }
            + {
                let block_len: usize = self.associated_providers.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Provider>());
                block_len + pad
            }
            + {
                let block_len: usize = self.associated_capability.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.name.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProviderCapability {
    pub inner: u32,
}
impl ProviderCapability {
    #[inline]
    pub fn source_output(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_source_output(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn sink_output(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_sink_output(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn source_offload(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_source_offload(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn sink_offload(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_sink_offload(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn new(
        source_output: bool,
        sink_output: bool,
        source_offload: bool,
        sink_offload: bool,
    ) -> Self {
        let mut inner: u32 = 0;
        if source_output {
            inner |= 1 << 0;
        }
        if sink_output {
            inner |= 1 << 1;
        }
        if source_offload {
            inner |= 1 << 2;
        }
        if sink_offload {
            inner |= 1 << 3;
        }
        ProviderCapability { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const SOURCE_OUTPUT: Self = Self { inner: 1 };
    pub const SINK_OUTPUT: Self = Self { inner: 2 };
    pub const SOURCE_OFFLOAD: Self = Self { inner: 4 };
    pub const SINK_OFFLOAD: Self = Self { inner: 8 };
    pub const COMPLETE: Self = Self { inner: 15 };
}
impl AsByteSequence for ProviderCapability {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((ProviderCapability { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for ProviderCapability {
    type Output = ProviderCapability;
    #[inline]
    fn not(self) -> ProviderCapability {
        ProviderCapability { inner: !self.inner }
    }
}
impl core::ops::BitAnd for ProviderCapability {
    type Output = ProviderCapability;
    #[inline]
    fn bitand(self, rhs: ProviderCapability) -> ProviderCapability {
        ProviderCapability {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for ProviderCapability {
    type Output = ProviderCapability;
    #[inline]
    fn bitor(self, rhs: ProviderCapability) -> ProviderCapability {
        ProviderCapability {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for ProviderCapability {
    type Output = ProviderCapability;
    #[inline]
    fn bitxor(self, rhs: ProviderCapability) -> ProviderCapability {
        ProviderCapability {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetProviderOffloadSinkRequest {
    pub req_type: u8,
    pub length: u16,
    pub provider: Provider,
    pub sink_provider: Provider,
    pub config_timestamp: Timestamp,
}
impl SetProviderOffloadSinkRequest {}
impl AsByteSequence for SetProviderOffloadSinkRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.provider.as_bytes(&mut bytes[index..]);
        index += self.sink_provider.as_bytes(&mut bytes[index..]);
        index += self.config_timestamp.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetProviderOffloadSinkRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (provider, sz): (Provider, usize) = <Provider>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sink_provider, sz): (Provider, usize) = <Provider>::from_bytes(&bytes[index..])?;
        index += sz;
        let (config_timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetProviderOffloadSinkRequest {
                req_type: req_type,
                length: length,
                provider: provider,
                sink_provider: sink_provider,
                config_timestamp: config_timestamp,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.provider.size()
            + self.sink_provider.size()
            + self.config_timestamp.size()
    }
}
impl Request for SetProviderOffloadSinkRequest {
    const OPCODE: u8 = 34;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetProviderOutputSourceRequest {
    pub req_type: u8,
    pub length: u16,
    pub provider: Provider,
    pub source_provider: Provider,
    pub config_timestamp: Timestamp,
}
impl SetProviderOutputSourceRequest {}
impl AsByteSequence for SetProviderOutputSourceRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.provider.as_bytes(&mut bytes[index..]);
        index += self.source_provider.as_bytes(&mut bytes[index..]);
        index += self.config_timestamp.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetProviderOutputSourceRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (provider, sz): (Provider, usize) = <Provider>::from_bytes(&bytes[index..])?;
        index += sz;
        let (source_provider, sz): (Provider, usize) = <Provider>::from_bytes(&bytes[index..])?;
        index += sz;
        let (config_timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetProviderOutputSourceRequest {
                req_type: req_type,
                length: length,
                provider: provider,
                source_provider: source_provider,
                config_timestamp: config_timestamp,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.provider.size()
            + self.source_provider.size()
            + self.config_timestamp.size()
    }
}
impl Request for SetProviderOutputSourceRequest {
    const OPCODE: u8 = 35;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ListProviderPropertiesRequest {
    pub req_type: u8,
    pub length: u16,
    pub provider: Provider,
}
impl ListProviderPropertiesRequest {}
impl AsByteSequence for ListProviderPropertiesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.provider.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListProviderPropertiesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (provider, sz): (Provider, usize) = <Provider>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ListProviderPropertiesRequest {
                req_type: req_type,
                length: length,
                provider: provider,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.provider.size()
    }
}
impl Request for ListProviderPropertiesRequest {
    const OPCODE: u8 = 36;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ListProviderPropertiesReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ListProviderPropertiesReply<'rb> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub atoms: Cow<'rb, [Atom]>,
}
impl<'rb> ListProviderPropertiesReply<'rb> {}
impl<'rb> AsByteSequence for ListProviderPropertiesReply<'rb> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.atoms.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 22;
        let block_len: usize = vector_as_bytes(&self.atoms, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListProviderPropertiesReply from byte buffer");
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
        let (atoms, block_len): (Cow<'_, [Atom]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        Some((
            ListProviderPropertiesReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                atoms: atoms,
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
                let block_len: usize = self.atoms.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryProviderPropertyRequest {
    pub req_type: u8,
    pub length: u16,
    pub provider: Provider,
    pub property: Atom,
}
impl QueryProviderPropertyRequest {}
impl AsByteSequence for QueryProviderPropertyRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.provider.as_bytes(&mut bytes[index..]);
        index += self.property.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryProviderPropertyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (provider, sz): (Provider, usize) = <Provider>::from_bytes(&bytes[index..])?;
        index += sz;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryProviderPropertyRequest {
                req_type: req_type,
                length: length,
                provider: provider,
                property: property,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.provider.size() + self.property.size()
    }
}
impl Request for QueryProviderPropertyRequest {
    const OPCODE: u8 = 37;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryProviderPropertyReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryProviderPropertyReply<'sb> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub pending: bool,
    pub range: bool,
    pub immutable: bool,
    pub valid_values: Cow<'sb, [Int32]>,
}
impl<'sb> QueryProviderPropertyReply<'sb> {}
impl<'sb> AsByteSequence for QueryProviderPropertyReply<'sb> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.pending.as_bytes(&mut bytes[index..]);
        index += self.range.as_bytes(&mut bytes[index..]);
        index += self.immutable.as_bytes(&mut bytes[index..]);
        index += 21;
        let block_len: usize = vector_as_bytes(&self.valid_values, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryProviderPropertyReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pending, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (range, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (immutable, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 21;
        let (valid_values, block_len): (Cow<'_, [Int32]>, usize) =
            vector_from_bytes(&bytes[index..], (length as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        Some((
            QueryProviderPropertyReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                pending: pending,
                range: range,
                immutable: immutable,
                valid_values: valid_values,
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
            + self.pending.size()
            + self.range.size()
            + self.immutable.size()
            + 21
            + {
                let block_len: usize = self.valid_values.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Int32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ConfigureProviderPropertyRequest<'tb> {
    pub req_type: u8,
    pub length: u16,
    pub provider: Provider,
    pub property: Atom,
    pub pending: bool,
    pub range: bool,
    pub values: Cow<'tb, [Int32]>,
}
impl<'tb> ConfigureProviderPropertyRequest<'tb> {}
impl<'tb> AsByteSequence for ConfigureProviderPropertyRequest<'tb> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.provider.as_bytes(&mut bytes[index..]);
        index += self.property.as_bytes(&mut bytes[index..]);
        index += self.pending.as_bytes(&mut bytes[index..]);
        index += self.range.as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = vector_as_bytes(&self.values, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ConfigureProviderPropertyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (provider, sz): (Provider, usize) = <Provider>::from_bytes(&bytes[index..])?;
        index += sz;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pending, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (range, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (values, block_len): (Cow<'_, [Int32]>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        Some((
            ConfigureProviderPropertyRequest {
                req_type: req_type,
                length: length,
                provider: provider,
                property: property,
                pending: pending,
                range: range,
                values: values,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.provider.size()
            + self.property.size()
            + self.pending.size()
            + self.range.size()
            + 2
            + {
                let block_len: usize = self.values.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Int32>());
                block_len + pad
            }
    }
}
impl<'tb> Request for ConfigureProviderPropertyRequest<'tb> {
    const OPCODE: u8 = 38;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ChangeProviderPropertyRequest<'ub> {
    pub req_type: u8,
    pub length: u16,
    pub provider: Provider,
    pub property: Atom,
    pub ty: Atom,
    pub format: Card8,
    pub mode: Card8,
    pub num_items: Card32,
    pub data: Cow<'ub, [Void]>,
}
impl<'ub> ChangeProviderPropertyRequest<'ub> {}
impl<'ub> AsByteSequence for ChangeProviderPropertyRequest<'ub> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.provider.as_bytes(&mut bytes[index..]);
        index += self.property.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.num_items.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Void>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangeProviderPropertyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (provider, sz): (Provider, usize) = <Provider>::from_bytes(&bytes[index..])?;
        index += sz;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (num_items, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (data, block_len): (Cow<'_, [Void]>, usize) = vector_from_bytes(
            &bytes[index..],
            ((num_items as usize) * ((format as usize) / (8))) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Void>());
        Some((
            ChangeProviderPropertyRequest {
                req_type: req_type,
                length: length,
                provider: provider,
                property: property,
                ty: ty,
                format: format,
                mode: mode,
                num_items: num_items,
                data: data,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.provider.size()
            + self.property.size()
            + self.ty.size()
            + self.format.size()
            + self.mode.size()
            + 2
            + self.num_items.size()
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Void>());
                block_len + pad
            }
    }
}
impl<'ub> Request for ChangeProviderPropertyRequest<'ub> {
    const OPCODE: u8 = 39;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeleteProviderPropertyRequest {
    pub req_type: u8,
    pub length: u16,
    pub provider: Provider,
    pub property: Atom,
}
impl DeleteProviderPropertyRequest {}
impl AsByteSequence for DeleteProviderPropertyRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.provider.as_bytes(&mut bytes[index..]);
        index += self.property.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeleteProviderPropertyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (provider, sz): (Provider, usize) = <Provider>::from_bytes(&bytes[index..])?;
        index += sz;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DeleteProviderPropertyRequest {
                req_type: req_type,
                length: length,
                provider: provider,
                property: property,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.provider.size() + self.property.size()
    }
}
impl Request for DeleteProviderPropertyRequest {
    const OPCODE: u8 = 40;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetProviderPropertyRequest {
    pub req_type: u8,
    pub length: u16,
    pub provider: Provider,
    pub property: Atom,
    pub ty: Atom,
    pub long_offset: Card32,
    pub long_length: Card32,
    pub delete: bool,
    pub pending: bool,
}
impl GetProviderPropertyRequest {}
impl AsByteSequence for GetProviderPropertyRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.provider.as_bytes(&mut bytes[index..]);
        index += self.property.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.long_offset.as_bytes(&mut bytes[index..]);
        index += self.long_length.as_bytes(&mut bytes[index..]);
        index += self.delete.as_bytes(&mut bytes[index..]);
        index += self.pending.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetProviderPropertyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (provider, sz): (Provider, usize) = <Provider>::from_bytes(&bytes[index..])?;
        index += sz;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (long_offset, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (long_length, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (delete, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pending, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            GetProviderPropertyRequest {
                req_type: req_type,
                length: length,
                provider: provider,
                property: property,
                ty: ty,
                long_offset: long_offset,
                long_length: long_length,
                delete: delete,
                pending: pending,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.provider.size()
            + self.property.size()
            + self.ty.size()
            + self.long_offset.size()
            + self.long_length.size()
            + self.delete.size()
            + self.pending.size()
            + 2
    }
}
impl Request for GetProviderPropertyRequest {
    const OPCODE: u8 = 41;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetProviderPropertyReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetProviderPropertyReply<'vb> {
    pub reply_type: u8,
    pub format: Card8,
    pub sequence: u16,
    pub length: u32,
    pub ty: Atom,
    pub bytes_after: Card32,
    pub num_items: Card32,
    pub data: Cow<'vb, [Void]>,
}
impl<'vb> GetProviderPropertyReply<'vb> {}
impl<'vb> AsByteSequence for GetProviderPropertyReply<'vb> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.bytes_after.as_bytes(&mut bytes[index..]);
        index += self.num_items.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Void>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetProviderPropertyReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bytes_after, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_items, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Cow<'_, [Void]>, usize) = vector_from_bytes(
            &bytes[index..],
            ((num_items as usize) * ((format as usize) / (8))) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Void>());
        Some((
            GetProviderPropertyReply {
                reply_type: reply_type,
                format: format,
                sequence: sequence,
                length: length,
                ty: ty,
                bytes_after: bytes_after,
                num_items: num_items,
                data: data,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.format.size()
            + self.sequence.size()
            + self.length.size()
            + self.ty.size()
            + self.bytes_after.size()
            + self.num_items.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Void>());
                block_len + pad
            }
    }
}
pub type Notify = Card8;
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CrtcChange {
    pub timestamp: Timestamp,
    pub window: Window,
    pub crtc: Crtc,
    pub mode: Mode,
    pub rotation: Rotation,
    pub x: Int16,
    pub y: Int16,
    pub width: Card16,
    pub height: Card16,
}
impl CrtcChange {}
impl AsByteSequence for CrtcChange {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.crtc.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.rotation.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CrtcChange from byte buffer");
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (crtc, sz): (Crtc, usize) = <Crtc>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (Mode, usize) = <Mode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (rotation, sz): (Rotation, usize) = <Rotation>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CrtcChange {
                timestamp: timestamp,
                window: window,
                crtc: crtc,
                mode: mode,
                rotation: rotation,
                x: x,
                y: y,
                width: width,
                height: height,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.timestamp.size()
            + self.window.size()
            + self.crtc.size()
            + self.mode.size()
            + self.rotation.size()
            + 2
            + self.x.size()
            + self.y.size()
            + self.width.size()
            + self.height.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct OutputChange {
    pub timestamp: Timestamp,
    pub config_timestamp: Timestamp,
    pub window: Window,
    pub output: Output,
    pub crtc: Crtc,
    pub mode: Mode,
    pub rotation: Rotation,
    pub connection: Connection,
    pub subpixel_order: SubPixel,
}
impl OutputChange {}
impl AsByteSequence for OutputChange {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += self.config_timestamp.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.output.as_bytes(&mut bytes[index..]);
        index += self.crtc.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.rotation.as_bytes(&mut bytes[index..]);
        index += self.connection.as_bytes(&mut bytes[index..]);
        index += self.subpixel_order.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing OutputChange from byte buffer");
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (config_timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (output, sz): (Output, usize) = <Output>::from_bytes(&bytes[index..])?;
        index += sz;
        let (crtc, sz): (Crtc, usize) = <Crtc>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (Mode, usize) = <Mode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (rotation, sz): (Rotation, usize) = <Rotation>::from_bytes(&bytes[index..])?;
        index += sz;
        let (connection, sz): (Connection, usize) = <Connection>::from_bytes(&bytes[index..])?;
        index += sz;
        let (subpixel_order, sz): (SubPixel, usize) = <SubPixel>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            OutputChange {
                timestamp: timestamp,
                config_timestamp: config_timestamp,
                window: window,
                output: output,
                crtc: crtc,
                mode: mode,
                rotation: rotation,
                connection: connection,
                subpixel_order: subpixel_order,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.timestamp.size()
            + self.config_timestamp.size()
            + self.window.size()
            + self.output.size()
            + self.crtc.size()
            + self.mode.size()
            + self.rotation.size()
            + self.connection.size()
            + self.subpixel_order.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct OutputProperty {
    pub window: Window,
    pub output: Output,
    pub atom: Atom,
    pub timestamp: Timestamp,
    pub status: Property,
}
impl OutputProperty {}
impl AsByteSequence for OutputProperty {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.output.as_bytes(&mut bytes[index..]);
        index += self.atom.as_bytes(&mut bytes[index..]);
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += 11;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing OutputProperty from byte buffer");
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (output, sz): (Output, usize) = <Output>::from_bytes(&bytes[index..])?;
        index += sz;
        let (atom, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (Property, usize) = <Property>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 11;
        Some((
            OutputProperty {
                window: window,
                output: output,
                atom: atom,
                timestamp: timestamp,
                status: status,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.window.size()
            + self.output.size()
            + self.atom.size()
            + self.timestamp.size()
            + self.status.size()
            + 11
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ProviderChange {
    pub timestamp: Timestamp,
    pub window: Window,
    pub provider: Provider,
}
impl ProviderChange {}
impl AsByteSequence for ProviderChange {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.provider.as_bytes(&mut bytes[index..]);
        index += 16;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ProviderChange from byte buffer");
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (provider, sz): (Provider, usize) = <Provider>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 16;
        Some((
            ProviderChange {
                timestamp: timestamp,
                window: window,
                provider: provider,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.timestamp.size() + self.window.size() + self.provider.size() + 16
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ProviderProperty {
    pub window: Window,
    pub provider: Provider,
    pub atom: Atom,
    pub timestamp: Timestamp,
    pub state: Card8,
}
impl ProviderProperty {}
impl AsByteSequence for ProviderProperty {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.provider.as_bytes(&mut bytes[index..]);
        index += self.atom.as_bytes(&mut bytes[index..]);
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += self.state.as_bytes(&mut bytes[index..]);
        index += 11;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ProviderProperty from byte buffer");
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (provider, sz): (Provider, usize) = <Provider>::from_bytes(&bytes[index..])?;
        index += sz;
        let (atom, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (state, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 11;
        Some((
            ProviderProperty {
                window: window,
                provider: provider,
                atom: atom,
                timestamp: timestamp,
                state: state,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.window.size()
            + self.provider.size()
            + self.atom.size()
            + self.timestamp.size()
            + self.state.size()
            + 11
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ResourceChange {
    pub timestamp: Timestamp,
    pub window: Window,
}
impl ResourceChange {}
impl AsByteSequence for ResourceChange {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += 20;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ResourceChange from byte buffer");
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        Some((
            ResourceChange {
                timestamp: timestamp,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.timestamp.size() + self.window.size() + 20
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct MonitorInfo<'wb> {
    pub name: Atom,
    pub primary: bool,
    pub automatic: bool,
    pub x: Int16,
    pub y: Int16,
    pub width: Card16,
    pub height: Card16,
    pub width_in_millimeters: Card32,
    pub height_in_millimeters: Card32,
    pub outputs: Cow<'wb, [Output]>,
}
impl<'wb> MonitorInfo<'wb> {}
impl<'wb> AsByteSequence for MonitorInfo<'wb> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.name.as_bytes(&mut bytes[index..]);
        index += self.primary.as_bytes(&mut bytes[index..]);
        index += self.automatic.as_bytes(&mut bytes[index..]);
        index += (self.outputs.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.width_in_millimeters.as_bytes(&mut bytes[index..]);
        index += self.height_in_millimeters.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.outputs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Output>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing MonitorInfo from byte buffer");
        let (name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (primary, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (automatic, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width_in_millimeters, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height_in_millimeters, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (outputs, block_len): (Cow<'_, [Output]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Output>());
        Some((
            MonitorInfo {
                name: name,
                primary: primary,
                automatic: automatic,
                x: x,
                y: y,
                width: width,
                height: height,
                width_in_millimeters: width_in_millimeters,
                height_in_millimeters: height_in_millimeters,
                outputs: outputs,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.name.size()
            + self.primary.size()
            + self.automatic.size()
            + ::core::mem::size_of::<Card16>()
            + self.x.size()
            + self.y.size()
            + self.width.size()
            + self.height.size()
            + self.width_in_millimeters.size()
            + self.height_in_millimeters.size()
            + {
                let block_len: usize = self.outputs.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Output>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetMonitorsRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub get_active: bool,
}
impl GetMonitorsRequest {}
impl AsByteSequence for GetMonitorsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.get_active.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetMonitorsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (get_active, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetMonitorsRequest {
                req_type: req_type,
                length: length,
                window: window,
                get_active: get_active,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size() + self.get_active.size()
    }
}
impl Request for GetMonitorsRequest {
    const OPCODE: u8 = 42;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetMonitorsReply<'static, 'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetMonitorsReply<'yb, 'xb> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: Timestamp,
    pub n_outputs: Card32,
    pub monitors: Cow<'yb, [MonitorInfo<'xb>]>,
}
impl<'yb, 'xb> GetMonitorsReply<'yb, 'xb> {}
impl<'yb, 'xb> AsByteSequence for GetMonitorsReply<'yb, 'xb> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += (self.monitors.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.n_outputs.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.monitors, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<MonitorInfo<'xb>>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetMonitorsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_outputs, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (monitors, block_len): (Cow<'_, [MonitorInfo<'_>]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<MonitorInfo<'xb>>());
        Some((
            GetMonitorsReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                timestamp: timestamp,
                n_outputs: n_outputs,
                monitors: monitors,
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
            + self.timestamp.size()
            + ::core::mem::size_of::<Card32>()
            + self.n_outputs.size()
            + 12
            + {
                let block_len: usize = self.monitors.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<MonitorInfo<'xb>>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetMonitorRequest<'zb> {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub monitorinfo: MonitorInfo<'zb>,
}
impl<'zb> SetMonitorRequest<'zb> {}
impl<'zb> AsByteSequence for SetMonitorRequest<'zb> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.monitorinfo.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetMonitorRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (monitorinfo, sz): (MonitorInfo<'zb>, usize) =
            <MonitorInfo<'zb>>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetMonitorRequest {
                req_type: req_type,
                length: length,
                window: window,
                monitorinfo: monitorinfo,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size() + self.monitorinfo.size()
    }
}
impl<'zb> Request for SetMonitorRequest<'zb> {
    const OPCODE: u8 = 43;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeleteMonitorRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub name: Atom,
}
impl DeleteMonitorRequest {}
impl AsByteSequence for DeleteMonitorRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.name.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeleteMonitorRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DeleteMonitorRequest {
                req_type: req_type,
                length: length,
                window: window,
                name: name,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size() + self.name.size()
    }
}
impl Request for DeleteMonitorRequest {
    const OPCODE: u8 = 44;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CreateLeaseRequest<'ac, 'bc> {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub lid: Lease,
    pub crtcs: Cow<'ac, [Crtc]>,
    pub outputs: Cow<'bc, [Output]>,
}
impl<'ac, 'bc> CreateLeaseRequest<'ac, 'bc> {}
impl<'ac, 'bc> AsByteSequence for CreateLeaseRequest<'ac, 'bc> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.lid.as_bytes(&mut bytes[index..]);
        index += (self.crtcs.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.outputs.len() as Card16).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.crtcs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Crtc>());
        let block_len: usize = vector_as_bytes(&self.outputs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Output>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateLeaseRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (lid, sz): (Lease, usize) = <Lease>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (crtcs, block_len): (Cow<'_, [Crtc]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Crtc>());
        let (outputs, block_len): (Cow<'_, [Output]>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Output>());
        Some((
            CreateLeaseRequest {
                req_type: req_type,
                length: length,
                window: window,
                lid: lid,
                crtcs: crtcs,
                outputs: outputs,
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
            + self.lid.size()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + {
                let block_len: usize = self.crtcs.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Crtc>());
                block_len + pad
            }
            + {
                let block_len: usize = self.outputs.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Output>());
                block_len + pad
            }
    }
}
impl<'ac, 'bc> Request for CreateLeaseRequest<'ac, 'bc> {
    const OPCODE: u8 = 45;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = true;
    type Reply = CreateLeaseReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CreateLeaseReply {
    pub reply_type: u8,
    pub nfd: Card8,
    pub sequence: u16,
    pub length: u32,
    pub master_fd: Vec<Fd>,
}
impl CreateLeaseReply {}
impl AsByteSequence for CreateLeaseReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.nfd.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 24;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateLeaseReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (nfd, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 24;
        Some((
            CreateLeaseReply {
                reply_type: reply_type,
                nfd: nfd,
                sequence: sequence,
                length: length,
                master_fd: vec![],
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + self.nfd.size() + self.sequence.size() + self.length.size() + 24
    }
    #[inline]
    fn file_descriptors(&mut self) -> Option<&mut Vec<Fd>> {
        Some(&mut self.master_fd)
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct FreeLeaseRequest {
    pub req_type: u8,
    pub length: u16,
    pub lid: Lease,
    pub terminate: Byte,
}
impl FreeLeaseRequest {}
impl AsByteSequence for FreeLeaseRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.lid.as_bytes(&mut bytes[index..]);
        index += self.terminate.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FreeLeaseRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (lid, sz): (Lease, usize) = <Lease>::from_bytes(&bytes[index..])?;
        index += sz;
        let (terminate, sz): (Byte, usize) = <Byte>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            FreeLeaseRequest {
                req_type: req_type,
                length: length,
                lid: lid,
                terminate: terminate,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.lid.size() + self.terminate.size()
    }
}
impl Request for FreeLeaseRequest {
    const OPCODE: u8 = 46;
    const EXTENSION: Option<&'static str> = Some("RANDR");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct LeaseNotify {
    pub timestamp: Timestamp,
    pub window: Window,
    pub lease: Lease,
    pub created: Card8,
}
impl LeaseNotify {}
impl AsByteSequence for LeaseNotify {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.lease.as_bytes(&mut bytes[index..]);
        index += self.created.as_bytes(&mut bytes[index..]);
        index += 15;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing LeaseNotify from byte buffer");
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (lease, sz): (Lease, usize) = <Lease>::from_bytes(&bytes[index..])?;
        index += sz;
        let (created, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 15;
        Some((
            LeaseNotify {
                timestamp: timestamp,
                window: window,
                lease: lease,
                created: created,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.timestamp.size() + self.window.size() + self.lease.size() + self.created.size() + 15
    }
}
pub const NOTIFY_CRTC_CHANGE: Notify = 0;
pub const NOTIFY_OUTPUT_CHANGE: Notify = 1;
pub const NOTIFY_OUTPUT_PROPERTY: Notify = 2;
pub const NOTIFY_PROVIDER_CHANGE: Notify = 3;
pub const NOTIFY_PROVIDER_PROPERTY: Notify = 4;
pub const NOTIFY_RESOURCE_CHANGE: Notify = 5;
pub const NOTIFY_LEASE: Notify = 6;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Transform {
    pub inner: i32,
}
impl Transform {
    #[inline]
    pub fn unit(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_unit(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn scale_up(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_scale_up(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn scale_down(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_scale_down(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn projective(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_projective(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn new(unit: bool, scale_up: bool, scale_down: bool, projective: bool) -> Self {
        let mut inner: i32 = 0;
        if unit {
            inner |= 1 << 0;
        }
        if scale_up {
            inner |= 1 << 1;
        }
        if scale_down {
            inner |= 1 << 2;
        }
        if projective {
            inner |= 1 << 3;
        }
        Transform { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const UNIT: Self = Self { inner: 1 };
    pub const SCALE_UP: Self = Self { inner: 2 };
    pub const SCALE_DOWN: Self = Self { inner: 4 };
    pub const PROJECTIVE: Self = Self { inner: 8 };
    pub const COMPLETE: Self = Self { inner: 15 };
}
impl AsByteSequence for Transform {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        Some((Transform { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for Transform {
    type Output = Transform;
    #[inline]
    fn not(self) -> Transform {
        Transform { inner: !self.inner }
    }
}
impl core::ops::BitAnd for Transform {
    type Output = Transform;
    #[inline]
    fn bitand(self, rhs: Transform) -> Transform {
        Transform {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for Transform {
    type Output = Transform;
    #[inline]
    fn bitor(self, rhs: Transform) -> Transform {
        Transform {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for Transform {
    type Output = Transform;
    #[inline]
    fn bitxor(self, rhs: Transform) -> Transform {
        Transform {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct NotifyEvent {
    pub event_type: u8,
    pub sub_code: Notify,
    pub sequence: u16,
    pub u: NotifyData,
}
impl NotifyEvent {}
impl AsByteSequence for NotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.sub_code.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.u.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing NotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sub_code, sz): (Notify, usize) = <Notify>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (u, sz): (NotifyData, usize) = <NotifyData>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            NotifyEvent {
                event_type: event_type,
                sub_code: sub_code,
                sequence: sequence,
                u: u,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size() + self.sub_code.size() + self.sequence.size() + self.u.size()
    }
}
impl crate::auto::Event for NotifyEvent {
    const OPCODE: u8 = 1;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ScreenChangeNotifyEvent {
    pub event_type: u8,
    pub rotation: Rotation,
    pub sequence: u16,
    pub timestamp: Timestamp,
    pub config_timestamp: Timestamp,
    pub root: Window,
    pub request_window: Window,
    pub size_id: Card16,
    pub subpixel_order: SubPixel,
    pub width: Card16,
    pub height: Card16,
    pub mwidth: Card16,
    pub mheight: Card16,
}
impl ScreenChangeNotifyEvent {}
impl AsByteSequence for ScreenChangeNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.rotation.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += self.config_timestamp.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.request_window.as_bytes(&mut bytes[index..]);
        index += self.size_id.as_bytes(&mut bytes[index..]);
        index += self.subpixel_order.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.mwidth.as_bytes(&mut bytes[index..]);
        index += self.mheight.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ScreenChangeNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (rotation, sz): (Rotation, usize) = <Rotation>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (config_timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (request_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (size_id, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (subpixel_order, sz): (SubPixel, usize) = <SubPixel>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mwidth, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mheight, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ScreenChangeNotifyEvent {
                event_type: event_type,
                rotation: rotation,
                sequence: sequence,
                timestamp: timestamp,
                config_timestamp: config_timestamp,
                root: root,
                request_window: request_window,
                size_id: size_id,
                subpixel_order: subpixel_order,
                width: width,
                height: height,
                mwidth: mwidth,
                mheight: mheight,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.rotation.size()
            + self.sequence.size()
            + self.timestamp.size()
            + self.config_timestamp.size()
            + self.root.size()
            + self.request_window.size()
            + self.size_id.size()
            + self.subpixel_order.size()
            + self.width.size()
            + self.height.size()
            + self.mwidth.size()
            + self.mheight.size()
    }
}
impl crate::auto::Event for ScreenChangeNotifyEvent {
    const OPCODE: u8 = 0;
}
