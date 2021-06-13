// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

use super::xv::*;
#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Context {
    pub xid: XID,
}
impl Context {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Context {
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
pub struct Surface {
    pub xid: XID,
}
impl Surface {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Surface {
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
pub struct Subpicture {
    pub xid: XID,
}
impl Subpicture {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Subpicture {
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
pub struct SurfaceInfo {
    pub id: Surface,
    pub chroma_format: Card16,
    pub pad0: Card16,
    pub max_width: Card16,
    pub max_height: Card16,
    pub subpicture_max_width: Card16,
    pub subpicture_max_height: Card16,
    pub mc_type: Card32,
    pub flags: Card32,
}
impl SurfaceInfo {}
impl AsByteSequence for SurfaceInfo {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.id.as_bytes(&mut bytes[index..]);
        index += self.chroma_format.as_bytes(&mut bytes[index..]);
        index += self.pad0.as_bytes(&mut bytes[index..]);
        index += self.max_width.as_bytes(&mut bytes[index..]);
        index += self.max_height.as_bytes(&mut bytes[index..]);
        index += self.subpicture_max_width.as_bytes(&mut bytes[index..]);
        index += self.subpicture_max_height.as_bytes(&mut bytes[index..]);
        index += self.mc_type.as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SurfaceInfo from byte buffer");
        let (id, sz): (Surface, usize) = <Surface>::from_bytes(&bytes[index..])?;
        index += sz;
        let (chroma_format, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pad0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (subpicture_max_width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (subpicture_max_height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mc_type, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SurfaceInfo {
                id: id,
                chroma_format: chroma_format,
                pad0: pad0,
                max_width: max_width,
                max_height: max_height,
                subpicture_max_width: subpicture_max_width,
                subpicture_max_height: subpicture_max_height,
                mc_type: mc_type,
                flags: flags,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.id.size()
            + self.chroma_format.size()
            + self.pad0.size()
            + self.max_width.size()
            + self.max_height.size()
            + self.subpicture_max_width.size()
            + self.subpicture_max_height.size()
            + self.mc_type.size()
            + self.flags.size()
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
    const EXTENSION: Option<&'static str> = Some("XVideo-MotionCompensation");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryVersionReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryVersionReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major: Card32,
    pub minor: Card32,
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
        index += self.major.as_bytes(&mut bytes[index..]);
        index += self.minor.as_bytes(&mut bytes[index..]);
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
        let (major, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryVersionReply {
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
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ListSurfaceTypesRequest {
    pub req_type: u8,
    pub length: u16,
    pub port_id: Port,
}
impl ListSurfaceTypesRequest {}
impl AsByteSequence for ListSurfaceTypesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.port_id.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListSurfaceTypesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (port_id, sz): (Port, usize) = <Port>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ListSurfaceTypesRequest {
                req_type: req_type,
                length: length,
                port_id: port_id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.port_id.size()
    }
}
impl Request for ListSurfaceTypesRequest {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = Some("XVideo-MotionCompensation");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ListSurfaceTypesReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ListSurfaceTypesReply<'a> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub surfaces: Cow<'a, [SurfaceInfo]>,
}
impl<'a> ListSurfaceTypesReply<'a> {}
impl<'a> AsByteSequence for ListSurfaceTypesReply<'a> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.surfaces.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.surfaces, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<SurfaceInfo>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListSurfaceTypesReply from byte buffer");
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
        let (surfaces, block_len): (Cow<'_, [SurfaceInfo]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<SurfaceInfo>());
        Some((
            ListSurfaceTypesReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                surfaces: surfaces,
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
                let block_len: usize = self.surfaces.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<SurfaceInfo>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CreateContextRequest {
    pub req_type: u8,
    pub length: u16,
    pub context_id: Context,
    pub port_id: Port,
    pub surface_id: Surface,
    pub width: Card16,
    pub height: Card16,
    pub flags: Card32,
}
impl CreateContextRequest {}
impl AsByteSequence for CreateContextRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.context_id.as_bytes(&mut bytes[index..]);
        index += self.port_id.as_bytes(&mut bytes[index..]);
        index += self.surface_id.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_id, sz): (Context, usize) = <Context>::from_bytes(&bytes[index..])?;
        index += sz;
        let (port_id, sz): (Port, usize) = <Port>::from_bytes(&bytes[index..])?;
        index += sz;
        let (surface_id, sz): (Surface, usize) = <Surface>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CreateContextRequest {
                req_type: req_type,
                length: length,
                context_id: context_id,
                port_id: port_id,
                surface_id: surface_id,
                width: width,
                height: height,
                flags: flags,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.context_id.size()
            + self.port_id.size()
            + self.surface_id.size()
            + self.width.size()
            + self.height.size()
            + self.flags.size()
    }
}
impl Request for CreateContextRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = Some("XVideo-MotionCompensation");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = CreateContextReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CreateContextReply<'b> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width_actual: Card16,
    pub height_actual: Card16,
    pub flags_return: Card32,
    pub priv_data: Cow<'b, [Card32]>,
}
impl<'b> CreateContextReply<'b> {}
impl<'b> AsByteSequence for CreateContextReply<'b> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.width_actual.as_bytes(&mut bytes[index..]);
        index += self.height_actual.as_bytes(&mut bytes[index..]);
        index += self.flags_return.as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.priv_data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateContextReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width_actual, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height_actual, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags_return, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        let (priv_data, block_len): (Cow<'_, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], (length as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            CreateContextReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                width_actual: width_actual,
                height_actual: height_actual,
                flags_return: flags_return,
                priv_data: priv_data,
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
            + self.width_actual.size()
            + self.height_actual.size()
            + self.flags_return.size()
            + 20
            + {
                let block_len: usize = self.priv_data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DestroyContextRequest {
    pub req_type: u8,
    pub length: u16,
    pub context_id: Context,
}
impl DestroyContextRequest {}
impl AsByteSequence for DestroyContextRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.context_id.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DestroyContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_id, sz): (Context, usize) = <Context>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DestroyContextRequest {
                req_type: req_type,
                length: length,
                context_id: context_id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.context_id.size()
    }
}
impl Request for DestroyContextRequest {
    const OPCODE: u8 = 3;
    const EXTENSION: Option<&'static str> = Some("XVideo-MotionCompensation");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CreateSurfaceRequest {
    pub req_type: u8,
    pub length: u16,
    pub surface_id: Surface,
    pub context_id: Context,
}
impl CreateSurfaceRequest {}
impl AsByteSequence for CreateSurfaceRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.surface_id.as_bytes(&mut bytes[index..]);
        index += self.context_id.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateSurfaceRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (surface_id, sz): (Surface, usize) = <Surface>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_id, sz): (Context, usize) = <Context>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CreateSurfaceRequest {
                req_type: req_type,
                length: length,
                surface_id: surface_id,
                context_id: context_id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.surface_id.size()
            + self.context_id.size()
    }
}
impl Request for CreateSurfaceRequest {
    const OPCODE: u8 = 4;
    const EXTENSION: Option<&'static str> = Some("XVideo-MotionCompensation");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = CreateSurfaceReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CreateSurfaceReply<'c> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub priv_data: Cow<'c, [Card32]>,
}
impl<'c> CreateSurfaceReply<'c> {}
impl<'c> AsByteSequence for CreateSurfaceReply<'c> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 24;
        let block_len: usize = vector_as_bytes(&self.priv_data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateSurfaceReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 24;
        let (priv_data, block_len): (Cow<'_, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], (length as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            CreateSurfaceReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                priv_data: priv_data,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + 1 + self.sequence.size() + self.length.size() + 24 + {
            let block_len: usize = self.priv_data.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
            block_len + pad
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DestroySurfaceRequest {
    pub req_type: u8,
    pub length: u16,
    pub surface_id: Surface,
}
impl DestroySurfaceRequest {}
impl AsByteSequence for DestroySurfaceRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.surface_id.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DestroySurfaceRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (surface_id, sz): (Surface, usize) = <Surface>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DestroySurfaceRequest {
                req_type: req_type,
                length: length,
                surface_id: surface_id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.surface_id.size()
    }
}
impl Request for DestroySurfaceRequest {
    const OPCODE: u8 = 5;
    const EXTENSION: Option<&'static str> = Some("XVideo-MotionCompensation");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CreateSubpictureRequest {
    pub req_type: u8,
    pub length: u16,
    pub subpicture_id: Subpicture,
    pub context: Context,
    pub xvimage_id: Card32,
    pub width: Card16,
    pub height: Card16,
}
impl CreateSubpictureRequest {}
impl AsByteSequence for CreateSubpictureRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.subpicture_id.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index += self.xvimage_id.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateSubpictureRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (subpicture_id, sz): (Subpicture, usize) = <Subpicture>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (Context, usize) = <Context>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xvimage_id, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CreateSubpictureRequest {
                req_type: req_type,
                length: length,
                subpicture_id: subpicture_id,
                context: context,
                xvimage_id: xvimage_id,
                width: width,
                height: height,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.subpicture_id.size()
            + self.context.size()
            + self.xvimage_id.size()
            + self.width.size()
            + self.height.size()
    }
}
impl Request for CreateSubpictureRequest {
    const OPCODE: u8 = 6;
    const EXTENSION: Option<&'static str> = Some("XVideo-MotionCompensation");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = CreateSubpictureReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CreateSubpictureReply<'d> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width_actual: Card16,
    pub height_actual: Card16,
    pub num_palette_entries: Card16,
    pub entry_bytes: Card16,
    pub component_order: [Card8; 4],
    pub priv_data: Cow<'d, [Card32]>,
}
impl<'d> CreateSubpictureReply<'d> {}
impl<'d> AsByteSequence for CreateSubpictureReply<'d> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.width_actual.as_bytes(&mut bytes[index..]);
        index += self.height_actual.as_bytes(&mut bytes[index..]);
        index += self.num_palette_entries.as_bytes(&mut bytes[index..]);
        index += self.entry_bytes.as_bytes(&mut bytes[index..]);
        index += self.component_order.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.priv_data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateSubpictureReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width_actual, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height_actual, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_palette_entries, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (entry_bytes, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (component_order, sz): ([Card8; 4], usize) = <[Card8; 4]>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (priv_data, block_len): (Cow<'_, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], (length as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            CreateSubpictureReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                width_actual: width_actual,
                height_actual: height_actual,
                num_palette_entries: num_palette_entries,
                entry_bytes: entry_bytes,
                component_order: component_order,
                priv_data: priv_data,
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
            + self.width_actual.size()
            + self.height_actual.size()
            + self.num_palette_entries.size()
            + self.entry_bytes.size()
            + self.component_order.size()
            + 12
            + {
                let block_len: usize = self.priv_data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DestroySubpictureRequest {
    pub req_type: u8,
    pub length: u16,
    pub subpicture_id: Subpicture,
}
impl DestroySubpictureRequest {}
impl AsByteSequence for DestroySubpictureRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.subpicture_id.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DestroySubpictureRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (subpicture_id, sz): (Subpicture, usize) = <Subpicture>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DestroySubpictureRequest {
                req_type: req_type,
                length: length,
                subpicture_id: subpicture_id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.subpicture_id.size()
    }
}
impl Request for DestroySubpictureRequest {
    const OPCODE: u8 = 7;
    const EXTENSION: Option<&'static str> = Some("XVideo-MotionCompensation");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ListSubpictureTypesRequest {
    pub req_type: u8,
    pub length: u16,
    pub port_id: Port,
    pub surface_id: Surface,
}
impl ListSubpictureTypesRequest {}
impl AsByteSequence for ListSubpictureTypesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.port_id.as_bytes(&mut bytes[index..]);
        index += self.surface_id.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListSubpictureTypesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (port_id, sz): (Port, usize) = <Port>::from_bytes(&bytes[index..])?;
        index += sz;
        let (surface_id, sz): (Surface, usize) = <Surface>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ListSubpictureTypesRequest {
                req_type: req_type,
                length: length,
                port_id: port_id,
                surface_id: surface_id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.port_id.size() + self.surface_id.size()
    }
}
impl Request for ListSubpictureTypesRequest {
    const OPCODE: u8 = 8;
    const EXTENSION: Option<&'static str> = Some("XVideo-MotionCompensation");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ListSubpictureTypesReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ListSubpictureTypesReply<'e> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub types: Cow<'e, [ImageFormatInfo]>,
}
impl<'e> ListSubpictureTypesReply<'e> {}
impl<'e> AsByteSequence for ListSubpictureTypesReply<'e> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.types.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.types, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ImageFormatInfo>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListSubpictureTypesReply from byte buffer");
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
        let (types, block_len): (Cow<'_, [ImageFormatInfo]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ImageFormatInfo>());
        Some((
            ListSubpictureTypesReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                types: types,
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
                let block_len: usize = self.types.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<ImageFormatInfo>());
                block_len + pad
            }
    }
}
