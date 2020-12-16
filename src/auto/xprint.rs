// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

use super::xproto::*;
pub type String8 = Char;
#[derive(Clone, Debug, Default)]
pub struct Printer {
    pub name: Vec<String8>,
    pub description: Vec<String8>,
}
impl Printer {}
impl AsByteSequence for Printer {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += (self.name.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, 4);
        index += (self.description.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.description, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, 4);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Printer from byte buffer");
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (name, block_len): (Vec<String8>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, 4);
        let (len1, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (description, block_len): (Vec<String8>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, 4);
        Some((
            Printer {
                name: name,
                description: description,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<Card32>()
            + {
                let block_len: usize = self.name.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, 4);
                block_len + pad
            }
            + ::core::mem::size_of::<Card32>()
            + {
                let block_len: usize = self.description.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, 4);
                block_len + pad
            }
    }
}
#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pcontext {
    pub xid: XID,
}
impl Pcontext {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Pcontext {
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
pub struct PrintQueryVersionRequest {
    pub req_type: u8,
    pub length: u16,
}
impl PrintQueryVersionRequest {}
impl AsByteSequence for PrintQueryVersionRequest {
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
        log::trace!("Deserializing PrintQueryVersionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PrintQueryVersionRequest {
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
impl Request for PrintQueryVersionRequest {
    const OPCODE: u8 = 0;
    const EXTENSION: Option<&'static str> = Some("XpExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = PrintQueryVersionReply;
}
#[derive(Clone, Debug, Default)]
pub struct PrintQueryVersionReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: Card16,
    pub minor_version: Card16,
}
impl PrintQueryVersionReply {}
impl AsByteSequence for PrintQueryVersionReply {
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
        log::trace!("Deserializing PrintQueryVersionReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PrintQueryVersionReply {
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
pub struct PrintGetPrinterListRequest {
    pub req_type: u8,
    pub length: u16,
    pub printer_name: Vec<String8>,
    pub locale: Vec<String8>,
}
impl PrintGetPrinterListRequest {}
impl AsByteSequence for PrintGetPrinterListRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += (self.printer_name.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.locale.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.printer_name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<String8>());
        let block_len: usize = vector_as_bytes(&self.locale, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<String8>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PrintGetPrinterListRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (printer_name, block_len): (Vec<String8>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<String8>());
        let (locale, block_len): (Vec<String8>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<String8>());
        Some((
            PrintGetPrinterListRequest {
                req_type: req_type,
                length: length,
                printer_name: printer_name,
                locale: locale,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + ::core::mem::size_of::<Card32>()
            + self.length.size()
            + ::core::mem::size_of::<Card32>()
            + {
                let block_len: usize = self.printer_name.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<String8>());
                block_len + pad
            }
            + {
                let block_len: usize = self.locale.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<String8>());
                block_len + pad
            }
    }
}
impl Request for PrintGetPrinterListRequest {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = Some("XpExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = PrintGetPrinterListReply;
}
#[derive(Clone, Debug, Default)]
pub struct PrintGetPrinterListReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub printers: Vec<Printer>,
}
impl PrintGetPrinterListReply {}
impl AsByteSequence for PrintGetPrinterListReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.printers.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.printers, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Printer>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PrintGetPrinterListReply from byte buffer");
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
        let (printers, block_len): (Vec<Printer>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Printer>());
        Some((
            PrintGetPrinterListReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                printers: printers,
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
                let block_len: usize = self.printers.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Printer>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct PrintRehashPrinterListRequest {
    pub req_type: u8,
    pub length: u16,
}
impl PrintRehashPrinterListRequest {}
impl AsByteSequence for PrintRehashPrinterListRequest {
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
        log::trace!("Deserializing PrintRehashPrinterListRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PrintRehashPrinterListRequest {
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
impl Request for PrintRehashPrinterListRequest {
    const OPCODE: u8 = 20;
    const EXTENSION: Option<&'static str> = Some("XpExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CreateContextRequest {
    pub req_type: u8,
    pub context_id: Card32,
    pub length: u16,
    pub printer_name: Vec<String8>,
    pub locale: Vec<String8>,
}
impl CreateContextRequest {}
impl AsByteSequence for CreateContextRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context_id.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.printer_name.len() as Card32).as_bytes(&mut bytes[index..]);
        index += (self.locale.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.printer_name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<String8>());
        let block_len: usize = vector_as_bytes(&self.locale, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<String8>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context_id, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (printer_name, block_len): (Vec<String8>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<String8>());
        let (locale, block_len): (Vec<String8>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<String8>());
        Some((
            CreateContextRequest {
                req_type: req_type,
                context_id: context_id,
                length: length,
                printer_name: printer_name,
                locale: locale,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context_id.size()
            + self.length.size()
            + ::core::mem::size_of::<Card32>()
            + ::core::mem::size_of::<Card32>()
            + {
                let block_len: usize = self.printer_name.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<String8>());
                block_len + pad
            }
            + {
                let block_len: usize = self.locale.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<String8>());
                block_len + pad
            }
    }
}
impl Request for CreateContextRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = Some("XpExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct PrintSetContextRequest {
    pub req_type: u8,
    pub context: Card32,
    pub length: u16,
}
impl PrintSetContextRequest {}
impl AsByteSequence for PrintSetContextRequest {
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
        log::trace!("Deserializing PrintSetContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PrintSetContextRequest {
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
impl Request for PrintSetContextRequest {
    const OPCODE: u8 = 3;
    const EXTENSION: Option<&'static str> = Some("XpExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct PrintGetContextRequest {
    pub req_type: u8,
    pub length: u16,
}
impl PrintGetContextRequest {}
impl AsByteSequence for PrintGetContextRequest {
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
        log::trace!("Deserializing PrintGetContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PrintGetContextRequest {
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
impl Request for PrintGetContextRequest {
    const OPCODE: u8 = 4;
    const EXTENSION: Option<&'static str> = Some("XpExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = PrintGetContextReply;
}
#[derive(Clone, Debug, Default)]
pub struct PrintGetContextReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Card32,
}
impl PrintGetContextReply {}
impl AsByteSequence for PrintGetContextReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PrintGetContextReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PrintGetContextReply {
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
        self.reply_type.size() + 1 + self.sequence.size() + self.length.size() + self.context.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct PrintDestroyContextRequest {
    pub req_type: u8,
    pub context: Card32,
    pub length: u16,
}
impl PrintDestroyContextRequest {}
impl AsByteSequence for PrintDestroyContextRequest {
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
        log::trace!("Deserializing PrintDestroyContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PrintDestroyContextRequest {
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
impl Request for PrintDestroyContextRequest {
    const OPCODE: u8 = 5;
    const EXTENSION: Option<&'static str> = Some("XpExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct PrintGetScreenOfContextRequest {
    pub req_type: u8,
    pub length: u16,
}
impl PrintGetScreenOfContextRequest {}
impl AsByteSequence for PrintGetScreenOfContextRequest {
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
        log::trace!("Deserializing PrintGetScreenOfContextRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PrintGetScreenOfContextRequest {
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
impl Request for PrintGetScreenOfContextRequest {
    const OPCODE: u8 = 6;
    const EXTENSION: Option<&'static str> = Some("XpExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = PrintGetScreenOfContextReply;
}
#[derive(Clone, Debug, Default)]
pub struct PrintGetScreenOfContextReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: Window,
}
impl PrintGetScreenOfContextReply {}
impl AsByteSequence for PrintGetScreenOfContextReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PrintGetScreenOfContextReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PrintGetScreenOfContextReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                root: root,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + 1 + self.sequence.size() + self.length.size() + self.root.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct PrintStartJobRequest {
    pub req_type: u8,
    pub output_mode: Card8,
    pub length: u16,
}
impl PrintStartJobRequest {}
impl AsByteSequence for PrintStartJobRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.output_mode.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PrintStartJobRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (output_mode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PrintStartJobRequest {
                req_type: req_type,
                output_mode: output_mode,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.output_mode.size() + self.length.size()
    }
}
impl Request for PrintStartJobRequest {
    const OPCODE: u8 = 7;
    const EXTENSION: Option<&'static str> = Some("XpExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct PrintEndJobRequest {
    pub req_type: u8,
    pub cancel: bool,
    pub length: u16,
}
impl PrintEndJobRequest {}
impl AsByteSequence for PrintEndJobRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.cancel.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PrintEndJobRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cancel, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PrintEndJobRequest {
                req_type: req_type,
                cancel: cancel,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.cancel.size() + self.length.size()
    }
}
impl Request for PrintEndJobRequest {
    const OPCODE: u8 = 8;
    const EXTENSION: Option<&'static str> = Some("XpExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct PrintStartDocRequest {
    pub req_type: u8,
    pub driver_mode: Card8,
    pub length: u16,
}
impl PrintStartDocRequest {}
impl AsByteSequence for PrintStartDocRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.driver_mode.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PrintStartDocRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (driver_mode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PrintStartDocRequest {
                req_type: req_type,
                driver_mode: driver_mode,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.driver_mode.size() + self.length.size()
    }
}
impl Request for PrintStartDocRequest {
    const OPCODE: u8 = 9;
    const EXTENSION: Option<&'static str> = Some("XpExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct PrintEndDocRequest {
    pub req_type: u8,
    pub cancel: bool,
    pub length: u16,
}
impl PrintEndDocRequest {}
impl AsByteSequence for PrintEndDocRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.cancel.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PrintEndDocRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cancel, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PrintEndDocRequest {
                req_type: req_type,
                cancel: cancel,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.cancel.size() + self.length.size()
    }
}
impl Request for PrintEndDocRequest {
    const OPCODE: u8 = 10;
    const EXTENSION: Option<&'static str> = Some("XpExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct PrintPutDocumentDataRequest {
    pub req_type: u8,
    pub drawable: Drawable,
    pub length: u16,
    pub data: Vec<Byte>,
    pub doc_format: Vec<String8>,
    pub options: Vec<String8>,
}
impl PrintPutDocumentDataRequest {}
impl AsByteSequence for PrintPutDocumentDataRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += (self.doc_format.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.options.len() as Card16).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        let block_len: usize = vector_as_bytes(&self.doc_format, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<String8>());
        let block_len: usize = vector_as_bytes(&self.options, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<String8>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PrintPutDocumentDataRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len2, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (data, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        let (doc_format, block_len): (Vec<String8>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<String8>());
        let (options, block_len): (Vec<String8>, usize) =
            vector_from_bytes(&bytes[index..], len2 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<String8>());
        Some((
            PrintPutDocumentDataRequest {
                req_type: req_type,
                drawable: drawable,
                length: length,
                data: data,
                doc_format: doc_format,
                options: options,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.drawable.size()
            + self.length.size()
            + ::core::mem::size_of::<Card32>()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
            + {
                let block_len: usize = self.doc_format.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<String8>());
                block_len + pad
            }
            + {
                let block_len: usize = self.options.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<String8>());
                block_len + pad
            }
    }
}
impl Request for PrintPutDocumentDataRequest {
    const OPCODE: u8 = 11;
    const EXTENSION: Option<&'static str> = Some("XpExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct PrintGetDocumentDataRequest {
    pub req_type: u8,
    pub context: Pcontext,
    pub length: u16,
    pub max_bytes: Card32,
}
impl PrintGetDocumentDataRequest {}
impl AsByteSequence for PrintGetDocumentDataRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.max_bytes.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PrintGetDocumentDataRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (Pcontext, usize) = <Pcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_bytes, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PrintGetDocumentDataRequest {
                req_type: req_type,
                context: context,
                length: length,
                max_bytes: max_bytes,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context.size() + self.length.size() + self.max_bytes.size()
    }
}
impl Request for PrintGetDocumentDataRequest {
    const OPCODE: u8 = 12;
    const EXTENSION: Option<&'static str> = Some("XpExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = PrintGetDocumentDataReply;
}
#[derive(Clone, Debug, Default)]
pub struct PrintGetDocumentDataReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status_code: Card32,
    pub finished_flag: Card32,
    pub data: Vec<Byte>,
}
impl PrintGetDocumentDataReply {}
impl AsByteSequence for PrintGetDocumentDataReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.status_code.as_bytes(&mut bytes[index..]);
        index += self.finished_flag.as_bytes(&mut bytes[index..]);
        index += (self.data.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PrintGetDocumentDataReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status_code, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (finished_flag, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (data, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            PrintGetDocumentDataReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                status_code: status_code,
                finished_flag: finished_flag,
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
            + self.status_code.size()
            + self.finished_flag.size()
            + ::core::mem::size_of::<Card32>()
            + 12
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct PrintStartPageRequest {
    pub req_type: u8,
    pub window: Window,
    pub length: u16,
}
impl PrintStartPageRequest {}
impl AsByteSequence for PrintStartPageRequest {
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
        log::trace!("Deserializing PrintStartPageRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PrintStartPageRequest {
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
impl Request for PrintStartPageRequest {
    const OPCODE: u8 = 13;
    const EXTENSION: Option<&'static str> = Some("XpExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct PrintEndPageRequest {
    pub req_type: u8,
    pub cancel: bool,
    pub length: u16,
}
impl PrintEndPageRequest {}
impl AsByteSequence for PrintEndPageRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.cancel.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PrintEndPageRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cancel, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            PrintEndPageRequest {
                req_type: req_type,
                cancel: cancel,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.cancel.size() + self.length.size() + 3
    }
}
impl Request for PrintEndPageRequest {
    const OPCODE: u8 = 14;
    const EXTENSION: Option<&'static str> = Some("XpExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct PrintSelectInputRequest {
    pub req_type: u8,
    pub context: Pcontext,
    pub length: u16,
    pub event_mask: Card32,
}
impl PrintSelectInputRequest {}
impl AsByteSequence for PrintSelectInputRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.event_mask.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PrintSelectInputRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (Pcontext, usize) = <Pcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PrintSelectInputRequest {
                req_type: req_type,
                context: context,
                length: length,
                event_mask: event_mask,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context.size() + self.length.size() + self.event_mask.size()
    }
}
impl Request for PrintSelectInputRequest {
    const OPCODE: u8 = 15;
    const EXTENSION: Option<&'static str> = Some("XpExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct PrintInputSelectedRequest {
    pub req_type: u8,
    pub context: Pcontext,
    pub length: u16,
}
impl PrintInputSelectedRequest {}
impl AsByteSequence for PrintInputSelectedRequest {
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
        log::trace!("Deserializing PrintInputSelectedRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (Pcontext, usize) = <Pcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PrintInputSelectedRequest {
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
impl Request for PrintInputSelectedRequest {
    const OPCODE: u8 = 16;
    const EXTENSION: Option<&'static str> = Some("XpExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = PrintInputSelectedReply;
}
#[derive(Clone, Debug, Default)]
pub struct PrintInputSelectedReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_mask: Card32,
    pub all_events_mask: Card32,
}
impl PrintInputSelectedReply {}
impl AsByteSequence for PrintInputSelectedReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.event_mask.as_bytes(&mut bytes[index..]);
        index += self.all_events_mask.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PrintInputSelectedReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (all_events_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PrintInputSelectedReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                event_mask: event_mask,
                all_events_mask: all_events_mask,
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
            + self.event_mask.size()
            + self.all_events_mask.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct PrintGetAttributesRequest {
    pub req_type: u8,
    pub context: Pcontext,
    pub length: u16,
    pub pool: Card8,
}
impl PrintGetAttributesRequest {}
impl AsByteSequence for PrintGetAttributesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.pool.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PrintGetAttributesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (Pcontext, usize) = <Pcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pool, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            PrintGetAttributesRequest {
                req_type: req_type,
                context: context,
                length: length,
                pool: pool,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.context.size() + self.length.size() + self.pool.size() + 3
    }
}
impl Request for PrintGetAttributesRequest {
    const OPCODE: u8 = 17;
    const EXTENSION: Option<&'static str> = Some("XpExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = PrintGetAttributesReply;
}
#[derive(Clone, Debug, Default)]
pub struct PrintGetAttributesReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub attributes: Vec<String8>,
}
impl PrintGetAttributesReply {}
impl AsByteSequence for PrintGetAttributesReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.attributes.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.attributes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<String8>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PrintGetAttributesReply from byte buffer");
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
        let (attributes, block_len): (Vec<String8>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<String8>());
        Some((
            PrintGetAttributesReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                attributes: attributes,
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
                let block_len: usize = self.attributes.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<String8>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct PrintGetOneAttributesRequest {
    pub req_type: u8,
    pub context: Pcontext,
    pub length: u16,
    pub pool: Card8,
    pub name: Vec<String8>,
}
impl PrintGetOneAttributesRequest {}
impl AsByteSequence for PrintGetOneAttributesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.name.len() as Card32).as_bytes(&mut bytes[index..]);
        index += self.pool.as_bytes(&mut bytes[index..]);
        index += 3;
        let block_len: usize = vector_as_bytes(&self.name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<String8>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PrintGetOneAttributesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (Pcontext, usize) = <Pcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pool, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (name, block_len): (Vec<String8>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<String8>());
        Some((
            PrintGetOneAttributesRequest {
                req_type: req_type,
                context: context,
                length: length,
                pool: pool,
                name: name,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context.size()
            + self.length.size()
            + ::core::mem::size_of::<Card32>()
            + self.pool.size()
            + 3
            + {
                let block_len: usize = self.name.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<String8>());
                block_len + pad
            }
    }
}
impl Request for PrintGetOneAttributesRequest {
    const OPCODE: u8 = 19;
    const EXTENSION: Option<&'static str> = Some("XpExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = PrintGetOneAttributesReply;
}
#[derive(Clone, Debug, Default)]
pub struct PrintGetOneAttributesReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub value: Vec<String8>,
}
impl PrintGetOneAttributesReply {}
impl AsByteSequence for PrintGetOneAttributesReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.value.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.value, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<String8>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PrintGetOneAttributesReply from byte buffer");
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
        let (value, block_len): (Vec<String8>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<String8>());
        Some((
            PrintGetOneAttributesReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                value: value,
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
                let block_len: usize = self.value.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<String8>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct PrintSetAttributesRequest {
    pub req_type: u8,
    pub context: Pcontext,
    pub length: u16,
    pub string_len: Card32,
    pub pool: Card8,
    pub rule: Card8,
    pub attributes: Vec<String8>,
}
impl PrintSetAttributesRequest {}
impl AsByteSequence for PrintSetAttributesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.string_len.as_bytes(&mut bytes[index..]);
        index += self.pool.as_bytes(&mut bytes[index..]);
        index += self.rule.as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = vector_as_bytes(&self.attributes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<String8>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PrintSetAttributesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (Pcontext, usize) = <Pcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (string_len, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pool, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (rule, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (attributes, block_len): (Vec<String8>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<String8>());
        Some((
            PrintSetAttributesRequest {
                req_type: req_type,
                context: context,
                length: length,
                string_len: string_len,
                pool: pool,
                rule: rule,
                attributes: attributes,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context.size()
            + self.length.size()
            + self.string_len.size()
            + self.pool.size()
            + self.rule.size()
            + 2
            + {
                let block_len: usize = self.attributes.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<String8>());
                block_len + pad
            }
    }
}
impl Request for PrintSetAttributesRequest {
    const OPCODE: u8 = 18;
    const EXTENSION: Option<&'static str> = Some("XpExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct PrintGetPageDimensionsRequest {
    pub req_type: u8,
    pub context: Pcontext,
    pub length: u16,
}
impl PrintGetPageDimensionsRequest {}
impl AsByteSequence for PrintGetPageDimensionsRequest {
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
        log::trace!("Deserializing PrintGetPageDimensionsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (Pcontext, usize) = <Pcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PrintGetPageDimensionsRequest {
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
impl Request for PrintGetPageDimensionsRequest {
    const OPCODE: u8 = 21;
    const EXTENSION: Option<&'static str> = Some("XpExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = PrintGetPageDimensionsReply;
}
#[derive(Clone, Debug, Default)]
pub struct PrintGetPageDimensionsReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: Card16,
    pub height: Card16,
    pub offset_x: Card16,
    pub offset_y: Card16,
    pub reproducible_width: Card16,
    pub reproducible_height: Card16,
}
impl PrintGetPageDimensionsReply {}
impl AsByteSequence for PrintGetPageDimensionsReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.offset_x.as_bytes(&mut bytes[index..]);
        index += self.offset_y.as_bytes(&mut bytes[index..]);
        index += self.reproducible_width.as_bytes(&mut bytes[index..]);
        index += self.reproducible_height.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PrintGetPageDimensionsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (offset_x, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (offset_y, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (reproducible_width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (reproducible_height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PrintGetPageDimensionsReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                width: width,
                height: height,
                offset_x: offset_x,
                offset_y: offset_y,
                reproducible_width: reproducible_width,
                reproducible_height: reproducible_height,
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
            + self.offset_x.size()
            + self.offset_y.size()
            + self.reproducible_width.size()
            + self.reproducible_height.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct PrintQueryScreensRequest {
    pub req_type: u8,
    pub length: u16,
}
impl PrintQueryScreensRequest {}
impl AsByteSequence for PrintQueryScreensRequest {
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
        log::trace!("Deserializing PrintQueryScreensRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PrintQueryScreensRequest {
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
impl Request for PrintQueryScreensRequest {
    const OPCODE: u8 = 22;
    const EXTENSION: Option<&'static str> = Some("XpExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = PrintQueryScreensReply;
}
#[derive(Clone, Debug, Default)]
pub struct PrintQueryScreensReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub roots: Vec<Window>,
}
impl PrintQueryScreensReply {}
impl AsByteSequence for PrintQueryScreensReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.roots.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.roots, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Window>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PrintQueryScreensReply from byte buffer");
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
        let (roots, block_len): (Vec<Window>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Window>());
        Some((
            PrintQueryScreensReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                roots: roots,
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
                let block_len: usize = self.roots.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Window>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct PrintSetImageResolutionRequest {
    pub req_type: u8,
    pub context: Pcontext,
    pub length: u16,
    pub image_resolution: Card16,
}
impl PrintSetImageResolutionRequest {}
impl AsByteSequence for PrintSetImageResolutionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.image_resolution.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PrintSetImageResolutionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (Pcontext, usize) = <Pcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (image_resolution, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PrintSetImageResolutionRequest {
                req_type: req_type,
                context: context,
                length: length,
                image_resolution: image_resolution,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.context.size()
            + self.length.size()
            + self.image_resolution.size()
    }
}
impl Request for PrintSetImageResolutionRequest {
    const OPCODE: u8 = 23;
    const EXTENSION: Option<&'static str> = Some("XpExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = PrintSetImageResolutionReply;
}
#[derive(Clone, Debug, Default)]
pub struct PrintSetImageResolutionReply {
    pub reply_type: u8,
    pub status: bool,
    pub sequence: u16,
    pub length: u32,
    pub previous_resolutions: Card16,
}
impl PrintSetImageResolutionReply {}
impl AsByteSequence for PrintSetImageResolutionReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.previous_resolutions.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PrintSetImageResolutionReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (previous_resolutions, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PrintSetImageResolutionReply {
                reply_type: reply_type,
                status: status,
                sequence: sequence,
                length: length,
                previous_resolutions: previous_resolutions,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.status.size()
            + self.sequence.size()
            + self.length.size()
            + self.previous_resolutions.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct PrintGetImageResolutionRequest {
    pub req_type: u8,
    pub context: Pcontext,
    pub length: u16,
}
impl PrintGetImageResolutionRequest {}
impl AsByteSequence for PrintGetImageResolutionRequest {
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
        log::trace!("Deserializing PrintGetImageResolutionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (Pcontext, usize) = <Pcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PrintGetImageResolutionRequest {
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
impl Request for PrintGetImageResolutionRequest {
    const OPCODE: u8 = 24;
    const EXTENSION: Option<&'static str> = Some("XpExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = PrintGetImageResolutionReply;
}
#[derive(Clone, Debug, Default)]
pub struct PrintGetImageResolutionReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub image_resolution: Card16,
}
impl PrintGetImageResolutionReply {}
impl AsByteSequence for PrintGetImageResolutionReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.image_resolution.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PrintGetImageResolutionReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (image_resolution, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PrintGetImageResolutionReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                image_resolution: image_resolution,
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
            + self.image_resolution.size()
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Attr {
    JobAttr = 1,
    DocAttr = 2,
    PageAttr = 3,
    PrinterAttr = 4,
    ServerAttr = 5,
    MediumAttr = 6,
    SpoolerAttr = 7,
}
impl AsByteSequence for Attr {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            1 => Some((Self::JobAttr, sz)),
            2 => Some((Self::DocAttr, sz)),
            3 => Some((Self::PageAttr, sz)),
            4 => Some((Self::PrinterAttr, sz)),
            5 => Some((Self::ServerAttr, sz)),
            6 => Some((Self::MediumAttr, sz)),
            7 => Some((Self::SpoolerAttr, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for Attr {
    #[inline]
    fn default() -> Attr {
        Attr::JobAttr
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct EvMask {
    pub inner: i32,
}
impl EvMask {
    #[inline]
    pub fn print_mask(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_print_mask(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn attribute_mask(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_attribute_mask(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn new(print_mask: bool, attribute_mask: bool) -> Self {
        let mut inner: i32 = 0;
        if print_mask {
            inner |= 1 << 0;
        } else {
            inner &= !(1 << 0);
        }
        if attribute_mask {
            inner |= 1 << 1;
        } else {
            inner &= !(1 << 1);
        }
        EvMask { inner: inner }
    }
}
impl AsByteSequence for EvMask {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        Some((EvMask { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GetDoc {
    Finished = 0,
    SecondConsumer = 1,
}
impl AsByteSequence for GetDoc {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Finished, sz)),
            1 => Some((Self::SecondConsumer, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for GetDoc {
    #[inline]
    fn default() -> GetDoc {
        GetDoc::Finished
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Detail {
    StartJobNotify = 1,
    EndJobNotify = 2,
    StartDocNotify = 3,
    EndDocNotify = 4,
    StartPageNotify = 5,
    EndPageNotify = 6,
}
impl AsByteSequence for Detail {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            1 => Some((Self::StartJobNotify, sz)),
            2 => Some((Self::EndJobNotify, sz)),
            3 => Some((Self::StartDocNotify, sz)),
            4 => Some((Self::EndDocNotify, sz)),
            5 => Some((Self::StartPageNotify, sz)),
            6 => Some((Self::EndPageNotify, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for Detail {
    #[inline]
    fn default() -> Detail {
        Detail::StartJobNotify
    }
}
#[derive(Clone, Debug, Default)]
pub struct NotifyEvent {
    pub event_type: u8,
    pub detail: Card8,
    pub sequence: u16,
    pub context: Pcontext,
    pub cancel: bool,
}
impl NotifyEvent {}
impl AsByteSequence for NotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index += self.cancel.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing NotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (Pcontext, usize) = <Pcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cancel, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            NotifyEvent {
                event_type: event_type,
                detail: detail,
                sequence: sequence,
                context: context,
                cancel: cancel,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.detail.size()
            + self.sequence.size()
            + self.context.size()
            + self.cancel.size()
    }
}
impl crate::auto::Event for NotifyEvent {
    const OPCODE: u8 = 0;
}
#[derive(Clone, Debug, Default)]
pub struct AttributNotifyEvent {
    pub event_type: u8,
    pub detail: Card8,
    pub sequence: u16,
    pub context: Pcontext,
}
impl AttributNotifyEvent {}
impl AsByteSequence for AttributNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.context.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AttributNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (context, sz): (Pcontext, usize) = <Pcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            AttributNotifyEvent {
                event_type: event_type,
                detail: detail,
                sequence: sequence,
                context: context,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size() + self.detail.size() + self.sequence.size() + self.context.size()
    }
}
impl crate::auto::Event for AttributNotifyEvent {
    const OPCODE: u8 = 1;
}
