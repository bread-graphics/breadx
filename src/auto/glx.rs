// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

use super::xproto::*;
#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pixmap {
    pub xid: XID,
}
impl Pixmap {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Pixmap {
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
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pbuffer {
    pub xid: XID,
}
impl Pbuffer {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Pbuffer {
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
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Window {
    pub xid: XID,
}
impl Window {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Window {
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
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fbconfig {
    pub xid: XID,
}
impl Fbconfig {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Fbconfig {
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
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Drawable {
    pub xid: XID,
}
impl Drawable {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Drawable {
    #[inline]
    fn xid(&self) -> XID {
        self.xid
    }
    #[inline]
    fn from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl From<super::xproto::Window> for Drawable {
    #[inline]
    fn from(base: super::xproto::Window) -> Self {
        <Drawable>::const_from_xid(base.xid)
    }
}
impl From<Pbuffer> for Drawable {
    #[inline]
    fn from(base: Pbuffer) -> Self {
        <Drawable>::const_from_xid(base.xid)
    }
}
impl From<super::glx::Pixmap> for Drawable {
    #[inline]
    fn from(base: super::glx::Pixmap) -> Self {
        <Drawable>::const_from_xid(base.xid)
    }
}
impl From<super::glx::Window> for Drawable {
    #[inline]
    fn from(base: super::glx::Window) -> Self {
        <Drawable>::const_from_xid(base.xid)
    }
}
pub type Float32 = Float;
pub type Float64 = Double;
pub type Bool32 = Card32;
pub type ContextTag = Card32;
#[derive(Clone, Debug, Default)]
pub struct RenderRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub data: Vec<Byte>,
}
impl RenderRequest {}
impl AsByteSequence for RenderRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing RenderRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (data, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            RenderRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                data: data,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size() + {
            let block_len: usize = self.data.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
            block_len + pad
        }
    }
}
impl Request for RenderRequest {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct RenderLargeRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub request_num: Card16,
    pub request_total: Card16,
    pub data: Vec<Byte>,
}
impl RenderLargeRequest {}
impl AsByteSequence for RenderLargeRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.request_num.as_bytes(&mut bytes[index..]);
        index += self.request_total.as_bytes(&mut bytes[index..]);
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing RenderLargeRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (request_num, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (request_total, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (data, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            RenderLargeRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                request_num: request_num,
                request_total: request_total,
                data: data,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.request_num.size()
            + self.request_total.size()
            + ::core::mem::size_of::<Card32>()
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
    }
}
impl Request for RenderLargeRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CreateContextRequest {
    pub req_type: u8,
    pub context: super::glx::Context,
    pub length: u16,
    pub visual: Visualid,
    pub screen: Card32,
    pub share_list: super::glx::Context,
    pub is_direct: bool,
}
impl CreateContextRequest {}
impl AsByteSequence for CreateContextRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.visual.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.share_list.as_bytes(&mut bytes[index..]);
        index += self.is_direct.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (super::glx::Context, usize) =
            <super::glx::Context>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (visual, sz): (Visualid, usize) = <Visualid>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (share_list, sz): (super::glx::Context, usize) =
            <super::glx::Context>::from_bytes(&bytes[index..])?;
        index += sz;
        let (is_direct, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            CreateContextRequest {
                req_type: req_type,
                context: context,
                length: length,
                visual: visual,
                screen: screen,
                share_list: share_list,
                is_direct: is_direct,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context.size()
            + self.length.size()
            + self.visual.size()
            + self.screen.size()
            + self.share_list.size()
            + self.is_direct.size()
            + 3
    }
}
impl Request for CreateContextRequest {
    const OPCODE: u8 = 3;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct DestroyContextRequest {
    pub req_type: u8,
    pub context: super::glx::Context,
    pub length: u16,
}
impl DestroyContextRequest {}
impl AsByteSequence for DestroyContextRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DestroyContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (super::glx::Context, usize) =
            <super::glx::Context>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DestroyContextRequest {
                req_type: req_type,
                context: context,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context.size() + self.length.size()
    }
}
impl Request for DestroyContextRequest {
    const OPCODE: u8 = 4;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct MakeCurrentRequest {
    pub req_type: u8,
    pub drawable: super::glx::Drawable,
    pub length: u16,
    pub context: super::glx::Context,
    pub old_context_tag: ContextTag,
}
impl MakeCurrentRequest {}
impl AsByteSequence for MakeCurrentRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index += self.old_context_tag.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing MakeCurrentRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (super::glx::Drawable, usize) =
            <super::glx::Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (super::glx::Context, usize) =
            <super::glx::Context>::from_bytes(&bytes[index..])?;
        index += sz;
        let (old_context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            MakeCurrentRequest {
                req_type: req_type,
                drawable: drawable,
                length: length,
                context: context,
                old_context_tag: old_context_tag,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.drawable.size()
            + self.length.size()
            + self.context.size()
            + self.old_context_tag.size()
    }
}
impl Request for MakeCurrentRequest {
    const OPCODE: u8 = 5;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = MakeCurrentReply;
}
#[derive(Clone, Debug, Default)]
pub struct MakeCurrentReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_tag: ContextTag,
}
impl MakeCurrentReply {}
impl AsByteSequence for MakeCurrentReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += 20;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing MakeCurrentReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        Some((
            MakeCurrentReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                context_tag: context_tag,
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
            + self.context_tag.size()
            + 20
    }
}
#[derive(Clone, Debug, Default)]
pub struct IsDirectRequest {
    pub req_type: u8,
    pub context: super::glx::Context,
    pub length: u16,
}
impl IsDirectRequest {}
impl AsByteSequence for IsDirectRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing IsDirectRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (super::glx::Context, usize) =
            <super::glx::Context>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            IsDirectRequest {
                req_type: req_type,
                context: context,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context.size() + self.length.size()
    }
}
impl Request for IsDirectRequest {
    const OPCODE: u8 = 6;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = IsDirectReply;
}
#[derive(Clone, Debug, Default)]
pub struct IsDirectReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub is_direct: bool,
}
impl IsDirectReply {}
impl AsByteSequence for IsDirectReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.is_direct.as_bytes(&mut bytes[index..]);
        index += 23;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing IsDirectReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (is_direct, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 23;
        Some((
            IsDirectReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                is_direct: is_direct,
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
            + self.is_direct.size()
            + 23
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
    const OPCODE: u8 = 7;
    const EXTENSION: Option<&'static str> = Some("GLX");
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
pub struct WaitGlRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
}
impl WaitGlRequest {}
impl AsByteSequence for WaitGlRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing WaitGlRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            WaitGlRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size()
    }
}
impl Request for WaitGlRequest {
    const OPCODE: u8 = 8;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct WaitXRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
}
impl WaitXRequest {}
impl AsByteSequence for WaitXRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing WaitXRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            WaitXRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size()
    }
}
impl Request for WaitXRequest {
    const OPCODE: u8 = 9;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CopyContextRequest {
    pub req_type: u8,
    pub src: super::glx::Context,
    pub length: u16,
    pub dest: super::glx::Context,
    pub mask: Card32,
    pub src_context_tag: ContextTag,
}
impl CopyContextRequest {}
impl AsByteSequence for CopyContextRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.src.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.dest.as_bytes(&mut bytes[index..]);
        index += self.mask.as_bytes(&mut bytes[index..]);
        index += self.src_context_tag.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CopyContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src, sz): (super::glx::Context, usize) =
            <super::glx::Context>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dest, sz): (super::glx::Context, usize) =
            <super::glx::Context>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CopyContextRequest {
                req_type: req_type,
                src: src,
                length: length,
                dest: dest,
                mask: mask,
                src_context_tag: src_context_tag,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.src.size()
            + self.length.size()
            + self.dest.size()
            + self.mask.size()
            + self.src_context_tag.size()
    }
}
impl Request for CopyContextRequest {
    const OPCODE: u8 = 10;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct SwapBuffersRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub drawable: super::glx::Drawable,
}
impl SwapBuffersRequest {}
impl AsByteSequence for SwapBuffersRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SwapBuffersRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (super::glx::Drawable, usize) =
            <super::glx::Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SwapBuffersRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                drawable: drawable,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size() + self.drawable.size()
    }
}
impl Request for SwapBuffersRequest {
    const OPCODE: u8 = 11;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct UseXFontRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub font: Font,
    pub first: Card32,
    pub count: Card32,
    pub list_base: Card32,
}
impl UseXFontRequest {}
impl AsByteSequence for UseXFontRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.font.as_bytes(&mut bytes[index..]);
        index += self.first.as_bytes(&mut bytes[index..]);
        index += self.count.as_bytes(&mut bytes[index..]);
        index += self.list_base.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing UseXFontRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (font, sz): (Font, usize) = <Font>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (count, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (list_base, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            UseXFontRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                font: font,
                first: first,
                count: count,
                list_base: list_base,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.font.size()
            + self.first.size()
            + self.count.size()
            + self.list_base.size()
    }
}
impl Request for UseXFontRequest {
    const OPCODE: u8 = 12;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CreateGlxPixmapRequest {
    pub req_type: u8,
    pub screen: Card32,
    pub length: u16,
    pub visual: Visualid,
    pub pixmap: super::xproto::Pixmap,
    pub glx_pixmap: super::glx::Pixmap,
}
impl CreateGlxPixmapRequest {}
impl AsByteSequence for CreateGlxPixmapRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.visual.as_bytes(&mut bytes[index..]);
        index += self.pixmap.as_bytes(&mut bytes[index..]);
        index += self.glx_pixmap.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateGlxPixmapRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (visual, sz): (Visualid, usize) = <Visualid>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pixmap, sz): (super::xproto::Pixmap, usize) =
            <super::xproto::Pixmap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (glx_pixmap, sz): (super::glx::Pixmap, usize) =
            <super::glx::Pixmap>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CreateGlxPixmapRequest {
                req_type: req_type,
                screen: screen,
                length: length,
                visual: visual,
                pixmap: pixmap,
                glx_pixmap: glx_pixmap,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.screen.size()
            + self.length.size()
            + self.visual.size()
            + self.pixmap.size()
            + self.glx_pixmap.size()
    }
}
impl Request for CreateGlxPixmapRequest {
    const OPCODE: u8 = 13;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GetVisualConfigsRequest {
    pub req_type: u8,
    pub screen: Card32,
    pub length: u16,
}
impl GetVisualConfigsRequest {}
impl AsByteSequence for GetVisualConfigsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetVisualConfigsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetVisualConfigsRequest {
                req_type: req_type,
                screen: screen,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.screen.size() + self.length.size()
    }
}
impl Request for GetVisualConfigsRequest {
    const OPCODE: u8 = 14;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetVisualConfigsReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetVisualConfigsReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub num_visuals: Card32,
    pub num_properties: Card32,
    pub property_list: Vec<Card32>,
}
impl GetVisualConfigsReply {}
impl AsByteSequence for GetVisualConfigsReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += (self.property_list.len() as u32).as_bytes(&mut bytes[index..]);
        index += self.num_visuals.as_bytes(&mut bytes[index..]);
        index += self.num_properties.as_bytes(&mut bytes[index..]);
        index += 16;
        let block_len: usize = vector_as_bytes(&self.property_list, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetVisualConfigsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_visuals, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_properties, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 16;
        let (property_list, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            GetVisualConfigsReply {
                reply_type: reply_type,
                sequence: sequence,
                num_visuals: num_visuals,
                num_properties: num_properties,
                property_list: property_list,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + ::core::mem::size_of::<u32>()
            + self.num_visuals.size()
            + self.num_properties.size()
            + 16
            + {
                let block_len: usize = self.property_list.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct DestroyGlxPixmapRequest {
    pub req_type: u8,
    pub glx_pixmap: super::glx::Pixmap,
    pub length: u16,
}
impl DestroyGlxPixmapRequest {}
impl AsByteSequence for DestroyGlxPixmapRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.glx_pixmap.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DestroyGlxPixmapRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (glx_pixmap, sz): (super::glx::Pixmap, usize) =
            <super::glx::Pixmap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DestroyGlxPixmapRequest {
                req_type: req_type,
                glx_pixmap: glx_pixmap,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.glx_pixmap.size() + self.length.size()
    }
}
impl Request for DestroyGlxPixmapRequest {
    const OPCODE: u8 = 15;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct VendorPrivateRequest {
    pub req_type: u8,
    pub vendor_code: Card32,
    pub length: u16,
    pub context_tag: ContextTag,
    pub data: Vec<Byte>,
}
impl VendorPrivateRequest {}
impl AsByteSequence for VendorPrivateRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.vendor_code.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing VendorPrivateRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vendor_code, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (data, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            VendorPrivateRequest {
                req_type: req_type,
                vendor_code: vendor_code,
                length: length,
                context_tag: context_tag,
                data: data,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.vendor_code.size()
            + self.length.size()
            + self.context_tag.size()
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
    }
}
impl Request for VendorPrivateRequest {
    const OPCODE: u8 = 16;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct VendorPrivateWithReplyRequest {
    pub req_type: u8,
    pub vendor_code: Card32,
    pub length: u16,
    pub context_tag: ContextTag,
    pub data: Vec<Byte>,
}
impl VendorPrivateWithReplyRequest {}
impl AsByteSequence for VendorPrivateWithReplyRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.vendor_code.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing VendorPrivateWithReplyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vendor_code, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (data, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            VendorPrivateWithReplyRequest {
                req_type: req_type,
                vendor_code: vendor_code,
                length: length,
                context_tag: context_tag,
                data: data,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.vendor_code.size()
            + self.length.size()
            + self.context_tag.size()
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
    }
}
impl Request for VendorPrivateWithReplyRequest {
    const OPCODE: u8 = 17;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = VendorPrivateWithReplyReply;
}
#[derive(Clone, Debug, Default)]
pub struct VendorPrivateWithReplyReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub retval: Card32,
    pub data1: [Byte; 24],
    pub data2: Vec<Byte>,
}
impl VendorPrivateWithReplyReply {}
impl AsByteSequence for VendorPrivateWithReplyReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.retval.as_bytes(&mut bytes[index..]);
        index += self.data1.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.data2, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing VendorPrivateWithReplyReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (retval, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (data1, sz): ([Byte; 24], usize) = <[Byte; 24]>::from_bytes(&bytes[index..])?;
        index += sz;
        let (data2, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize) * (4)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            VendorPrivateWithReplyReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                retval: retval,
                data1: data1,
                data2: data2,
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
            + self.retval.size()
            + self.data1.size()
            + {
                let block_len: usize = self.data2.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct QueryExtensionsStringRequest {
    pub req_type: u8,
    pub screen: Card32,
    pub length: u16,
}
impl QueryExtensionsStringRequest {}
impl AsByteSequence for QueryExtensionsStringRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryExtensionsStringRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryExtensionsStringRequest {
                req_type: req_type,
                screen: screen,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.screen.size() + self.length.size()
    }
}
impl Request for QueryExtensionsStringRequest {
    const OPCODE: u8 = 18;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryExtensionsStringReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryExtensionsStringReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub n: Card32,
}
impl QueryExtensionsStringReply {}
impl AsByteSequence for QueryExtensionsStringReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += self.n.as_bytes(&mut bytes[index..]);
        index += 16;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryExtensionsStringReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (n, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 16;
        Some((
            QueryExtensionsStringReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                n: n,
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
            + 4
            + self.n.size()
            + 16
    }
}
#[derive(Clone, Debug, Default)]
pub struct QueryServerStringRequest {
    pub req_type: u8,
    pub screen: Card32,
    pub length: u16,
    pub name: Card32,
}
impl QueryServerStringRequest {}
impl AsByteSequence for QueryServerStringRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.name.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryServerStringRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (name, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryServerStringRequest {
                req_type: req_type,
                screen: screen,
                length: length,
                name: name,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.screen.size() + self.length.size() + self.name.size()
    }
}
impl Request for QueryServerStringRequest {
    const OPCODE: u8 = 19;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryServerStringReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryServerStringReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub string: String,
}
impl QueryServerStringReply {}
impl AsByteSequence for QueryServerStringReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.string.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 16;
        let block_len: usize = string_as_bytes(&self.string, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryServerStringReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 16;
        let (string, block_len): (String, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            QueryServerStringReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                string: string,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + 16
            + {
                let block_len: usize = self.string.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct ClientInfoRequest {
    pub req_type: u8,
    pub major_version: Card32,
    pub length: u16,
    pub minor_version: Card32,
    pub string: String,
}
impl ClientInfoRequest {}
impl AsByteSequence for ClientInfoRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.major_version.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.minor_version.as_bytes(&mut bytes[index..]);
        index += (self.string.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = string_as_bytes(&self.string, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ClientInfoRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_version, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_version, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (string, block_len): (String, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            ClientInfoRequest {
                req_type: req_type,
                major_version: major_version,
                length: length,
                minor_version: minor_version,
                string: string,
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
            + ::core::mem::size_of::<Card32>()
            + {
                let block_len: usize = self.string.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
impl Request for ClientInfoRequest {
    const OPCODE: u8 = 20;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GetFbConfigsRequest {
    pub req_type: u8,
    pub screen: Card32,
    pub length: u16,
}
impl GetFbConfigsRequest {}
impl AsByteSequence for GetFbConfigsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetFbConfigsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetFbConfigsRequest {
                req_type: req_type,
                screen: screen,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.screen.size() + self.length.size()
    }
}
impl Request for GetFbConfigsRequest {
    const OPCODE: u8 = 21;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetFbConfigsReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetFbConfigsReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub num_fb_configs: Card32,
    pub num_properties: Card32,
    pub property_list: Vec<Card32>,
}
impl GetFbConfigsReply {}
impl AsByteSequence for GetFbConfigsReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += (self.property_list.len() as u32).as_bytes(&mut bytes[index..]);
        index += self.num_fb_configs.as_bytes(&mut bytes[index..]);
        index += self.num_properties.as_bytes(&mut bytes[index..]);
        index += 16;
        let block_len: usize = vector_as_bytes(&self.property_list, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetFbConfigsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_fb_configs, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_properties, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 16;
        let (property_list, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            GetFbConfigsReply {
                reply_type: reply_type,
                sequence: sequence,
                num_fb_configs: num_fb_configs,
                num_properties: num_properties,
                property_list: property_list,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + ::core::mem::size_of::<u32>()
            + self.num_fb_configs.size()
            + self.num_properties.size()
            + 16
            + {
                let block_len: usize = self.property_list.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct CreatePixmapRequest {
    pub req_type: u8,
    pub screen: Card32,
    pub length: u16,
    pub fbconfig: Fbconfig,
    pub pixmap: super::xproto::Pixmap,
    pub glx_pixmap: super::glx::Pixmap,
    pub num_attribs: Card32,
    pub attribs: Vec<Card32>,
}
impl CreatePixmapRequest {}
impl AsByteSequence for CreatePixmapRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.fbconfig.as_bytes(&mut bytes[index..]);
        index += self.pixmap.as_bytes(&mut bytes[index..]);
        index += self.glx_pixmap.as_bytes(&mut bytes[index..]);
        index += self.num_attribs.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.attribs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreatePixmapRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (fbconfig, sz): (Fbconfig, usize) = <Fbconfig>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pixmap, sz): (super::xproto::Pixmap, usize) =
            <super::xproto::Pixmap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (glx_pixmap, sz): (super::glx::Pixmap, usize) =
            <super::glx::Pixmap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_attribs, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (attribs, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], ((num_attribs as usize) * (2)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            CreatePixmapRequest {
                req_type: req_type,
                screen: screen,
                length: length,
                fbconfig: fbconfig,
                pixmap: pixmap,
                glx_pixmap: glx_pixmap,
                num_attribs: num_attribs,
                attribs: attribs,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.screen.size()
            + self.length.size()
            + self.fbconfig.size()
            + self.pixmap.size()
            + self.glx_pixmap.size()
            + self.num_attribs.size()
            + {
                let block_len: usize = self.attribs.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
impl Request for CreatePixmapRequest {
    const OPCODE: u8 = 22;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct DestroyPixmapRequest {
    pub req_type: u8,
    pub glx_pixmap: super::glx::Pixmap,
    pub length: u16,
}
impl DestroyPixmapRequest {}
impl AsByteSequence for DestroyPixmapRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.glx_pixmap.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DestroyPixmapRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (glx_pixmap, sz): (super::glx::Pixmap, usize) =
            <super::glx::Pixmap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DestroyPixmapRequest {
                req_type: req_type,
                glx_pixmap: glx_pixmap,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.glx_pixmap.size() + self.length.size()
    }
}
impl Request for DestroyPixmapRequest {
    const OPCODE: u8 = 23;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CreateNewContextRequest {
    pub req_type: u8,
    pub context: super::glx::Context,
    pub length: u16,
    pub fbconfig: Fbconfig,
    pub screen: Card32,
    pub render_type: Card32,
    pub share_list: super::glx::Context,
    pub is_direct: bool,
}
impl CreateNewContextRequest {}
impl AsByteSequence for CreateNewContextRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.fbconfig.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.render_type.as_bytes(&mut bytes[index..]);
        index += self.share_list.as_bytes(&mut bytes[index..]);
        index += self.is_direct.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateNewContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (super::glx::Context, usize) =
            <super::glx::Context>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (fbconfig, sz): (Fbconfig, usize) = <Fbconfig>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (render_type, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (share_list, sz): (super::glx::Context, usize) =
            <super::glx::Context>::from_bytes(&bytes[index..])?;
        index += sz;
        let (is_direct, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            CreateNewContextRequest {
                req_type: req_type,
                context: context,
                length: length,
                fbconfig: fbconfig,
                screen: screen,
                render_type: render_type,
                share_list: share_list,
                is_direct: is_direct,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context.size()
            + self.length.size()
            + self.fbconfig.size()
            + self.screen.size()
            + self.render_type.size()
            + self.share_list.size()
            + self.is_direct.size()
            + 3
    }
}
impl Request for CreateNewContextRequest {
    const OPCODE: u8 = 24;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct QueryContextRequest {
    pub req_type: u8,
    pub context: super::glx::Context,
    pub length: u16,
}
impl QueryContextRequest {}
impl AsByteSequence for QueryContextRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (super::glx::Context, usize) =
            <super::glx::Context>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryContextRequest {
                req_type: req_type,
                context: context,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context.size() + self.length.size()
    }
}
impl Request for QueryContextRequest {
    const OPCODE: u8 = 25;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryContextReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryContextReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_attribs: Card32,
    pub attribs: Vec<Card32>,
}
impl QueryContextReply {}
impl AsByteSequence for QueryContextReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.num_attribs.as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.attribs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryContextReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_attribs, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        let (attribs, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], ((num_attribs as usize) * (2)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            QueryContextReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                num_attribs: num_attribs,
                attribs: attribs,
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
            + self.num_attribs.size()
            + 20
            + {
                let block_len: usize = self.attribs.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct MakeContextCurrentRequest {
    pub req_type: u8,
    pub old_context_tag: ContextTag,
    pub length: u16,
    pub drawable: super::glx::Drawable,
    pub read_drawable: super::glx::Drawable,
    pub context: super::glx::Context,
}
impl MakeContextCurrentRequest {}
impl AsByteSequence for MakeContextCurrentRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.old_context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.read_drawable.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing MakeContextCurrentRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (old_context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (super::glx::Drawable, usize) =
            <super::glx::Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (read_drawable, sz): (super::glx::Drawable, usize) =
            <super::glx::Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (super::glx::Context, usize) =
            <super::glx::Context>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            MakeContextCurrentRequest {
                req_type: req_type,
                old_context_tag: old_context_tag,
                length: length,
                drawable: drawable,
                read_drawable: read_drawable,
                context: context,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.old_context_tag.size()
            + self.length.size()
            + self.drawable.size()
            + self.read_drawable.size()
            + self.context.size()
    }
}
impl Request for MakeContextCurrentRequest {
    const OPCODE: u8 = 26;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = MakeContextCurrentReply;
}
#[derive(Clone, Debug, Default)]
pub struct MakeContextCurrentReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_tag: ContextTag,
}
impl MakeContextCurrentReply {}
impl AsByteSequence for MakeContextCurrentReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += 20;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing MakeContextCurrentReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        Some((
            MakeContextCurrentReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                context_tag: context_tag,
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
            + self.context_tag.size()
            + 20
    }
}
#[derive(Clone, Debug, Default)]
pub struct CreatePbufferRequest {
    pub req_type: u8,
    pub screen: Card32,
    pub length: u16,
    pub fbconfig: Fbconfig,
    pub pbuffer: Pbuffer,
    pub num_attribs: Card32,
    pub attribs: Vec<Card32>,
}
impl CreatePbufferRequest {}
impl AsByteSequence for CreatePbufferRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.fbconfig.as_bytes(&mut bytes[index..]);
        index += self.pbuffer.as_bytes(&mut bytes[index..]);
        index += self.num_attribs.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.attribs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreatePbufferRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (fbconfig, sz): (Fbconfig, usize) = <Fbconfig>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pbuffer, sz): (Pbuffer, usize) = <Pbuffer>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_attribs, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (attribs, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], ((num_attribs as usize) * (2)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            CreatePbufferRequest {
                req_type: req_type,
                screen: screen,
                length: length,
                fbconfig: fbconfig,
                pbuffer: pbuffer,
                num_attribs: num_attribs,
                attribs: attribs,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.screen.size()
            + self.length.size()
            + self.fbconfig.size()
            + self.pbuffer.size()
            + self.num_attribs.size()
            + {
                let block_len: usize = self.attribs.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
impl Request for CreatePbufferRequest {
    const OPCODE: u8 = 27;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct DestroyPbufferRequest {
    pub req_type: u8,
    pub pbuffer: Pbuffer,
    pub length: u16,
}
impl DestroyPbufferRequest {}
impl AsByteSequence for DestroyPbufferRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.pbuffer.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DestroyPbufferRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pbuffer, sz): (Pbuffer, usize) = <Pbuffer>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DestroyPbufferRequest {
                req_type: req_type,
                pbuffer: pbuffer,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.pbuffer.size() + self.length.size()
    }
}
impl Request for DestroyPbufferRequest {
    const OPCODE: u8 = 28;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GetDrawableAttributesRequest {
    pub req_type: u8,
    pub drawable: super::glx::Drawable,
    pub length: u16,
}
impl GetDrawableAttributesRequest {}
impl AsByteSequence for GetDrawableAttributesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDrawableAttributesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (super::glx::Drawable, usize) =
            <super::glx::Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetDrawableAttributesRequest {
                req_type: req_type,
                drawable: drawable,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.drawable.size() + self.length.size()
    }
}
impl Request for GetDrawableAttributesRequest {
    const OPCODE: u8 = 29;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetDrawableAttributesReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetDrawableAttributesReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_attribs: Card32,
    pub attribs: Vec<Card32>,
}
impl GetDrawableAttributesReply {}
impl AsByteSequence for GetDrawableAttributesReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.num_attribs.as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.attribs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDrawableAttributesReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_attribs, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        let (attribs, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], ((num_attribs as usize) * (2)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            GetDrawableAttributesReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                num_attribs: num_attribs,
                attribs: attribs,
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
            + self.num_attribs.size()
            + 20
            + {
                let block_len: usize = self.attribs.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct ChangeDrawableAttributesRequest {
    pub req_type: u8,
    pub drawable: super::glx::Drawable,
    pub length: u16,
    pub num_attribs: Card32,
    pub attribs: Vec<Card32>,
}
impl ChangeDrawableAttributesRequest {}
impl AsByteSequence for ChangeDrawableAttributesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.num_attribs.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.attribs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangeDrawableAttributesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (super::glx::Drawable, usize) =
            <super::glx::Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_attribs, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (attribs, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], ((num_attribs as usize) * (2)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            ChangeDrawableAttributesRequest {
                req_type: req_type,
                drawable: drawable,
                length: length,
                num_attribs: num_attribs,
                attribs: attribs,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.drawable.size()
            + self.length.size()
            + self.num_attribs.size()
            + {
                let block_len: usize = self.attribs.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
impl Request for ChangeDrawableAttributesRequest {
    const OPCODE: u8 = 30;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CreateWindowRequest {
    pub req_type: u8,
    pub screen: Card32,
    pub length: u16,
    pub fbconfig: Fbconfig,
    pub window: super::xproto::Window,
    pub glx_window: super::glx::Window,
    pub num_attribs: Card32,
    pub attribs: Vec<Card32>,
}
impl CreateWindowRequest {}
impl AsByteSequence for CreateWindowRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.fbconfig.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.glx_window.as_bytes(&mut bytes[index..]);
        index += self.num_attribs.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.attribs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateWindowRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (fbconfig, sz): (Fbconfig, usize) = <Fbconfig>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (super::xproto::Window, usize) =
            <super::xproto::Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (glx_window, sz): (super::glx::Window, usize) =
            <super::glx::Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_attribs, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (attribs, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], ((num_attribs as usize) * (2)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            CreateWindowRequest {
                req_type: req_type,
                screen: screen,
                length: length,
                fbconfig: fbconfig,
                window: window,
                glx_window: glx_window,
                num_attribs: num_attribs,
                attribs: attribs,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.screen.size()
            + self.length.size()
            + self.fbconfig.size()
            + self.window.size()
            + self.glx_window.size()
            + self.num_attribs.size()
            + {
                let block_len: usize = self.attribs.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
impl Request for CreateWindowRequest {
    const OPCODE: u8 = 31;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct DeleteWindowRequest {
    pub req_type: u8,
    pub glxwindow: super::glx::Window,
    pub length: u16,
}
impl DeleteWindowRequest {}
impl AsByteSequence for DeleteWindowRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.glxwindow.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeleteWindowRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (glxwindow, sz): (super::glx::Window, usize) =
            <super::glx::Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DeleteWindowRequest {
                req_type: req_type,
                glxwindow: glxwindow,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.glxwindow.size() + self.length.size()
    }
}
impl Request for DeleteWindowRequest {
    const OPCODE: u8 = 32;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct SetClientInfoArbRequest {
    pub req_type: u8,
    pub major_version: Card32,
    pub length: u16,
    pub minor_version: Card32,
    pub num_versions: Card32,
    pub gl_versions: Vec<Card32>,
    pub gl_extension_string: String,
    pub glx_extension_string: String,
}
impl SetClientInfoArbRequest {}
impl AsByteSequence for SetClientInfoArbRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.major_version.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.minor_version.as_bytes(&mut bytes[index..]);
        index += self.num_versions.as_bytes(&mut bytes[index..]);
        index += (self.gl_extension_string.len() as Card32).as_bytes(&mut bytes[index..]);
        index += (self.glx_extension_string.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.gl_versions, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = string_as_bytes(&self.gl_extension_string, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        let block_len: usize = string_as_bytes(&self.glx_extension_string, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetClientInfoArbRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_version, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_version, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_versions, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gl_versions, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], ((num_versions as usize) * (2)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (gl_extension_string, block_len): (String, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        let (glx_extension_string, block_len): (String, usize) =
            string_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            SetClientInfoArbRequest {
                req_type: req_type,
                major_version: major_version,
                length: length,
                minor_version: minor_version,
                num_versions: num_versions,
                gl_versions: gl_versions,
                gl_extension_string: gl_extension_string,
                glx_extension_string: glx_extension_string,
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
            + self.num_versions.size()
            + ::core::mem::size_of::<Card32>()
            + ::core::mem::size_of::<Card32>()
            + {
                let block_len: usize = self.gl_versions.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.gl_extension_string.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
            + {
                let block_len: usize = self.glx_extension_string.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
impl Request for SetClientInfoArbRequest {
    const OPCODE: u8 = 33;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CreateContextAttribsArbRequest {
    pub req_type: u8,
    pub context: super::glx::Context,
    pub length: u16,
    pub fbconfig: Fbconfig,
    pub screen: Card32,
    pub share_list: super::glx::Context,
    pub is_direct: bool,
    pub num_attribs: Card32,
    pub attribs: Vec<Card32>,
}
impl CreateContextAttribsArbRequest {}
impl AsByteSequence for CreateContextAttribsArbRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.fbconfig.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.share_list.as_bytes(&mut bytes[index..]);
        index += self.is_direct.as_bytes(&mut bytes[index..]);
        index += 3;
        index += self.num_attribs.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.attribs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateContextAttribsArbRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (super::glx::Context, usize) =
            <super::glx::Context>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (fbconfig, sz): (Fbconfig, usize) = <Fbconfig>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (share_list, sz): (super::glx::Context, usize) =
            <super::glx::Context>::from_bytes(&bytes[index..])?;
        index += sz;
        let (is_direct, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (num_attribs, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (attribs, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], ((num_attribs as usize) * (2)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            CreateContextAttribsArbRequest {
                req_type: req_type,
                context: context,
                length: length,
                fbconfig: fbconfig,
                screen: screen,
                share_list: share_list,
                is_direct: is_direct,
                num_attribs: num_attribs,
                attribs: attribs,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context.size()
            + self.length.size()
            + self.fbconfig.size()
            + self.screen.size()
            + self.share_list.size()
            + self.is_direct.size()
            + 3
            + self.num_attribs.size()
            + {
                let block_len: usize = self.attribs.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
impl Request for CreateContextAttribsArbRequest {
    const OPCODE: u8 = 34;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct SetClientInfo2ArbRequest {
    pub req_type: u8,
    pub major_version: Card32,
    pub length: u16,
    pub minor_version: Card32,
    pub num_versions: Card32,
    pub gl_versions: Vec<Card32>,
    pub gl_extension_string: String,
    pub glx_extension_string: String,
}
impl SetClientInfo2ArbRequest {}
impl AsByteSequence for SetClientInfo2ArbRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.major_version.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.minor_version.as_bytes(&mut bytes[index..]);
        index += self.num_versions.as_bytes(&mut bytes[index..]);
        index += (self.gl_extension_string.len() as Card32).as_bytes(&mut bytes[index..]);
        index += (self.glx_extension_string.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.gl_versions, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = string_as_bytes(&self.gl_extension_string, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        let block_len: usize = string_as_bytes(&self.glx_extension_string, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetClientInfo2ArbRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_version, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_version, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_versions, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gl_versions, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], ((num_versions as usize) * (3)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (gl_extension_string, block_len): (String, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        let (glx_extension_string, block_len): (String, usize) =
            string_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            SetClientInfo2ArbRequest {
                req_type: req_type,
                major_version: major_version,
                length: length,
                minor_version: minor_version,
                num_versions: num_versions,
                gl_versions: gl_versions,
                gl_extension_string: gl_extension_string,
                glx_extension_string: glx_extension_string,
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
            + self.num_versions.size()
            + ::core::mem::size_of::<Card32>()
            + ::core::mem::size_of::<Card32>()
            + {
                let block_len: usize = self.gl_versions.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.gl_extension_string.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
            + {
                let block_len: usize = self.glx_extension_string.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
impl Request for SetClientInfo2ArbRequest {
    const OPCODE: u8 = 35;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct NewListRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub list: Card32,
    pub mode: Card32,
}
impl NewListRequest {}
impl AsByteSequence for NewListRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.list.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing NewListRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (list, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            NewListRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                list: list,
                mode: mode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.list.size()
            + self.mode.size()
    }
}
impl Request for NewListRequest {
    const OPCODE: u8 = 101;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct EndListRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
}
impl EndListRequest {}
impl AsByteSequence for EndListRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing EndListRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            EndListRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size()
    }
}
impl Request for EndListRequest {
    const OPCODE: u8 = 102;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct DeleteListsRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub list: Card32,
    pub range: Int32,
}
impl DeleteListsRequest {}
impl AsByteSequence for DeleteListsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.list.as_bytes(&mut bytes[index..]);
        index += self.range.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeleteListsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (list, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (range, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DeleteListsRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                list: list,
                range: range,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.list.size()
            + self.range.size()
    }
}
impl Request for DeleteListsRequest {
    const OPCODE: u8 = 103;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GenListsRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub range: Int32,
}
impl GenListsRequest {}
impl AsByteSequence for GenListsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.range.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GenListsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (range, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GenListsRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                range: range,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size() + self.range.size()
    }
}
impl Request for GenListsRequest {
    const OPCODE: u8 = 104;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GenListsReply;
}
#[derive(Clone, Debug, Default)]
pub struct GenListsReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: Card32,
}
impl GenListsReply {}
impl AsByteSequence for GenListsReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.ret_val.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GenListsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ret_val, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GenListsReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                ret_val: ret_val,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + 1 + self.sequence.size() + self.length.size() + self.ret_val.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct FeedbackBufferRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub size: Int32,
    pub ty: Int32,
}
impl FeedbackBufferRequest {}
impl AsByteSequence for FeedbackBufferRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.size.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FeedbackBufferRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (size, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            FeedbackBufferRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                size: size,
                ty: ty,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.size.size()
            + self.ty.size()
    }
}
impl Request for FeedbackBufferRequest {
    const OPCODE: u8 = 105;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct SelectBufferRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub size: Int32,
}
impl SelectBufferRequest {}
impl AsByteSequence for SelectBufferRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.size.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SelectBufferRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (size, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SelectBufferRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                size: size,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size() + self.size.size()
    }
}
impl Request for SelectBufferRequest {
    const OPCODE: u8 = 106;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct RenderModeRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub mode: Card32,
}
impl RenderModeRequest {}
impl AsByteSequence for RenderModeRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing RenderModeRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            RenderModeRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                mode: mode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size() + self.mode.size()
    }
}
impl Request for RenderModeRequest {
    const OPCODE: u8 = 107;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = RenderModeReply;
}
#[derive(Clone, Debug, Default)]
pub struct RenderModeReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: Card32,
    pub new_mode: Card32,
    pub data: Vec<Card32>,
}
impl RenderModeReply {}
impl AsByteSequence for RenderModeReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.ret_val.as_bytes(&mut bytes[index..]);
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.new_mode.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing RenderModeReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ret_val, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (new_mode, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            RenderModeReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                ret_val: ret_val,
                new_mode: new_mode,
                data: data,
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
            + self.ret_val.size()
            + ::core::mem::size_of::<Card32>()
            + self.new_mode.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct FinishRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
}
impl FinishRequest {}
impl AsByteSequence for FinishRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FinishRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            FinishRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size()
    }
}
impl Request for FinishRequest {
    const OPCODE: u8 = 108;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = FinishReply;
}
#[derive(Clone, Debug, Default)]
pub struct FinishReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
}
impl FinishReply {}
impl AsByteSequence for FinishReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FinishReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            FinishReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + 1 + self.sequence.size() + self.length.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct PixelStorefRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub pname: Card32,
    pub datum: Float32,
}
impl PixelStorefRequest {}
impl AsByteSequence for PixelStorefRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PixelStorefRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Float32, usize) = <Float32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PixelStorefRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                pname: pname,
                datum: datum,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.pname.size()
            + self.datum.size()
    }
}
impl Request for PixelStorefRequest {
    const OPCODE: u8 = 109;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct PixelStoreiRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub pname: Card32,
    pub datum: Int32,
}
impl PixelStoreiRequest {}
impl AsByteSequence for PixelStoreiRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PixelStoreiRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PixelStoreiRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                pname: pname,
                datum: datum,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.pname.size()
            + self.datum.size()
    }
}
impl Request for PixelStoreiRequest {
    const OPCODE: u8 = 110;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct ReadPixelsRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub x: Int32,
    pub y: Int32,
    pub width: Int32,
    pub height: Int32,
    pub format: Card32,
    pub ty: Card32,
    pub swap_bytes: bool,
    pub lsb_first: bool,
}
impl ReadPixelsRequest {}
impl AsByteSequence for ReadPixelsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.swap_bytes.as_bytes(&mut bytes[index..]);
        index += self.lsb_first.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ReadPixelsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (swap_bytes, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (lsb_first, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ReadPixelsRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                x: x,
                y: y,
                width: width,
                height: height,
                format: format,
                ty: ty,
                swap_bytes: swap_bytes,
                lsb_first: lsb_first,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.x.size()
            + self.y.size()
            + self.width.size()
            + self.height.size()
            + self.format.size()
            + self.ty.size()
            + self.swap_bytes.size()
            + self.lsb_first.size()
    }
}
impl Request for ReadPixelsRequest {
    const OPCODE: u8 = 111;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ReadPixelsReply;
}
#[derive(Clone, Debug, Default)]
pub struct ReadPixelsReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub data: Vec<Byte>,
}
impl ReadPixelsReply {}
impl AsByteSequence for ReadPixelsReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 24;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ReadPixelsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 24;
        let (data, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize) * (4)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            ReadPixelsReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                data: data,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + 1 + self.sequence.size() + self.length.size() + 24 + {
            let block_len: usize = self.data.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
            block_len + pad
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetBooleanvRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub pname: Int32,
}
impl GetBooleanvRequest {}
impl AsByteSequence for GetBooleanvRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetBooleanvRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetBooleanvRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size() + self.pname.size()
    }
}
impl Request for GetBooleanvRequest {
    const OPCODE: u8 = 112;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetBooleanvReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetBooleanvReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: bool,
    pub data: Vec<bool>,
}
impl GetBooleanvReply {}
impl AsByteSequence for GetBooleanvReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 15;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<bool>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetBooleanvReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 15;
        let (data, block_len): (Vec<bool>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<bool>());
        Some((
            GetBooleanvReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 15
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<bool>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetClipPlaneRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub plane: Int32,
}
impl GetClipPlaneRequest {}
impl AsByteSequence for GetClipPlaneRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.plane.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetClipPlaneRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (plane, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetClipPlaneRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                plane: plane,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size() + self.plane.size()
    }
}
impl Request for GetClipPlaneRequest {
    const OPCODE: u8 = 113;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetClipPlaneReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetClipPlaneReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub data: Vec<Float64>,
}
impl GetClipPlaneReply {}
impl AsByteSequence for GetClipPlaneReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 24;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float64>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetClipPlaneReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 24;
        let (data, block_len): (Vec<Float64>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize) / (2)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float64>());
        Some((
            GetClipPlaneReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                data: data,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + 1 + self.sequence.size() + self.length.size() + 24 + {
            let block_len: usize = self.data.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Float64>());
            block_len + pad
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetDoublevRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub pname: Card32,
}
impl GetDoublevRequest {}
impl AsByteSequence for GetDoublevRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDoublevRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetDoublevRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size() + self.pname.size()
    }
}
impl Request for GetDoublevRequest {
    const OPCODE: u8 = 114;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetDoublevReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetDoublevReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float64,
    pub data: Vec<Float64>,
}
impl GetDoublevReply {}
impl AsByteSequence for GetDoublevReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 8;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float64>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDoublevReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Float64, usize) = <Float64>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 8;
        let (data, block_len): (Vec<Float64>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float64>());
        Some((
            GetDoublevReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 8
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Float64>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetErrorRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
}
impl GetErrorRequest {}
impl AsByteSequence for GetErrorRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetErrorRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetErrorRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size()
    }
}
impl Request for GetErrorRequest {
    const OPCODE: u8 = 115;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetErrorReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetErrorReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub error: Int32,
}
impl GetErrorReply {}
impl AsByteSequence for GetErrorReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.error.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetErrorReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (error, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetErrorReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                error: error,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + 1 + self.sequence.size() + self.length.size() + self.error.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetFloatvRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub pname: Card32,
}
impl GetFloatvRequest {}
impl AsByteSequence for GetFloatvRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetFloatvRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetFloatvRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size() + self.pname.size()
    }
}
impl Request for GetFloatvRequest {
    const OPCODE: u8 = 116;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetFloatvReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetFloatvReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetFloatvReply {}
impl AsByteSequence for GetFloatvReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetFloatvReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Float32, usize) = <Float32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Float32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        Some((
            GetFloatvReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Float32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetIntegervRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub pname: Card32,
}
impl GetIntegervRequest {}
impl AsByteSequence for GetIntegervRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetIntegervRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetIntegervRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size() + self.pname.size()
    }
}
impl Request for GetIntegervRequest {
    const OPCODE: u8 = 117;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetIntegervReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetIntegervReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Int32,
    pub data: Vec<Int32>,
}
impl GetIntegervReply {}
impl AsByteSequence for GetIntegervReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetIntegervReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Int32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        Some((
            GetIntegervReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Int32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetLightfvRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub light: Card32,
    pub pname: Card32,
}
impl GetLightfvRequest {}
impl AsByteSequence for GetLightfvRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.light.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetLightfvRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (light, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetLightfvRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                light: light,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.light.size()
            + self.pname.size()
    }
}
impl Request for GetLightfvRequest {
    const OPCODE: u8 = 118;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetLightfvReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetLightfvReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetLightfvReply {}
impl AsByteSequence for GetLightfvReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetLightfvReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Float32, usize) = <Float32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Float32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        Some((
            GetLightfvReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Float32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetLightivRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub light: Card32,
    pub pname: Card32,
}
impl GetLightivRequest {}
impl AsByteSequence for GetLightivRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.light.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetLightivRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (light, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetLightivRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                light: light,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.light.size()
            + self.pname.size()
    }
}
impl Request for GetLightivRequest {
    const OPCODE: u8 = 119;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetLightivReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetLightivReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Int32,
    pub data: Vec<Int32>,
}
impl GetLightivReply {}
impl AsByteSequence for GetLightivReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetLightivReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Int32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        Some((
            GetLightivReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Int32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetMapdvRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub target: Card32,
    pub query: Card32,
}
impl GetMapdvRequest {}
impl AsByteSequence for GetMapdvRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.query.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetMapdvRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (query, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetMapdvRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                target: target,
                query: query,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.target.size()
            + self.query.size()
    }
}
impl Request for GetMapdvRequest {
    const OPCODE: u8 = 120;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetMapdvReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetMapdvReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float64,
    pub data: Vec<Float64>,
}
impl GetMapdvReply {}
impl AsByteSequence for GetMapdvReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 8;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float64>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetMapdvReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Float64, usize) = <Float64>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 8;
        let (data, block_len): (Vec<Float64>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float64>());
        Some((
            GetMapdvReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 8
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Float64>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetMapfvRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub target: Card32,
    pub query: Card32,
}
impl GetMapfvRequest {}
impl AsByteSequence for GetMapfvRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.query.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetMapfvRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (query, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetMapfvRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                target: target,
                query: query,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.target.size()
            + self.query.size()
    }
}
impl Request for GetMapfvRequest {
    const OPCODE: u8 = 121;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetMapfvReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetMapfvReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetMapfvReply {}
impl AsByteSequence for GetMapfvReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetMapfvReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Float32, usize) = <Float32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Float32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        Some((
            GetMapfvReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Float32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetMapivRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub target: Card32,
    pub query: Card32,
}
impl GetMapivRequest {}
impl AsByteSequence for GetMapivRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.query.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetMapivRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (query, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetMapivRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                target: target,
                query: query,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.target.size()
            + self.query.size()
    }
}
impl Request for GetMapivRequest {
    const OPCODE: u8 = 122;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetMapivReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetMapivReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Int32,
    pub data: Vec<Int32>,
}
impl GetMapivReply {}
impl AsByteSequence for GetMapivReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetMapivReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Int32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        Some((
            GetMapivReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Int32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetMaterialfvRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub face: Card32,
    pub pname: Card32,
}
impl GetMaterialfvRequest {}
impl AsByteSequence for GetMaterialfvRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.face.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetMaterialfvRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (face, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetMaterialfvRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                face: face,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.face.size()
            + self.pname.size()
    }
}
impl Request for GetMaterialfvRequest {
    const OPCODE: u8 = 123;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetMaterialfvReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetMaterialfvReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetMaterialfvReply {}
impl AsByteSequence for GetMaterialfvReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetMaterialfvReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Float32, usize) = <Float32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Float32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        Some((
            GetMaterialfvReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Float32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetMaterialivRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub face: Card32,
    pub pname: Card32,
}
impl GetMaterialivRequest {}
impl AsByteSequence for GetMaterialivRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.face.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetMaterialivRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (face, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetMaterialivRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                face: face,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.face.size()
            + self.pname.size()
    }
}
impl Request for GetMaterialivRequest {
    const OPCODE: u8 = 124;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetMaterialivReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetMaterialivReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Int32,
    pub data: Vec<Int32>,
}
impl GetMaterialivReply {}
impl AsByteSequence for GetMaterialivReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetMaterialivReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Int32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        Some((
            GetMaterialivReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Int32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetPixelMapfvRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub map: Card32,
}
impl GetPixelMapfvRequest {}
impl AsByteSequence for GetPixelMapfvRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.map.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPixelMapfvRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (map, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetPixelMapfvRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                map: map,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size() + self.map.size()
    }
}
impl Request for GetPixelMapfvRequest {
    const OPCODE: u8 = 125;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetPixelMapfvReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetPixelMapfvReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetPixelMapfvReply {}
impl AsByteSequence for GetPixelMapfvReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPixelMapfvReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Float32, usize) = <Float32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Float32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        Some((
            GetPixelMapfvReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Float32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetPixelMapuivRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub map: Card32,
}
impl GetPixelMapuivRequest {}
impl AsByteSequence for GetPixelMapuivRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.map.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPixelMapuivRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (map, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetPixelMapuivRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                map: map,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size() + self.map.size()
    }
}
impl Request for GetPixelMapuivRequest {
    const OPCODE: u8 = 126;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetPixelMapuivReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetPixelMapuivReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Card32,
    pub data: Vec<Card32>,
}
impl GetPixelMapuivReply {}
impl AsByteSequence for GetPixelMapuivReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPixelMapuivReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            GetPixelMapuivReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetPixelMapusvRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub map: Card32,
}
impl GetPixelMapusvRequest {}
impl AsByteSequence for GetPixelMapusvRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.map.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPixelMapusvRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (map, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetPixelMapusvRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                map: map,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size() + self.map.size()
    }
}
impl Request for GetPixelMapusvRequest {
    const OPCODE: u8 = 127;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetPixelMapusvReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetPixelMapusvReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Card16,
    pub data: Vec<Card16>,
}
impl GetPixelMapusvReply {}
impl AsByteSequence for GetPixelMapusvReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 16;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPixelMapusvReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 16;
        let (data, block_len): (Vec<Card16>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        Some((
            GetPixelMapusvReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 16
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card16>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetPolygonStippleRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub lsb_first: bool,
}
impl GetPolygonStippleRequest {}
impl AsByteSequence for GetPolygonStippleRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.lsb_first.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPolygonStippleRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (lsb_first, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetPolygonStippleRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                lsb_first: lsb_first,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size() + self.lsb_first.size()
    }
}
impl Request for GetPolygonStippleRequest {
    const OPCODE: u8 = 128;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetPolygonStippleReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetPolygonStippleReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub data: Vec<Byte>,
}
impl GetPolygonStippleReply {}
impl AsByteSequence for GetPolygonStippleReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 24;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPolygonStippleReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 24;
        let (data, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize) * (4)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            GetPolygonStippleReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                data: data,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + 1 + self.sequence.size() + self.length.size() + 24 + {
            let block_len: usize = self.data.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
            block_len + pad
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetStringRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub name: Card32,
}
impl GetStringRequest {}
impl AsByteSequence for GetStringRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.name.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetStringRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (name, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetStringRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                name: name,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size() + self.name.size()
    }
}
impl Request for GetStringRequest {
    const OPCODE: u8 = 129;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetStringReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetStringReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub string: String,
}
impl GetStringReply {}
impl AsByteSequence for GetStringReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.string.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 16;
        let block_len: usize = string_as_bytes(&self.string, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetStringReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 16;
        let (string, block_len): (String, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            GetStringReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                string: string,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + 16
            + {
                let block_len: usize = self.string.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetTexEnvfvRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub target: Card32,
    pub pname: Card32,
}
impl GetTexEnvfvRequest {}
impl AsByteSequence for GetTexEnvfvRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetTexEnvfvRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetTexEnvfvRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                target: target,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.target.size()
            + self.pname.size()
    }
}
impl Request for GetTexEnvfvRequest {
    const OPCODE: u8 = 130;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetTexEnvfvReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetTexEnvfvReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetTexEnvfvReply {}
impl AsByteSequence for GetTexEnvfvReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetTexEnvfvReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Float32, usize) = <Float32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Float32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        Some((
            GetTexEnvfvReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Float32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetTexEnvivRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub target: Card32,
    pub pname: Card32,
}
impl GetTexEnvivRequest {}
impl AsByteSequence for GetTexEnvivRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetTexEnvivRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetTexEnvivRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                target: target,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.target.size()
            + self.pname.size()
    }
}
impl Request for GetTexEnvivRequest {
    const OPCODE: u8 = 131;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetTexEnvivReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetTexEnvivReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Int32,
    pub data: Vec<Int32>,
}
impl GetTexEnvivReply {}
impl AsByteSequence for GetTexEnvivReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetTexEnvivReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Int32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        Some((
            GetTexEnvivReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Int32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetTexGendvRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub coord: Card32,
    pub pname: Card32,
}
impl GetTexGendvRequest {}
impl AsByteSequence for GetTexGendvRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.coord.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetTexGendvRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (coord, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetTexGendvRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                coord: coord,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.coord.size()
            + self.pname.size()
    }
}
impl Request for GetTexGendvRequest {
    const OPCODE: u8 = 132;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetTexGendvReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetTexGendvReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float64,
    pub data: Vec<Float64>,
}
impl GetTexGendvReply {}
impl AsByteSequence for GetTexGendvReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 8;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float64>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetTexGendvReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Float64, usize) = <Float64>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 8;
        let (data, block_len): (Vec<Float64>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float64>());
        Some((
            GetTexGendvReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 8
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Float64>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetTexGenfvRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub coord: Card32,
    pub pname: Card32,
}
impl GetTexGenfvRequest {}
impl AsByteSequence for GetTexGenfvRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.coord.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetTexGenfvRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (coord, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetTexGenfvRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                coord: coord,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.coord.size()
            + self.pname.size()
    }
}
impl Request for GetTexGenfvRequest {
    const OPCODE: u8 = 133;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetTexGenfvReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetTexGenfvReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetTexGenfvReply {}
impl AsByteSequence for GetTexGenfvReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetTexGenfvReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Float32, usize) = <Float32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Float32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        Some((
            GetTexGenfvReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Float32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetTexGenivRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub coord: Card32,
    pub pname: Card32,
}
impl GetTexGenivRequest {}
impl AsByteSequence for GetTexGenivRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.coord.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetTexGenivRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (coord, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetTexGenivRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                coord: coord,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.coord.size()
            + self.pname.size()
    }
}
impl Request for GetTexGenivRequest {
    const OPCODE: u8 = 134;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetTexGenivReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetTexGenivReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Int32,
    pub data: Vec<Int32>,
}
impl GetTexGenivReply {}
impl AsByteSequence for GetTexGenivReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetTexGenivReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Int32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        Some((
            GetTexGenivReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Int32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetTexImageRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub target: Card32,
    pub level: Int32,
    pub format: Card32,
    pub ty: Card32,
    pub swap_bytes: bool,
}
impl GetTexImageRequest {}
impl AsByteSequence for GetTexImageRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.level.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.swap_bytes.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetTexImageRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (level, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (swap_bytes, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetTexImageRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                target: target,
                level: level,
                format: format,
                ty: ty,
                swap_bytes: swap_bytes,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.target.size()
            + self.level.size()
            + self.format.size()
            + self.ty.size()
            + self.swap_bytes.size()
    }
}
impl Request for GetTexImageRequest {
    const OPCODE: u8 = 135;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetTexImageReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetTexImageReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: Int32,
    pub height: Int32,
    pub depth: Int32,
    pub data: Vec<Byte>,
}
impl GetTexImageReply {}
impl AsByteSequence for GetTexImageReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 8;
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.depth.as_bytes(&mut bytes[index..]);
        index += 4;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetTexImageReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 8;
        let (width, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (depth, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (data, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize) * (4)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            GetTexImageReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                width: width,
                height: height,
                depth: depth,
                data: data,
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
            + 8
            + self.width.size()
            + self.height.size()
            + self.depth.size()
            + 4
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetTexParameterfvRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub target: Card32,
    pub pname: Card32,
}
impl GetTexParameterfvRequest {}
impl AsByteSequence for GetTexParameterfvRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetTexParameterfvRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetTexParameterfvRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                target: target,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.target.size()
            + self.pname.size()
    }
}
impl Request for GetTexParameterfvRequest {
    const OPCODE: u8 = 136;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetTexParameterfvReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetTexParameterfvReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetTexParameterfvReply {}
impl AsByteSequence for GetTexParameterfvReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetTexParameterfvReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Float32, usize) = <Float32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Float32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        Some((
            GetTexParameterfvReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Float32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetTexParameterivRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub target: Card32,
    pub pname: Card32,
}
impl GetTexParameterivRequest {}
impl AsByteSequence for GetTexParameterivRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetTexParameterivRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetTexParameterivRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                target: target,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.target.size()
            + self.pname.size()
    }
}
impl Request for GetTexParameterivRequest {
    const OPCODE: u8 = 137;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetTexParameterivReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetTexParameterivReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Int32,
    pub data: Vec<Int32>,
}
impl GetTexParameterivReply {}
impl AsByteSequence for GetTexParameterivReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetTexParameterivReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Int32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        Some((
            GetTexParameterivReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Int32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetTexLevelParameterfvRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub target: Card32,
    pub level: Int32,
    pub pname: Card32,
}
impl GetTexLevelParameterfvRequest {}
impl AsByteSequence for GetTexLevelParameterfvRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.level.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetTexLevelParameterfvRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (level, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetTexLevelParameterfvRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                target: target,
                level: level,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.target.size()
            + self.level.size()
            + self.pname.size()
    }
}
impl Request for GetTexLevelParameterfvRequest {
    const OPCODE: u8 = 138;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetTexLevelParameterfvReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetTexLevelParameterfvReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetTexLevelParameterfvReply {}
impl AsByteSequence for GetTexLevelParameterfvReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetTexLevelParameterfvReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Float32, usize) = <Float32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Float32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        Some((
            GetTexLevelParameterfvReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Float32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetTexLevelParameterivRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub target: Card32,
    pub level: Int32,
    pub pname: Card32,
}
impl GetTexLevelParameterivRequest {}
impl AsByteSequence for GetTexLevelParameterivRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.level.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetTexLevelParameterivRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (level, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetTexLevelParameterivRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                target: target,
                level: level,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.target.size()
            + self.level.size()
            + self.pname.size()
    }
}
impl Request for GetTexLevelParameterivRequest {
    const OPCODE: u8 = 139;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetTexLevelParameterivReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetTexLevelParameterivReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Int32,
    pub data: Vec<Int32>,
}
impl GetTexLevelParameterivReply {}
impl AsByteSequence for GetTexLevelParameterivReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetTexLevelParameterivReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Int32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        Some((
            GetTexLevelParameterivReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Int32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct IsEnabledRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub capability: Card32,
}
impl IsEnabledRequest {}
impl AsByteSequence for IsEnabledRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.capability.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing IsEnabledRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (capability, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            IsEnabledRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                capability: capability,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size() + self.capability.size()
    }
}
impl Request for IsEnabledRequest {
    const OPCODE: u8 = 140;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = IsEnabledReply;
}
#[derive(Clone, Debug, Default)]
pub struct IsEnabledReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: Bool32,
}
impl IsEnabledReply {}
impl AsByteSequence for IsEnabledReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.ret_val.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing IsEnabledReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ret_val, sz): (Bool32, usize) = <Bool32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            IsEnabledReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                ret_val: ret_val,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + 1 + self.sequence.size() + self.length.size() + self.ret_val.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct IsListRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub list: Card32,
}
impl IsListRequest {}
impl AsByteSequence for IsListRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.list.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing IsListRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (list, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            IsListRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                list: list,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size() + self.list.size()
    }
}
impl Request for IsListRequest {
    const OPCODE: u8 = 141;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = IsListReply;
}
#[derive(Clone, Debug, Default)]
pub struct IsListReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: Bool32,
}
impl IsListReply {}
impl AsByteSequence for IsListReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.ret_val.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing IsListReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ret_val, sz): (Bool32, usize) = <Bool32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            IsListReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                ret_val: ret_val,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + 1 + self.sequence.size() + self.length.size() + self.ret_val.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct FlushRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
}
impl FlushRequest {}
impl AsByteSequence for FlushRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FlushRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            FlushRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size()
    }
}
impl Request for FlushRequest {
    const OPCODE: u8 = 142;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct AreTexturesResidentRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub textures: Vec<Card32>,
}
impl AreTexturesResidentRequest {}
impl AsByteSequence for AreTexturesResidentRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.textures.len() as Int32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.textures, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AreTexturesResidentRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (textures, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            AreTexturesResidentRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                textures: textures,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + ::core::mem::size_of::<Int32>()
            + {
                let block_len: usize = self.textures.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
impl Request for AreTexturesResidentRequest {
    const OPCODE: u8 = 143;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = AreTexturesResidentReply;
}
#[derive(Clone, Debug, Default)]
pub struct AreTexturesResidentReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: Bool32,
    pub data: Vec<bool>,
}
impl AreTexturesResidentReply {}
impl AsByteSequence for AreTexturesResidentReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.ret_val.as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<bool>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AreTexturesResidentReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ret_val, sz): (Bool32, usize) = <Bool32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        let (data, block_len): (Vec<bool>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize) * (4)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<bool>());
        Some((
            AreTexturesResidentReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                ret_val: ret_val,
                data: data,
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
            + self.ret_val.size()
            + 20
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<bool>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct DeleteTexturesRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub textures: Vec<Card32>,
}
impl DeleteTexturesRequest {}
impl AsByteSequence for DeleteTexturesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.textures.len() as Int32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.textures, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeleteTexturesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (textures, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            DeleteTexturesRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                textures: textures,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + ::core::mem::size_of::<Int32>()
            + {
                let block_len: usize = self.textures.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
impl Request for DeleteTexturesRequest {
    const OPCODE: u8 = 144;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GenTexturesRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub n: Int32,
}
impl GenTexturesRequest {}
impl AsByteSequence for GenTexturesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.n.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GenTexturesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GenTexturesRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                n: n,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size() + self.n.size()
    }
}
impl Request for GenTexturesRequest {
    const OPCODE: u8 = 145;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GenTexturesReply;
}
#[derive(Clone, Debug, Default)]
pub struct GenTexturesReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub data: Vec<Card32>,
}
impl GenTexturesReply {}
impl AsByteSequence for GenTexturesReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += (self.data.len() as u32).as_bytes(&mut bytes[index..]);
        index += 24;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GenTexturesReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 24;
        let (data, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            GenTexturesReply {
                reply_type: reply_type,
                sequence: sequence,
                data: data,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + 1 + self.sequence.size() + ::core::mem::size_of::<u32>() + 24 + {
            let block_len: usize = self.data.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
            block_len + pad
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct IsTextureRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub texture: Card32,
}
impl IsTextureRequest {}
impl AsByteSequence for IsTextureRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.texture.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing IsTextureRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (texture, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            IsTextureRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                texture: texture,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size() + self.texture.size()
    }
}
impl Request for IsTextureRequest {
    const OPCODE: u8 = 146;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = IsTextureReply;
}
#[derive(Clone, Debug, Default)]
pub struct IsTextureReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: Bool32,
}
impl IsTextureReply {}
impl AsByteSequence for IsTextureReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.ret_val.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing IsTextureReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ret_val, sz): (Bool32, usize) = <Bool32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            IsTextureReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                ret_val: ret_val,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + 1 + self.sequence.size() + self.length.size() + self.ret_val.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetColorTableRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub target: Card32,
    pub format: Card32,
    pub ty: Card32,
    pub swap_bytes: bool,
}
impl GetColorTableRequest {}
impl AsByteSequence for GetColorTableRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.swap_bytes.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetColorTableRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (swap_bytes, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetColorTableRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                target: target,
                format: format,
                ty: ty,
                swap_bytes: swap_bytes,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.target.size()
            + self.format.size()
            + self.ty.size()
            + self.swap_bytes.size()
    }
}
impl Request for GetColorTableRequest {
    const OPCODE: u8 = 147;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetColorTableReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetColorTableReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: Int32,
    pub data: Vec<Byte>,
}
impl GetColorTableReply {}
impl AsByteSequence for GetColorTableReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 8;
        index += self.width.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetColorTableReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 8;
        let (width, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize) * (4)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            GetColorTableReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                width: width,
                data: data,
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
            + 8
            + self.width.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetColorTableParameterfvRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub target: Card32,
    pub pname: Card32,
}
impl GetColorTableParameterfvRequest {}
impl AsByteSequence for GetColorTableParameterfvRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetColorTableParameterfvRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetColorTableParameterfvRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                target: target,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.target.size()
            + self.pname.size()
    }
}
impl Request for GetColorTableParameterfvRequest {
    const OPCODE: u8 = 148;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetColorTableParameterfvReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetColorTableParameterfvReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetColorTableParameterfvReply {}
impl AsByteSequence for GetColorTableParameterfvReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetColorTableParameterfvReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Float32, usize) = <Float32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Float32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        Some((
            GetColorTableParameterfvReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Float32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetColorTableParameterivRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub target: Card32,
    pub pname: Card32,
}
impl GetColorTableParameterivRequest {}
impl AsByteSequence for GetColorTableParameterivRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetColorTableParameterivRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetColorTableParameterivRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                target: target,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.target.size()
            + self.pname.size()
    }
}
impl Request for GetColorTableParameterivRequest {
    const OPCODE: u8 = 149;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetColorTableParameterivReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetColorTableParameterivReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Int32,
    pub data: Vec<Int32>,
}
impl GetColorTableParameterivReply {}
impl AsByteSequence for GetColorTableParameterivReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetColorTableParameterivReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Int32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        Some((
            GetColorTableParameterivReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Int32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetConvolutionFilterRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub target: Card32,
    pub format: Card32,
    pub ty: Card32,
    pub swap_bytes: bool,
}
impl GetConvolutionFilterRequest {}
impl AsByteSequence for GetConvolutionFilterRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.swap_bytes.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetConvolutionFilterRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (swap_bytes, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetConvolutionFilterRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                target: target,
                format: format,
                ty: ty,
                swap_bytes: swap_bytes,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.target.size()
            + self.format.size()
            + self.ty.size()
            + self.swap_bytes.size()
    }
}
impl Request for GetConvolutionFilterRequest {
    const OPCODE: u8 = 150;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetConvolutionFilterReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetConvolutionFilterReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: Int32,
    pub height: Int32,
    pub data: Vec<Byte>,
}
impl GetConvolutionFilterReply {}
impl AsByteSequence for GetConvolutionFilterReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 8;
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetConvolutionFilterReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 8;
        let (width, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (data, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize) * (4)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            GetConvolutionFilterReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                width: width,
                height: height,
                data: data,
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
            + 8
            + self.width.size()
            + self.height.size()
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetConvolutionParameterfvRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub target: Card32,
    pub pname: Card32,
}
impl GetConvolutionParameterfvRequest {}
impl AsByteSequence for GetConvolutionParameterfvRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetConvolutionParameterfvRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetConvolutionParameterfvRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                target: target,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.target.size()
            + self.pname.size()
    }
}
impl Request for GetConvolutionParameterfvRequest {
    const OPCODE: u8 = 151;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetConvolutionParameterfvReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetConvolutionParameterfvReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetConvolutionParameterfvReply {}
impl AsByteSequence for GetConvolutionParameterfvReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetConvolutionParameterfvReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Float32, usize) = <Float32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Float32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        Some((
            GetConvolutionParameterfvReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Float32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetConvolutionParameterivRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub target: Card32,
    pub pname: Card32,
}
impl GetConvolutionParameterivRequest {}
impl AsByteSequence for GetConvolutionParameterivRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetConvolutionParameterivRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetConvolutionParameterivRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                target: target,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.target.size()
            + self.pname.size()
    }
}
impl Request for GetConvolutionParameterivRequest {
    const OPCODE: u8 = 152;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetConvolutionParameterivReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetConvolutionParameterivReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Int32,
    pub data: Vec<Int32>,
}
impl GetConvolutionParameterivReply {}
impl AsByteSequence for GetConvolutionParameterivReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetConvolutionParameterivReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Int32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        Some((
            GetConvolutionParameterivReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Int32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetSeparableFilterRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub target: Card32,
    pub format: Card32,
    pub ty: Card32,
    pub swap_bytes: bool,
}
impl GetSeparableFilterRequest {}
impl AsByteSequence for GetSeparableFilterRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.swap_bytes.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetSeparableFilterRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (swap_bytes, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetSeparableFilterRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                target: target,
                format: format,
                ty: ty,
                swap_bytes: swap_bytes,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.target.size()
            + self.format.size()
            + self.ty.size()
            + self.swap_bytes.size()
    }
}
impl Request for GetSeparableFilterRequest {
    const OPCODE: u8 = 153;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetSeparableFilterReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetSeparableFilterReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub row_w: Int32,
    pub col_h: Int32,
    pub rows_and_cols: Vec<Byte>,
}
impl GetSeparableFilterReply {}
impl AsByteSequence for GetSeparableFilterReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 8;
        index += self.row_w.as_bytes(&mut bytes[index..]);
        index += self.col_h.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.rows_and_cols, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetSeparableFilterReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 8;
        let (row_w, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (col_h, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (rows_and_cols, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize) * (4)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            GetSeparableFilterReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                row_w: row_w,
                col_h: col_h,
                rows_and_cols: rows_and_cols,
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
            + 8
            + self.row_w.size()
            + self.col_h.size()
            + {
                let block_len: usize = self.rows_and_cols.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetHistogramRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub target: Card32,
    pub format: Card32,
    pub ty: Card32,
    pub swap_bytes: bool,
    pub reset: bool,
}
impl GetHistogramRequest {}
impl AsByteSequence for GetHistogramRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.swap_bytes.as_bytes(&mut bytes[index..]);
        index += self.reset.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetHistogramRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (swap_bytes, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (reset, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetHistogramRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                target: target,
                format: format,
                ty: ty,
                swap_bytes: swap_bytes,
                reset: reset,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.target.size()
            + self.format.size()
            + self.ty.size()
            + self.swap_bytes.size()
            + self.reset.size()
    }
}
impl Request for GetHistogramRequest {
    const OPCODE: u8 = 154;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetHistogramReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetHistogramReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: Int32,
    pub data: Vec<Byte>,
}
impl GetHistogramReply {}
impl AsByteSequence for GetHistogramReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 8;
        index += self.width.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetHistogramReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 8;
        let (width, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize) * (4)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            GetHistogramReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                width: width,
                data: data,
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
            + 8
            + self.width.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetHistogramParameterfvRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub target: Card32,
    pub pname: Card32,
}
impl GetHistogramParameterfvRequest {}
impl AsByteSequence for GetHistogramParameterfvRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetHistogramParameterfvRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetHistogramParameterfvRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                target: target,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.target.size()
            + self.pname.size()
    }
}
impl Request for GetHistogramParameterfvRequest {
    const OPCODE: u8 = 155;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetHistogramParameterfvReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetHistogramParameterfvReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetHistogramParameterfvReply {}
impl AsByteSequence for GetHistogramParameterfvReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetHistogramParameterfvReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Float32, usize) = <Float32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Float32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        Some((
            GetHistogramParameterfvReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Float32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetHistogramParameterivRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub target: Card32,
    pub pname: Card32,
}
impl GetHistogramParameterivRequest {}
impl AsByteSequence for GetHistogramParameterivRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetHistogramParameterivRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetHistogramParameterivRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                target: target,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.target.size()
            + self.pname.size()
    }
}
impl Request for GetHistogramParameterivRequest {
    const OPCODE: u8 = 156;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetHistogramParameterivReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetHistogramParameterivReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Int32,
    pub data: Vec<Int32>,
}
impl GetHistogramParameterivReply {}
impl AsByteSequence for GetHistogramParameterivReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetHistogramParameterivReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Int32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        Some((
            GetHistogramParameterivReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Int32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetMinmaxRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub target: Card32,
    pub format: Card32,
    pub ty: Card32,
    pub swap_bytes: bool,
    pub reset: bool,
}
impl GetMinmaxRequest {}
impl AsByteSequence for GetMinmaxRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.swap_bytes.as_bytes(&mut bytes[index..]);
        index += self.reset.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetMinmaxRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (swap_bytes, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (reset, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetMinmaxRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                target: target,
                format: format,
                ty: ty,
                swap_bytes: swap_bytes,
                reset: reset,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.target.size()
            + self.format.size()
            + self.ty.size()
            + self.swap_bytes.size()
            + self.reset.size()
    }
}
impl Request for GetMinmaxRequest {
    const OPCODE: u8 = 157;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetMinmaxReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetMinmaxReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub data: Vec<Byte>,
}
impl GetMinmaxReply {}
impl AsByteSequence for GetMinmaxReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 24;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetMinmaxReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 24;
        let (data, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize) * (4)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            GetMinmaxReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                data: data,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + 1 + self.sequence.size() + self.length.size() + 24 + {
            let block_len: usize = self.data.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
            block_len + pad
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetMinmaxParameterfvRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub target: Card32,
    pub pname: Card32,
}
impl GetMinmaxParameterfvRequest {}
impl AsByteSequence for GetMinmaxParameterfvRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetMinmaxParameterfvRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetMinmaxParameterfvRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                target: target,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.target.size()
            + self.pname.size()
    }
}
impl Request for GetMinmaxParameterfvRequest {
    const OPCODE: u8 = 158;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetMinmaxParameterfvReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetMinmaxParameterfvReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Float32,
    pub data: Vec<Float32>,
}
impl GetMinmaxParameterfvReply {}
impl AsByteSequence for GetMinmaxParameterfvReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetMinmaxParameterfvReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Float32, usize) = <Float32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Float32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Float32>());
        Some((
            GetMinmaxParameterfvReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Float32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetMinmaxParameterivRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub target: Card32,
    pub pname: Card32,
}
impl GetMinmaxParameterivRequest {}
impl AsByteSequence for GetMinmaxParameterivRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetMinmaxParameterivRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetMinmaxParameterivRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                target: target,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.target.size()
            + self.pname.size()
    }
}
impl Request for GetMinmaxParameterivRequest {
    const OPCODE: u8 = 159;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetMinmaxParameterivReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetMinmaxParameterivReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Int32,
    pub data: Vec<Int32>,
}
impl GetMinmaxParameterivReply {}
impl AsByteSequence for GetMinmaxParameterivReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetMinmaxParameterivReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Int32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        Some((
            GetMinmaxParameterivReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Int32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetCompressedTexImageArbRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub target: Card32,
    pub level: Int32,
}
impl GetCompressedTexImageArbRequest {}
impl AsByteSequence for GetCompressedTexImageArbRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.level.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetCompressedTexImageArbRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (level, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetCompressedTexImageArbRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                target: target,
                level: level,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.target.size()
            + self.level.size()
    }
}
impl Request for GetCompressedTexImageArbRequest {
    const OPCODE: u8 = 160;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetCompressedTexImageArbReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetCompressedTexImageArbReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub size: Int32,
    pub data: Vec<Byte>,
}
impl GetCompressedTexImageArbReply {}
impl AsByteSequence for GetCompressedTexImageArbReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 8;
        index += self.size.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetCompressedTexImageArbReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 8;
        let (size, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize) * (4)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            GetCompressedTexImageArbReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                size: size,
                data: data,
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
            + 8
            + self.size.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct DeleteQueriesArbRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub ids: Vec<Card32>,
}
impl DeleteQueriesArbRequest {}
impl AsByteSequence for DeleteQueriesArbRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.ids.len() as Int32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.ids, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeleteQueriesArbRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ids, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            DeleteQueriesArbRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                ids: ids,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + ::core::mem::size_of::<Int32>()
            + {
                let block_len: usize = self.ids.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
impl Request for DeleteQueriesArbRequest {
    const OPCODE: u8 = 161;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GenQueriesArbRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub n: Int32,
}
impl GenQueriesArbRequest {}
impl AsByteSequence for GenQueriesArbRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.n.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GenQueriesArbRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GenQueriesArbRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                n: n,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size() + self.n.size()
    }
}
impl Request for GenQueriesArbRequest {
    const OPCODE: u8 = 162;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GenQueriesArbReply;
}
#[derive(Clone, Debug, Default)]
pub struct GenQueriesArbReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub data: Vec<Card32>,
}
impl GenQueriesArbReply {}
impl AsByteSequence for GenQueriesArbReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += (self.data.len() as u32).as_bytes(&mut bytes[index..]);
        index += 24;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GenQueriesArbReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 24;
        let (data, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            GenQueriesArbReply {
                reply_type: reply_type,
                sequence: sequence,
                data: data,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + 1 + self.sequence.size() + ::core::mem::size_of::<u32>() + 24 + {
            let block_len: usize = self.data.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
            block_len + pad
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct IsQueryArbRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub id: Card32,
}
impl IsQueryArbRequest {}
impl AsByteSequence for IsQueryArbRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.id.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing IsQueryArbRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (id, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            IsQueryArbRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                id: id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context_tag.size() + self.length.size() + self.id.size()
    }
}
impl Request for IsQueryArbRequest {
    const OPCODE: u8 = 163;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = IsQueryArbReply;
}
#[derive(Clone, Debug, Default)]
pub struct IsQueryArbReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: Bool32,
}
impl IsQueryArbReply {}
impl AsByteSequence for IsQueryArbReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.ret_val.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing IsQueryArbReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ret_val, sz): (Bool32, usize) = <Bool32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            IsQueryArbReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                ret_val: ret_val,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + 1 + self.sequence.size() + self.length.size() + self.ret_val.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetQueryivArbRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub target: Card32,
    pub pname: Card32,
}
impl GetQueryivArbRequest {}
impl AsByteSequence for GetQueryivArbRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetQueryivArbRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetQueryivArbRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                target: target,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.target.size()
            + self.pname.size()
    }
}
impl Request for GetQueryivArbRequest {
    const OPCODE: u8 = 164;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetQueryivArbReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetQueryivArbReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Int32,
    pub data: Vec<Int32>,
}
impl GetQueryivArbReply {}
impl AsByteSequence for GetQueryivArbReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetQueryivArbReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Int32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        Some((
            GetQueryivArbReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Int32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetQueryObjectivArbRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub id: Card32,
    pub pname: Card32,
}
impl GetQueryObjectivArbRequest {}
impl AsByteSequence for GetQueryObjectivArbRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.id.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetQueryObjectivArbRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (id, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetQueryObjectivArbRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                id: id,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.id.size()
            + self.pname.size()
    }
}
impl Request for GetQueryObjectivArbRequest {
    const OPCODE: u8 = 165;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetQueryObjectivArbReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetQueryObjectivArbReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Int32,
    pub data: Vec<Int32>,
}
impl GetQueryObjectivArbReply {}
impl AsByteSequence for GetQueryObjectivArbReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetQueryObjectivArbReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Int32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        Some((
            GetQueryObjectivArbReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Int32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetQueryObjectuivArbRequest {
    pub req_type: u8,
    pub context_tag: ContextTag,
    pub length: u16,
    pub id: Card32,
    pub pname: Card32,
}
impl GetQueryObjectuivArbRequest {}
impl AsByteSequence for GetQueryObjectuivArbRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_tag.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.id.as_bytes(&mut bytes[index..]);
        index += self.pname.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetQueryObjectuivArbRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_tag, sz): (ContextTag, usize) = <ContextTag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (id, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pname, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetQueryObjectuivArbRequest {
                req_type: req_type,
                context_tag: context_tag,
                length: length,
                id: id,
                pname: pname,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_tag.size()
            + self.length.size()
            + self.id.size()
            + self.pname.size()
    }
}
impl Request for GetQueryObjectuivArbRequest {
    const OPCODE: u8 = 166;
    const EXTENSION: Option<&'static str> = Some("GLX");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetQueryObjectuivArbReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetQueryObjectuivArbReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub datum: Card32,
    pub data: Vec<Card32>,
}
impl GetQueryObjectuivArbReply {}
impl AsByteSequence for GetQueryObjectuivArbReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 4;
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.datum.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetQueryObjectuivArbReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (datum, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            GetQueryObjectuivArbReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                datum: datum,
                data: data,
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
            + 4
            + ::core::mem::size_of::<Card32>()
            + self.datum.size()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Gc {
    pub inner: i32,
}
impl Gc {
    #[inline]
    pub fn gl_current_bit(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_gl_current_bit(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn gl_point_bit(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_gl_point_bit(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn gl_line_bit(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_gl_line_bit(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn gl_polygon_bit(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_gl_polygon_bit(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn gl_polygon_stipple_bit(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_gl_polygon_stipple_bit(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn gl_pixel_mode_bit(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_gl_pixel_mode_bit(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn gl_lighting_bit(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_gl_lighting_bit(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn gl_fog_bit(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_gl_fog_bit(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn gl_depth_buffer_bit(&self) -> bool {
        self.inner & (1 << 8) != 0
    }
    #[inline]
    pub fn set_gl_depth_buffer_bit(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 8;
        } else {
            self.inner &= !(1 << 8);
        }
        self
    }
    #[inline]
    pub fn gl_accum_buffer_bit(&self) -> bool {
        self.inner & (1 << 9) != 0
    }
    #[inline]
    pub fn set_gl_accum_buffer_bit(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 9;
        } else {
            self.inner &= !(1 << 9);
        }
        self
    }
    #[inline]
    pub fn gl_stencil_buffer_bit(&self) -> bool {
        self.inner & (1 << 10) != 0
    }
    #[inline]
    pub fn set_gl_stencil_buffer_bit(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 10;
        } else {
            self.inner &= !(1 << 10);
        }
        self
    }
    #[inline]
    pub fn gl_viewport_bit(&self) -> bool {
        self.inner & (1 << 11) != 0
    }
    #[inline]
    pub fn set_gl_viewport_bit(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 11;
        } else {
            self.inner &= !(1 << 11);
        }
        self
    }
    #[inline]
    pub fn gl_transform_bit(&self) -> bool {
        self.inner & (1 << 12) != 0
    }
    #[inline]
    pub fn set_gl_transform_bit(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 12;
        } else {
            self.inner &= !(1 << 12);
        }
        self
    }
    #[inline]
    pub fn gl_enable_bit(&self) -> bool {
        self.inner & (1 << 13) != 0
    }
    #[inline]
    pub fn set_gl_enable_bit(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 13;
        } else {
            self.inner &= !(1 << 13);
        }
        self
    }
    #[inline]
    pub fn gl_color_buffer_bit(&self) -> bool {
        self.inner & (1 << 14) != 0
    }
    #[inline]
    pub fn set_gl_color_buffer_bit(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 14;
        } else {
            self.inner &= !(1 << 14);
        }
        self
    }
    #[inline]
    pub fn gl_hint_bit(&self) -> bool {
        self.inner & (1 << 15) != 0
    }
    #[inline]
    pub fn set_gl_hint_bit(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 15;
        } else {
            self.inner &= !(1 << 15);
        }
        self
    }
    #[inline]
    pub fn gl_eval_bit(&self) -> bool {
        self.inner & (1 << 16) != 0
    }
    #[inline]
    pub fn set_gl_eval_bit(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 16;
        } else {
            self.inner &= !(1 << 16);
        }
        self
    }
    #[inline]
    pub fn gl_list_bit(&self) -> bool {
        self.inner & (1 << 17) != 0
    }
    #[inline]
    pub fn set_gl_list_bit(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 17;
        } else {
            self.inner &= !(1 << 17);
        }
        self
    }
    #[inline]
    pub fn gl_texture_bit(&self) -> bool {
        self.inner & (1 << 18) != 0
    }
    #[inline]
    pub fn set_gl_texture_bit(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 18;
        } else {
            self.inner &= !(1 << 18);
        }
        self
    }
    #[inline]
    pub fn gl_scissor_bit(&self) -> bool {
        self.inner & (1 << 19) != 0
    }
    #[inline]
    pub fn set_gl_scissor_bit(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 19;
        } else {
            self.inner &= !(1 << 19);
        }
        self
    }
    #[inline]
    pub fn new(
        gl_current_bit: bool,
        gl_point_bit: bool,
        gl_line_bit: bool,
        gl_polygon_bit: bool,
        gl_polygon_stipple_bit: bool,
        gl_pixel_mode_bit: bool,
        gl_lighting_bit: bool,
        gl_fog_bit: bool,
        gl_depth_buffer_bit: bool,
        gl_accum_buffer_bit: bool,
        gl_stencil_buffer_bit: bool,
        gl_viewport_bit: bool,
        gl_transform_bit: bool,
        gl_enable_bit: bool,
        gl_color_buffer_bit: bool,
        gl_hint_bit: bool,
        gl_eval_bit: bool,
        gl_list_bit: bool,
        gl_texture_bit: bool,
        gl_scissor_bit: bool,
    ) -> Self {
        let mut inner: i32 = 0;
        if gl_current_bit {
            inner |= 1 << 0;
        }
        if gl_point_bit {
            inner |= 1 << 1;
        }
        if gl_line_bit {
            inner |= 1 << 2;
        }
        if gl_polygon_bit {
            inner |= 1 << 3;
        }
        if gl_polygon_stipple_bit {
            inner |= 1 << 4;
        }
        if gl_pixel_mode_bit {
            inner |= 1 << 5;
        }
        if gl_lighting_bit {
            inner |= 1 << 6;
        }
        if gl_fog_bit {
            inner |= 1 << 7;
        }
        if gl_depth_buffer_bit {
            inner |= 1 << 8;
        }
        if gl_accum_buffer_bit {
            inner |= 1 << 9;
        }
        if gl_stencil_buffer_bit {
            inner |= 1 << 10;
        }
        if gl_viewport_bit {
            inner |= 1 << 11;
        }
        if gl_transform_bit {
            inner |= 1 << 12;
        }
        if gl_enable_bit {
            inner |= 1 << 13;
        }
        if gl_color_buffer_bit {
            inner |= 1 << 14;
        }
        if gl_hint_bit {
            inner |= 1 << 15;
        }
        if gl_eval_bit {
            inner |= 1 << 16;
        }
        if gl_list_bit {
            inner |= 1 << 17;
        }
        if gl_texture_bit {
            inner |= 1 << 18;
        }
        if gl_scissor_bit {
            inner |= 1 << 19;
        }
        Gc { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
}
impl AsByteSequence for Gc {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        Some((Gc { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for Gc {
    type Output = Gc;
    #[inline]
    fn not(self) -> Gc {
        Gc { inner: !self.inner }
    }
}
impl core::ops::BitAnd for Gc {
    type Output = Gc;
    #[inline]
    fn bitand(self, rhs: Gc) -> Gc {
        Gc {
            inner: self.inner & rhs.inner,
        }
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Pbcet {
    Damaged = 32791,
    Saved = 32792,
}
impl AsByteSequence for Pbcet {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            32791 => Some((Self::Damaged, sz)),
            32792 => Some((Self::Saved, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for Pbcet {
    #[inline]
    fn default() -> Pbcet {
        Pbcet::Damaged
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Pbcdt {
    Window = 32793,
    Pbuffer = 32794,
}
impl AsByteSequence for Pbcdt {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            32793 => Some((Self::Window, sz)),
            32794 => Some((Self::Pbuffer, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for Pbcdt {
    #[inline]
    fn default() -> Pbcdt {
        Pbcdt::Window
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rm {
    GlRender = 7168,
    GlFeedback = 7169,
    GlSelect = 7170,
}
impl AsByteSequence for Rm {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            7168 => Some((Self::GlRender, sz)),
            7169 => Some((Self::GlFeedback, sz)),
            7170 => Some((Self::GlSelect, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for Rm {
    #[inline]
    fn default() -> Rm {
        Rm::GlRender
    }
}
#[derive(Clone, Debug, Default)]
pub struct BufferSwapCompleteEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub event_type_: Card16,
    pub drawable: super::glx::Drawable,
    pub ust_hi: Card32,
    pub ust_lo: Card32,
    pub msc_hi: Card32,
    pub msc_lo: Card32,
    pub sbc: Card32,
}
impl BufferSwapCompleteEvent {}
impl AsByteSequence for BufferSwapCompleteEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.event_type_.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.ust_hi.as_bytes(&mut bytes[index..]);
        index += self.ust_lo.as_bytes(&mut bytes[index..]);
        index += self.msc_hi.as_bytes(&mut bytes[index..]);
        index += self.msc_lo.as_bytes(&mut bytes[index..]);
        index += self.sbc.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing BufferSwapCompleteEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_type_, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (drawable, sz): (super::glx::Drawable, usize) =
            <super::glx::Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ust_hi, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ust_lo, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (msc_hi, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (msc_lo, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sbc, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            BufferSwapCompleteEvent {
                event_type: event_type,
                sequence: sequence,
                event_type_: event_type_,
                drawable: drawable,
                ust_hi: ust_hi,
                ust_lo: ust_lo,
                msc_hi: msc_hi,
                msc_lo: msc_lo,
                sbc: sbc,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + 1
            + self.sequence.size()
            + self.event_type_.size()
            + 2
            + self.drawable.size()
            + self.ust_hi.size()
            + self.ust_lo.size()
            + self.msc_hi.size()
            + self.msc_lo.size()
            + self.sbc.size()
    }
}
impl crate::auto::Event for BufferSwapCompleteEvent {
    const OPCODE: u8 = 1;
}
#[derive(Clone, Debug, Default)]
pub struct PbufferClobberEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub event_type_: Card16,
    pub draw_type: Card16,
    pub drawable: super::glx::Drawable,
    pub b_mask: Card32,
    pub aux_buffer: Card16,
    pub x: Card16,
    pub y: Card16,
    pub width: Card16,
    pub height: Card16,
    pub count: Card16,
}
impl PbufferClobberEvent {}
impl AsByteSequence for PbufferClobberEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.event_type_.as_bytes(&mut bytes[index..]);
        index += self.draw_type.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.b_mask.as_bytes(&mut bytes[index..]);
        index += self.aux_buffer.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.count.as_bytes(&mut bytes[index..]);
        index += 4;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PbufferClobberEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_type_, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (draw_type, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (super::glx::Drawable, usize) =
            <super::glx::Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (b_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (aux_buffer, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (count, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        Some((
            PbufferClobberEvent {
                event_type: event_type,
                sequence: sequence,
                event_type_: event_type_,
                draw_type: draw_type,
                drawable: drawable,
                b_mask: b_mask,
                aux_buffer: aux_buffer,
                x: x,
                y: y,
                width: width,
                height: height,
                count: count,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + 1
            + self.sequence.size()
            + self.event_type_.size()
            + self.draw_type.size()
            + self.drawable.size()
            + self.b_mask.size()
            + self.aux_buffer.size()
            + self.x.size()
            + self.y.size()
            + self.width.size()
            + self.height.size()
            + self.count.size()
            + 4
    }
}
impl crate::auto::Event for PbufferClobberEvent {
    const OPCODE: u8 = 0;
}
