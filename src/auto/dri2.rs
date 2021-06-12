// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

use super::xproto::*;
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Dri2Buffer {
    pub attachment: Attachment,
    pub name: Card32,
    pub pitch: Card32,
    pub cpp: Card32,
    pub flags: Card32,
}
impl Dri2Buffer {}
impl AsByteSequence for Dri2Buffer {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.attachment.as_bytes(&mut bytes[index..]);
        index += self.name.as_bytes(&mut bytes[index..]);
        index += self.pitch.as_bytes(&mut bytes[index..]);
        index += self.cpp.as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Dri2Buffer from byte buffer");
        let (attachment, sz): (Attachment, usize) = <Attachment>::from_bytes(&bytes[index..])?;
        index += sz;
        let (name, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pitch, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cpp, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Dri2Buffer {
                attachment: attachment,
                name: name,
                pitch: pitch,
                cpp: cpp,
                flags: flags,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.attachment.size()
            + self.name.size()
            + self.pitch.size()
            + self.cpp.size()
            + self.flags.size()
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Attachment {
    BufferFrontLeft = 0,
    BufferBackLeft = 1,
    BufferFrontRight = 2,
    BufferBackRight = 3,
    BufferDepth = 4,
    BufferStencil = 5,
    BufferAccum = 6,
    BufferFakeFrontLeft = 7,
    BufferFakeFrontRight = 8,
    BufferDepthStencil = 9,
    BufferHiz = 10,
}
impl AsByteSequence for Attachment {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::BufferFrontLeft, sz)),
            1 => Some((Self::BufferBackLeft, sz)),
            2 => Some((Self::BufferFrontRight, sz)),
            3 => Some((Self::BufferBackRight, sz)),
            4 => Some((Self::BufferDepth, sz)),
            5 => Some((Self::BufferStencil, sz)),
            6 => Some((Self::BufferAccum, sz)),
            7 => Some((Self::BufferFakeFrontLeft, sz)),
            8 => Some((Self::BufferFakeFrontRight, sz)),
            9 => Some((Self::BufferDepthStencil, sz)),
            10 => Some((Self::BufferHiz, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u32>()
    }
}
impl Default for Attachment {
    #[inline]
    fn default() -> Attachment {
        Attachment::BufferFrontLeft
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct AttachFormat {
    pub attachment: Attachment,
    pub format: Card32,
}
impl AttachFormat {}
impl AsByteSequence for AttachFormat {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.attachment.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AttachFormat from byte buffer");
        let (attachment, sz): (Attachment, usize) = <Attachment>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            AttachFormat {
                attachment: attachment,
                format: format,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.attachment.size() + self.format.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
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
    const EXTENSION: Option<&'static str> = Some("DRI2");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryVersionReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
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
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ConnectRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub driver_type: DriverType,
}
impl ConnectRequest {}
impl AsByteSequence for ConnectRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.driver_type.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ConnectRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (driver_type, sz): (DriverType, usize) = <DriverType>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ConnectRequest {
                req_type: req_type,
                length: length,
                window: window,
                driver_type: driver_type,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size() + self.driver_type.size()
    }
}
impl Request for ConnectRequest {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = Some("DRI2");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ConnectReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ConnectReply<'a, 'b, 'c> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub driver_name_length: Card32,
    pub driver_name: Cow<'a, str>,
    pub alignment_pad: Cow<'b, [Void]>,
    pub device_name: Cow<'c, str>,
}
impl<'a, 'b, 'c> ConnectReply {}
impl<'a, 'b, 'c> AsByteSequence for ConnectReply<'a, 'b, 'c> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.driver_name_length.as_bytes(&mut bytes[index..]);
        index += (self.device_name.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 16;
        let block_len: usize = string_as_bytes(&self.driver_name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        let block_len: usize = vector_as_bytes(&self.alignment_pad, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Void>());
        let block_len: usize = string_as_bytes(&self.device_name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ConnectReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (driver_name_length, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 16;
        let (driver_name, block_len): (Cow<'static, str>, usize) =
            string_from_bytes(&bytes[index..], (driver_name_length as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        let (alignment_pad, block_len): (Cow<'static, [Void]>, usize) = vector_from_bytes(
            &bytes[index..],
            ((((driver_name_length as usize) + (3)) & (!(3))) - (driver_name_length as usize))
                as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Void>());
        let (device_name, block_len): (Cow<'static, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            ConnectReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                driver_name_length: driver_name_length,
                driver_name: driver_name,
                alignment_pad: alignment_pad,
                device_name: device_name,
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
            + self.driver_name_length.size()
            + ::core::mem::size_of::<Card32>()
            + 16
            + {
                let block_len: usize = self.driver_name.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
            + {
                let block_len: usize = self.alignment_pad.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Void>());
                block_len + pad
            }
            + {
                let block_len: usize = self.device_name.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum DriverType {
    Dri = 0,
    Vdpau = 1,
}
impl AsByteSequence for DriverType {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Dri, sz)),
            1 => Some((Self::Vdpau, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u32>()
    }
}
impl Default for DriverType {
    #[inline]
    fn default() -> DriverType {
        DriverType::Dri
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct AuthenticateRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub magic: Card32,
}
impl AuthenticateRequest {}
impl AsByteSequence for AuthenticateRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.magic.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AuthenticateRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (magic, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            AuthenticateRequest {
                req_type: req_type,
                length: length,
                window: window,
                magic: magic,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size() + self.magic.size()
    }
}
impl Request for AuthenticateRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = Some("DRI2");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = AuthenticateReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct AuthenticateReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub authenticated: Card32,
}
impl AuthenticateReply {}
impl AsByteSequence for AuthenticateReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.authenticated.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AuthenticateReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (authenticated, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            AuthenticateReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                authenticated: authenticated,
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
            + self.authenticated.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CreateDrawableRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
}
impl CreateDrawableRequest {}
impl AsByteSequence for CreateDrawableRequest {
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
        log::trace!("Deserializing CreateDrawableRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CreateDrawableRequest {
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
impl Request for CreateDrawableRequest {
    const OPCODE: u8 = 3;
    const EXTENSION: Option<&'static str> = Some("DRI2");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DestroyDrawableRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
}
impl DestroyDrawableRequest {}
impl AsByteSequence for DestroyDrawableRequest {
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
        log::trace!("Deserializing DestroyDrawableRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DestroyDrawableRequest {
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
impl Request for DestroyDrawableRequest {
    const OPCODE: u8 = 4;
    const EXTENSION: Option<&'static str> = Some("DRI2");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetBuffersRequest<'e> {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub count: Card32,
    pub attachments: Cow<'e, [Card32]>,
}
impl<'e> GetBuffersRequest {}
impl<'e> AsByteSequence for GetBuffersRequest<'e> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.count.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.attachments, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetBuffersRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (count, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (attachments, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            GetBuffersRequest {
                req_type: req_type,
                length: length,
                drawable: drawable,
                count: count,
                attachments: attachments,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.drawable.size() + self.count.size() + {
            let block_len: usize = self.attachments.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
            block_len + pad
        }
    }
}
impl Request for GetBuffersRequest {
    const OPCODE: u8 = 5;
    const EXTENSION: Option<&'static str> = Some("DRI2");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetBuffersReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetBuffersReply<'d> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: Card32,
    pub height: Card32,
    pub buffers: Cow<'d, [Dri2Buffer]>,
}
impl<'d> GetBuffersReply {}
impl<'d> AsByteSequence for GetBuffersReply<'d> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += (self.buffers.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.buffers, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Dri2Buffer>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetBuffersReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (buffers, block_len): (Cow<'static, [Dri2Buffer]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Dri2Buffer>());
        Some((
            GetBuffersReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                width: width,
                height: height,
                buffers: buffers,
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
            + self.width.size()
            + self.height.size()
            + ::core::mem::size_of::<Card32>()
            + 12
            + {
                let block_len: usize = self.buffers.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Dri2Buffer>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CopyRegionRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub region: Card32,
    pub dest: Card32,
    pub src: Card32,
}
impl CopyRegionRequest {}
impl AsByteSequence for CopyRegionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.region.as_bytes(&mut bytes[index..]);
        index += self.dest.as_bytes(&mut bytes[index..]);
        index += self.src.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CopyRegionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (region, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dest, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CopyRegionRequest {
                req_type: req_type,
                length: length,
                drawable: drawable,
                region: region,
                dest: dest,
                src: src,
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
            + self.region.size()
            + self.dest.size()
            + self.src.size()
    }
}
impl Request for CopyRegionRequest {
    const OPCODE: u8 = 6;
    const EXTENSION: Option<&'static str> = Some("DRI2");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = CopyRegionReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CopyRegionReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
}
impl CopyRegionReply {}
impl AsByteSequence for CopyRegionReply {
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
        log::trace!("Deserializing CopyRegionReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CopyRegionReply {
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
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetBuffersWithFormatRequest<'g> {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub count: Card32,
    pub attachments: Cow<'g, [AttachFormat]>,
}
impl<'g> GetBuffersWithFormatRequest {}
impl<'g> AsByteSequence for GetBuffersWithFormatRequest<'g> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.count.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.attachments, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<AttachFormat>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetBuffersWithFormatRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (count, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (attachments, block_len): (Cow<'static, [AttachFormat]>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<AttachFormat>());
        Some((
            GetBuffersWithFormatRequest {
                req_type: req_type,
                length: length,
                drawable: drawable,
                count: count,
                attachments: attachments,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.drawable.size() + self.count.size() + {
            let block_len: usize = self.attachments.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<AttachFormat>());
            block_len + pad
        }
    }
}
impl Request for GetBuffersWithFormatRequest {
    const OPCODE: u8 = 7;
    const EXTENSION: Option<&'static str> = Some("DRI2");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetBuffersWithFormatReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetBuffersWithFormatReply<'f> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: Card32,
    pub height: Card32,
    pub buffers: Cow<'f, [Dri2Buffer]>,
}
impl<'f> GetBuffersWithFormatReply {}
impl<'f> AsByteSequence for GetBuffersWithFormatReply<'f> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += (self.buffers.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.buffers, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Dri2Buffer>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetBuffersWithFormatReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (buffers, block_len): (Cow<'static, [Dri2Buffer]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Dri2Buffer>());
        Some((
            GetBuffersWithFormatReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                width: width,
                height: height,
                buffers: buffers,
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
            + self.width.size()
            + self.height.size()
            + ::core::mem::size_of::<Card32>()
            + 12
            + {
                let block_len: usize = self.buffers.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Dri2Buffer>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SwapBuffersRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub target_msc_hi: Card32,
    pub target_msc_lo: Card32,
    pub divisor_hi: Card32,
    pub divisor_lo: Card32,
    pub remainder_hi: Card32,
    pub remainder_lo: Card32,
}
impl SwapBuffersRequest {}
impl AsByteSequence for SwapBuffersRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.target_msc_hi.as_bytes(&mut bytes[index..]);
        index += self.target_msc_lo.as_bytes(&mut bytes[index..]);
        index += self.divisor_hi.as_bytes(&mut bytes[index..]);
        index += self.divisor_lo.as_bytes(&mut bytes[index..]);
        index += self.remainder_hi.as_bytes(&mut bytes[index..]);
        index += self.remainder_lo.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SwapBuffersRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target_msc_hi, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target_msc_lo, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (divisor_hi, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (divisor_lo, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (remainder_hi, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (remainder_lo, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SwapBuffersRequest {
                req_type: req_type,
                length: length,
                drawable: drawable,
                target_msc_hi: target_msc_hi,
                target_msc_lo: target_msc_lo,
                divisor_hi: divisor_hi,
                divisor_lo: divisor_lo,
                remainder_hi: remainder_hi,
                remainder_lo: remainder_lo,
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
            + self.target_msc_hi.size()
            + self.target_msc_lo.size()
            + self.divisor_hi.size()
            + self.divisor_lo.size()
            + self.remainder_hi.size()
            + self.remainder_lo.size()
    }
}
impl Request for SwapBuffersRequest {
    const OPCODE: u8 = 8;
    const EXTENSION: Option<&'static str> = Some("DRI2");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = SwapBuffersReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SwapBuffersReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub swap_hi: Card32,
    pub swap_lo: Card32,
}
impl SwapBuffersReply {}
impl AsByteSequence for SwapBuffersReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.swap_hi.as_bytes(&mut bytes[index..]);
        index += self.swap_lo.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SwapBuffersReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (swap_hi, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (swap_lo, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SwapBuffersReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                swap_hi: swap_hi,
                swap_lo: swap_lo,
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
            + self.swap_hi.size()
            + self.swap_lo.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetMscRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
}
impl GetMscRequest {}
impl AsByteSequence for GetMscRequest {
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
        log::trace!("Deserializing GetMscRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetMscRequest {
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
impl Request for GetMscRequest {
    const OPCODE: u8 = 9;
    const EXTENSION: Option<&'static str> = Some("DRI2");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetMscReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetMscReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ust_hi: Card32,
    pub ust_lo: Card32,
    pub msc_hi: Card32,
    pub msc_lo: Card32,
    pub sbc_hi: Card32,
    pub sbc_lo: Card32,
}
impl GetMscReply {}
impl AsByteSequence for GetMscReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.ust_hi.as_bytes(&mut bytes[index..]);
        index += self.ust_lo.as_bytes(&mut bytes[index..]);
        index += self.msc_hi.as_bytes(&mut bytes[index..]);
        index += self.msc_lo.as_bytes(&mut bytes[index..]);
        index += self.sbc_hi.as_bytes(&mut bytes[index..]);
        index += self.sbc_lo.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetMscReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ust_hi, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ust_lo, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (msc_hi, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (msc_lo, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sbc_hi, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sbc_lo, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetMscReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                ust_hi: ust_hi,
                ust_lo: ust_lo,
                msc_hi: msc_hi,
                msc_lo: msc_lo,
                sbc_hi: sbc_hi,
                sbc_lo: sbc_lo,
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
            + self.ust_hi.size()
            + self.ust_lo.size()
            + self.msc_hi.size()
            + self.msc_lo.size()
            + self.sbc_hi.size()
            + self.sbc_lo.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct WaitMscRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub target_msc_hi: Card32,
    pub target_msc_lo: Card32,
    pub divisor_hi: Card32,
    pub divisor_lo: Card32,
    pub remainder_hi: Card32,
    pub remainder_lo: Card32,
}
impl WaitMscRequest {}
impl AsByteSequence for WaitMscRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.target_msc_hi.as_bytes(&mut bytes[index..]);
        index += self.target_msc_lo.as_bytes(&mut bytes[index..]);
        index += self.divisor_hi.as_bytes(&mut bytes[index..]);
        index += self.divisor_lo.as_bytes(&mut bytes[index..]);
        index += self.remainder_hi.as_bytes(&mut bytes[index..]);
        index += self.remainder_lo.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing WaitMscRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target_msc_hi, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target_msc_lo, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (divisor_hi, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (divisor_lo, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (remainder_hi, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (remainder_lo, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            WaitMscRequest {
                req_type: req_type,
                length: length,
                drawable: drawable,
                target_msc_hi: target_msc_hi,
                target_msc_lo: target_msc_lo,
                divisor_hi: divisor_hi,
                divisor_lo: divisor_lo,
                remainder_hi: remainder_hi,
                remainder_lo: remainder_lo,
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
            + self.target_msc_hi.size()
            + self.target_msc_lo.size()
            + self.divisor_hi.size()
            + self.divisor_lo.size()
            + self.remainder_hi.size()
            + self.remainder_lo.size()
    }
}
impl Request for WaitMscRequest {
    const OPCODE: u8 = 10;
    const EXTENSION: Option<&'static str> = Some("DRI2");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = WaitMscReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct WaitMscReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ust_hi: Card32,
    pub ust_lo: Card32,
    pub msc_hi: Card32,
    pub msc_lo: Card32,
    pub sbc_hi: Card32,
    pub sbc_lo: Card32,
}
impl WaitMscReply {}
impl AsByteSequence for WaitMscReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.ust_hi.as_bytes(&mut bytes[index..]);
        index += self.ust_lo.as_bytes(&mut bytes[index..]);
        index += self.msc_hi.as_bytes(&mut bytes[index..]);
        index += self.msc_lo.as_bytes(&mut bytes[index..]);
        index += self.sbc_hi.as_bytes(&mut bytes[index..]);
        index += self.sbc_lo.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing WaitMscReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ust_hi, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ust_lo, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (msc_hi, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (msc_lo, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sbc_hi, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sbc_lo, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            WaitMscReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                ust_hi: ust_hi,
                ust_lo: ust_lo,
                msc_hi: msc_hi,
                msc_lo: msc_lo,
                sbc_hi: sbc_hi,
                sbc_lo: sbc_lo,
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
            + self.ust_hi.size()
            + self.ust_lo.size()
            + self.msc_hi.size()
            + self.msc_lo.size()
            + self.sbc_hi.size()
            + self.sbc_lo.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct WaitSbcRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub target_sbc_hi: Card32,
    pub target_sbc_lo: Card32,
}
impl WaitSbcRequest {}
impl AsByteSequence for WaitSbcRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.target_sbc_hi.as_bytes(&mut bytes[index..]);
        index += self.target_sbc_lo.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing WaitSbcRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target_sbc_hi, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target_sbc_lo, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            WaitSbcRequest {
                req_type: req_type,
                length: length,
                drawable: drawable,
                target_sbc_hi: target_sbc_hi,
                target_sbc_lo: target_sbc_lo,
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
            + self.target_sbc_hi.size()
            + self.target_sbc_lo.size()
    }
}
impl Request for WaitSbcRequest {
    const OPCODE: u8 = 11;
    const EXTENSION: Option<&'static str> = Some("DRI2");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = WaitSbcReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct WaitSbcReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ust_hi: Card32,
    pub ust_lo: Card32,
    pub msc_hi: Card32,
    pub msc_lo: Card32,
    pub sbc_hi: Card32,
    pub sbc_lo: Card32,
}
impl WaitSbcReply {}
impl AsByteSequence for WaitSbcReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.ust_hi.as_bytes(&mut bytes[index..]);
        index += self.ust_lo.as_bytes(&mut bytes[index..]);
        index += self.msc_hi.as_bytes(&mut bytes[index..]);
        index += self.msc_lo.as_bytes(&mut bytes[index..]);
        index += self.sbc_hi.as_bytes(&mut bytes[index..]);
        index += self.sbc_lo.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing WaitSbcReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ust_hi, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ust_lo, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (msc_hi, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (msc_lo, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sbc_hi, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sbc_lo, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            WaitSbcReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                ust_hi: ust_hi,
                ust_lo: ust_lo,
                msc_hi: msc_hi,
                msc_lo: msc_lo,
                sbc_hi: sbc_hi,
                sbc_lo: sbc_lo,
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
            + self.ust_hi.size()
            + self.ust_lo.size()
            + self.msc_hi.size()
            + self.msc_lo.size()
            + self.sbc_hi.size()
            + self.sbc_lo.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SwapIntervalRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub interval: Card32,
}
impl SwapIntervalRequest {}
impl AsByteSequence for SwapIntervalRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.interval.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SwapIntervalRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (interval, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SwapIntervalRequest {
                req_type: req_type,
                length: length,
                drawable: drawable,
                interval: interval,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.drawable.size() + self.interval.size()
    }
}
impl Request for SwapIntervalRequest {
    const OPCODE: u8 = 12;
    const EXTENSION: Option<&'static str> = Some("DRI2");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetParamRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub param: Card32,
}
impl GetParamRequest {}
impl AsByteSequence for GetParamRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.param.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetParamRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (param, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetParamRequest {
                req_type: req_type,
                length: length,
                drawable: drawable,
                param: param,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.drawable.size() + self.param.size()
    }
}
impl Request for GetParamRequest {
    const OPCODE: u8 = 13;
    const EXTENSION: Option<&'static str> = Some("DRI2");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetParamReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetParamReply {
    pub reply_type: u8,
    pub is_param_recognized: bool,
    pub sequence: u16,
    pub length: u32,
    pub value_hi: Card32,
    pub value_lo: Card32,
}
impl GetParamReply {}
impl AsByteSequence for GetParamReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.is_param_recognized.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.value_hi.as_bytes(&mut bytes[index..]);
        index += self.value_lo.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetParamReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (is_param_recognized, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value_hi, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value_lo, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetParamReply {
                reply_type: reply_type,
                is_param_recognized: is_param_recognized,
                sequence: sequence,
                length: length,
                value_hi: value_hi,
                value_lo: value_lo,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.is_param_recognized.size()
            + self.sequence.size()
            + self.length.size()
            + self.value_hi.size()
            + self.value_lo.size()
    }
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum EventType {
    ExchangeComplete = 1,
    BlitComplete = 2,
    FlipComplete = 3,
}
impl AsByteSequence for EventType {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u16).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        match underlying {
            1 => Some((Self::ExchangeComplete, sz)),
            2 => Some((Self::BlitComplete, sz)),
            3 => Some((Self::FlipComplete, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u16>()
    }
}
impl Default for EventType {
    #[inline]
    fn default() -> EventType {
        EventType::ExchangeComplete
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct BufferSwapCompleteEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub event_type_: EventType,
    pub drawable: Drawable,
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
        let (event_type_, sz): (EventType, usize) = <EventType>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
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
    const OPCODE: u8 = 0;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct InvalidateBuffersEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub drawable: Drawable,
}
impl InvalidateBuffersEvent {}
impl AsByteSequence for InvalidateBuffersEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing InvalidateBuffersEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            InvalidateBuffersEvent {
                event_type: event_type,
                sequence: sequence,
                drawable: drawable,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size() + 1 + self.sequence.size() + self.drawable.size()
    }
}
impl crate::auto::Event for InvalidateBuffersEvent {
    const OPCODE: u8 = 1;
}
