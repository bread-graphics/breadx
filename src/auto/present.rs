// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

use super::randr::*;
use super::sync::*;
use super::xfixes::*;
use super::xproto::*;
#[derive(Clone, Debug, Default)]
pub struct Notify {
    pub window: Window,
    pub serial: Card32,
}
impl Notify {}
impl AsByteSequence for Notify {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.serial.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Notify from byte buffer");
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (serial, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Notify {
                window: window,
                serial: serial,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.window.size() + self.serial.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct QueryVersionRequest {
    pub req_type: u8,
    pub major_version: Card32,
    pub length: u16,
    pub minor_version: Card32,
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
        let (major_version, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_version, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
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
    const EXTENSION: Option<&'static str> = Some("Present");
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
pub struct PixmapRequest {
    pub req_type: u8,
    pub window: Window,
    pub length: u16,
    pub pixmap: Pixmap,
    pub serial: Card32,
    pub valid: Region,
    pub update: Region,
    pub x_off: Int16,
    pub y_off: Int16,
    pub target_crtc: Crtc,
    pub wait_fence: Fence,
    pub idle_fence: Fence,
    pub options: Card32,
    pub target_msc: Card64,
    pub divisor: Card64,
    pub remainder: Card64,
    pub notifies: Vec<Notify>,
}
impl PixmapRequest {}
impl AsByteSequence for PixmapRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.pixmap.as_bytes(&mut bytes[index..]);
        index += self.serial.as_bytes(&mut bytes[index..]);
        index += self.valid.as_bytes(&mut bytes[index..]);
        index += self.update.as_bytes(&mut bytes[index..]);
        index += self.x_off.as_bytes(&mut bytes[index..]);
        index += self.y_off.as_bytes(&mut bytes[index..]);
        index += self.target_crtc.as_bytes(&mut bytes[index..]);
        index += self.wait_fence.as_bytes(&mut bytes[index..]);
        index += self.idle_fence.as_bytes(&mut bytes[index..]);
        index += self.options.as_bytes(&mut bytes[index..]);
        index += 4;
        index += self.target_msc.as_bytes(&mut bytes[index..]);
        index += self.divisor.as_bytes(&mut bytes[index..]);
        index += self.remainder.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.notifies, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Notify>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PixmapRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pixmap, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (serial, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (valid, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (update, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x_off, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y_off, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target_crtc, sz): (Crtc, usize) = <Crtc>::from_bytes(&bytes[index..])?;
        index += sz;
        let (wait_fence, sz): (Fence, usize) = <Fence>::from_bytes(&bytes[index..])?;
        index += sz;
        let (idle_fence, sz): (Fence, usize) = <Fence>::from_bytes(&bytes[index..])?;
        index += sz;
        let (options, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (target_msc, sz): (Card64, usize) = <Card64>::from_bytes(&bytes[index..])?;
        index += sz;
        let (divisor, sz): (Card64, usize) = <Card64>::from_bytes(&bytes[index..])?;
        index += sz;
        let (remainder, sz): (Card64, usize) = <Card64>::from_bytes(&bytes[index..])?;
        index += sz;
        let (notifies, block_len): (Vec<Notify>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Notify>());
        Some((
            PixmapRequest {
                req_type: req_type,
                window: window,
                length: length,
                pixmap: pixmap,
                serial: serial,
                valid: valid,
                update: update,
                x_off: x_off,
                y_off: y_off,
                target_crtc: target_crtc,
                wait_fence: wait_fence,
                idle_fence: idle_fence,
                options: options,
                target_msc: target_msc,
                divisor: divisor,
                remainder: remainder,
                notifies: notifies,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.window.size()
            + self.length.size()
            + self.pixmap.size()
            + self.serial.size()
            + self.valid.size()
            + self.update.size()
            + self.x_off.size()
            + self.y_off.size()
            + self.target_crtc.size()
            + self.wait_fence.size()
            + self.idle_fence.size()
            + self.options.size()
            + 4
            + self.target_msc.size()
            + self.divisor.size()
            + self.remainder.size()
            + {
                let block_len: usize = self.notifies.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Notify>());
                block_len + pad
            }
    }
}
impl Request for PixmapRequest {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = Some("Present");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct NotifyMscRequest {
    pub req_type: u8,
    pub window: Window,
    pub length: u16,
    pub serial: Card32,
    pub target_msc: Card64,
    pub divisor: Card64,
    pub remainder: Card64,
}
impl NotifyMscRequest {}
impl AsByteSequence for NotifyMscRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.serial.as_bytes(&mut bytes[index..]);
        index += 4;
        index += self.target_msc.as_bytes(&mut bytes[index..]);
        index += self.divisor.as_bytes(&mut bytes[index..]);
        index += self.remainder.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing NotifyMscRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (serial, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (target_msc, sz): (Card64, usize) = <Card64>::from_bytes(&bytes[index..])?;
        index += sz;
        let (divisor, sz): (Card64, usize) = <Card64>::from_bytes(&bytes[index..])?;
        index += sz;
        let (remainder, sz): (Card64, usize) = <Card64>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            NotifyMscRequest {
                req_type: req_type,
                window: window,
                length: length,
                serial: serial,
                target_msc: target_msc,
                divisor: divisor,
                remainder: remainder,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.window.size()
            + self.length.size()
            + self.serial.size()
            + 4
            + self.target_msc.size()
            + self.divisor.size()
            + self.remainder.size()
    }
}
impl Request for NotifyMscRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = Some("Present");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Event {
    pub xid: XID,
}
impl Event {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Event {
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
pub struct SelectInputRequest {
    pub req_type: u8,
    pub eid: Event,
    pub length: u16,
    pub window: Window,
    pub event_mask: EventMask,
}
impl SelectInputRequest {}
impl AsByteSequence for SelectInputRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.eid.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.event_mask.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SelectInputRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (eid, sz): (Event, usize) = <Event>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_mask, sz): (EventMask, usize) = <EventMask>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SelectInputRequest {
                req_type: req_type,
                eid: eid,
                length: length,
                window: window,
                event_mask: event_mask,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.eid.size()
            + self.length.size()
            + self.window.size()
            + self.event_mask.size()
    }
}
impl Request for SelectInputRequest {
    const OPCODE: u8 = 3;
    const EXTENSION: Option<&'static str> = Some("Present");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct EventMask {
    pub inner: u32,
}
impl EventMask {
    #[inline]
    pub fn configure_notify(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_configure_notify(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn complete_notify(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_complete_notify(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn idle_notify(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_idle_notify(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn redirect_notify(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_redirect_notify(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn new(
        configure_notify: bool,
        complete_notify: bool,
        idle_notify: bool,
        redirect_notify: bool,
    ) -> Self {
        let mut inner: u32 = 0;
        if configure_notify {
            inner |= 1 << 0;
        } else {
            inner &= !(1 << 0);
        }
        if complete_notify {
            inner |= 1 << 1;
        } else {
            inner &= !(1 << 1);
        }
        if idle_notify {
            inner |= 1 << 2;
        } else {
            inner &= !(1 << 2);
        }
        if redirect_notify {
            inner |= 1 << 3;
        } else {
            inner &= !(1 << 3);
        }
        EventMask { inner: inner }
    }
}
impl AsByteSequence for EventMask {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((EventMask { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct QueryCapabilitiesRequest {
    pub req_type: u8,
    pub target: Card32,
    pub length: u16,
}
impl QueryCapabilitiesRequest {}
impl AsByteSequence for QueryCapabilitiesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryCapabilitiesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryCapabilitiesRequest {
                req_type: req_type,
                target: target,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.target.size() + self.length.size()
    }
}
impl Request for QueryCapabilitiesRequest {
    const OPCODE: u8 = 4;
    const EXTENSION: Option<&'static str> = Some("Present");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryCapabilitiesReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryCapabilitiesReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub capabilities: Card32,
}
impl QueryCapabilitiesReply {}
impl AsByteSequence for QueryCapabilitiesReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.capabilities.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryCapabilitiesReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (capabilities, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryCapabilitiesReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                capabilities: capabilities,
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
            + self.capabilities.size()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompleteKind {
    Pixmap = 0,
    NotifyMsc = 1,
}
impl AsByteSequence for CompleteKind {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Pixmap, sz)),
            1 => Some((Self::NotifyMsc, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for CompleteKind {
    #[inline]
    fn default() -> CompleteKind {
        CompleteKind::Pixmap
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompleteMode {
    Copy = 0,
    Flip = 1,
    Skip = 2,
    SuboptimalCopy = 3,
}
impl AsByteSequence for CompleteMode {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Copy, sz)),
            1 => Some((Self::Flip, sz)),
            2 => Some((Self::Skip, sz)),
            3 => Some((Self::SuboptimalCopy, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for CompleteMode {
    #[inline]
    fn default() -> CompleteMode {
        CompleteMode::Copy
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Capability {
    pub inner: i32,
}
impl Capability {
    #[inline]
    pub fn async_(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_async_(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn fence(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_fence(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn ust(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_ust(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn new(async_: bool, fence: bool, ust: bool) -> Self {
        let mut inner: i32 = 0;
        if async_ {
            inner |= 1 << 0;
        } else {
            inner &= !(1 << 0);
        }
        if fence {
            inner |= 1 << 1;
        } else {
            inner &= !(1 << 1);
        }
        if ust {
            inner |= 1 << 2;
        } else {
            inner &= !(1 << 2);
        }
        Capability { inner: inner }
    }
}
impl AsByteSequence for Capability {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        Some((Capability { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
pub const EVENT_CONFIGURE_NOTIFY: Event = <Event>::const_from_xid(0);
pub const EVENT_COMPLETE_NOTIFY: Event = <Event>::const_from_xid(1);
pub const EVENT_IDLE_NOTIFY: Event = <Event>::const_from_xid(2);
pub const EVENT_REDIRECT_NOTIFY: Event = <Event>::const_from_xid(3);
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Option_ {
    pub inner: i32,
}
impl Option_ {
    #[inline]
    pub fn async_(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_async_(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn copy(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_copy(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn ust(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_ust(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn suboptimal(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_suboptimal(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn new(async_: bool, copy: bool, ust: bool, suboptimal: bool) -> Self {
        let mut inner: i32 = 0;
        if async_ {
            inner |= 1 << 0;
        } else {
            inner &= !(1 << 0);
        }
        if copy {
            inner |= 1 << 1;
        } else {
            inner &= !(1 << 1);
        }
        if ust {
            inner |= 1 << 2;
        } else {
            inner &= !(1 << 2);
        }
        if suboptimal {
            inner |= 1 << 3;
        } else {
            inner &= !(1 << 3);
        }
        Option_ { inner: inner }
    }
}
impl AsByteSequence for Option_ {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        Some((Option_ { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct IdleNotifyEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub event: Event,
    pub window: Window,
    pub serial: Card32,
    pub pixmap: Pixmap,
    pub idle_fence: Fence,
}
impl IdleNotifyEvent {}
impl AsByteSequence for IdleNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.serial.as_bytes(&mut bytes[index..]);
        index += self.pixmap.as_bytes(&mut bytes[index..]);
        index += self.idle_fence.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing IdleNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Event, usize) = <Event>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (serial, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pixmap, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (idle_fence, sz): (Fence, usize) = <Fence>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            IdleNotifyEvent {
                event_type: event_type,
                sequence: sequence,
                event: event,
                window: window,
                serial: serial,
                pixmap: pixmap,
                idle_fence: idle_fence,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + 2
            + self.sequence.size()
            + self.event.size()
            + self.window.size()
            + self.serial.size()
            + self.pixmap.size()
            + self.idle_fence.size()
    }
}
impl crate::auto::Event for IdleNotifyEvent {
    const OPCODE: u8 = 2;
}
#[derive(Clone, Debug, Default)]
pub struct ConfigureNotifyEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub event: Event,
    pub window: Window,
    pub x: Int16,
    pub y: Int16,
    pub width: Card16,
    pub height: Card16,
    pub off_x: Int16,
    pub off_y: Int16,
    pub pixmap_width: Card16,
    pub pixmap_height: Card16,
    pub pixmap_flags: Card32,
}
impl ConfigureNotifyEvent {}
impl AsByteSequence for ConfigureNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.off_x.as_bytes(&mut bytes[index..]);
        index += self.off_y.as_bytes(&mut bytes[index..]);
        index += self.pixmap_width.as_bytes(&mut bytes[index..]);
        index += self.pixmap_height.as_bytes(&mut bytes[index..]);
        index += self.pixmap_flags.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ConfigureNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Event, usize) = <Event>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (off_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (off_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pixmap_width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pixmap_height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pixmap_flags, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ConfigureNotifyEvent {
                event_type: event_type,
                sequence: sequence,
                event: event,
                window: window,
                x: x,
                y: y,
                width: width,
                height: height,
                off_x: off_x,
                off_y: off_y,
                pixmap_width: pixmap_width,
                pixmap_height: pixmap_height,
                pixmap_flags: pixmap_flags,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + 2
            + self.sequence.size()
            + self.event.size()
            + self.window.size()
            + self.x.size()
            + self.y.size()
            + self.width.size()
            + self.height.size()
            + self.off_x.size()
            + self.off_y.size()
            + self.pixmap_width.size()
            + self.pixmap_height.size()
            + self.pixmap_flags.size()
    }
}
impl crate::auto::Event for ConfigureNotifyEvent {
    const OPCODE: u8 = 0;
}
#[derive(Clone, Debug, Default)]
pub struct CompleteNotifyEvent {
    pub event_type: u8,
    pub kind: CompleteKind,
    pub sequence: u16,
    pub mode: CompleteMode,
    pub event: Event,
    pub window: Window,
    pub serial: Card32,
    pub ust: Card64,
    pub msc: Card64,
}
impl CompleteNotifyEvent {}
impl AsByteSequence for CompleteNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.kind.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.serial.as_bytes(&mut bytes[index..]);
        index += self.ust.as_bytes(&mut bytes[index..]);
        index += self.msc.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CompleteNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (kind, sz): (CompleteKind, usize) = <CompleteKind>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (CompleteMode, usize) = <CompleteMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Event, usize) = <Event>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (serial, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ust, sz): (Card64, usize) = <Card64>::from_bytes(&bytes[index..])?;
        index += sz;
        let (msc, sz): (Card64, usize) = <Card64>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CompleteNotifyEvent {
                event_type: event_type,
                kind: kind,
                sequence: sequence,
                mode: mode,
                event: event,
                window: window,
                serial: serial,
                ust: ust,
                msc: msc,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.kind.size()
            + self.sequence.size()
            + self.mode.size()
            + self.event.size()
            + self.window.size()
            + self.serial.size()
            + self.ust.size()
            + self.msc.size()
    }
}
impl crate::auto::Event for CompleteNotifyEvent {
    const OPCODE: u8 = 1;
}
#[derive(Clone, Debug, Default)]
pub struct GenericEvent {
    pub event_type: u8,
    pub extension: Card8,
    pub sequence: u16,
    pub length: Card32,
    pub evtype: Card16,
    pub event: Event,
}
impl GenericEvent {}
impl AsByteSequence for GenericEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.extension.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.evtype.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.event.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GenericEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (extension, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (evtype, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (event, sz): (Event, usize) = <Event>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GenericEvent {
                event_type: event_type,
                extension: extension,
                sequence: sequence,
                length: length,
                evtype: evtype,
                event: event,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.extension.size()
            + self.sequence.size()
            + self.length.size()
            + self.evtype.size()
            + 2
            + self.event.size()
    }
}
impl crate::auto::Event for GenericEvent {
    const OPCODE: u8 = 0;
}
