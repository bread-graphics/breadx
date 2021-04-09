// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

pub type Syncrange = Card32;
pub type Dotclock = Card32;
pub type ClockFlag = Card32;
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ModeInfo {
    pub dotclock: Dotclock,
    pub hdisplay: Card16,
    pub hsyncstart: Card16,
    pub hsyncend: Card16,
    pub htotal: Card16,
    pub hskew: Card32,
    pub vdisplay: Card16,
    pub vsyncstart: Card16,
    pub vsyncend: Card16,
    pub vtotal: Card16,
    pub flags: ModeFlag,
    pub privsize: Card32,
}
impl ModeInfo {}
impl AsByteSequence for ModeInfo {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.dotclock.as_bytes(&mut bytes[index..]);
        index += self.hdisplay.as_bytes(&mut bytes[index..]);
        index += self.hsyncstart.as_bytes(&mut bytes[index..]);
        index += self.hsyncend.as_bytes(&mut bytes[index..]);
        index += self.htotal.as_bytes(&mut bytes[index..]);
        index += self.hskew.as_bytes(&mut bytes[index..]);
        index += self.vdisplay.as_bytes(&mut bytes[index..]);
        index += self.vsyncstart.as_bytes(&mut bytes[index..]);
        index += self.vsyncend.as_bytes(&mut bytes[index..]);
        index += self.vtotal.as_bytes(&mut bytes[index..]);
        index += 4;
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += 12;
        index += self.privsize.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ModeInfo from byte buffer");
        let (dotclock, sz): (Dotclock, usize) = <Dotclock>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hdisplay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hsyncstart, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hsyncend, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (htotal, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hskew, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vdisplay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vsyncstart, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vsyncend, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vtotal, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (flags, sz): (ModeFlag, usize) = <ModeFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (privsize, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ModeInfo {
                dotclock: dotclock,
                hdisplay: hdisplay,
                hsyncstart: hsyncstart,
                hsyncend: hsyncend,
                htotal: htotal,
                hskew: hskew,
                vdisplay: vdisplay,
                vsyncstart: vsyncstart,
                vsyncend: vsyncend,
                vtotal: vtotal,
                flags: flags,
                privsize: privsize,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.dotclock.size()
            + self.hdisplay.size()
            + self.hsyncstart.size()
            + self.hsyncend.size()
            + self.htotal.size()
            + self.hskew.size()
            + self.vdisplay.size()
            + self.vsyncstart.size()
            + self.vsyncend.size()
            + self.vtotal.size()
            + 4
            + self.flags.size()
            + 12
            + self.privsize.size()
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModeFlag {
    pub inner: u32,
}
impl ModeFlag {
    #[inline]
    pub fn positive_h_sync(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_positive_h_sync(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn negative_h_sync(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_negative_h_sync(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn positive_v_sync(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_positive_v_sync(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn negative_v_sync(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_negative_v_sync(&mut self, val: bool) -> &mut Self {
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
    pub fn composite_sync(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_composite_sync(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn positive_c_sync(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_positive_c_sync(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn negative_c_sync(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_negative_c_sync(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn h_skew(&self) -> bool {
        self.inner & (1 << 8) != 0
    }
    #[inline]
    pub fn set_h_skew(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 8;
        } else {
            self.inner &= !(1 << 8);
        }
        self
    }
    #[inline]
    pub fn broadcast(&self) -> bool {
        self.inner & (1 << 9) != 0
    }
    #[inline]
    pub fn set_broadcast(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 9;
        } else {
            self.inner &= !(1 << 9);
        }
        self
    }
    #[inline]
    pub fn pixmux(&self) -> bool {
        self.inner & (1 << 10) != 0
    }
    #[inline]
    pub fn set_pixmux(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 10;
        } else {
            self.inner &= !(1 << 10);
        }
        self
    }
    #[inline]
    pub fn double_clock(&self) -> bool {
        self.inner & (1 << 11) != 0
    }
    #[inline]
    pub fn set_double_clock(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 11;
        } else {
            self.inner &= !(1 << 11);
        }
        self
    }
    #[inline]
    pub fn half_clock(&self) -> bool {
        self.inner & (1 << 12) != 0
    }
    #[inline]
    pub fn set_half_clock(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 12;
        } else {
            self.inner &= !(1 << 12);
        }
        self
    }
    #[inline]
    pub fn new(
        positive_h_sync: bool,
        negative_h_sync: bool,
        positive_v_sync: bool,
        negative_v_sync: bool,
        interlace: bool,
        composite_sync: bool,
        positive_c_sync: bool,
        negative_c_sync: bool,
        h_skew: bool,
        broadcast: bool,
        pixmux: bool,
        double_clock: bool,
        half_clock: bool,
    ) -> Self {
        let mut inner: u32 = 0;
        if positive_h_sync {
            inner |= 1 << 0;
        }
        if negative_h_sync {
            inner |= 1 << 1;
        }
        if positive_v_sync {
            inner |= 1 << 2;
        }
        if negative_v_sync {
            inner |= 1 << 3;
        }
        if interlace {
            inner |= 1 << 4;
        }
        if composite_sync {
            inner |= 1 << 5;
        }
        if positive_c_sync {
            inner |= 1 << 6;
        }
        if negative_c_sync {
            inner |= 1 << 7;
        }
        if h_skew {
            inner |= 1 << 8;
        }
        if broadcast {
            inner |= 1 << 9;
        }
        if pixmux {
            inner |= 1 << 10;
        }
        if double_clock {
            inner |= 1 << 11;
        }
        if half_clock {
            inner |= 1 << 12;
        }
        ModeFlag { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const POSITIVE_H_SYNC: Self = Self { inner: 1 };
    pub const NEGATIVE_H_SYNC: Self = Self { inner: 2 };
    pub const POSITIVE_V_SYNC: Self = Self { inner: 4 };
    pub const NEGATIVE_V_SYNC: Self = Self { inner: 8 };
    pub const INTERLACE: Self = Self { inner: 16 };
    pub const COMPOSITE_SYNC: Self = Self { inner: 32 };
    pub const POSITIVE_C_SYNC: Self = Self { inner: 64 };
    pub const NEGATIVE_C_SYNC: Self = Self { inner: 128 };
    pub const H_SKEW: Self = Self { inner: 256 };
    pub const BROADCAST: Self = Self { inner: 512 };
    pub const PIXMUX: Self = Self { inner: 1024 };
    pub const DOUBLE_CLOCK: Self = Self { inner: 2048 };
    pub const HALF_CLOCK: Self = Self { inner: 4096 };
    pub const COMPLETE: Self = Self { inner: 8191 };
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
    const EXTENSION: Option<&'static str> = Some("XFree86-VidModeExtension");
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
pub struct GetModeLineRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card16,
}
impl GetModeLineRequest {}
impl AsByteSequence for GetModeLineRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetModeLineRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            GetModeLineRequest {
                req_type: req_type,
                length: length,
                screen: screen,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.screen.size() + 2
    }
}
impl Request for GetModeLineRequest {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = Some("XFree86-VidModeExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetModeLineReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetModeLineReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub dotclock: Dotclock,
    pub hdisplay: Card16,
    pub hsyncstart: Card16,
    pub hsyncend: Card16,
    pub htotal: Card16,
    pub hskew: Card16,
    pub vdisplay: Card16,
    pub vsyncstart: Card16,
    pub vsyncend: Card16,
    pub vtotal: Card16,
    pub flags: ModeFlag,
    pub private: Vec<Card8>,
}
impl GetModeLineReply {}
impl AsByteSequence for GetModeLineReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.dotclock.as_bytes(&mut bytes[index..]);
        index += self.hdisplay.as_bytes(&mut bytes[index..]);
        index += self.hsyncstart.as_bytes(&mut bytes[index..]);
        index += self.hsyncend.as_bytes(&mut bytes[index..]);
        index += self.htotal.as_bytes(&mut bytes[index..]);
        index += self.hskew.as_bytes(&mut bytes[index..]);
        index += self.vdisplay.as_bytes(&mut bytes[index..]);
        index += self.vsyncstart.as_bytes(&mut bytes[index..]);
        index += self.vsyncend.as_bytes(&mut bytes[index..]);
        index += self.vtotal.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += 12;
        index += (self.private.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.private, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetModeLineReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dotclock, sz): (Dotclock, usize) = <Dotclock>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hdisplay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hsyncstart, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hsyncend, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (htotal, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hskew, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vdisplay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vsyncstart, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vsyncend, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vtotal, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (flags, sz): (ModeFlag, usize) = <ModeFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (private, block_len): (Vec<Card8>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        Some((
            GetModeLineReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                dotclock: dotclock,
                hdisplay: hdisplay,
                hsyncstart: hsyncstart,
                hsyncend: hsyncend,
                htotal: htotal,
                hskew: hskew,
                vdisplay: vdisplay,
                vsyncstart: vsyncstart,
                vsyncend: vsyncend,
                vtotal: vtotal,
                flags: flags,
                private: private,
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
            + self.dotclock.size()
            + self.hdisplay.size()
            + self.hsyncstart.size()
            + self.hsyncend.size()
            + self.htotal.size()
            + self.hskew.size()
            + self.vdisplay.size()
            + self.vsyncstart.size()
            + self.vsyncend.size()
            + self.vtotal.size()
            + 2
            + self.flags.size()
            + 12
            + ::core::mem::size_of::<Card32>()
            + {
                let block_len: usize = self.private.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ModModeLineRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card32,
    pub hdisplay: Card16,
    pub hsyncstart: Card16,
    pub hsyncend: Card16,
    pub htotal: Card16,
    pub hskew: Card16,
    pub vdisplay: Card16,
    pub vsyncstart: Card16,
    pub vsyncend: Card16,
    pub vtotal: Card16,
    pub flags: ModeFlag,
    pub private: Vec<Card8>,
}
impl ModModeLineRequest {}
impl AsByteSequence for ModModeLineRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.hdisplay.as_bytes(&mut bytes[index..]);
        index += self.hsyncstart.as_bytes(&mut bytes[index..]);
        index += self.hsyncend.as_bytes(&mut bytes[index..]);
        index += self.htotal.as_bytes(&mut bytes[index..]);
        index += self.hskew.as_bytes(&mut bytes[index..]);
        index += self.vdisplay.as_bytes(&mut bytes[index..]);
        index += self.vsyncstart.as_bytes(&mut bytes[index..]);
        index += self.vsyncend.as_bytes(&mut bytes[index..]);
        index += self.vtotal.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += 12;
        index += (self.private.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.private, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ModModeLineRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hdisplay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hsyncstart, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hsyncend, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (htotal, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hskew, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vdisplay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vsyncstart, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vsyncend, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vtotal, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (flags, sz): (ModeFlag, usize) = <ModeFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (private, block_len): (Vec<Card8>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        Some((
            ModModeLineRequest {
                req_type: req_type,
                length: length,
                screen: screen,
                hdisplay: hdisplay,
                hsyncstart: hsyncstart,
                hsyncend: hsyncend,
                htotal: htotal,
                hskew: hskew,
                vdisplay: vdisplay,
                vsyncstart: vsyncstart,
                vsyncend: vsyncend,
                vtotal: vtotal,
                flags: flags,
                private: private,
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
            + self.hdisplay.size()
            + self.hsyncstart.size()
            + self.hsyncend.size()
            + self.htotal.size()
            + self.hskew.size()
            + self.vdisplay.size()
            + self.vsyncstart.size()
            + self.vsyncend.size()
            + self.vtotal.size()
            + 2
            + self.flags.size()
            + 12
            + ::core::mem::size_of::<Card32>()
            + {
                let block_len: usize = self.private.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
    }
}
impl Request for ModModeLineRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = Some("XFree86-VidModeExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SwitchModeRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card16,
    pub zoom: Card16,
}
impl SwitchModeRequest {}
impl AsByteSequence for SwitchModeRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.zoom.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SwitchModeRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (zoom, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SwitchModeRequest {
                req_type: req_type,
                length: length,
                screen: screen,
                zoom: zoom,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.screen.size() + self.zoom.size()
    }
}
impl Request for SwitchModeRequest {
    const OPCODE: u8 = 3;
    const EXTENSION: Option<&'static str> = Some("XFree86-VidModeExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetMonitorRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card16,
}
impl GetMonitorRequest {}
impl AsByteSequence for GetMonitorRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetMonitorRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            GetMonitorRequest {
                req_type: req_type,
                length: length,
                screen: screen,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.screen.size() + 2
    }
}
impl Request for GetMonitorRequest {
    const OPCODE: u8 = 4;
    const EXTENSION: Option<&'static str> = Some("XFree86-VidModeExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetMonitorReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetMonitorReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub vendor_length: Card8,
    pub hsync: Vec<Syncrange>,
    pub vsync: Vec<Syncrange>,
    pub vendor: String,
    pub alignment_pad: Vec<Void>,
    pub model: String,
}
impl GetMonitorReply {}
impl AsByteSequence for GetMonitorReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.vendor_length.as_bytes(&mut bytes[index..]);
        index += (self.model.len() as Card8).as_bytes(&mut bytes[index..]);
        index += (self.hsync.len() as Card8).as_bytes(&mut bytes[index..]);
        index += (self.vsync.len() as Card8).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.hsync, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Syncrange>());
        let block_len: usize = vector_as_bytes(&self.vsync, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Syncrange>());
        let block_len: usize = string_as_bytes(&self.vendor, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        let block_len: usize = vector_as_bytes(&self.alignment_pad, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Void>());
        let block_len: usize = string_as_bytes(&self.model, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetMonitorReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vendor_length, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len2, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        let (hsync, block_len): (Vec<Syncrange>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Syncrange>());
        let (vsync, block_len): (Vec<Syncrange>, usize) =
            vector_from_bytes(&bytes[index..], len2 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Syncrange>());
        let (vendor, block_len): (String, usize) =
            string_from_bytes(&bytes[index..], (vendor_length as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        let (alignment_pad, block_len): (Vec<Void>, usize) = vector_from_bytes(
            &bytes[index..],
            ((((vendor_length as usize) + (3)) & (!(3))) - (vendor_length as usize)) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Void>());
        let (model, block_len): (String, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            GetMonitorReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                vendor_length: vendor_length,
                hsync: hsync,
                vsync: vsync,
                vendor: vendor,
                alignment_pad: alignment_pad,
                model: model,
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
            + self.vendor_length.size()
            + ::core::mem::size_of::<Card8>()
            + ::core::mem::size_of::<Card8>()
            + ::core::mem::size_of::<Card8>()
            + 20
            + {
                let block_len: usize = self.hsync.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Syncrange>());
                block_len + pad
            }
            + {
                let block_len: usize = self.vsync.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Syncrange>());
                block_len + pad
            }
            + {
                let block_len: usize = self.vendor.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
            + {
                let block_len: usize = self.alignment_pad.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Void>());
                block_len + pad
            }
            + {
                let block_len: usize = self.model.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct LockModeSwitchRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card16,
    pub lock: Card16,
}
impl LockModeSwitchRequest {}
impl AsByteSequence for LockModeSwitchRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.lock.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing LockModeSwitchRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (lock, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            LockModeSwitchRequest {
                req_type: req_type,
                length: length,
                screen: screen,
                lock: lock,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.screen.size() + self.lock.size()
    }
}
impl Request for LockModeSwitchRequest {
    const OPCODE: u8 = 5;
    const EXTENSION: Option<&'static str> = Some("XFree86-VidModeExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetAllModeLinesRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card16,
}
impl GetAllModeLinesRequest {}
impl AsByteSequence for GetAllModeLinesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetAllModeLinesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            GetAllModeLinesRequest {
                req_type: req_type,
                length: length,
                screen: screen,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.screen.size() + 2
    }
}
impl Request for GetAllModeLinesRequest {
    const OPCODE: u8 = 6;
    const EXTENSION: Option<&'static str> = Some("XFree86-VidModeExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetAllModeLinesReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetAllModeLinesReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub modeinfo: Vec<ModeInfo>,
}
impl GetAllModeLinesReply {}
impl AsByteSequence for GetAllModeLinesReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.modeinfo.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.modeinfo, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ModeInfo>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetAllModeLinesReply from byte buffer");
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
        let (modeinfo, block_len): (Vec<ModeInfo>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ModeInfo>());
        Some((
            GetAllModeLinesReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                modeinfo: modeinfo,
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
                let block_len: usize = self.modeinfo.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<ModeInfo>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct AddModeLineRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card32,
    pub dotclock: Dotclock,
    pub hdisplay: Card16,
    pub hsyncstart: Card16,
    pub hsyncend: Card16,
    pub htotal: Card16,
    pub hskew: Card16,
    pub vdisplay: Card16,
    pub vsyncstart: Card16,
    pub vsyncend: Card16,
    pub vtotal: Card16,
    pub flags: ModeFlag,
    pub after_dotclock: Dotclock,
    pub after_hdisplay: Card16,
    pub after_hsyncstart: Card16,
    pub after_hsyncend: Card16,
    pub after_htotal: Card16,
    pub after_hskew: Card16,
    pub after_vdisplay: Card16,
    pub after_vsyncstart: Card16,
    pub after_vsyncend: Card16,
    pub after_vtotal: Card16,
    pub after_flags: ModeFlag,
    pub private: Vec<Card8>,
}
impl AddModeLineRequest {}
impl AsByteSequence for AddModeLineRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.dotclock.as_bytes(&mut bytes[index..]);
        index += self.hdisplay.as_bytes(&mut bytes[index..]);
        index += self.hsyncstart.as_bytes(&mut bytes[index..]);
        index += self.hsyncend.as_bytes(&mut bytes[index..]);
        index += self.htotal.as_bytes(&mut bytes[index..]);
        index += self.hskew.as_bytes(&mut bytes[index..]);
        index += self.vdisplay.as_bytes(&mut bytes[index..]);
        index += self.vsyncstart.as_bytes(&mut bytes[index..]);
        index += self.vsyncend.as_bytes(&mut bytes[index..]);
        index += self.vtotal.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += 12;
        index += (self.private.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.after_dotclock.as_bytes(&mut bytes[index..]);
        index += self.after_hdisplay.as_bytes(&mut bytes[index..]);
        index += self.after_hsyncstart.as_bytes(&mut bytes[index..]);
        index += self.after_hsyncend.as_bytes(&mut bytes[index..]);
        index += self.after_htotal.as_bytes(&mut bytes[index..]);
        index += self.after_hskew.as_bytes(&mut bytes[index..]);
        index += self.after_vdisplay.as_bytes(&mut bytes[index..]);
        index += self.after_vsyncstart.as_bytes(&mut bytes[index..]);
        index += self.after_vsyncend.as_bytes(&mut bytes[index..]);
        index += self.after_vtotal.as_bytes(&mut bytes[index..]);
        index += self.after_flags.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.private, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AddModeLineRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dotclock, sz): (Dotclock, usize) = <Dotclock>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hdisplay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hsyncstart, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hsyncend, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (htotal, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hskew, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vdisplay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vsyncstart, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vsyncend, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vtotal, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (flags, sz): (ModeFlag, usize) = <ModeFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (after_dotclock, sz): (Dotclock, usize) = <Dotclock>::from_bytes(&bytes[index..])?;
        index += sz;
        let (after_hdisplay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (after_hsyncstart, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (after_hsyncend, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (after_htotal, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (after_hskew, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (after_vdisplay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (after_vsyncstart, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (after_vsyncend, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (after_vtotal, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (after_flags, sz): (ModeFlag, usize) = <ModeFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (private, block_len): (Vec<Card8>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        Some((
            AddModeLineRequest {
                req_type: req_type,
                length: length,
                screen: screen,
                dotclock: dotclock,
                hdisplay: hdisplay,
                hsyncstart: hsyncstart,
                hsyncend: hsyncend,
                htotal: htotal,
                hskew: hskew,
                vdisplay: vdisplay,
                vsyncstart: vsyncstart,
                vsyncend: vsyncend,
                vtotal: vtotal,
                flags: flags,
                after_dotclock: after_dotclock,
                after_hdisplay: after_hdisplay,
                after_hsyncstart: after_hsyncstart,
                after_hsyncend: after_hsyncend,
                after_htotal: after_htotal,
                after_hskew: after_hskew,
                after_vdisplay: after_vdisplay,
                after_vsyncstart: after_vsyncstart,
                after_vsyncend: after_vsyncend,
                after_vtotal: after_vtotal,
                after_flags: after_flags,
                private: private,
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
            + self.dotclock.size()
            + self.hdisplay.size()
            + self.hsyncstart.size()
            + self.hsyncend.size()
            + self.htotal.size()
            + self.hskew.size()
            + self.vdisplay.size()
            + self.vsyncstart.size()
            + self.vsyncend.size()
            + self.vtotal.size()
            + 2
            + self.flags.size()
            + 12
            + ::core::mem::size_of::<Card32>()
            + self.after_dotclock.size()
            + self.after_hdisplay.size()
            + self.after_hsyncstart.size()
            + self.after_hsyncend.size()
            + self.after_htotal.size()
            + self.after_hskew.size()
            + self.after_vdisplay.size()
            + self.after_vsyncstart.size()
            + self.after_vsyncend.size()
            + self.after_vtotal.size()
            + self.after_flags.size()
            + {
                let block_len: usize = self.private.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
    }
}
impl Request for AddModeLineRequest {
    const OPCODE: u8 = 7;
    const EXTENSION: Option<&'static str> = Some("XFree86-VidModeExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeleteModeLineRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card32,
    pub dotclock: Dotclock,
    pub hdisplay: Card16,
    pub hsyncstart: Card16,
    pub hsyncend: Card16,
    pub htotal: Card16,
    pub hskew: Card16,
    pub vdisplay: Card16,
    pub vsyncstart: Card16,
    pub vsyncend: Card16,
    pub vtotal: Card16,
    pub flags: ModeFlag,
    pub private: Vec<Card8>,
}
impl DeleteModeLineRequest {}
impl AsByteSequence for DeleteModeLineRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.dotclock.as_bytes(&mut bytes[index..]);
        index += self.hdisplay.as_bytes(&mut bytes[index..]);
        index += self.hsyncstart.as_bytes(&mut bytes[index..]);
        index += self.hsyncend.as_bytes(&mut bytes[index..]);
        index += self.htotal.as_bytes(&mut bytes[index..]);
        index += self.hskew.as_bytes(&mut bytes[index..]);
        index += self.vdisplay.as_bytes(&mut bytes[index..]);
        index += self.vsyncstart.as_bytes(&mut bytes[index..]);
        index += self.vsyncend.as_bytes(&mut bytes[index..]);
        index += self.vtotal.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += 12;
        index += (self.private.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.private, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeleteModeLineRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dotclock, sz): (Dotclock, usize) = <Dotclock>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hdisplay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hsyncstart, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hsyncend, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (htotal, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hskew, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vdisplay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vsyncstart, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vsyncend, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vtotal, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (flags, sz): (ModeFlag, usize) = <ModeFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (private, block_len): (Vec<Card8>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        Some((
            DeleteModeLineRequest {
                req_type: req_type,
                length: length,
                screen: screen,
                dotclock: dotclock,
                hdisplay: hdisplay,
                hsyncstart: hsyncstart,
                hsyncend: hsyncend,
                htotal: htotal,
                hskew: hskew,
                vdisplay: vdisplay,
                vsyncstart: vsyncstart,
                vsyncend: vsyncend,
                vtotal: vtotal,
                flags: flags,
                private: private,
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
            + self.dotclock.size()
            + self.hdisplay.size()
            + self.hsyncstart.size()
            + self.hsyncend.size()
            + self.htotal.size()
            + self.hskew.size()
            + self.vdisplay.size()
            + self.vsyncstart.size()
            + self.vsyncend.size()
            + self.vtotal.size()
            + 2
            + self.flags.size()
            + 12
            + ::core::mem::size_of::<Card32>()
            + {
                let block_len: usize = self.private.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
    }
}
impl Request for DeleteModeLineRequest {
    const OPCODE: u8 = 8;
    const EXTENSION: Option<&'static str> = Some("XFree86-VidModeExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ValidateModeLineRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card32,
    pub dotclock: Dotclock,
    pub hdisplay: Card16,
    pub hsyncstart: Card16,
    pub hsyncend: Card16,
    pub htotal: Card16,
    pub hskew: Card16,
    pub vdisplay: Card16,
    pub vsyncstart: Card16,
    pub vsyncend: Card16,
    pub vtotal: Card16,
    pub flags: ModeFlag,
    pub private: Vec<Card8>,
}
impl ValidateModeLineRequest {}
impl AsByteSequence for ValidateModeLineRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.dotclock.as_bytes(&mut bytes[index..]);
        index += self.hdisplay.as_bytes(&mut bytes[index..]);
        index += self.hsyncstart.as_bytes(&mut bytes[index..]);
        index += self.hsyncend.as_bytes(&mut bytes[index..]);
        index += self.htotal.as_bytes(&mut bytes[index..]);
        index += self.hskew.as_bytes(&mut bytes[index..]);
        index += self.vdisplay.as_bytes(&mut bytes[index..]);
        index += self.vsyncstart.as_bytes(&mut bytes[index..]);
        index += self.vsyncend.as_bytes(&mut bytes[index..]);
        index += self.vtotal.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += 12;
        index += (self.private.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.private, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ValidateModeLineRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dotclock, sz): (Dotclock, usize) = <Dotclock>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hdisplay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hsyncstart, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hsyncend, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (htotal, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hskew, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vdisplay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vsyncstart, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vsyncend, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vtotal, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (flags, sz): (ModeFlag, usize) = <ModeFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (private, block_len): (Vec<Card8>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        Some((
            ValidateModeLineRequest {
                req_type: req_type,
                length: length,
                screen: screen,
                dotclock: dotclock,
                hdisplay: hdisplay,
                hsyncstart: hsyncstart,
                hsyncend: hsyncend,
                htotal: htotal,
                hskew: hskew,
                vdisplay: vdisplay,
                vsyncstart: vsyncstart,
                vsyncend: vsyncend,
                vtotal: vtotal,
                flags: flags,
                private: private,
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
            + self.dotclock.size()
            + self.hdisplay.size()
            + self.hsyncstart.size()
            + self.hsyncend.size()
            + self.htotal.size()
            + self.hskew.size()
            + self.vdisplay.size()
            + self.vsyncstart.size()
            + self.vsyncend.size()
            + self.vtotal.size()
            + 2
            + self.flags.size()
            + 12
            + ::core::mem::size_of::<Card32>()
            + {
                let block_len: usize = self.private.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
    }
}
impl Request for ValidateModeLineRequest {
    const OPCODE: u8 = 9;
    const EXTENSION: Option<&'static str> = Some("XFree86-VidModeExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ValidateModeLineReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ValidateModeLineReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: Card32,
}
impl ValidateModeLineReply {}
impl AsByteSequence for ValidateModeLineReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += 20;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ValidateModeLineReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        Some((
            ValidateModeLineReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                status: status,
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
            + self.status.size()
            + 20
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SwitchToModeRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card32,
    pub dotclock: Dotclock,
    pub hdisplay: Card16,
    pub hsyncstart: Card16,
    pub hsyncend: Card16,
    pub htotal: Card16,
    pub hskew: Card16,
    pub vdisplay: Card16,
    pub vsyncstart: Card16,
    pub vsyncend: Card16,
    pub vtotal: Card16,
    pub flags: ModeFlag,
    pub private: Vec<Card8>,
}
impl SwitchToModeRequest {}
impl AsByteSequence for SwitchToModeRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.dotclock.as_bytes(&mut bytes[index..]);
        index += self.hdisplay.as_bytes(&mut bytes[index..]);
        index += self.hsyncstart.as_bytes(&mut bytes[index..]);
        index += self.hsyncend.as_bytes(&mut bytes[index..]);
        index += self.htotal.as_bytes(&mut bytes[index..]);
        index += self.hskew.as_bytes(&mut bytes[index..]);
        index += self.vdisplay.as_bytes(&mut bytes[index..]);
        index += self.vsyncstart.as_bytes(&mut bytes[index..]);
        index += self.vsyncend.as_bytes(&mut bytes[index..]);
        index += self.vtotal.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += 12;
        index += (self.private.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.private, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SwitchToModeRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dotclock, sz): (Dotclock, usize) = <Dotclock>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hdisplay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hsyncstart, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hsyncend, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (htotal, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (hskew, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vdisplay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vsyncstart, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vsyncend, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vtotal, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (flags, sz): (ModeFlag, usize) = <ModeFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (private, block_len): (Vec<Card8>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        Some((
            SwitchToModeRequest {
                req_type: req_type,
                length: length,
                screen: screen,
                dotclock: dotclock,
                hdisplay: hdisplay,
                hsyncstart: hsyncstart,
                hsyncend: hsyncend,
                htotal: htotal,
                hskew: hskew,
                vdisplay: vdisplay,
                vsyncstart: vsyncstart,
                vsyncend: vsyncend,
                vtotal: vtotal,
                flags: flags,
                private: private,
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
            + self.dotclock.size()
            + self.hdisplay.size()
            + self.hsyncstart.size()
            + self.hsyncend.size()
            + self.htotal.size()
            + self.hskew.size()
            + self.vdisplay.size()
            + self.vsyncstart.size()
            + self.vsyncend.size()
            + self.vtotal.size()
            + 2
            + self.flags.size()
            + 12
            + ::core::mem::size_of::<Card32>()
            + {
                let block_len: usize = self.private.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
    }
}
impl Request for SwitchToModeRequest {
    const OPCODE: u8 = 10;
    const EXTENSION: Option<&'static str> = Some("XFree86-VidModeExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetViewPortRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card16,
}
impl GetViewPortRequest {}
impl AsByteSequence for GetViewPortRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetViewPortRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            GetViewPortRequest {
                req_type: req_type,
                length: length,
                screen: screen,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.screen.size() + 2
    }
}
impl Request for GetViewPortRequest {
    const OPCODE: u8 = 11;
    const EXTENSION: Option<&'static str> = Some("XFree86-VidModeExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetViewPortReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetViewPortReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub x: Card32,
    pub y: Card32,
}
impl GetViewPortReply {}
impl AsByteSequence for GetViewPortReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index += 16;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetViewPortReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 16;
        Some((
            GetViewPortReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                x: x,
                y: y,
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
            + self.x.size()
            + self.y.size()
            + 16
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetViewPortRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card16,
    pub x: Card32,
    pub y: Card32,
}
impl SetViewPortRequest {}
impl AsByteSequence for SetViewPortRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetViewPortRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (x, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetViewPortRequest {
                req_type: req_type,
                length: length,
                screen: screen,
                x: x,
                y: y,
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
            + 2
            + self.x.size()
            + self.y.size()
    }
}
impl Request for SetViewPortRequest {
    const OPCODE: u8 = 12;
    const EXTENSION: Option<&'static str> = Some("XFree86-VidModeExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetDotClocksRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card16,
}
impl GetDotClocksRequest {}
impl AsByteSequence for GetDotClocksRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDotClocksRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            GetDotClocksRequest {
                req_type: req_type,
                length: length,
                screen: screen,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.screen.size() + 2
    }
}
impl Request for GetDotClocksRequest {
    const OPCODE: u8 = 13;
    const EXTENSION: Option<&'static str> = Some("XFree86-VidModeExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetDotClocksReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetDotClocksReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub flags: ClockFlag,
    pub clocks: Card32,
    pub maxclocks: Card32,
    pub clock: Vec<Card32>,
}
impl GetDotClocksReply {}
impl AsByteSequence for GetDotClocksReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.clocks.as_bytes(&mut bytes[index..]);
        index += self.maxclocks.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.clock, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDotClocksReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (ClockFlag, usize) = <ClockFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (clocks, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (maxclocks, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (clock, block_len): (Vec<Card32>, usize) = vector_from_bytes(
            &bytes[index..],
            (((1) - ((flags as usize) & (1))) * (clocks as usize)) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            GetDotClocksReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                flags: flags,
                clocks: clocks,
                maxclocks: maxclocks,
                clock: clock,
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
            + self.flags.size()
            + self.clocks.size()
            + self.maxclocks.size()
            + 12
            + {
                let block_len: usize = self.clock.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
pub const CLOCK_FLAG_PROGRAMABLE: ClockFlag = 1;
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetClientVersionRequest {
    pub req_type: u8,
    pub length: u16,
    pub major: Card16,
    pub minor: Card16,
}
impl SetClientVersionRequest {}
impl AsByteSequence for SetClientVersionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.major.as_bytes(&mut bytes[index..]);
        index += self.minor.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetClientVersionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetClientVersionRequest {
                req_type: req_type,
                length: length,
                major: major,
                minor: minor,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.major.size() + self.minor.size()
    }
}
impl Request for SetClientVersionRequest {
    const OPCODE: u8 = 14;
    const EXTENSION: Option<&'static str> = Some("XFree86-VidModeExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetGammaRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card16,
    pub red: Card32,
    pub green: Card32,
    pub blue: Card32,
}
impl SetGammaRequest {}
impl AsByteSequence for SetGammaRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.red.as_bytes(&mut bytes[index..]);
        index += self.green.as_bytes(&mut bytes[index..]);
        index += self.blue.as_bytes(&mut bytes[index..]);
        index += 12;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetGammaRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (red, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (green, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (blue, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        Some((
            SetGammaRequest {
                req_type: req_type,
                length: length,
                screen: screen,
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
            + self.screen.size()
            + 2
            + self.red.size()
            + self.green.size()
            + self.blue.size()
            + 12
    }
}
impl Request for SetGammaRequest {
    const OPCODE: u8 = 15;
    const EXTENSION: Option<&'static str> = Some("XFree86-VidModeExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetGammaRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card16,
}
impl GetGammaRequest {}
impl AsByteSequence for GetGammaRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += 26;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetGammaRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 26;
        Some((
            GetGammaRequest {
                req_type: req_type,
                length: length,
                screen: screen,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.screen.size() + 26
    }
}
impl Request for GetGammaRequest {
    const OPCODE: u8 = 16;
    const EXTENSION: Option<&'static str> = Some("XFree86-VidModeExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetGammaReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetGammaReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub red: Card32,
    pub green: Card32,
    pub blue: Card32,
}
impl GetGammaReply {}
impl AsByteSequence for GetGammaReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.red.as_bytes(&mut bytes[index..]);
        index += self.green.as_bytes(&mut bytes[index..]);
        index += self.blue.as_bytes(&mut bytes[index..]);
        index += 12;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetGammaReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (red, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (green, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (blue, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        Some((
            GetGammaReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
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
            + self.red.size()
            + self.green.size()
            + self.blue.size()
            + 12
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetGammaRampRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card16,
    pub size: Card16,
}
impl GetGammaRampRequest {}
impl AsByteSequence for GetGammaRampRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.size.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetGammaRampRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (size, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetGammaRampRequest {
                req_type: req_type,
                length: length,
                screen: screen,
                size: size,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.screen.size() + self.size.size()
    }
}
impl Request for GetGammaRampRequest {
    const OPCODE: u8 = 17;
    const EXTENSION: Option<&'static str> = Some("XFree86-VidModeExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetGammaRampReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetGammaRampReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub size: Card16,
    pub red: Vec<Card16>,
    pub green: Vec<Card16>,
    pub blue: Vec<Card16>,
}
impl GetGammaRampReply {}
impl AsByteSequence for GetGammaRampReply {
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
        log::trace!("Deserializing GetGammaRampReply from byte buffer");
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
        let (red, block_len): (Vec<Card16>, usize) =
            vector_from_bytes(&bytes[index..], (((size as usize) + (1)) & (!(1))) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        let (green, block_len): (Vec<Card16>, usize) =
            vector_from_bytes(&bytes[index..], (((size as usize) + (1)) & (!(1))) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        let (blue, block_len): (Vec<Card16>, usize) =
            vector_from_bytes(&bytes[index..], (((size as usize) + (1)) & (!(1))) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        Some((
            GetGammaRampReply {
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
pub struct SetGammaRampRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card16,
    pub size: Card16,
    pub red: Vec<Card16>,
    pub green: Vec<Card16>,
    pub blue: Vec<Card16>,
}
impl SetGammaRampRequest {}
impl AsByteSequence for SetGammaRampRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.size.as_bytes(&mut bytes[index..]);
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
        log::trace!("Deserializing SetGammaRampRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (size, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (red, block_len): (Vec<Card16>, usize) =
            vector_from_bytes(&bytes[index..], (((size as usize) + (1)) & (!(1))) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        let (green, block_len): (Vec<Card16>, usize) =
            vector_from_bytes(&bytes[index..], (((size as usize) + (1)) & (!(1))) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        let (blue, block_len): (Vec<Card16>, usize) =
            vector_from_bytes(&bytes[index..], (((size as usize) + (1)) & (!(1))) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        Some((
            SetGammaRampRequest {
                req_type: req_type,
                length: length,
                screen: screen,
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
            + self.screen.size()
            + self.size.size()
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
impl Request for SetGammaRampRequest {
    const OPCODE: u8 = 18;
    const EXTENSION: Option<&'static str> = Some("XFree86-VidModeExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetGammaRampSizeRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card16,
}
impl GetGammaRampSizeRequest {}
impl AsByteSequence for GetGammaRampSizeRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetGammaRampSizeRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            GetGammaRampSizeRequest {
                req_type: req_type,
                length: length,
                screen: screen,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.screen.size() + 2
    }
}
impl Request for GetGammaRampSizeRequest {
    const OPCODE: u8 = 19;
    const EXTENSION: Option<&'static str> = Some("XFree86-VidModeExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetGammaRampSizeReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetGammaRampSizeReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub size: Card16,
}
impl GetGammaRampSizeReply {}
impl AsByteSequence for GetGammaRampSizeReply {
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
        log::trace!("Deserializing GetGammaRampSizeReply from byte buffer");
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
            GetGammaRampSizeReply {
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
pub struct GetPermissionsRequest {
    pub req_type: u8,
    pub length: u16,
    pub screen: Card16,
}
impl GetPermissionsRequest {}
impl AsByteSequence for GetPermissionsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPermissionsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            GetPermissionsRequest {
                req_type: req_type,
                length: length,
                screen: screen,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.screen.size() + 2
    }
}
impl Request for GetPermissionsRequest {
    const OPCODE: u8 = 20;
    const EXTENSION: Option<&'static str> = Some("XFree86-VidModeExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetPermissionsReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetPermissionsReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub permissions: Permission,
}
impl GetPermissionsReply {}
impl AsByteSequence for GetPermissionsReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.permissions.as_bytes(&mut bytes[index..]);
        index += 20;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPermissionsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (permissions, sz): (Permission, usize) = <Permission>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        Some((
            GetPermissionsReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                permissions: permissions,
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
            + self.permissions.size()
            + 20
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Permission {
    pub inner: u32,
}
impl Permission {
    #[inline]
    pub fn read(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_read(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn write(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_write(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn new(read: bool, write: bool) -> Self {
        let mut inner: u32 = 0;
        if read {
            inner |= 1 << 0;
        }
        if write {
            inner |= 1 << 1;
        }
        Permission { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const READ: Self = Self { inner: 1 };
    pub const WRITE: Self = Self { inner: 2 };
    pub const COMPLETE: Self = Self { inner: 3 };
}
impl AsByteSequence for Permission {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((Permission { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for Permission {
    type Output = Permission;
    #[inline]
    fn not(self) -> Permission {
        Permission { inner: !self.inner }
    }
}
impl core::ops::BitAnd for Permission {
    type Output = Permission;
    #[inline]
    fn bitand(self, rhs: Permission) -> Permission {
        Permission {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for Permission {
    type Output = Permission;
    #[inline]
    fn bitor(self, rhs: Permission) -> Permission {
        Permission {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for Permission {
    type Output = Permission;
    #[inline]
    fn bitxor(self, rhs: Permission) -> Permission {
        Permission {
            inner: self.inner ^ rhs.inner,
        }
    }
}
