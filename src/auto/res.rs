// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

use super::xproto::*;
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Client {
    pub resource_base: Card32,
    pub resource_mask: Card32,
}
impl Client {}
impl AsByteSequence for Client {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.resource_base.as_bytes(&mut bytes[index..]);
        index += self.resource_mask.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Client from byte buffer");
        let (resource_base, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (resource_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Client {
                resource_base: resource_base,
                resource_mask: resource_mask,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.resource_base.size() + self.resource_mask.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Type {
    pub resource_type: Atom,
    pub count: Card32,
}
impl Type {}
impl AsByteSequence for Type {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.resource_type.as_bytes(&mut bytes[index..]);
        index += self.count.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Type from byte buffer");
        let (resource_type, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (count, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Type {
                resource_type: resource_type,
                count: count,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.resource_type.size() + self.count.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ClientIdSpec {
    pub client: Card32,
    pub mask: ClientIdMask,
}
impl ClientIdSpec {}
impl AsByteSequence for ClientIdSpec {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.client.as_bytes(&mut bytes[index..]);
        index += self.mask.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ClientIdSpec from byte buffer");
        let (client, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mask, sz): (ClientIdMask, usize) = <ClientIdMask>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ClientIdSpec {
                client: client,
                mask: mask,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.client.size() + self.mask.size()
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClientIdMask {
    pub inner: u32,
}
impl ClientIdMask {
    #[inline]
    pub fn client_xid(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_client_xid(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn local_client_pid(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_local_client_pid(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn new(client_xid: bool, local_client_pid: bool) -> Self {
        let mut inner: u32 = 0;
        if client_xid {
            inner |= 1 << 0;
        }
        if local_client_pid {
            inner |= 1 << 1;
        }
        ClientIdMask { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const CLIENT_XID: Self = Self { inner: 1 };
    pub const LOCAL_CLIENT_PID: Self = Self { inner: 2 };
    pub const COMPLETE: Self = Self { inner: 3 };
}
impl AsByteSequence for ClientIdMask {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((ClientIdMask { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for ClientIdMask {
    type Output = ClientIdMask;
    #[inline]
    fn not(self) -> ClientIdMask {
        ClientIdMask { inner: !self.inner }
    }
}
impl core::ops::BitAnd for ClientIdMask {
    type Output = ClientIdMask;
    #[inline]
    fn bitand(self, rhs: ClientIdMask) -> ClientIdMask {
        ClientIdMask {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for ClientIdMask {
    type Output = ClientIdMask;
    #[inline]
    fn bitor(self, rhs: ClientIdMask) -> ClientIdMask {
        ClientIdMask {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for ClientIdMask {
    type Output = ClientIdMask;
    #[inline]
    fn bitxor(self, rhs: ClientIdMask) -> ClientIdMask {
        ClientIdMask {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ClientIdValue<'a> {
    pub spec: ClientIdSpec,
    pub length: Card32,
    pub value: Cow<'a, [Card32]>,
}
impl<'a> ClientIdValue {}
impl<'a> AsByteSequence for ClientIdValue<'a> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.spec.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.value, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ClientIdValue from byte buffer");
        let (spec, sz): (ClientIdSpec, usize) = <ClientIdSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize) / (4)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            ClientIdValue {
                spec: spec,
                length: length,
                value: value,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.spec.size() + self.length.size() + {
            let block_len: usize = self.value.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
            block_len + pad
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ResourceIdSpec {
    pub resource: Card32,
    pub ty: Card32,
}
impl ResourceIdSpec {}
impl AsByteSequence for ResourceIdSpec {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.resource.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ResourceIdSpec from byte buffer");
        let (resource, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ResourceIdSpec {
                resource: resource,
                ty: ty,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.resource.size() + self.ty.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ResourceSizeSpec {
    pub spec: ResourceIdSpec,
    pub bytes_: Card32,
    pub ref_count: Card32,
    pub use_count: Card32,
}
impl ResourceSizeSpec {}
impl AsByteSequence for ResourceSizeSpec {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.spec.as_bytes(&mut bytes[index..]);
        index += self.bytes_.as_bytes(&mut bytes[index..]);
        index += self.ref_count.as_bytes(&mut bytes[index..]);
        index += self.use_count.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ResourceSizeSpec from byte buffer");
        let (spec, sz): (ResourceIdSpec, usize) = <ResourceIdSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bytes_, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ref_count, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (use_count, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ResourceSizeSpec {
                spec: spec,
                bytes_: bytes_,
                ref_count: ref_count,
                use_count: use_count,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.spec.size() + self.bytes_.size() + self.ref_count.size() + self.use_count.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ResourceSizeValue<'b> {
    pub size: ResourceSizeSpec,
    pub cross_references: Cow<'b, [ResourceSizeSpec]>,
}
impl<'b> ResourceSizeValue {}
impl<'b> AsByteSequence for ResourceSizeValue<'b> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.size.as_bytes(&mut bytes[index..]);
        index += (self.cross_references.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.cross_references, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ResourceSizeSpec>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ResourceSizeValue from byte buffer");
        let (size, sz): (ResourceSizeSpec, usize) =
            <ResourceSizeSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cross_references, block_len): (Cow<'static, [ResourceSizeSpec]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ResourceSizeSpec>());
        Some((
            ResourceSizeValue {
                size: size,
                cross_references: cross_references,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.size.size() + ::core::mem::size_of::<Card32>() + {
            let block_len: usize = self.cross_references.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<ResourceSizeSpec>());
            block_len + pad
        }
    }
}
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
    const EXTENSION: Option<&'static str> = Some("X-Resource");
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
pub struct QueryClientsRequest {
    pub req_type: u8,
    pub length: u16,
}
impl QueryClientsRequest {}
impl AsByteSequence for QueryClientsRequest {
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
        log::trace!("Deserializing QueryClientsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryClientsRequest {
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
impl Request for QueryClientsRequest {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = Some("X-Resource");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryClientsReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryClientsReply<'c> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub clients: Cow<'c, [Client]>,
}
impl<'c> QueryClientsReply {}
impl<'c> AsByteSequence for QueryClientsReply<'c> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.clients.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.clients, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Client>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryClientsReply from byte buffer");
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
        let (clients, block_len): (Cow<'static, [Client]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Client>());
        Some((
            QueryClientsReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                clients: clients,
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
                let block_len: usize = self.clients.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Client>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryClientResourcesRequest {
    pub req_type: u8,
    pub length: u16,
    pub xid: Card32,
}
impl QueryClientResourcesRequest {}
impl AsByteSequence for QueryClientResourcesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.xid.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryClientResourcesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xid, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryClientResourcesRequest {
                req_type: req_type,
                length: length,
                xid: xid,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.xid.size()
    }
}
impl Request for QueryClientResourcesRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = Some("X-Resource");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryClientResourcesReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryClientResourcesReply<'d> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub types: Cow<'d, [Type]>,
}
impl<'d> QueryClientResourcesReply {}
impl<'d> AsByteSequence for QueryClientResourcesReply<'d> {
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
        index += buffer_pad(block_len, ::core::mem::align_of::<Type>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryClientResourcesReply from byte buffer");
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
        let (types, block_len): (Cow<'static, [Type]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Type>());
        Some((
            QueryClientResourcesReply {
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
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Type>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryClientPixmapBytesRequest {
    pub req_type: u8,
    pub length: u16,
    pub xid: Card32,
}
impl QueryClientPixmapBytesRequest {}
impl AsByteSequence for QueryClientPixmapBytesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.xid.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryClientPixmapBytesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xid, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryClientPixmapBytesRequest {
                req_type: req_type,
                length: length,
                xid: xid,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.xid.size()
    }
}
impl Request for QueryClientPixmapBytesRequest {
    const OPCODE: u8 = 3;
    const EXTENSION: Option<&'static str> = Some("X-Resource");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryClientPixmapBytesReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryClientPixmapBytesReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub bytes_: Card32,
    pub bytes_overflow: Card32,
}
impl QueryClientPixmapBytesReply {}
impl AsByteSequence for QueryClientPixmapBytesReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.bytes_.as_bytes(&mut bytes[index..]);
        index += self.bytes_overflow.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryClientPixmapBytesReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bytes_, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bytes_overflow, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryClientPixmapBytesReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                bytes_: bytes_,
                bytes_overflow: bytes_overflow,
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
            + self.bytes_.size()
            + self.bytes_overflow.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryClientIdsRequest<'g> {
    pub req_type: u8,
    pub length: u16,
    pub specs: Cow<'g, [ClientIdSpec]>,
}
impl<'g> QueryClientIdsRequest {}
impl<'g> AsByteSequence for QueryClientIdsRequest<'g> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.specs.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.specs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ClientIdSpec>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryClientIdsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (specs, block_len): (Cow<'static, [ClientIdSpec]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ClientIdSpec>());
        Some((
            QueryClientIdsRequest {
                req_type: req_type,
                length: length,
                specs: specs,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + ::core::mem::size_of::<Card32>() + {
            let block_len: usize = self.specs.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<ClientIdSpec>());
            block_len + pad
        }
    }
}
impl Request for QueryClientIdsRequest {
    const OPCODE: u8 = 4;
    const EXTENSION: Option<&'static str> = Some("X-Resource");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryClientIdsReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryClientIdsReply<'f, 'e> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ids: Cow<'f, [ClientIdValue<'e>]>,
}
impl<'f, 'e> QueryClientIdsReply {}
impl<'f, 'e> AsByteSequence for QueryClientIdsReply<'f, 'e> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.ids.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.ids, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ClientIdValue<'e>>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryClientIdsReply from byte buffer");
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
        let (ids, block_len): (Cow<'static, [ClientIdValue<'e>]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ClientIdValue<'e>>());
        Some((
            QueryClientIdsReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                ids: ids,
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
                let block_len: usize = self.ids.iter().map(|i| i.size()).sum();
                let pad: usize =
                    buffer_pad(block_len, ::core::mem::align_of::<ClientIdValue<'e>>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryResourceBytesRequest<'j> {
    pub req_type: u8,
    pub length: u16,
    pub client: Card32,
    pub specs: Cow<'j, [ResourceIdSpec]>,
}
impl<'j> QueryResourceBytesRequest {}
impl<'j> AsByteSequence for QueryResourceBytesRequest<'j> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.client.as_bytes(&mut bytes[index..]);
        index += (self.specs.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.specs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ResourceIdSpec>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryResourceBytesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (client, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (specs, block_len): (Cow<'static, [ResourceIdSpec]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ResourceIdSpec>());
        Some((
            QueryResourceBytesRequest {
                req_type: req_type,
                length: length,
                client: client,
                specs: specs,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.client.size()
            + ::core::mem::size_of::<Card32>()
            + {
                let block_len: usize = self.specs.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<ResourceIdSpec>());
                block_len + pad
            }
    }
}
impl Request for QueryResourceBytesRequest {
    const OPCODE: u8 = 5;
    const EXTENSION: Option<&'static str> = Some("X-Resource");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryResourceBytesReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryResourceBytesReply<'i, 'h> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub sizes: Cow<'i, [ResourceSizeValue<'h>]>,
}
impl<'i, 'h> QueryResourceBytesReply {}
impl<'i, 'h> AsByteSequence for QueryResourceBytesReply<'i, 'h> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.sizes.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.sizes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ResourceSizeValue<'h>>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryResourceBytesReply from byte buffer");
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
        let (sizes, block_len): (Cow<'static, [ResourceSizeValue<'h>]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ResourceSizeValue<'h>>());
        Some((
            QueryResourceBytesReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                sizes: sizes,
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
                let block_len: usize = self.sizes.iter().map(|i| i.size()).sum();
                let pad: usize =
                    buffer_pad(block_len, ::core::mem::align_of::<ResourceSizeValue<'h>>());
                block_len + pad
            }
    }
}
