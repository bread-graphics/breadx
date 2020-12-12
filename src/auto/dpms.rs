// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct GetVersionRequest {
    pub req_type: u8,
    pub client_major_version: Card16,
    pub length: u16,
    pub client_minor_version: Card16,
}
impl GetVersionRequest {}
impl AsByteSequence for GetVersionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.client_major_version.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.client_minor_version.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetVersionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (client_major_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (client_minor_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetVersionRequest {
                req_type: req_type,
                client_major_version: client_major_version,
                length: length,
                client_minor_version: client_minor_version,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.client_major_version.size()
            + self.length.size()
            + self.client_minor_version.size()
    }
}
impl Request for GetVersionRequest {
    const OPCODE: u8 = 0;
    const EXTENSION: Option<&'static str> = Some("DPMS");
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
pub struct CapableRequest {
    pub req_type: u8,
    pub length: u16,
}
impl CapableRequest {}
impl AsByteSequence for CapableRequest {
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
        log::trace!("Deserializing CapableRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CapableRequest {
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
impl Request for CapableRequest {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = Some("DPMS");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = CapableReply;
}
#[derive(Clone, Debug, Default)]
pub struct CapableReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub capable: bool,
}
impl CapableReply {}
impl AsByteSequence for CapableReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.capable.as_bytes(&mut bytes[index..]);
        index += 23;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CapableReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (capable, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 23;
        Some((
            CapableReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                capable: capable,
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
            + self.capable.size()
            + 23
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetTimeoutsRequest {
    pub req_type: u8,
    pub length: u16,
}
impl GetTimeoutsRequest {}
impl AsByteSequence for GetTimeoutsRequest {
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
        log::trace!("Deserializing GetTimeoutsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetTimeoutsRequest {
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
impl Request for GetTimeoutsRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = Some("DPMS");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetTimeoutsReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetTimeoutsReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub standby_timeout: Card16,
    pub suspend_timeout: Card16,
    pub off_timeout: Card16,
}
impl GetTimeoutsReply {}
impl AsByteSequence for GetTimeoutsReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.standby_timeout.as_bytes(&mut bytes[index..]);
        index += self.suspend_timeout.as_bytes(&mut bytes[index..]);
        index += self.off_timeout.as_bytes(&mut bytes[index..]);
        index += 18;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetTimeoutsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (standby_timeout, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (suspend_timeout, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (off_timeout, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 18;
        Some((
            GetTimeoutsReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                standby_timeout: standby_timeout,
                suspend_timeout: suspend_timeout,
                off_timeout: off_timeout,
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
            + self.standby_timeout.size()
            + self.suspend_timeout.size()
            + self.off_timeout.size()
            + 18
    }
}
#[derive(Clone, Debug, Default)]
pub struct SetTimeoutsRequest {
    pub req_type: u8,
    pub standby_timeout: Card16,
    pub length: u16,
    pub suspend_timeout: Card16,
    pub off_timeout: Card16,
}
impl SetTimeoutsRequest {}
impl AsByteSequence for SetTimeoutsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.standby_timeout.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.suspend_timeout.as_bytes(&mut bytes[index..]);
        index += self.off_timeout.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetTimeoutsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (standby_timeout, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (suspend_timeout, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (off_timeout, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetTimeoutsRequest {
                req_type: req_type,
                standby_timeout: standby_timeout,
                length: length,
                suspend_timeout: suspend_timeout,
                off_timeout: off_timeout,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.standby_timeout.size()
            + self.length.size()
            + self.suspend_timeout.size()
            + self.off_timeout.size()
    }
}
impl Request for SetTimeoutsRequest {
    const OPCODE: u8 = 3;
    const EXTENSION: Option<&'static str> = Some("DPMS");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct EnableRequest {
    pub req_type: u8,
    pub length: u16,
}
impl EnableRequest {}
impl AsByteSequence for EnableRequest {
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
        log::trace!("Deserializing EnableRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            EnableRequest {
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
impl Request for EnableRequest {
    const OPCODE: u8 = 4;
    const EXTENSION: Option<&'static str> = Some("DPMS");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct DisableRequest {
    pub req_type: u8,
    pub length: u16,
}
impl DisableRequest {}
impl AsByteSequence for DisableRequest {
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
        log::trace!("Deserializing DisableRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DisableRequest {
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
impl Request for DisableRequest {
    const OPCODE: u8 = 5;
    const EXTENSION: Option<&'static str> = Some("DPMS");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct ForceLevelRequest {
    pub req_type: u8,
    pub power_level: DpmsMode,
    pub length: u16,
}
impl ForceLevelRequest {}
impl AsByteSequence for ForceLevelRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.power_level.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ForceLevelRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (power_level, sz): (DpmsMode, usize) = <DpmsMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ForceLevelRequest {
                req_type: req_type,
                power_level: power_level,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.power_level.size() + self.length.size()
    }
}
impl Request for ForceLevelRequest {
    const OPCODE: u8 = 6;
    const EXTENSION: Option<&'static str> = Some("DPMS");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DpmsMode {
    On = 0,
    Standby = 1,
    Suspend = 2,
    Off = 3,
}
impl AsByteSequence for DpmsMode {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u16).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::On, sz)),
            1 => Some((Self::Standby, sz)),
            2 => Some((Self::Suspend, sz)),
            3 => Some((Self::Off, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u16>()
    }
}
impl Default for DpmsMode {
    #[inline]
    fn default() -> DpmsMode {
        DpmsMode::On
    }
}
#[derive(Clone, Debug, Default)]
pub struct InfoRequest {
    pub req_type: u8,
    pub length: u16,
}
impl InfoRequest {}
impl AsByteSequence for InfoRequest {
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
        log::trace!("Deserializing InfoRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            InfoRequest {
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
impl Request for InfoRequest {
    const OPCODE: u8 = 7;
    const EXTENSION: Option<&'static str> = Some("DPMS");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = InfoReply;
}
#[derive(Clone, Debug, Default)]
pub struct InfoReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub power_level: DpmsMode,
    pub state: bool,
}
impl InfoReply {}
impl AsByteSequence for InfoReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.power_level.as_bytes(&mut bytes[index..]);
        index += self.state.as_bytes(&mut bytes[index..]);
        index += 21;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing InfoReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (power_level, sz): (DpmsMode, usize) = <DpmsMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (state, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 21;
        Some((
            InfoReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                power_level: power_level,
                state: state,
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
            + self.power_level.size()
            + self.state.size()
            + 21
    }
}
