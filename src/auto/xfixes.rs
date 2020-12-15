// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

use super::render::*;
use super::shape::*;
use super::xproto::*;
#[derive(Clone, Debug, Default)]
pub struct QueryVersionRequest {
    pub req_type: u8,
    pub client_major_version: Card32,
    pub length: u16,
    pub client_minor_version: Card32,
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
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryVersionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (client_major_version, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (client_minor_version, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
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
    }
}
impl Request for QueryVersionRequest {
    const OPCODE: u8 = 0;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
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
pub struct ChangeSaveSetRequest {
    pub req_type: u8,
    pub mode: SaveSetMode,
    pub length: u16,
    pub target: SaveSetTarget,
    pub map: SaveSetMapping,
    pub window: Window,
}
impl ChangeSaveSetRequest {}
impl AsByteSequence for ChangeSaveSetRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.map.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangeSaveSetRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (SaveSetMode, usize) = <SaveSetMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (SaveSetTarget, usize) = <SaveSetTarget>::from_bytes(&bytes[index..])?;
        index += sz;
        let (map, sz): (SaveSetMapping, usize) = <SaveSetMapping>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ChangeSaveSetRequest {
                req_type: req_type,
                mode: mode,
                length: length,
                target: target,
                map: map,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.mode.size()
            + self.length.size()
            + self.target.size()
            + self.map.size()
            + 1
            + self.window.size()
    }
}
impl Request for ChangeSaveSetRequest {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SaveSetMode {
    Insert = 0,
    Delete = 1,
}
impl AsByteSequence for SaveSetMode {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Insert, sz)),
            1 => Some((Self::Delete, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for SaveSetMode {
    #[inline]
    fn default() -> SaveSetMode {
        SaveSetMode::Insert
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SaveSetTarget {
    Nearest = 0,
    Root = 1,
}
impl AsByteSequence for SaveSetTarget {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Nearest, sz)),
            1 => Some((Self::Root, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for SaveSetTarget {
    #[inline]
    fn default() -> SaveSetTarget {
        SaveSetTarget::Nearest
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SaveSetMapping {
    Map = 0,
    Unmap = 1,
}
impl AsByteSequence for SaveSetMapping {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Map, sz)),
            1 => Some((Self::Unmap, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for SaveSetMapping {
    #[inline]
    fn default() -> SaveSetMapping {
        SaveSetMapping::Map
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SelectionEvent {
    SetSelectionOwner = 0,
    SelectionWindowDestroy = 1,
    SelectionClientClose = 2,
}
impl AsByteSequence for SelectionEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::SetSelectionOwner, sz)),
            1 => Some((Self::SelectionWindowDestroy, sz)),
            2 => Some((Self::SelectionClientClose, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for SelectionEvent {
    #[inline]
    fn default() -> SelectionEvent {
        SelectionEvent::SetSelectionOwner
    }
}
#[derive(Clone, Debug, Default)]
pub struct SelectSelectionInputRequest {
    pub req_type: u8,
    pub window: Window,
    pub length: u16,
    pub selection: Atom,
    pub event_mask: SelectionEventMask,
}
impl SelectSelectionInputRequest {}
impl AsByteSequence for SelectSelectionInputRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.selection.as_bytes(&mut bytes[index..]);
        index += self.event_mask.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SelectSelectionInputRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (selection, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_mask, sz): (SelectionEventMask, usize) =
            <SelectionEventMask>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SelectSelectionInputRequest {
                req_type: req_type,
                window: window,
                length: length,
                selection: selection,
                event_mask: event_mask,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.window.size()
            + self.length.size()
            + self.selection.size()
            + self.event_mask.size()
    }
}
impl Request for SelectSelectionInputRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct SelectionEventMask {
    pub inner: u32,
}
impl SelectionEventMask {
    #[inline]
    pub fn set_selection_owner(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_set_selection_owner(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn selection_window_destroy(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_selection_window_destroy(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn selection_client_close(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_selection_client_close(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn new(
        set_selection_owner: bool,
        selection_window_destroy: bool,
        selection_client_close: bool,
    ) -> Self {
        let mut inner: u32 = 0;
        if set_selection_owner {
            inner |= 1 << 0;
        } else {
            inner &= !(1 << 0);
        }
        if selection_window_destroy {
            inner |= 1 << 1;
        } else {
            inner &= !(1 << 1);
        }
        if selection_client_close {
            inner |= 1 << 2;
        } else {
            inner &= !(1 << 2);
        }
        SelectionEventMask { inner: inner }
    }
}
impl AsByteSequence for SelectionEventMask {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((SelectionEventMask { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct SelectCursorInputRequest {
    pub req_type: u8,
    pub window: Window,
    pub length: u16,
    pub event_mask: CursorNotifyMask,
}
impl SelectCursorInputRequest {}
impl AsByteSequence for SelectCursorInputRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.event_mask.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SelectCursorInputRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_mask, sz): (CursorNotifyMask, usize) =
            <CursorNotifyMask>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SelectCursorInputRequest {
                req_type: req_type,
                window: window,
                length: length,
                event_mask: event_mask,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.window.size() + self.length.size() + self.event_mask.size()
    }
}
impl Request for SelectCursorInputRequest {
    const OPCODE: u8 = 3;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct CursorNotifyMask {
    pub inner: u32,
}
impl CursorNotifyMask {
    #[inline]
    pub fn display_cursor(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_display_cursor(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn new(display_cursor: bool) -> Self {
        let mut inner: u32 = 0;
        if display_cursor {
            inner |= 1 << 0;
        } else {
            inner &= !(1 << 0);
        }
        CursorNotifyMask { inner: inner }
    }
}
impl AsByteSequence for CursorNotifyMask {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((CursorNotifyMask { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetCursorImageRequest {
    pub req_type: u8,
    pub length: u16,
}
impl GetCursorImageRequest {}
impl AsByteSequence for GetCursorImageRequest {
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
        log::trace!("Deserializing GetCursorImageRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetCursorImageRequest {
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
impl Request for GetCursorImageRequest {
    const OPCODE: u8 = 4;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetCursorImageReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetCursorImageReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub x: Int16,
    pub y: Int16,
    pub width: Card16,
    pub height: Card16,
    pub xhot: Card16,
    pub yhot: Card16,
    pub cursor_serial: Card32,
    pub cursor_image: Vec<Card32>,
}
impl GetCursorImageReply {}
impl AsByteSequence for GetCursorImageReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.xhot.as_bytes(&mut bytes[index..]);
        index += self.yhot.as_bytes(&mut bytes[index..]);
        index += self.cursor_serial.as_bytes(&mut bytes[index..]);
        index += 8;
        let block_len: usize = vector_as_bytes(&self.cursor_image, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetCursorImageReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xhot, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (yhot, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cursor_serial, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 8;
        let (cursor_image, block_len): (Vec<Card32>, usize) = vector_from_bytes(
            &bytes[index..],
            ((width as usize) * (height as usize)) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            GetCursorImageReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                x: x,
                y: y,
                width: width,
                height: height,
                xhot: xhot,
                yhot: yhot,
                cursor_serial: cursor_serial,
                cursor_image: cursor_image,
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
            + self.width.size()
            + self.height.size()
            + self.xhot.size()
            + self.yhot.size()
            + self.cursor_serial.size()
            + 8
            + {
                let block_len: usize = self.cursor_image.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Region {
    pub xid: XID,
}
impl Region {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Region {
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
pub struct CreateRegionRequest {
    pub req_type: u8,
    pub region: Region,
    pub length: u16,
    pub rectangles: Vec<Rectangle>,
}
impl CreateRegionRequest {}
impl AsByteSequence for CreateRegionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.region.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.rectangles, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateRegionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (region, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (rectangles, block_len): (Vec<Rectangle>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
        Some((
            CreateRegionRequest {
                req_type: req_type,
                region: region,
                length: length,
                rectangles: rectangles,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.region.size() + self.length.size() + {
            let block_len: usize = self.rectangles.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
            block_len + pad
        }
    }
}
impl Request for CreateRegionRequest {
    const OPCODE: u8 = 5;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CreateRegionFromBitmapRequest {
    pub req_type: u8,
    pub region: Region,
    pub length: u16,
    pub bitmap: Pixmap,
}
impl CreateRegionFromBitmapRequest {}
impl AsByteSequence for CreateRegionFromBitmapRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.region.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.bitmap.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateRegionFromBitmapRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (region, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bitmap, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CreateRegionFromBitmapRequest {
                req_type: req_type,
                region: region,
                length: length,
                bitmap: bitmap,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.region.size() + self.length.size() + self.bitmap.size()
    }
}
impl Request for CreateRegionFromBitmapRequest {
    const OPCODE: u8 = 6;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CreateRegionFromWindowRequest {
    pub req_type: u8,
    pub region: Region,
    pub length: u16,
    pub window: Window,
    pub kind: Sk,
}
impl CreateRegionFromWindowRequest {}
impl AsByteSequence for CreateRegionFromWindowRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.region.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.kind.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateRegionFromWindowRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (region, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (kind, sz): (Sk, usize) = <Sk>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            CreateRegionFromWindowRequest {
                req_type: req_type,
                region: region,
                length: length,
                window: window,
                kind: kind,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.region.size()
            + self.length.size()
            + self.window.size()
            + self.kind.size()
            + 3
    }
}
impl Request for CreateRegionFromWindowRequest {
    const OPCODE: u8 = 7;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CreateRegionFromGcRequest {
    pub req_type: u8,
    pub region: Region,
    pub length: u16,
    pub gc: Gcontext,
}
impl CreateRegionFromGcRequest {}
impl AsByteSequence for CreateRegionFromGcRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.region.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateRegionFromGcRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (region, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CreateRegionFromGcRequest {
                req_type: req_type,
                region: region,
                length: length,
                gc: gc,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.region.size() + self.length.size() + self.gc.size()
    }
}
impl Request for CreateRegionFromGcRequest {
    const OPCODE: u8 = 8;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CreateRegionFromPictureRequest {
    pub req_type: u8,
    pub region: Region,
    pub length: u16,
    pub picture: Picture,
}
impl CreateRegionFromPictureRequest {}
impl AsByteSequence for CreateRegionFromPictureRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.region.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.picture.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateRegionFromPictureRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (region, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (picture, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CreateRegionFromPictureRequest {
                req_type: req_type,
                region: region,
                length: length,
                picture: picture,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.region.size() + self.length.size() + self.picture.size()
    }
}
impl Request for CreateRegionFromPictureRequest {
    const OPCODE: u8 = 9;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct DestroyRegionRequest {
    pub req_type: u8,
    pub region: Region,
    pub length: u16,
}
impl DestroyRegionRequest {}
impl AsByteSequence for DestroyRegionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.region.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DestroyRegionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (region, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DestroyRegionRequest {
                req_type: req_type,
                region: region,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.region.size() + self.length.size()
    }
}
impl Request for DestroyRegionRequest {
    const OPCODE: u8 = 10;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct SetRegionRequest {
    pub req_type: u8,
    pub region: Region,
    pub length: u16,
    pub rectangles: Vec<Rectangle>,
}
impl SetRegionRequest {}
impl AsByteSequence for SetRegionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.region.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.rectangles, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetRegionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (region, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (rectangles, block_len): (Vec<Rectangle>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
        Some((
            SetRegionRequest {
                req_type: req_type,
                region: region,
                length: length,
                rectangles: rectangles,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.region.size() + self.length.size() + {
            let block_len: usize = self.rectangles.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
            block_len + pad
        }
    }
}
impl Request for SetRegionRequest {
    const OPCODE: u8 = 11;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CopyRegionRequest {
    pub req_type: u8,
    pub source: Region,
    pub length: u16,
    pub destination: Region,
}
impl CopyRegionRequest {}
impl AsByteSequence for CopyRegionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.source.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.destination.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CopyRegionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (source, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (destination, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CopyRegionRequest {
                req_type: req_type,
                source: source,
                length: length,
                destination: destination,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.source.size() + self.length.size() + self.destination.size()
    }
}
impl Request for CopyRegionRequest {
    const OPCODE: u8 = 12;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct UnionRegionRequest {
    pub req_type: u8,
    pub source1: Region,
    pub length: u16,
    pub source2: Region,
    pub destination: Region,
}
impl UnionRegionRequest {}
impl AsByteSequence for UnionRegionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.source1.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.source2.as_bytes(&mut bytes[index..]);
        index += self.destination.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing UnionRegionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (source1, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (source2, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (destination, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            UnionRegionRequest {
                req_type: req_type,
                source1: source1,
                length: length,
                source2: source2,
                destination: destination,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.source1.size()
            + self.length.size()
            + self.source2.size()
            + self.destination.size()
    }
}
impl Request for UnionRegionRequest {
    const OPCODE: u8 = 13;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct IntersectRegionRequest {
    pub req_type: u8,
    pub source1: Region,
    pub length: u16,
    pub source2: Region,
    pub destination: Region,
}
impl IntersectRegionRequest {}
impl AsByteSequence for IntersectRegionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.source1.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.source2.as_bytes(&mut bytes[index..]);
        index += self.destination.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing IntersectRegionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (source1, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (source2, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (destination, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            IntersectRegionRequest {
                req_type: req_type,
                source1: source1,
                length: length,
                source2: source2,
                destination: destination,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.source1.size()
            + self.length.size()
            + self.source2.size()
            + self.destination.size()
    }
}
impl Request for IntersectRegionRequest {
    const OPCODE: u8 = 14;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct SubtractRegionRequest {
    pub req_type: u8,
    pub source1: Region,
    pub length: u16,
    pub source2: Region,
    pub destination: Region,
}
impl SubtractRegionRequest {}
impl AsByteSequence for SubtractRegionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.source1.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.source2.as_bytes(&mut bytes[index..]);
        index += self.destination.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SubtractRegionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (source1, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (source2, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (destination, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SubtractRegionRequest {
                req_type: req_type,
                source1: source1,
                length: length,
                source2: source2,
                destination: destination,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.source1.size()
            + self.length.size()
            + self.source2.size()
            + self.destination.size()
    }
}
impl Request for SubtractRegionRequest {
    const OPCODE: u8 = 15;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct InvertRegionRequest {
    pub req_type: u8,
    pub source: Region,
    pub length: u16,
    pub bounds: Rectangle,
    pub destination: Region,
}
impl InvertRegionRequest {}
impl AsByteSequence for InvertRegionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.source.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.bounds.as_bytes(&mut bytes[index..]);
        index += self.destination.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing InvertRegionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (source, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bounds, sz): (Rectangle, usize) = <Rectangle>::from_bytes(&bytes[index..])?;
        index += sz;
        let (destination, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            InvertRegionRequest {
                req_type: req_type,
                source: source,
                length: length,
                bounds: bounds,
                destination: destination,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.source.size()
            + self.length.size()
            + self.bounds.size()
            + self.destination.size()
    }
}
impl Request for InvertRegionRequest {
    const OPCODE: u8 = 16;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct TranslateRegionRequest {
    pub req_type: u8,
    pub region: Region,
    pub length: u16,
    pub dx: Int16,
    pub dy: Int16,
}
impl TranslateRegionRequest {}
impl AsByteSequence for TranslateRegionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.region.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.dx.as_bytes(&mut bytes[index..]);
        index += self.dy.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing TranslateRegionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (region, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dx, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dy, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            TranslateRegionRequest {
                req_type: req_type,
                region: region,
                length: length,
                dx: dx,
                dy: dy,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.region.size()
            + self.length.size()
            + self.dx.size()
            + self.dy.size()
    }
}
impl Request for TranslateRegionRequest {
    const OPCODE: u8 = 17;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct RegionExtentsRequest {
    pub req_type: u8,
    pub source: Region,
    pub length: u16,
    pub destination: Region,
}
impl RegionExtentsRequest {}
impl AsByteSequence for RegionExtentsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.source.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.destination.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing RegionExtentsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (source, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (destination, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            RegionExtentsRequest {
                req_type: req_type,
                source: source,
                length: length,
                destination: destination,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.source.size() + self.length.size() + self.destination.size()
    }
}
impl Request for RegionExtentsRequest {
    const OPCODE: u8 = 18;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct FetchRegionRequest {
    pub req_type: u8,
    pub region: Region,
    pub length: u16,
}
impl FetchRegionRequest {}
impl AsByteSequence for FetchRegionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.region.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FetchRegionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (region, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            FetchRegionRequest {
                req_type: req_type,
                region: region,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.region.size() + self.length.size()
    }
}
impl Request for FetchRegionRequest {
    const OPCODE: u8 = 19;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = FetchRegionReply;
}
#[derive(Clone, Debug, Default)]
pub struct FetchRegionReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub extents: Rectangle,
    pub rectangles: Vec<Rectangle>,
}
impl FetchRegionReply {}
impl AsByteSequence for FetchRegionReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.extents.as_bytes(&mut bytes[index..]);
        index += 16;
        let block_len: usize = vector_as_bytes(&self.rectangles, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FetchRegionReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (extents, sz): (Rectangle, usize) = <Rectangle>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 16;
        let (rectangles, block_len): (Vec<Rectangle>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize) / (2)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
        Some((
            FetchRegionReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                extents: extents,
                rectangles: rectangles,
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
            + self.extents.size()
            + 16
            + {
                let block_len: usize = self.rectangles.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct SetGcClipRegionRequest {
    pub req_type: u8,
    pub gc: Gcontext,
    pub length: u16,
    pub region: Region,
    pub x_origin: Int16,
    pub y_origin: Int16,
}
impl SetGcClipRegionRequest {}
impl AsByteSequence for SetGcClipRegionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.region.as_bytes(&mut bytes[index..]);
        index += self.x_origin.as_bytes(&mut bytes[index..]);
        index += self.y_origin.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetGcClipRegionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (region, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x_origin, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y_origin, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetGcClipRegionRequest {
                req_type: req_type,
                gc: gc,
                length: length,
                region: region,
                x_origin: x_origin,
                y_origin: y_origin,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.gc.size()
            + self.length.size()
            + self.region.size()
            + self.x_origin.size()
            + self.y_origin.size()
    }
}
impl Request for SetGcClipRegionRequest {
    const OPCODE: u8 = 20;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
pub const REGION_NONE: Region = <Region>::const_from_xid(0);
#[derive(Clone, Debug, Default)]
pub struct SetWindowShapeRegionRequest {
    pub req_type: u8,
    pub dest: Window,
    pub length: u16,
    pub dest_kind: Sk,
    pub x_offset: Int16,
    pub y_offset: Int16,
    pub region: Region,
}
impl SetWindowShapeRegionRequest {}
impl AsByteSequence for SetWindowShapeRegionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.dest.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.dest_kind.as_bytes(&mut bytes[index..]);
        index += 3;
        index += self.x_offset.as_bytes(&mut bytes[index..]);
        index += self.y_offset.as_bytes(&mut bytes[index..]);
        index += self.region.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetWindowShapeRegionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dest, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dest_kind, sz): (Sk, usize) = <Sk>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (x_offset, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y_offset, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (region, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetWindowShapeRegionRequest {
                req_type: req_type,
                dest: dest,
                length: length,
                dest_kind: dest_kind,
                x_offset: x_offset,
                y_offset: y_offset,
                region: region,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.dest.size()
            + self.length.size()
            + self.dest_kind.size()
            + 3
            + self.x_offset.size()
            + self.y_offset.size()
            + self.region.size()
    }
}
impl Request for SetWindowShapeRegionRequest {
    const OPCODE: u8 = 21;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct SetPictureClipRegionRequest {
    pub req_type: u8,
    pub picture: Picture,
    pub length: u16,
    pub region: Region,
    pub x_origin: Int16,
    pub y_origin: Int16,
}
impl SetPictureClipRegionRequest {}
impl AsByteSequence for SetPictureClipRegionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.picture.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.region.as_bytes(&mut bytes[index..]);
        index += self.x_origin.as_bytes(&mut bytes[index..]);
        index += self.y_origin.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetPictureClipRegionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (picture, sz): (Picture, usize) = <Picture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (region, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x_origin, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y_origin, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetPictureClipRegionRequest {
                req_type: req_type,
                picture: picture,
                length: length,
                region: region,
                x_origin: x_origin,
                y_origin: y_origin,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.picture.size()
            + self.length.size()
            + self.region.size()
            + self.x_origin.size()
            + self.y_origin.size()
    }
}
impl Request for SetPictureClipRegionRequest {
    const OPCODE: u8 = 22;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct SetCursorNameRequest {
    pub req_type: u8,
    pub cursor: Cursor,
    pub length: u16,
    pub name: String,
}
impl SetCursorNameRequest {}
impl AsByteSequence for SetCursorNameRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.cursor.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.name.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = string_as_bytes(&self.name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetCursorNameRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cursor, sz): (Cursor, usize) = <Cursor>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (name, block_len): (String, usize) = string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            SetCursorNameRequest {
                req_type: req_type,
                cursor: cursor,
                length: length,
                name: name,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.cursor.size()
            + self.length.size()
            + ::core::mem::size_of::<Card16>()
            + 2
            + {
                let block_len: usize = self.name.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
impl Request for SetCursorNameRequest {
    const OPCODE: u8 = 23;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GetCursorNameRequest {
    pub req_type: u8,
    pub cursor: Cursor,
    pub length: u16,
}
impl GetCursorNameRequest {}
impl AsByteSequence for GetCursorNameRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.cursor.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetCursorNameRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cursor, sz): (Cursor, usize) = <Cursor>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetCursorNameRequest {
                req_type: req_type,
                cursor: cursor,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.cursor.size() + self.length.size()
    }
}
impl Request for GetCursorNameRequest {
    const OPCODE: u8 = 24;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetCursorNameReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetCursorNameReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub atom: Atom,
    pub name: String,
}
impl GetCursorNameReply {}
impl AsByteSequence for GetCursorNameReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.atom.as_bytes(&mut bytes[index..]);
        index += (self.name.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 18;
        let block_len: usize = string_as_bytes(&self.name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetCursorNameReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (atom, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 18;
        let (name, block_len): (String, usize) = string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            GetCursorNameReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                atom: atom,
                name: name,
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
            + self.atom.size()
            + ::core::mem::size_of::<Card16>()
            + 18
            + {
                let block_len: usize = self.name.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetCursorImageAndNameRequest {
    pub req_type: u8,
    pub length: u16,
}
impl GetCursorImageAndNameRequest {}
impl AsByteSequence for GetCursorImageAndNameRequest {
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
        log::trace!("Deserializing GetCursorImageAndNameRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetCursorImageAndNameRequest {
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
impl Request for GetCursorImageAndNameRequest {
    const OPCODE: u8 = 25;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetCursorImageAndNameReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetCursorImageAndNameReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub x: Int16,
    pub y: Int16,
    pub width: Card16,
    pub height: Card16,
    pub xhot: Card16,
    pub yhot: Card16,
    pub cursor_serial: Card32,
    pub cursor_atom: Atom,
    pub cursor_image: Vec<Card32>,
    pub name: String,
}
impl GetCursorImageAndNameReply {}
impl AsByteSequence for GetCursorImageAndNameReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.xhot.as_bytes(&mut bytes[index..]);
        index += self.yhot.as_bytes(&mut bytes[index..]);
        index += self.cursor_serial.as_bytes(&mut bytes[index..]);
        index += self.cursor_atom.as_bytes(&mut bytes[index..]);
        index += (self.name.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = vector_as_bytes(&self.cursor_image, &mut bytes[index..]);
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
        log::trace!("Deserializing GetCursorImageAndNameReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xhot, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (yhot, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cursor_serial, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cursor_atom, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (cursor_image, block_len): (Vec<Card32>, usize) = vector_from_bytes(
            &bytes[index..],
            ((width as usize) * (height as usize)) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (name, block_len): (String, usize) = string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            GetCursorImageAndNameReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                x: x,
                y: y,
                width: width,
                height: height,
                xhot: xhot,
                yhot: yhot,
                cursor_serial: cursor_serial,
                cursor_atom: cursor_atom,
                cursor_image: cursor_image,
                name: name,
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
            + self.width.size()
            + self.height.size()
            + self.xhot.size()
            + self.yhot.size()
            + self.cursor_serial.size()
            + self.cursor_atom.size()
            + ::core::mem::size_of::<Card16>()
            + 2
            + {
                let block_len: usize = self.cursor_image.iter().map(|i| i.size()).sum();
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
#[derive(Clone, Debug, Default)]
pub struct ChangeCursorRequest {
    pub req_type: u8,
    pub source: Cursor,
    pub length: u16,
    pub destination: Cursor,
}
impl ChangeCursorRequest {}
impl AsByteSequence for ChangeCursorRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.source.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.destination.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangeCursorRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (source, sz): (Cursor, usize) = <Cursor>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (destination, sz): (Cursor, usize) = <Cursor>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ChangeCursorRequest {
                req_type: req_type,
                source: source,
                length: length,
                destination: destination,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.source.size() + self.length.size() + self.destination.size()
    }
}
impl Request for ChangeCursorRequest {
    const OPCODE: u8 = 26;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct ChangeCursorByNameRequest {
    pub req_type: u8,
    pub src: Cursor,
    pub length: u16,
    pub name: String,
}
impl ChangeCursorByNameRequest {}
impl AsByteSequence for ChangeCursorByNameRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.src.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.name.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = string_as_bytes(&self.name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangeCursorByNameRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src, sz): (Cursor, usize) = <Cursor>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (name, block_len): (String, usize) = string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            ChangeCursorByNameRequest {
                req_type: req_type,
                src: src,
                length: length,
                name: name,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.src.size()
            + self.length.size()
            + ::core::mem::size_of::<Card16>()
            + 2
            + {
                let block_len: usize = self.name.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
impl Request for ChangeCursorByNameRequest {
    const OPCODE: u8 = 27;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct ExpandRegionRequest {
    pub req_type: u8,
    pub source: Region,
    pub length: u16,
    pub destination: Region,
    pub left: Card16,
    pub right: Card16,
    pub top: Card16,
    pub bottom: Card16,
}
impl ExpandRegionRequest {}
impl AsByteSequence for ExpandRegionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.source.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.destination.as_bytes(&mut bytes[index..]);
        index += self.left.as_bytes(&mut bytes[index..]);
        index += self.right.as_bytes(&mut bytes[index..]);
        index += self.top.as_bytes(&mut bytes[index..]);
        index += self.bottom.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ExpandRegionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (source, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (destination, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (left, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (right, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (top, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bottom, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ExpandRegionRequest {
                req_type: req_type,
                source: source,
                length: length,
                destination: destination,
                left: left,
                right: right,
                top: top,
                bottom: bottom,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.source.size()
            + self.length.size()
            + self.destination.size()
            + self.left.size()
            + self.right.size()
            + self.top.size()
            + self.bottom.size()
    }
}
impl Request for ExpandRegionRequest {
    const OPCODE: u8 = 28;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct HideCursorRequest {
    pub req_type: u8,
    pub window: Window,
    pub length: u16,
}
impl HideCursorRequest {}
impl AsByteSequence for HideCursorRequest {
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
        log::trace!("Deserializing HideCursorRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            HideCursorRequest {
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
impl Request for HideCursorRequest {
    const OPCODE: u8 = 29;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct ShowCursorRequest {
    pub req_type: u8,
    pub window: Window,
    pub length: u16,
}
impl ShowCursorRequest {}
impl AsByteSequence for ShowCursorRequest {
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
        log::trace!("Deserializing ShowCursorRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ShowCursorRequest {
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
impl Request for ShowCursorRequest {
    const OPCODE: u8 = 30;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Barrier {
    pub xid: XID,
}
impl Barrier {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Barrier {
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
pub struct CreatePointerBarrierRequest {
    pub req_type: u8,
    pub barrier: Barrier,
    pub length: u16,
    pub window: Window,
    pub x1: Card16,
    pub y1: Card16,
    pub x2: Card16,
    pub y2: Card16,
    pub directions: BarrierDirections,
    pub devices: Vec<Card16>,
}
impl CreatePointerBarrierRequest {}
impl AsByteSequence for CreatePointerBarrierRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.barrier.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.x1.as_bytes(&mut bytes[index..]);
        index += self.y1.as_bytes(&mut bytes[index..]);
        index += self.x2.as_bytes(&mut bytes[index..]);
        index += self.y2.as_bytes(&mut bytes[index..]);
        index += self.directions.as_bytes(&mut bytes[index..]);
        index += 2;
        index += (self.devices.len() as Card16).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.devices, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreatePointerBarrierRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (barrier, sz): (Barrier, usize) = <Barrier>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x2, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y2, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (directions, sz): (BarrierDirections, usize) =
            <BarrierDirections>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (devices, block_len): (Vec<Card16>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        Some((
            CreatePointerBarrierRequest {
                req_type: req_type,
                barrier: barrier,
                length: length,
                window: window,
                x1: x1,
                y1: y1,
                x2: x2,
                y2: y2,
                directions: directions,
                devices: devices,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.barrier.size()
            + self.length.size()
            + self.window.size()
            + self.x1.size()
            + self.y1.size()
            + self.x2.size()
            + self.y2.size()
            + self.directions.size()
            + 2
            + ::core::mem::size_of::<Card16>()
            + {
                let block_len: usize = self.devices.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card16>());
                block_len + pad
            }
    }
}
impl Request for CreatePointerBarrierRequest {
    const OPCODE: u8 = 31;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BarrierDirections {
    pub inner: u32,
}
impl BarrierDirections {
    #[inline]
    pub fn positive_x(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_positive_x(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn positive_y(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_positive_y(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn negative_x(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_negative_x(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn negative_y(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_negative_y(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn new(positive_x: bool, positive_y: bool, negative_x: bool, negative_y: bool) -> Self {
        let mut inner: u32 = 0;
        if positive_x {
            inner |= 1 << 0;
        } else {
            inner &= !(1 << 0);
        }
        if positive_y {
            inner |= 1 << 1;
        } else {
            inner &= !(1 << 1);
        }
        if negative_x {
            inner |= 1 << 2;
        } else {
            inner &= !(1 << 2);
        }
        if negative_y {
            inner |= 1 << 3;
        } else {
            inner &= !(1 << 3);
        }
        BarrierDirections { inner: inner }
    }
}
impl AsByteSequence for BarrierDirections {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((BarrierDirections { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct DeletePointerBarrierRequest {
    pub req_type: u8,
    pub barrier: Barrier,
    pub length: u16,
}
impl DeletePointerBarrierRequest {}
impl AsByteSequence for DeletePointerBarrierRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.barrier.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeletePointerBarrierRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (barrier, sz): (Barrier, usize) = <Barrier>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DeletePointerBarrierRequest {
                req_type: req_type,
                barrier: barrier,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.barrier.size() + self.length.size()
    }
}
impl Request for DeletePointerBarrierRequest {
    const OPCODE: u8 = 32;
    const EXTENSION: Option<&'static str> = Some("XFIXES");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct SelectionNotifyEvent {
    pub event_type: u8,
    pub subtype: SelectionEvent,
    pub sequence: u16,
    pub window: Window,
    pub owner: Window,
    pub selection: Atom,
    pub timestamp: Timestamp,
    pub selection_timestamp: Timestamp,
}
impl SelectionNotifyEvent {}
impl AsByteSequence for SelectionNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.subtype.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.owner.as_bytes(&mut bytes[index..]);
        index += self.selection.as_bytes(&mut bytes[index..]);
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += self.selection_timestamp.as_bytes(&mut bytes[index..]);
        index += 8;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SelectionNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (subtype, sz): (SelectionEvent, usize) = <SelectionEvent>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (owner, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (selection, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (selection_timestamp, sz): (Timestamp, usize) =
            <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 8;
        Some((
            SelectionNotifyEvent {
                event_type: event_type,
                subtype: subtype,
                sequence: sequence,
                window: window,
                owner: owner,
                selection: selection,
                timestamp: timestamp,
                selection_timestamp: selection_timestamp,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.subtype.size()
            + self.sequence.size()
            + self.window.size()
            + self.owner.size()
            + self.selection.size()
            + self.timestamp.size()
            + self.selection_timestamp.size()
            + 8
    }
}
impl crate::auto::Event for SelectionNotifyEvent {
    const OPCODE: u8 = 0;
}
#[derive(Clone, Debug, Default)]
pub struct CursorNotifyEvent {
    pub event_type: u8,
    pub subtype: Card8,
    pub sequence: u16,
    pub window: Window,
    pub cursor_serial: Card32,
    pub timestamp: Timestamp,
    pub name: Atom,
}
impl CursorNotifyEvent {}
impl AsByteSequence for CursorNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.subtype.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.cursor_serial.as_bytes(&mut bytes[index..]);
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += self.name.as_bytes(&mut bytes[index..]);
        index += 12;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CursorNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (subtype, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cursor_serial, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        Some((
            CursorNotifyEvent {
                event_type: event_type,
                subtype: subtype,
                sequence: sequence,
                window: window,
                cursor_serial: cursor_serial,
                timestamp: timestamp,
                name: name,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.subtype.size()
            + self.sequence.size()
            + self.window.size()
            + self.cursor_serial.size()
            + self.timestamp.size()
            + self.name.size()
            + 12
    }
}
impl crate::auto::Event for CursorNotifyEvent {
    const OPCODE: u8 = 1;
}
