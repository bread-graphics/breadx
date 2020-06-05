// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

use super::xproto::*;
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryVersionRequest {
    pub req_type: u8,
    pub length: u16,
    pub client_major: Card8,
    pub client_minor: Card8,
}
impl QueryVersionRequest {}
impl AsByteSequence for QueryVersionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.client_major.as_bytes(&mut bytes[index..]);
        index += self.client_minor.as_bytes(&mut bytes[index..]);
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
        let (client_major, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (client_minor, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryVersionRequest {
                req_type: req_type,
                length: length,
                client_major: client_major,
                client_minor: client_minor,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.client_major.size()
            + self.client_minor.size()
    }
}
impl Request for QueryVersionRequest {
    const OPCODE: u8 = 0;
    const EXTENSION: Option<&'static str> = Some("SELinux");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryVersionReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryVersionReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major: Card16,
    pub server_minor: Card16,
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
        index += self.server_major.as_bytes(&mut bytes[index..]);
        index += self.server_minor.as_bytes(&mut bytes[index..]);
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
        let (server_major, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (server_minor, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryVersionReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                server_major: server_major,
                server_minor: server_minor,
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
            + self.server_major.size()
            + self.server_minor.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetDeviceCreateContextRequest<'a> {
    pub req_type: u8,
    pub length: u16,
    pub context: Cow<'a, str>,
}
impl<'a> SetDeviceCreateContextRequest<'a> {}
impl<'a> AsByteSequence for SetDeviceCreateContextRequest<'a> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.context.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = string_as_bytes(&self.context, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetDeviceCreateContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            SetDeviceCreateContextRequest {
                req_type: req_type,
                length: length,
                context: context,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + ::core::mem::size_of::<Card32>() + {
            let block_len: usize = self.context.len();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
            block_len + pad
        }
    }
}
impl<'a> Request for SetDeviceCreateContextRequest<'a> {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = Some("SELinux");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetDeviceCreateContextRequest {
    pub req_type: u8,
    pub length: u16,
}
impl GetDeviceCreateContextRequest {}
impl AsByteSequence for GetDeviceCreateContextRequest {
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
        log::trace!("Deserializing GetDeviceCreateContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetDeviceCreateContextRequest {
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
impl Request for GetDeviceCreateContextRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = Some("SELinux");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetDeviceCreateContextReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetDeviceCreateContextReply<'b> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Cow<'b, str>,
}
impl<'b> GetDeviceCreateContextReply<'b> {}
impl<'b> AsByteSequence for GetDeviceCreateContextReply<'b> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.context.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = string_as_bytes(&self.context, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDeviceCreateContextReply from byte buffer");
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
        let (context, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            GetDeviceCreateContextReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                context: context,
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
                let block_len: usize = self.context.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetDeviceContextRequest<'c> {
    pub req_type: u8,
    pub length: u16,
    pub device: Card32,
    pub context: Cow<'c, str>,
}
impl<'c> SetDeviceContextRequest<'c> {}
impl<'c> AsByteSequence for SetDeviceContextRequest<'c> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.device.as_bytes(&mut bytes[index..]);
        index += (self.context.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = string_as_bytes(&self.context, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetDeviceContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            SetDeviceContextRequest {
                req_type: req_type,
                length: length,
                device: device,
                context: context,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.device.size()
            + ::core::mem::size_of::<Card32>()
            + {
                let block_len: usize = self.context.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
impl<'c> Request for SetDeviceContextRequest<'c> {
    const OPCODE: u8 = 3;
    const EXTENSION: Option<&'static str> = Some("SELinux");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetDeviceContextRequest {
    pub req_type: u8,
    pub length: u16,
    pub device: Card32,
}
impl GetDeviceContextRequest {}
impl AsByteSequence for GetDeviceContextRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.device.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDeviceContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetDeviceContextRequest {
                req_type: req_type,
                length: length,
                device: device,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.device.size()
    }
}
impl Request for GetDeviceContextRequest {
    const OPCODE: u8 = 4;
    const EXTENSION: Option<&'static str> = Some("SELinux");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetDeviceContextReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetDeviceContextReply<'d> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Cow<'d, str>,
}
impl<'d> GetDeviceContextReply<'d> {}
impl<'d> AsByteSequence for GetDeviceContextReply<'d> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.context.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = string_as_bytes(&self.context, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDeviceContextReply from byte buffer");
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
        let (context, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            GetDeviceContextReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                context: context,
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
                let block_len: usize = self.context.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetWindowCreateContextRequest<'e> {
    pub req_type: u8,
    pub length: u16,
    pub context: Cow<'e, str>,
}
impl<'e> SetWindowCreateContextRequest<'e> {}
impl<'e> AsByteSequence for SetWindowCreateContextRequest<'e> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.context.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = string_as_bytes(&self.context, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetWindowCreateContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            SetWindowCreateContextRequest {
                req_type: req_type,
                length: length,
                context: context,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + ::core::mem::size_of::<Card32>() + {
            let block_len: usize = self.context.len();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
            block_len + pad
        }
    }
}
impl<'e> Request for SetWindowCreateContextRequest<'e> {
    const OPCODE: u8 = 5;
    const EXTENSION: Option<&'static str> = Some("SELinux");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetWindowCreateContextRequest {
    pub req_type: u8,
    pub length: u16,
}
impl GetWindowCreateContextRequest {}
impl AsByteSequence for GetWindowCreateContextRequest {
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
        log::trace!("Deserializing GetWindowCreateContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetWindowCreateContextRequest {
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
impl Request for GetWindowCreateContextRequest {
    const OPCODE: u8 = 6;
    const EXTENSION: Option<&'static str> = Some("SELinux");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetWindowCreateContextReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetWindowCreateContextReply<'f> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Cow<'f, str>,
}
impl<'f> GetWindowCreateContextReply<'f> {}
impl<'f> AsByteSequence for GetWindowCreateContextReply<'f> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.context.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = string_as_bytes(&self.context, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetWindowCreateContextReply from byte buffer");
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
        let (context, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            GetWindowCreateContextReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                context: context,
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
                let block_len: usize = self.context.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetWindowContextRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
}
impl GetWindowContextRequest {}
impl AsByteSequence for GetWindowContextRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetWindowContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetWindowContextRequest {
                req_type: req_type,
                length: length,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size()
    }
}
impl Request for GetWindowContextRequest {
    const OPCODE: u8 = 7;
    const EXTENSION: Option<&'static str> = Some("SELinux");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetWindowContextReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetWindowContextReply<'g> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Cow<'g, str>,
}
impl<'g> GetWindowContextReply<'g> {}
impl<'g> AsByteSequence for GetWindowContextReply<'g> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.context.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = string_as_bytes(&self.context, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetWindowContextReply from byte buffer");
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
        let (context, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            GetWindowContextReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                context: context,
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
                let block_len: usize = self.context.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ListItem<'h, 'i> {
    pub name: Atom,
    pub object_context: Cow<'h, str>,
    pub data_context: Cow<'i, str>,
}
impl<'h, 'i> ListItem<'h, 'i> {}
impl<'h, 'i> AsByteSequence for ListItem<'h, 'i> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.name.as_bytes(&mut bytes[index..]);
        index += (self.object_context.len() as Card32).as_bytes(&mut bytes[index..]);
        index += (self.data_context.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = string_as_bytes(&self.object_context, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, 4);
        let block_len: usize = string_as_bytes(&self.data_context, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, 4);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListItem from byte buffer");
        let (name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (object_context, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, 4);
        let (data_context, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, 4);
        Some((
            ListItem {
                name: name,
                object_context: object_context,
                data_context: data_context,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.name.size()
            + ::core::mem::size_of::<Card32>()
            + ::core::mem::size_of::<Card32>()
            + {
                let block_len: usize = self.object_context.len();
                let pad: usize = buffer_pad(block_len, 4);
                block_len + pad
            }
            + {
                let block_len: usize = self.data_context.len();
                let pad: usize = buffer_pad(block_len, 4);
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetPropertyCreateContextRequest<'j> {
    pub req_type: u8,
    pub length: u16,
    pub context: Cow<'j, str>,
}
impl<'j> SetPropertyCreateContextRequest<'j> {}
impl<'j> AsByteSequence for SetPropertyCreateContextRequest<'j> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.context.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = string_as_bytes(&self.context, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetPropertyCreateContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            SetPropertyCreateContextRequest {
                req_type: req_type,
                length: length,
                context: context,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + ::core::mem::size_of::<Card32>() + {
            let block_len: usize = self.context.len();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
            block_len + pad
        }
    }
}
impl<'j> Request for SetPropertyCreateContextRequest<'j> {
    const OPCODE: u8 = 8;
    const EXTENSION: Option<&'static str> = Some("SELinux");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetPropertyCreateContextRequest {
    pub req_type: u8,
    pub length: u16,
}
impl GetPropertyCreateContextRequest {}
impl AsByteSequence for GetPropertyCreateContextRequest {
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
        log::trace!("Deserializing GetPropertyCreateContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetPropertyCreateContextRequest {
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
impl Request for GetPropertyCreateContextRequest {
    const OPCODE: u8 = 9;
    const EXTENSION: Option<&'static str> = Some("SELinux");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetPropertyCreateContextReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetPropertyCreateContextReply<'k> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Cow<'k, str>,
}
impl<'k> GetPropertyCreateContextReply<'k> {}
impl<'k> AsByteSequence for GetPropertyCreateContextReply<'k> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.context.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = string_as_bytes(&self.context, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPropertyCreateContextReply from byte buffer");
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
        let (context, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            GetPropertyCreateContextReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                context: context,
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
                let block_len: usize = self.context.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetPropertyUseContextRequest<'l> {
    pub req_type: u8,
    pub length: u16,
    pub context: Cow<'l, str>,
}
impl<'l> SetPropertyUseContextRequest<'l> {}
impl<'l> AsByteSequence for SetPropertyUseContextRequest<'l> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.context.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = string_as_bytes(&self.context, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetPropertyUseContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            SetPropertyUseContextRequest {
                req_type: req_type,
                length: length,
                context: context,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + ::core::mem::size_of::<Card32>() + {
            let block_len: usize = self.context.len();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
            block_len + pad
        }
    }
}
impl<'l> Request for SetPropertyUseContextRequest<'l> {
    const OPCODE: u8 = 10;
    const EXTENSION: Option<&'static str> = Some("SELinux");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetPropertyUseContextRequest {
    pub req_type: u8,
    pub length: u16,
}
impl GetPropertyUseContextRequest {}
impl AsByteSequence for GetPropertyUseContextRequest {
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
        log::trace!("Deserializing GetPropertyUseContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetPropertyUseContextRequest {
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
impl Request for GetPropertyUseContextRequest {
    const OPCODE: u8 = 11;
    const EXTENSION: Option<&'static str> = Some("SELinux");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetPropertyUseContextReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetPropertyUseContextReply<'m> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Cow<'m, str>,
}
impl<'m> GetPropertyUseContextReply<'m> {}
impl<'m> AsByteSequence for GetPropertyUseContextReply<'m> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.context.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = string_as_bytes(&self.context, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPropertyUseContextReply from byte buffer");
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
        let (context, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            GetPropertyUseContextReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                context: context,
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
                let block_len: usize = self.context.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetPropertyContextRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub property: Atom,
}
impl GetPropertyContextRequest {}
impl AsByteSequence for GetPropertyContextRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.property.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPropertyContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetPropertyContextRequest {
                req_type: req_type,
                length: length,
                window: window,
                property: property,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size() + self.property.size()
    }
}
impl Request for GetPropertyContextRequest {
    const OPCODE: u8 = 12;
    const EXTENSION: Option<&'static str> = Some("SELinux");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetPropertyContextReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetPropertyContextReply<'n> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Cow<'n, str>,
}
impl<'n> GetPropertyContextReply<'n> {}
impl<'n> AsByteSequence for GetPropertyContextReply<'n> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.context.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = string_as_bytes(&self.context, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPropertyContextReply from byte buffer");
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
        let (context, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            GetPropertyContextReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                context: context,
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
                let block_len: usize = self.context.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetPropertyDataContextRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub property: Atom,
}
impl GetPropertyDataContextRequest {}
impl AsByteSequence for GetPropertyDataContextRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.property.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPropertyDataContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetPropertyDataContextRequest {
                req_type: req_type,
                length: length,
                window: window,
                property: property,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size() + self.property.size()
    }
}
impl Request for GetPropertyDataContextRequest {
    const OPCODE: u8 = 13;
    const EXTENSION: Option<&'static str> = Some("SELinux");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetPropertyDataContextReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetPropertyDataContextReply<'o> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Cow<'o, str>,
}
impl<'o> GetPropertyDataContextReply<'o> {}
impl<'o> AsByteSequence for GetPropertyDataContextReply<'o> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.context.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = string_as_bytes(&self.context, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPropertyDataContextReply from byte buffer");
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
        let (context, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            GetPropertyDataContextReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                context: context,
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
                let block_len: usize = self.context.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ListPropertiesRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
}
impl ListPropertiesRequest {}
impl AsByteSequence for ListPropertiesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListPropertiesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ListPropertiesRequest {
                req_type: req_type,
                length: length,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size()
    }
}
impl Request for ListPropertiesRequest {
    const OPCODE: u8 = 14;
    const EXTENSION: Option<&'static str> = Some("SELinux");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ListPropertiesReply<'static, 'static, 'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ListPropertiesReply<'r, 'p, 'q> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub properties: Cow<'r, [ListItem<'p, 'q>]>,
}
impl<'r, 'p, 'q> ListPropertiesReply<'r, 'p, 'q> {}
impl<'r, 'p, 'q> AsByteSequence for ListPropertiesReply<'r, 'p, 'q> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.properties.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.properties, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ListItem<'p, 'q>>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListPropertiesReply from byte buffer");
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
        let (properties, block_len): (Cow<'_, [ListItem<'_, '_>]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ListItem<'p, 'q>>());
        Some((
            ListPropertiesReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                properties: properties,
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
                let block_len: usize = self.properties.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<ListItem<'p, 'q>>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetSelectionCreateContextRequest<'s> {
    pub req_type: u8,
    pub length: u16,
    pub context: Cow<'s, str>,
}
impl<'s> SetSelectionCreateContextRequest<'s> {}
impl<'s> AsByteSequence for SetSelectionCreateContextRequest<'s> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.context.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = string_as_bytes(&self.context, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetSelectionCreateContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            SetSelectionCreateContextRequest {
                req_type: req_type,
                length: length,
                context: context,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + ::core::mem::size_of::<Card32>() + {
            let block_len: usize = self.context.len();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
            block_len + pad
        }
    }
}
impl<'s> Request for SetSelectionCreateContextRequest<'s> {
    const OPCODE: u8 = 15;
    const EXTENSION: Option<&'static str> = Some("SELinux");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetSelectionCreateContextRequest {
    pub req_type: u8,
    pub length: u16,
}
impl GetSelectionCreateContextRequest {}
impl AsByteSequence for GetSelectionCreateContextRequest {
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
        log::trace!("Deserializing GetSelectionCreateContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetSelectionCreateContextRequest {
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
impl Request for GetSelectionCreateContextRequest {
    const OPCODE: u8 = 16;
    const EXTENSION: Option<&'static str> = Some("SELinux");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetSelectionCreateContextReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetSelectionCreateContextReply<'t> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Cow<'t, str>,
}
impl<'t> GetSelectionCreateContextReply<'t> {}
impl<'t> AsByteSequence for GetSelectionCreateContextReply<'t> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.context.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = string_as_bytes(&self.context, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetSelectionCreateContextReply from byte buffer");
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
        let (context, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            GetSelectionCreateContextReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                context: context,
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
                let block_len: usize = self.context.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetSelectionUseContextRequest<'u> {
    pub req_type: u8,
    pub length: u16,
    pub context: Cow<'u, str>,
}
impl<'u> SetSelectionUseContextRequest<'u> {}
impl<'u> AsByteSequence for SetSelectionUseContextRequest<'u> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.context.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = string_as_bytes(&self.context, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetSelectionUseContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            SetSelectionUseContextRequest {
                req_type: req_type,
                length: length,
                context: context,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + ::core::mem::size_of::<Card32>() + {
            let block_len: usize = self.context.len();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
            block_len + pad
        }
    }
}
impl<'u> Request for SetSelectionUseContextRequest<'u> {
    const OPCODE: u8 = 17;
    const EXTENSION: Option<&'static str> = Some("SELinux");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetSelectionUseContextRequest {
    pub req_type: u8,
    pub length: u16,
}
impl GetSelectionUseContextRequest {}
impl AsByteSequence for GetSelectionUseContextRequest {
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
        log::trace!("Deserializing GetSelectionUseContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetSelectionUseContextRequest {
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
impl Request for GetSelectionUseContextRequest {
    const OPCODE: u8 = 18;
    const EXTENSION: Option<&'static str> = Some("SELinux");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetSelectionUseContextReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetSelectionUseContextReply<'v> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Cow<'v, str>,
}
impl<'v> GetSelectionUseContextReply<'v> {}
impl<'v> AsByteSequence for GetSelectionUseContextReply<'v> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.context.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = string_as_bytes(&self.context, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetSelectionUseContextReply from byte buffer");
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
        let (context, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            GetSelectionUseContextReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                context: context,
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
                let block_len: usize = self.context.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetSelectionContextRequest {
    pub req_type: u8,
    pub length: u16,
    pub selection: Atom,
}
impl GetSelectionContextRequest {}
impl AsByteSequence for GetSelectionContextRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.selection.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetSelectionContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (selection, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetSelectionContextRequest {
                req_type: req_type,
                length: length,
                selection: selection,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.selection.size()
    }
}
impl Request for GetSelectionContextRequest {
    const OPCODE: u8 = 19;
    const EXTENSION: Option<&'static str> = Some("SELinux");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetSelectionContextReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetSelectionContextReply<'w> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Cow<'w, str>,
}
impl<'w> GetSelectionContextReply<'w> {}
impl<'w> AsByteSequence for GetSelectionContextReply<'w> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.context.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = string_as_bytes(&self.context, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetSelectionContextReply from byte buffer");
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
        let (context, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            GetSelectionContextReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                context: context,
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
                let block_len: usize = self.context.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetSelectionDataContextRequest {
    pub req_type: u8,
    pub length: u16,
    pub selection: Atom,
}
impl GetSelectionDataContextRequest {}
impl AsByteSequence for GetSelectionDataContextRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.selection.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetSelectionDataContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (selection, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetSelectionDataContextRequest {
                req_type: req_type,
                length: length,
                selection: selection,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.selection.size()
    }
}
impl Request for GetSelectionDataContextRequest {
    const OPCODE: u8 = 20;
    const EXTENSION: Option<&'static str> = Some("SELinux");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetSelectionDataContextReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetSelectionDataContextReply<'x> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Cow<'x, str>,
}
impl<'x> GetSelectionDataContextReply<'x> {}
impl<'x> AsByteSequence for GetSelectionDataContextReply<'x> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.context.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = string_as_bytes(&self.context, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetSelectionDataContextReply from byte buffer");
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
        let (context, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            GetSelectionDataContextReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                context: context,
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
                let block_len: usize = self.context.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ListSelectionsRequest {
    pub req_type: u8,
    pub length: u16,
}
impl ListSelectionsRequest {}
impl AsByteSequence for ListSelectionsRequest {
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
        log::trace!("Deserializing ListSelectionsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ListSelectionsRequest {
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
impl Request for ListSelectionsRequest {
    const OPCODE: u8 = 21;
    const EXTENSION: Option<&'static str> = Some("SELinux");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ListSelectionsReply<'static, 'static, 'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ListSelectionsReply<'ab, 'y, 'z> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub selections: Cow<'ab, [ListItem<'y, 'z>]>,
}
impl<'ab, 'y, 'z> ListSelectionsReply<'ab, 'y, 'z> {}
impl<'ab, 'y, 'z> AsByteSequence for ListSelectionsReply<'ab, 'y, 'z> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.selections.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.selections, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ListItem<'y, 'z>>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListSelectionsReply from byte buffer");
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
        let (selections, block_len): (Cow<'_, [ListItem<'_, '_>]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ListItem<'y, 'z>>());
        Some((
            ListSelectionsReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                selections: selections,
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
                let block_len: usize = self.selections.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<ListItem<'y, 'z>>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetClientContextRequest {
    pub req_type: u8,
    pub length: u16,
    pub resource: Card32,
}
impl GetClientContextRequest {}
impl AsByteSequence for GetClientContextRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.resource.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetClientContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (resource, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetClientContextRequest {
                req_type: req_type,
                length: length,
                resource: resource,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.resource.size()
    }
}
impl Request for GetClientContextRequest {
    const OPCODE: u8 = 22;
    const EXTENSION: Option<&'static str> = Some("SELinux");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetClientContextReply<'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetClientContextReply<'bb> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Cow<'bb, str>,
}
impl<'bb> GetClientContextReply<'bb> {}
impl<'bb> AsByteSequence for GetClientContextReply<'bb> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.context.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = string_as_bytes(&self.context, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetClientContextReply from byte buffer");
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
        let (context, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            GetClientContextReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                context: context,
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
                let block_len: usize = self.context.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
