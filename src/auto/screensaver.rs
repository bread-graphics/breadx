// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

use super::xproto::*;
#[derive(Clone, Debug, Default)]
pub struct QueryVersionRequest {
    pub req_type: u8,
    pub client_major_version: Card8,
    pub length: u16,
    pub client_minor_version: Card8,
}
impl QueryVersionRequest {}
impl AsByteSequence for QueryVersionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.client_major_version.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.client_minor_version.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryVersionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (client_major_version, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (client_minor_version, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            QueryVersionRequest {
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
            + 2
    }
}
impl Request for QueryVersionRequest {
    const OPCODE: u8 = 0;
    const EXTENSION: Option<&'static str> = Some("MIT-SCREEN-SAVER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryVersionReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryVersionReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major_version: Card16,
    pub server_minor_version: Card16,
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
        index += self.server_major_version.as_bytes(&mut bytes[index..]);
        index += self.server_minor_version.as_bytes(&mut bytes[index..]);
        index += 20;
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
        let (server_major_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (server_minor_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        Some((
            QueryVersionReply {
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
            + 20
    }
}
#[derive(Clone, Debug, Default)]
pub struct QueryInfoRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
}
impl QueryInfoRequest {}
impl AsByteSequence for QueryInfoRequest {
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
        log::trace!("Deserializing QueryInfoRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryInfoRequest {
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
impl Request for QueryInfoRequest {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = Some("MIT-SCREEN-SAVER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryInfoReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryInfoReply {
    pub reply_type: u8,
    pub state: Card8,
    pub sequence: u16,
    pub length: u32,
    pub saver_window: Window,
    pub ms_until_server: Card32,
    pub ms_since_user_input: Card32,
    pub event_mask: Card32,
    pub kind: Kind,
}
impl QueryInfoReply {}
impl AsByteSequence for QueryInfoReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.state.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.saver_window.as_bytes(&mut bytes[index..]);
        index += self.ms_until_server.as_bytes(&mut bytes[index..]);
        index += self.ms_since_user_input.as_bytes(&mut bytes[index..]);
        index += self.event_mask.as_bytes(&mut bytes[index..]);
        index += self.kind.as_bytes(&mut bytes[index..]);
        index += 7;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryInfoReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (state, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (saver_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ms_until_server, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ms_since_user_input, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (kind, sz): (Kind, usize) = <Kind>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 7;
        Some((
            QueryInfoReply {
                reply_type: reply_type,
                state: state,
                sequence: sequence,
                length: length,
                saver_window: saver_window,
                ms_until_server: ms_until_server,
                ms_since_user_input: ms_since_user_input,
                event_mask: event_mask,
                kind: kind,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.state.size()
            + self.sequence.size()
            + self.length.size()
            + self.saver_window.size()
            + self.ms_until_server.size()
            + self.ms_since_user_input.size()
            + self.event_mask.size()
            + self.kind.size()
            + 7
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Blanked = 0,
    Internal = 1,
    External = 2,
}
impl AsByteSequence for Kind {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Blanked, sz)),
            1 => Some((Self::Internal, sz)),
            2 => Some((Self::External, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for Kind {
    #[inline]
    fn default() -> Kind {
        Kind::Blanked
    }
}
#[derive(Clone, Debug, Default)]
pub struct SelectInputRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub event_mask: Event,
}
impl SelectInputRequest {}
impl AsByteSequence for SelectInputRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.event_mask.as_bytes(&mut bytes[index..]);
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
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_mask, sz): (Event, usize) = <Event>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SelectInputRequest {
                req_type: req_type,
                length: length,
                drawable: drawable,
                event_mask: event_mask,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.drawable.size()
            + self.event_mask.size()
    }
}
impl Request for SelectInputRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = Some("MIT-SCREEN-SAVER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Event {
    pub inner: u32,
}
impl Event {
    #[inline]
    pub fn notify_mask(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_notify_mask(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn cycle_mask(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_cycle_mask(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn new(notify_mask: bool, cycle_mask: bool) -> Self {
        let mut inner: u32 = 0;
        if notify_mask {
            inner |= 1 << 0;
        }
        if cycle_mask {
            inner |= 1 << 1;
        }
        Event { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const NOTIFY_MASK: Self = Self { inner: 1 };
    pub const CYCLE_MASK: Self = Self { inner: 2 };
    pub const COMPLETE: Self = Self { inner: 3 };
}
impl AsByteSequence for Event {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((Event { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for Event {
    type Output = Event;
    #[inline]
    fn not(self) -> Event {
        Event { inner: !self.inner }
    }
}
impl core::ops::BitAnd for Event {
    type Output = Event;
    #[inline]
    fn bitand(self, rhs: Event) -> Event {
        Event {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for Event {
    type Output = Event;
    #[inline]
    fn bitor(self, rhs: Event) -> Event {
        Event {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for Event {
    type Output = Event;
    #[inline]
    fn bitxor(self, rhs: Event) -> Event {
        Event {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct SetAttributesRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub x: Int16,
    pub y: Int16,
    pub width: Card16,
    pub height: Card16,
    pub border_width: Card16,
    pub class: WindowClass,
    pub depth: Card8,
    pub visual: Visualid,
    pub value_mask: Cw,
    pub background_pixmap: Pixmap,
    pub background_pixel: Card32,
    pub border_pixmap: Pixmap,
    pub border_pixel: Card32,
    pub bit_gravity: Gravity,
    pub win_gravity: Gravity,
    pub backing_store: BackingStore,
    pub backing_planes: Card32,
    pub backing_pixel: Card32,
    pub override_redirect: Bool32,
    pub save_under: Bool32,
    pub event_mask: EventMask,
    pub do_not_propogate_mask: EventMask,
    pub colormap: Colormap,
    pub cursor: Cursor,
}
impl SetAttributesRequest {}
impl AsByteSequence for SetAttributesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.border_width.as_bytes(&mut bytes[index..]);
        index += self.class.as_bytes(&mut bytes[index..]);
        index += self.depth.as_bytes(&mut bytes[index..]);
        index += self.visual.as_bytes(&mut bytes[index..]);
        index += self.value_mask.as_bytes(&mut bytes[index..]);
        let cond0 = (self.value_mask);
        if cond0.back_pixmap() {
            index += self.background_pixmap.as_bytes(&mut bytes[index..]);
        }
        if cond0.back_pixel() {
            index += self.background_pixel.as_bytes(&mut bytes[index..]);
        }
        if cond0.border_pixmap() {
            index += self.border_pixmap.as_bytes(&mut bytes[index..]);
        }
        if cond0.border_pixel() {
            index += self.border_pixel.as_bytes(&mut bytes[index..]);
        }
        if cond0.bit_gravity() {
            index += self.bit_gravity.as_bytes(&mut bytes[index..]);
        }
        if cond0.win_gravity() {
            index += self.win_gravity.as_bytes(&mut bytes[index..]);
        }
        if cond0.backing_store() {
            index += self.backing_store.as_bytes(&mut bytes[index..]);
        }
        if cond0.backing_planes() {
            index += self.backing_planes.as_bytes(&mut bytes[index..]);
        }
        if cond0.backing_pixel() {
            index += self.backing_pixel.as_bytes(&mut bytes[index..]);
        }
        if cond0.override_redirect() {
            index += self.override_redirect.as_bytes(&mut bytes[index..]);
        }
        if cond0.save_under() {
            index += self.save_under.as_bytes(&mut bytes[index..]);
        }
        if cond0.event_mask() {
            index += self.event_mask.as_bytes(&mut bytes[index..]);
        }
        if cond0.dont_propagate() {
            index += self.do_not_propogate_mask.as_bytes(&mut bytes[index..]);
        }
        if cond0.colormap() {
            index += self.colormap.as_bytes(&mut bytes[index..]);
        }
        if cond0.cursor() {
            index += self.cursor.as_bytes(&mut bytes[index..]);
        }
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetAttributesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (border_width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (class, sz): (WindowClass, usize) = <WindowClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (depth, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (visual, sz): (Visualid, usize) = <Visualid>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value_mask, sz): (Cw, usize) = <Cw>::from_bytes(&bytes[index..])?;
        index += sz;
        let cond0 = (value_mask);
        let background_pixmap: Pixmap = if cond0.back_pixmap() {
            let (background_pixmap, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
            index += sz;
            background_pixmap
        } else {
            Default::default()
        };
        let background_pixel: Card32 = if cond0.back_pixel() {
            let (background_pixel, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            background_pixel
        } else {
            Default::default()
        };
        let border_pixmap: Pixmap = if cond0.border_pixmap() {
            let (border_pixmap, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
            index += sz;
            border_pixmap
        } else {
            Default::default()
        };
        let border_pixel: Card32 = if cond0.border_pixel() {
            let (border_pixel, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            border_pixel
        } else {
            Default::default()
        };
        let bit_gravity: Gravity = if cond0.bit_gravity() {
            let (bit_gravity, sz): (Gravity, usize) = <Gravity>::from_bytes(&bytes[index..])?;
            index += sz;
            bit_gravity
        } else {
            Default::default()
        };
        let win_gravity: Gravity = if cond0.win_gravity() {
            let (win_gravity, sz): (Gravity, usize) = <Gravity>::from_bytes(&bytes[index..])?;
            index += sz;
            win_gravity
        } else {
            Default::default()
        };
        let backing_store: BackingStore = if cond0.backing_store() {
            let (backing_store, sz): (BackingStore, usize) =
                <BackingStore>::from_bytes(&bytes[index..])?;
            index += sz;
            backing_store
        } else {
            Default::default()
        };
        let backing_planes: Card32 = if cond0.backing_planes() {
            let (backing_planes, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            backing_planes
        } else {
            Default::default()
        };
        let backing_pixel: Card32 = if cond0.backing_pixel() {
            let (backing_pixel, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            backing_pixel
        } else {
            Default::default()
        };
        let override_redirect: Bool32 = if cond0.override_redirect() {
            let (override_redirect, sz): (Bool32, usize) = <Bool32>::from_bytes(&bytes[index..])?;
            index += sz;
            override_redirect
        } else {
            Default::default()
        };
        let save_under: Bool32 = if cond0.save_under() {
            let (save_under, sz): (Bool32, usize) = <Bool32>::from_bytes(&bytes[index..])?;
            index += sz;
            save_under
        } else {
            Default::default()
        };
        let event_mask: EventMask = if cond0.event_mask() {
            let (event_mask, sz): (EventMask, usize) = <EventMask>::from_bytes(&bytes[index..])?;
            index += sz;
            event_mask
        } else {
            Default::default()
        };
        let do_not_propogate_mask: EventMask = if cond0.dont_propagate() {
            let (do_not_propogate_mask, sz): (EventMask, usize) =
                <EventMask>::from_bytes(&bytes[index..])?;
            index += sz;
            do_not_propogate_mask
        } else {
            Default::default()
        };
        let colormap: Colormap = if cond0.colormap() {
            let (colormap, sz): (Colormap, usize) = <Colormap>::from_bytes(&bytes[index..])?;
            index += sz;
            colormap
        } else {
            Default::default()
        };
        let cursor: Cursor = if cond0.cursor() {
            let (cursor, sz): (Cursor, usize) = <Cursor>::from_bytes(&bytes[index..])?;
            index += sz;
            cursor
        } else {
            Default::default()
        };
        Some((
            SetAttributesRequest {
                req_type: req_type,
                length: length,
                drawable: drawable,
                x: x,
                y: y,
                width: width,
                height: height,
                border_width: border_width,
                class: class,
                depth: depth,
                visual: visual,
                value_mask: value_mask,
                background_pixmap: background_pixmap,
                background_pixel: background_pixel,
                border_pixmap: border_pixmap,
                border_pixel: border_pixel,
                bit_gravity: bit_gravity,
                win_gravity: win_gravity,
                backing_store: backing_store,
                backing_planes: backing_planes,
                backing_pixel: backing_pixel,
                override_redirect: override_redirect,
                save_under: save_under,
                event_mask: event_mask,
                do_not_propogate_mask: do_not_propogate_mask,
                colormap: colormap,
                cursor: cursor,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.drawable.size()
            + self.x.size()
            + self.y.size()
            + self.width.size()
            + self.height.size()
            + self.border_width.size()
            + self.class.size()
            + self.depth.size()
            + self.visual.size()
            + self.value_mask.size()
            + self.background_pixmap.size()
            + self.background_pixel.size()
            + self.border_pixmap.size()
            + self.border_pixel.size()
            + self.bit_gravity.size()
            + self.win_gravity.size()
            + self.backing_store.size()
            + self.backing_planes.size()
            + self.backing_pixel.size()
            + self.override_redirect.size()
            + self.save_under.size()
            + self.event_mask.size()
            + self.do_not_propogate_mask.size()
            + self.colormap.size()
            + self.cursor.size()
    }
}
impl Request for SetAttributesRequest {
    const OPCODE: u8 = 3;
    const EXTENSION: Option<&'static str> = Some("MIT-SCREEN-SAVER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct UnsetAttributesRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
}
impl UnsetAttributesRequest {}
impl AsByteSequence for UnsetAttributesRequest {
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
        log::trace!("Deserializing UnsetAttributesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            UnsetAttributesRequest {
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
impl Request for UnsetAttributesRequest {
    const OPCODE: u8 = 4;
    const EXTENSION: Option<&'static str> = Some("MIT-SCREEN-SAVER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct SuspendRequest {
    pub req_type: u8,
    pub length: u16,
    pub suspend: Card32,
}
impl SuspendRequest {}
impl AsByteSequence for SuspendRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.suspend.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SuspendRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (suspend, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SuspendRequest {
                req_type: req_type,
                length: length,
                suspend: suspend,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.suspend.size()
    }
}
impl Request for SuspendRequest {
    const OPCODE: u8 = 5;
    const EXTENSION: Option<&'static str> = Some("MIT-SCREEN-SAVER");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum State {
    Off = 0,
    On = 1,
    Cycle = 2,
    Disabled = 3,
}
impl AsByteSequence for State {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Off, sz)),
            1 => Some((Self::On, sz)),
            2 => Some((Self::Cycle, sz)),
            3 => Some((Self::Disabled, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for State {
    #[inline]
    fn default() -> State {
        State::Off
    }
}
#[derive(Clone, Debug, Default)]
pub struct NotifyEvent {
    pub event_type: u8,
    pub state: State,
    pub sequence: u16,
    pub time: Timestamp,
    pub root: Window,
    pub window: Window,
    pub kind: Kind,
    pub forced: bool,
}
impl NotifyEvent {}
impl AsByteSequence for NotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.state.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.kind.as_bytes(&mut bytes[index..]);
        index += self.forced.as_bytes(&mut bytes[index..]);
        index += 14;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing NotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (state, sz): (State, usize) = <State>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (kind, sz): (Kind, usize) = <Kind>::from_bytes(&bytes[index..])?;
        index += sz;
        let (forced, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 14;
        Some((
            NotifyEvent {
                event_type: event_type,
                state: state,
                sequence: sequence,
                time: time,
                root: root,
                window: window,
                kind: kind,
                forced: forced,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.state.size()
            + self.sequence.size()
            + self.time.size()
            + self.root.size()
            + self.window.size()
            + self.kind.size()
            + self.forced.size()
            + 14
    }
}
impl crate::auto::Event for NotifyEvent {
    const OPCODE: u8 = 0;
}
