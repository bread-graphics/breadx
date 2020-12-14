// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

use super::xfixes::*;
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
    const EXTENSION: Option<&'static str> = Some("Composite");
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
pub struct RedirectWindowRequest {
    pub req_type: u8,
    pub window: Window,
    pub length: u16,
    pub update: Redirect,
}
impl RedirectWindowRequest {}
impl AsByteSequence for RedirectWindowRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.update.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing RedirectWindowRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (update, sz): (Redirect, usize) = <Redirect>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            RedirectWindowRequest {
                req_type: req_type,
                window: window,
                length: length,
                update: update,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.window.size() + self.length.size() + self.update.size() + 3
    }
}
impl Request for RedirectWindowRequest {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = Some("Composite");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Redirect {
    Automatic = 0,
    Manual = 1,
}
impl AsByteSequence for Redirect {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Automatic, sz)),
            1 => Some((Self::Manual, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for Redirect {
    #[inline]
    fn default() -> Redirect {
        Redirect::Automatic
    }
}
#[derive(Clone, Debug, Default)]
pub struct RedirectSubwindowsRequest {
    pub req_type: u8,
    pub window: Window,
    pub length: u16,
    pub update: Redirect,
}
impl RedirectSubwindowsRequest {}
impl AsByteSequence for RedirectSubwindowsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.update.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing RedirectSubwindowsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (update, sz): (Redirect, usize) = <Redirect>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            RedirectSubwindowsRequest {
                req_type: req_type,
                window: window,
                length: length,
                update: update,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.window.size() + self.length.size() + self.update.size() + 3
    }
}
impl Request for RedirectSubwindowsRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = Some("Composite");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct UnredirectWindowRequest {
    pub req_type: u8,
    pub window: Window,
    pub length: u16,
    pub update: Redirect,
}
impl UnredirectWindowRequest {}
impl AsByteSequence for UnredirectWindowRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.update.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing UnredirectWindowRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (update, sz): (Redirect, usize) = <Redirect>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            UnredirectWindowRequest {
                req_type: req_type,
                window: window,
                length: length,
                update: update,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.window.size() + self.length.size() + self.update.size() + 3
    }
}
impl Request for UnredirectWindowRequest {
    const OPCODE: u8 = 3;
    const EXTENSION: Option<&'static str> = Some("Composite");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct UnredirectSubwindowsRequest {
    pub req_type: u8,
    pub window: Window,
    pub length: u16,
    pub update: Redirect,
}
impl UnredirectSubwindowsRequest {}
impl AsByteSequence for UnredirectSubwindowsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.update.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing UnredirectSubwindowsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (update, sz): (Redirect, usize) = <Redirect>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            UnredirectSubwindowsRequest {
                req_type: req_type,
                window: window,
                length: length,
                update: update,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.window.size() + self.length.size() + self.update.size() + 3
    }
}
impl Request for UnredirectSubwindowsRequest {
    const OPCODE: u8 = 4;
    const EXTENSION: Option<&'static str> = Some("Composite");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CreateRegionFromBorderClipRequest {
    pub req_type: u8,
    pub region: Region,
    pub length: u16,
    pub window: Window,
}
impl CreateRegionFromBorderClipRequest {}
impl AsByteSequence for CreateRegionFromBorderClipRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.region.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateRegionFromBorderClipRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (region, sz): (Region, usize) = <Region>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CreateRegionFromBorderClipRequest {
                req_type: req_type,
                region: region,
                length: length,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.region.size() + self.length.size() + self.window.size()
    }
}
impl Request for CreateRegionFromBorderClipRequest {
    const OPCODE: u8 = 5;
    const EXTENSION: Option<&'static str> = Some("Composite");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct NameWindowPixmapRequest {
    pub req_type: u8,
    pub window: Window,
    pub length: u16,
    pub pixmap: Pixmap,
}
impl NameWindowPixmapRequest {}
impl AsByteSequence for NameWindowPixmapRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.pixmap.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing NameWindowPixmapRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pixmap, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            NameWindowPixmapRequest {
                req_type: req_type,
                window: window,
                length: length,
                pixmap: pixmap,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.window.size() + self.length.size() + self.pixmap.size()
    }
}
impl Request for NameWindowPixmapRequest {
    const OPCODE: u8 = 6;
    const EXTENSION: Option<&'static str> = Some("Composite");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GetOverlayWindowRequest {
    pub req_type: u8,
    pub window: Window,
    pub length: u16,
}
impl GetOverlayWindowRequest {}
impl AsByteSequence for GetOverlayWindowRequest {
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
        log::trace!("Deserializing GetOverlayWindowRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetOverlayWindowRequest {
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
impl Request for GetOverlayWindowRequest {
    const OPCODE: u8 = 7;
    const EXTENSION: Option<&'static str> = Some("Composite");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetOverlayWindowReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetOverlayWindowReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub overlay_win: Window,
}
impl GetOverlayWindowReply {}
impl AsByteSequence for GetOverlayWindowReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.overlay_win.as_bytes(&mut bytes[index..]);
        index += 20;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetOverlayWindowReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (overlay_win, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        Some((
            GetOverlayWindowReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                overlay_win: overlay_win,
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
            + self.overlay_win.size()
            + 20
    }
}
#[derive(Clone, Debug, Default)]
pub struct ReleaseOverlayWindowRequest {
    pub req_type: u8,
    pub window: Window,
    pub length: u16,
}
impl ReleaseOverlayWindowRequest {}
impl AsByteSequence for ReleaseOverlayWindowRequest {
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
        log::trace!("Deserializing ReleaseOverlayWindowRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ReleaseOverlayWindowRequest {
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
impl Request for ReleaseOverlayWindowRequest {
    const OPCODE: u8 = 8;
    const EXTENSION: Option<&'static str> = Some("Composite");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
