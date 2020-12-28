// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

use super::xproto::*;
#[derive(Clone, Debug, Default)]
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
    const EXTENSION: Option<&'static str> = Some("DRI3");
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
pub struct OpenRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub provider: Card32,
}
impl OpenRequest {}
impl AsByteSequence for OpenRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.provider.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing OpenRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (provider, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            OpenRequest {
                req_type: req_type,
                length: length,
                drawable: drawable,
                provider: provider,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.drawable.size() + self.provider.size()
    }
}
impl Request for OpenRequest {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = Some("DRI3");
    const REPLY_EXPECTS_FDS: bool = true;
    type Reply = OpenReply;
}
#[derive(Clone, Debug, Default)]
pub struct OpenReply {
    pub reply_type: u8,
    pub nfd: Card8,
    pub sequence: u16,
    pub length: u32,
    pub device_fd: Vec<Fd>,
}
impl OpenReply {}
impl AsByteSequence for OpenReply {
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
        log::trace!("Deserializing OpenReply from byte buffer");
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
            OpenReply {
                reply_type: reply_type,
                nfd: nfd,
                sequence: sequence,
                length: length,
                device_fd: vec![],
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
        Some(&mut self.device_fd)
    }
}
#[derive(Clone, Debug, Default)]
pub struct PixmapFromBufferRequest {
    pub req_type: u8,
    pub pixmap: Pixmap,
    pub length: u16,
    pub drawable: Drawable,
    pub size: Card32,
    pub width: Card16,
    pub height: Card16,
    pub stride: Card16,
    pub depth: Card8,
    pub bpp: Card8,
    pub pixmap_fd: Vec<Fd>,
}
impl PixmapFromBufferRequest {}
impl AsByteSequence for PixmapFromBufferRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.pixmap.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.size.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.stride.as_bytes(&mut bytes[index..]);
        index += self.depth.as_bytes(&mut bytes[index..]);
        index += self.bpp.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PixmapFromBufferRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pixmap, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (size, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (stride, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (depth, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bpp, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PixmapFromBufferRequest {
                req_type: req_type,
                pixmap: pixmap,
                length: length,
                drawable: drawable,
                size: size,
                width: width,
                height: height,
                stride: stride,
                depth: depth,
                bpp: bpp,
                pixmap_fd: vec![],
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.pixmap.size()
            + self.length.size()
            + self.drawable.size()
            + self.size.size()
            + self.width.size()
            + self.height.size()
            + self.stride.size()
            + self.depth.size()
            + self.bpp.size()
    }
    #[inline]
    fn file_descriptors(&mut self) -> Option<&mut Vec<Fd>> {
        Some(&mut self.pixmap_fd)
    }
}
impl Request for PixmapFromBufferRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = Some("DRI3");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct BufferFromPixmapRequest {
    pub req_type: u8,
    pub pixmap: Pixmap,
    pub length: u16,
}
impl BufferFromPixmapRequest {}
impl AsByteSequence for BufferFromPixmapRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.pixmap.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing BufferFromPixmapRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pixmap, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            BufferFromPixmapRequest {
                req_type: req_type,
                pixmap: pixmap,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.pixmap.size() + self.length.size()
    }
}
impl Request for BufferFromPixmapRequest {
    const OPCODE: u8 = 3;
    const EXTENSION: Option<&'static str> = Some("DRI3");
    const REPLY_EXPECTS_FDS: bool = true;
    type Reply = BufferFromPixmapReply;
}
#[derive(Clone, Debug, Default)]
pub struct BufferFromPixmapReply {
    pub reply_type: u8,
    pub nfd: Card8,
    pub sequence: u16,
    pub length: u32,
    pub size: Card32,
    pub width: Card16,
    pub height: Card16,
    pub stride: Card16,
    pub depth: Card8,
    pub bpp: Card8,
    pub pixmap_fd: Vec<Fd>,
}
impl BufferFromPixmapReply {}
impl AsByteSequence for BufferFromPixmapReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.nfd.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.size.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.stride.as_bytes(&mut bytes[index..]);
        index += self.depth.as_bytes(&mut bytes[index..]);
        index += self.bpp.as_bytes(&mut bytes[index..]);
        index += 12;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing BufferFromPixmapReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (nfd, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (size, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (stride, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (depth, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bpp, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        Some((
            BufferFromPixmapReply {
                reply_type: reply_type,
                nfd: nfd,
                sequence: sequence,
                length: length,
                size: size,
                width: width,
                height: height,
                stride: stride,
                depth: depth,
                bpp: bpp,
                pixmap_fd: vec![],
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.nfd.size()
            + self.sequence.size()
            + self.length.size()
            + self.size.size()
            + self.width.size()
            + self.height.size()
            + self.stride.size()
            + self.depth.size()
            + self.bpp.size()
            + 12
    }
    #[inline]
    fn file_descriptors(&mut self) -> Option<&mut Vec<Fd>> {
        Some(&mut self.pixmap_fd)
    }
}
#[derive(Clone, Debug, Default)]
pub struct FenceFromFdRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub fence: Card32,
    pub initially_triggered: bool,
    pub fence_fd: Vec<Fd>,
}
impl FenceFromFdRequest {}
impl AsByteSequence for FenceFromFdRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.fence.as_bytes(&mut bytes[index..]);
        index += self.initially_triggered.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FenceFromFdRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (fence, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (initially_triggered, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            FenceFromFdRequest {
                req_type: req_type,
                length: length,
                drawable: drawable,
                fence: fence,
                initially_triggered: initially_triggered,
                fence_fd: vec![],
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
            + self.fence.size()
            + self.initially_triggered.size()
            + 3
    }
    #[inline]
    fn file_descriptors(&mut self) -> Option<&mut Vec<Fd>> {
        Some(&mut self.fence_fd)
    }
}
impl Request for FenceFromFdRequest {
    const OPCODE: u8 = 4;
    const EXTENSION: Option<&'static str> = Some("DRI3");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct FdFromFenceRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub fence: Card32,
}
impl FdFromFenceRequest {}
impl AsByteSequence for FdFromFenceRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.fence.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FdFromFenceRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (fence, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            FdFromFenceRequest {
                req_type: req_type,
                length: length,
                drawable: drawable,
                fence: fence,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.drawable.size() + self.fence.size()
    }
}
impl Request for FdFromFenceRequest {
    const OPCODE: u8 = 5;
    const EXTENSION: Option<&'static str> = Some("DRI3");
    const REPLY_EXPECTS_FDS: bool = true;
    type Reply = FdFromFenceReply;
}
#[derive(Clone, Debug, Default)]
pub struct FdFromFenceReply {
    pub reply_type: u8,
    pub nfd: Card8,
    pub sequence: u16,
    pub length: u32,
    pub fence_fd: Vec<Fd>,
}
impl FdFromFenceReply {}
impl AsByteSequence for FdFromFenceReply {
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
        log::trace!("Deserializing FdFromFenceReply from byte buffer");
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
            FdFromFenceReply {
                reply_type: reply_type,
                nfd: nfd,
                sequence: sequence,
                length: length,
                fence_fd: vec![],
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
        Some(&mut self.fence_fd)
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetSupportedModifiersRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Card32,
    pub depth: Card8,
    pub bpp: Card8,
}
impl GetSupportedModifiersRequest {}
impl AsByteSequence for GetSupportedModifiersRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.depth.as_bytes(&mut bytes[index..]);
        index += self.bpp.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetSupportedModifiersRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (depth, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bpp, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            GetSupportedModifiersRequest {
                req_type: req_type,
                length: length,
                window: window,
                depth: depth,
                bpp: bpp,
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
            + self.depth.size()
            + self.bpp.size()
            + 2
    }
}
impl Request for GetSupportedModifiersRequest {
    const OPCODE: u8 = 6;
    const EXTENSION: Option<&'static str> = Some("DRI3");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetSupportedModifiersReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetSupportedModifiersReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub window_modifiers: Vec<Card64>,
    pub screen_modifiers: Vec<Card64>,
}
impl GetSupportedModifiersReply {}
impl AsByteSequence for GetSupportedModifiersReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.window_modifiers.len() as Card32).as_bytes(&mut bytes[index..]);
        index += (self.screen_modifiers.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 16;
        let block_len: usize = vector_as_bytes(&self.window_modifiers, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card64>());
        let block_len: usize = vector_as_bytes(&self.screen_modifiers, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card64>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetSupportedModifiersReply from byte buffer");
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
        let (window_modifiers, block_len): (Vec<Card64>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card64>());
        let (screen_modifiers, block_len): (Vec<Card64>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card64>());
        Some((
            GetSupportedModifiersReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                window_modifiers: window_modifiers,
                screen_modifiers: screen_modifiers,
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
                let block_len: usize = self.window_modifiers.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card64>());
                block_len + pad
            }
            + {
                let block_len: usize = self.screen_modifiers.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card64>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct PixmapFromBuffersRequest {
    pub req_type: u8,
    pub pixmap: Pixmap,
    pub length: u16,
    pub window: Window,
    pub width: Card16,
    pub height: Card16,
    pub stride0: Card32,
    pub offset0: Card32,
    pub stride1: Card32,
    pub offset1: Card32,
    pub stride2: Card32,
    pub offset2: Card32,
    pub stride3: Card32,
    pub offset3: Card32,
    pub depth: Card8,
    pub bpp: Card8,
    pub modifier: Card64,
    pub buffers: Vec<Fd>,
}
impl PixmapFromBuffersRequest {}
impl AsByteSequence for PixmapFromBuffersRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.pixmap.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += (self.buffers.len() as Card8).as_bytes(&mut bytes[index..]);
        index += 3;
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.stride0.as_bytes(&mut bytes[index..]);
        index += self.offset0.as_bytes(&mut bytes[index..]);
        index += self.stride1.as_bytes(&mut bytes[index..]);
        index += self.offset1.as_bytes(&mut bytes[index..]);
        index += self.stride2.as_bytes(&mut bytes[index..]);
        index += self.offset2.as_bytes(&mut bytes[index..]);
        index += self.stride3.as_bytes(&mut bytes[index..]);
        index += self.offset3.as_bytes(&mut bytes[index..]);
        index += self.depth.as_bytes(&mut bytes[index..]);
        index += self.bpp.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.modifier.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.buffers, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fd>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PixmapFromBuffersRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pixmap, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (stride0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (offset0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (stride1, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (offset1, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (stride2, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (offset2, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (stride3, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (offset3, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (depth, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bpp, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (modifier, sz): (Card64, usize) = <Card64>::from_bytes(&bytes[index..])?;
        index += sz;
        let (buffers, block_len): (Vec<Fd>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fd>());
        Some((
            PixmapFromBuffersRequest {
                req_type: req_type,
                pixmap: pixmap,
                length: length,
                window: window,
                width: width,
                height: height,
                stride0: stride0,
                offset0: offset0,
                stride1: stride1,
                offset1: offset1,
                stride2: stride2,
                offset2: offset2,
                stride3: stride3,
                offset3: offset3,
                depth: depth,
                bpp: bpp,
                modifier: modifier,
                buffers: buffers,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.pixmap.size()
            + self.length.size()
            + self.window.size()
            + ::core::mem::size_of::<Card8>()
            + 3
            + self.width.size()
            + self.height.size()
            + self.stride0.size()
            + self.offset0.size()
            + self.stride1.size()
            + self.offset1.size()
            + self.stride2.size()
            + self.offset2.size()
            + self.stride3.size()
            + self.offset3.size()
            + self.depth.size()
            + self.bpp.size()
            + 2
            + self.modifier.size()
            + {
                let block_len: usize = self.buffers.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fd>());
                block_len + pad
            }
    }
}
impl Request for PixmapFromBuffersRequest {
    const OPCODE: u8 = 7;
    const EXTENSION: Option<&'static str> = Some("DRI3");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct BuffersFromPixmapRequest {
    pub req_type: u8,
    pub pixmap: Pixmap,
    pub length: u16,
}
impl BuffersFromPixmapRequest {}
impl AsByteSequence for BuffersFromPixmapRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.pixmap.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing BuffersFromPixmapRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pixmap, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            BuffersFromPixmapRequest {
                req_type: req_type,
                pixmap: pixmap,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.pixmap.size() + self.length.size()
    }
}
impl Request for BuffersFromPixmapRequest {
    const OPCODE: u8 = 8;
    const EXTENSION: Option<&'static str> = Some("DRI3");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = BuffersFromPixmapReply;
}
#[derive(Clone, Debug, Default)]
pub struct BuffersFromPixmapReply {
    pub reply_type: u8,
    pub nfd: Card8,
    pub sequence: u16,
    pub length: u32,
    pub width: Card16,
    pub height: Card16,
    pub modifier: Card64,
    pub depth: Card8,
    pub bpp: Card8,
    pub strides: Vec<Card32>,
    pub offsets: Vec<Card32>,
    pub buffers: Vec<Fd>,
}
impl BuffersFromPixmapReply {}
impl AsByteSequence for BuffersFromPixmapReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.nfd.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += 4;
        index += self.modifier.as_bytes(&mut bytes[index..]);
        index += self.depth.as_bytes(&mut bytes[index..]);
        index += self.bpp.as_bytes(&mut bytes[index..]);
        index += 6;
        let block_len: usize = vector_as_bytes(&self.strides, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.offsets, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.buffers, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fd>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing BuffersFromPixmapReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (nfd, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (modifier, sz): (Card64, usize) = <Card64>::from_bytes(&bytes[index..])?;
        index += sz;
        let (depth, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bpp, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 6;
        let (strides, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], (nfd as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (offsets, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], (nfd as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (buffers, block_len): (Vec<Fd>, usize) =
            vector_from_bytes(&bytes[index..], (nfd as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fd>());
        Some((
            BuffersFromPixmapReply {
                reply_type: reply_type,
                nfd: nfd,
                sequence: sequence,
                length: length,
                width: width,
                height: height,
                modifier: modifier,
                depth: depth,
                bpp: bpp,
                strides: strides,
                offsets: offsets,
                buffers: buffers,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.nfd.size()
            + self.sequence.size()
            + self.length.size()
            + self.width.size()
            + self.height.size()
            + 4
            + self.modifier.size()
            + self.depth.size()
            + self.bpp.size()
            + 6
            + {
                let block_len: usize = self.strides.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.offsets.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.buffers.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fd>());
                block_len + pad
            }
    }
}
