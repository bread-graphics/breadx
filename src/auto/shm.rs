// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

use super::xproto::*;
#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Seg {
    pub xid: XID,
}
impl Seg {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Seg {
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
    const EXTENSION: Option<&'static str> = Some("MIT-SHM");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryVersionReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryVersionReply {
    pub reply_type: u8,
    pub shared_pixmaps: bool,
    pub sequence: u16,
    pub length: u32,
    pub major_version: Card16,
    pub minor_version: Card16,
    pub uid: Card16,
    pub gid: Card16,
    pub pixmap_format: Card8,
}
impl QueryVersionReply {}
impl AsByteSequence for QueryVersionReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.shared_pixmaps.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.major_version.as_bytes(&mut bytes[index..]);
        index += self.minor_version.as_bytes(&mut bytes[index..]);
        index += self.uid.as_bytes(&mut bytes[index..]);
        index += self.gid.as_bytes(&mut bytes[index..]);
        index += self.pixmap_format.as_bytes(&mut bytes[index..]);
        index += 15;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryVersionReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (shared_pixmaps, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (uid, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gid, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pixmap_format, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 15;
        Some((
            QueryVersionReply {
                reply_type: reply_type,
                shared_pixmaps: shared_pixmaps,
                sequence: sequence,
                length: length,
                major_version: major_version,
                minor_version: minor_version,
                uid: uid,
                gid: gid,
                pixmap_format: pixmap_format,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.shared_pixmaps.size()
            + self.sequence.size()
            + self.length.size()
            + self.major_version.size()
            + self.minor_version.size()
            + self.uid.size()
            + self.gid.size()
            + self.pixmap_format.size()
            + 15
    }
}
#[derive(Clone, Debug, Default)]
pub struct AttachRequest {
    pub req_type: u8,
    pub shmseg: Seg,
    pub length: u16,
    pub shmid: Card32,
    pub read_only: bool,
}
impl AttachRequest {}
impl AsByteSequence for AttachRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.shmseg.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.shmid.as_bytes(&mut bytes[index..]);
        index += self.read_only.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AttachRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (shmseg, sz): (Seg, usize) = <Seg>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (shmid, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (read_only, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            AttachRequest {
                req_type: req_type,
                shmseg: shmseg,
                length: length,
                shmid: shmid,
                read_only: read_only,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.shmseg.size()
            + self.length.size()
            + self.shmid.size()
            + self.read_only.size()
            + 3
    }
}
impl Request for AttachRequest {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = Some("MIT-SHM");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct DetachRequest {
    pub req_type: u8,
    pub shmseg: Seg,
    pub length: u16,
}
impl DetachRequest {}
impl AsByteSequence for DetachRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.shmseg.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DetachRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (shmseg, sz): (Seg, usize) = <Seg>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DetachRequest {
                req_type: req_type,
                shmseg: shmseg,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.shmseg.size() + self.length.size()
    }
}
impl Request for DetachRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = Some("MIT-SHM");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct PutImageRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub total_width: Card16,
    pub total_height: Card16,
    pub src_x: Card16,
    pub src_y: Card16,
    pub src_width: Card16,
    pub src_height: Card16,
    pub dst_x: Int16,
    pub dst_y: Int16,
    pub depth: Card8,
    pub format: Card8,
    pub send_event: bool,
    pub shmseg: Seg,
    pub offset: Card32,
}
impl PutImageRequest {}
impl AsByteSequence for PutImageRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        index += self.total_width.as_bytes(&mut bytes[index..]);
        index += self.total_height.as_bytes(&mut bytes[index..]);
        index += self.src_x.as_bytes(&mut bytes[index..]);
        index += self.src_y.as_bytes(&mut bytes[index..]);
        index += self.src_width.as_bytes(&mut bytes[index..]);
        index += self.src_height.as_bytes(&mut bytes[index..]);
        index += self.dst_x.as_bytes(&mut bytes[index..]);
        index += self.dst_y.as_bytes(&mut bytes[index..]);
        index += self.depth.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index += self.send_event.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.shmseg.as_bytes(&mut bytes[index..]);
        index += self.offset.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PutImageRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (total_width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (total_height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_x, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_y, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (depth, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (send_event, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (shmseg, sz): (Seg, usize) = <Seg>::from_bytes(&bytes[index..])?;
        index += sz;
        let (offset, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PutImageRequest {
                req_type: req_type,
                length: length,
                drawable: drawable,
                gc: gc,
                total_width: total_width,
                total_height: total_height,
                src_x: src_x,
                src_y: src_y,
                src_width: src_width,
                src_height: src_height,
                dst_x: dst_x,
                dst_y: dst_y,
                depth: depth,
                format: format,
                send_event: send_event,
                shmseg: shmseg,
                offset: offset,
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
            + self.gc.size()
            + self.total_width.size()
            + self.total_height.size()
            + self.src_x.size()
            + self.src_y.size()
            + self.src_width.size()
            + self.src_height.size()
            + self.dst_x.size()
            + self.dst_y.size()
            + self.depth.size()
            + self.format.size()
            + self.send_event.size()
            + 1
            + self.shmseg.size()
            + self.offset.size()
    }
}
impl Request for PutImageRequest {
    const OPCODE: u8 = 3;
    const EXTENSION: Option<&'static str> = Some("MIT-SHM");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GetImageRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub x: Int16,
    pub y: Int16,
    pub width: Card16,
    pub height: Card16,
    pub plane_mask: Card32,
    pub format: Card8,
    pub shmseg: Seg,
    pub offset: Card32,
}
impl GetImageRequest {}
impl AsByteSequence for GetImageRequest {
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
        index += self.plane_mask.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index += 3;
        index += self.shmseg.as_bytes(&mut bytes[index..]);
        index += self.offset.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetImageRequest from byte buffer");
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
        let (plane_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (shmseg, sz): (Seg, usize) = <Seg>::from_bytes(&bytes[index..])?;
        index += sz;
        let (offset, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetImageRequest {
                req_type: req_type,
                length: length,
                drawable: drawable,
                x: x,
                y: y,
                width: width,
                height: height,
                plane_mask: plane_mask,
                format: format,
                shmseg: shmseg,
                offset: offset,
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
            + self.plane_mask.size()
            + self.format.size()
            + 3
            + self.shmseg.size()
            + self.offset.size()
    }
}
impl Request for GetImageRequest {
    const OPCODE: u8 = 4;
    const EXTENSION: Option<&'static str> = Some("MIT-SHM");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetImageReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetImageReply {
    pub reply_type: u8,
    pub depth: Card8,
    pub sequence: u16,
    pub length: u32,
    pub visual: Visualid,
    pub size: Card32,
}
impl GetImageReply {}
impl AsByteSequence for GetImageReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.depth.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.visual.as_bytes(&mut bytes[index..]);
        index += self.size.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetImageReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (depth, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (visual, sz): (Visualid, usize) = <Visualid>::from_bytes(&bytes[index..])?;
        index += sz;
        let (size, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetImageReply {
                reply_type: reply_type,
                depth: depth,
                sequence: sequence,
                length: length,
                visual: visual,
                size: size,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.depth.size()
            + self.sequence.size()
            + self.length.size()
            + self.visual.size()
            + self.size.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct CreatePixmapRequest {
    pub req_type: u8,
    pub pid: Pixmap,
    pub length: u16,
    pub drawable: Drawable,
    pub width: Card16,
    pub height: Card16,
    pub depth: Card8,
    pub shmseg: Seg,
    pub offset: Card32,
}
impl CreatePixmapRequest {}
impl AsByteSequence for CreatePixmapRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.pid.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.depth.as_bytes(&mut bytes[index..]);
        index += 3;
        index += self.shmseg.as_bytes(&mut bytes[index..]);
        index += self.offset.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreatePixmapRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pid, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (depth, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (shmseg, sz): (Seg, usize) = <Seg>::from_bytes(&bytes[index..])?;
        index += sz;
        let (offset, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CreatePixmapRequest {
                req_type: req_type,
                pid: pid,
                length: length,
                drawable: drawable,
                width: width,
                height: height,
                depth: depth,
                shmseg: shmseg,
                offset: offset,
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
            + self.width.size()
            + self.height.size()
            + self.depth.size()
            + 3
            + self.shmseg.size()
            + self.offset.size()
    }
}
impl Request for CreatePixmapRequest {
    const OPCODE: u8 = 5;
    const EXTENSION: Option<&'static str> = Some("MIT-SHM");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct AttachFdRequest {
    pub req_type: u8,
    pub shmseg: Seg,
    pub length: u16,
    pub read_only: bool,
    pub shm_fd: Vec<Fd>,
}
impl AttachFdRequest {}
impl AsByteSequence for AttachFdRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.shmseg.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.read_only.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AttachFdRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (shmseg, sz): (Seg, usize) = <Seg>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (read_only, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            AttachFdRequest {
                req_type: req_type,
                shmseg: shmseg,
                length: length,
                read_only: read_only,
                shm_fd: vec![],
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.shmseg.size() + self.length.size() + self.read_only.size() + 3
    }
    #[inline]
    fn file_descriptors(&mut self) -> Option<&mut Vec<Fd>> {
        Some(&mut self.shm_fd)
    }
}
impl Request for AttachFdRequest {
    const OPCODE: u8 = 6;
    const EXTENSION: Option<&'static str> = Some("MIT-SHM");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CreateSegmentRequest {
    pub req_type: u8,
    pub shmseg: Seg,
    pub length: u16,
    pub size: Card32,
    pub read_only: bool,
}
impl CreateSegmentRequest {}
impl AsByteSequence for CreateSegmentRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.shmseg.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.size.as_bytes(&mut bytes[index..]);
        index += self.read_only.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateSegmentRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (shmseg, sz): (Seg, usize) = <Seg>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (size, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (read_only, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            CreateSegmentRequest {
                req_type: req_type,
                shmseg: shmseg,
                length: length,
                size: size,
                read_only: read_only,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.shmseg.size()
            + self.length.size()
            + self.size.size()
            + self.read_only.size()
            + 3
    }
}
impl Request for CreateSegmentRequest {
    const OPCODE: u8 = 7;
    const EXTENSION: Option<&'static str> = Some("MIT-SHM");
    const REPLY_EXPECTS_FDS: bool = true;
    type Reply = CreateSegmentReply;
}
#[derive(Clone, Debug, Default)]
pub struct CreateSegmentReply {
    pub reply_type: u8,
    pub nfd: Card8,
    pub sequence: u16,
    pub length: u32,
    pub shm_fd: Vec<Fd>,
}
impl CreateSegmentReply {}
impl AsByteSequence for CreateSegmentReply {
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
        log::trace!("Deserializing CreateSegmentReply from byte buffer");
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
            CreateSegmentReply {
                reply_type: reply_type,
                nfd: nfd,
                sequence: sequence,
                length: length,
                shm_fd: vec![],
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
        Some(&mut self.shm_fd)
    }
}
#[derive(Clone, Debug, Default)]
pub struct CompletionEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub drawable: Drawable,
    pub minor_event: Card16,
    pub major_event: Byte,
    pub shmseg: Seg,
    pub offset: Card32,
}
impl CompletionEvent {}
impl AsByteSequence for CompletionEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.minor_event.as_bytes(&mut bytes[index..]);
        index += self.major_event.as_bytes(&mut bytes[index..]);
        index += self.shmseg.as_bytes(&mut bytes[index..]);
        index += self.offset.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CompletionEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_event, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_event, sz): (Byte, usize) = <Byte>::from_bytes(&bytes[index..])?;
        index += sz;
        let (shmseg, sz): (Seg, usize) = <Seg>::from_bytes(&bytes[index..])?;
        index += sz;
        let (offset, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CompletionEvent {
                event_type: event_type,
                sequence: sequence,
                drawable: drawable,
                minor_event: minor_event,
                major_event: major_event,
                shmseg: shmseg,
                offset: offset,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + 1
            + self.sequence.size()
            + self.drawable.size()
            + self.minor_event.size()
            + self.major_event.size()
            + self.shmseg.size()
            + self.offset.size()
    }
}
impl crate::auto::Event for CompletionEvent {
    const OPCODE: u8 = 0;
}
