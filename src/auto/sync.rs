// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

use super::xproto::*;
#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Alarm {
    pub xid: XID,
}
impl Alarm {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Alarm {
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
pub struct Counter {
    pub xid: XID,
}
impl Counter {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Counter {
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
pub struct Fence {
    pub xid: XID,
}
impl Fence {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Fence {
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
pub struct Systemcounter<'a> {
    pub counter: Counter,
    pub resolution: Int64,
    pub name: Cow<'a, str>,
}
impl<'a> Systemcounter<'a> {}
impl<'a> AsByteSequence for Systemcounter<'a> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.counter.as_bytes(&mut bytes[index..]);
        index += self.resolution.as_bytes(&mut bytes[index..]);
        index += (self.name.len() as Card16).as_bytes(&mut bytes[index..]);
        let block_len: usize = string_as_bytes(&self.name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, 4);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Systemcounter from byte buffer");
        let (counter, sz): (Counter, usize) = <Counter>::from_bytes(&bytes[index..])?;
        index += sz;
        let (resolution, sz): (Int64, usize) = <Int64>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (name, block_len): (Cow<'_, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, 4);
        Some((
            Systemcounter {
                counter: counter,
                resolution: resolution,
                name: name,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.counter.size() + self.resolution.size() + ::core::mem::size_of::<Card16>() + {
            let block_len: usize = self.name.len();
            let pad: usize = buffer_pad(block_len, 4);
            block_len + pad
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Trigger {
    pub counter: Counter,
    pub wait_type: Valuetype,
    pub wait_value: Int64,
    pub test_type: Testtype,
}
impl Trigger {}
impl AsByteSequence for Trigger {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.counter.as_bytes(&mut bytes[index..]);
        index += self.wait_type.as_bytes(&mut bytes[index..]);
        index += self.wait_value.as_bytes(&mut bytes[index..]);
        index += self.test_type.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Trigger from byte buffer");
        let (counter, sz): (Counter, usize) = <Counter>::from_bytes(&bytes[index..])?;
        index += sz;
        let (wait_type, sz): (Valuetype, usize) = <Valuetype>::from_bytes(&bytes[index..])?;
        index += sz;
        let (wait_value, sz): (Int64, usize) = <Int64>::from_bytes(&bytes[index..])?;
        index += sz;
        let (test_type, sz): (Testtype, usize) = <Testtype>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Trigger {
                counter: counter,
                wait_type: wait_type,
                wait_value: wait_value,
                test_type: test_type,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.counter.size() + self.wait_type.size() + self.wait_value.size() + self.test_type.size()
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Valuetype {
    Absolute = 0,
    Relative = 1,
}
impl AsByteSequence for Valuetype {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Absolute, sz)),
            1 => Some((Self::Relative, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u32>()
    }
}
impl Default for Valuetype {
    #[inline]
    fn default() -> Valuetype {
        Valuetype::Absolute
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Testtype {
    PositiveTransition = 0,
    NegativeTransition = 1,
    PositiveComparison = 2,
    NegativeComparison = 3,
}
impl AsByteSequence for Testtype {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::PositiveTransition, sz)),
            1 => Some((Self::NegativeTransition, sz)),
            2 => Some((Self::PositiveComparison, sz)),
            3 => Some((Self::NegativeComparison, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u32>()
    }
}
impl Default for Testtype {
    #[inline]
    fn default() -> Testtype {
        Testtype::PositiveTransition
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Waitcondition {
    pub trigger: Trigger,
    pub event_threshold: Int64,
}
impl Waitcondition {}
impl AsByteSequence for Waitcondition {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.trigger.as_bytes(&mut bytes[index..]);
        index += self.event_threshold.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Waitcondition from byte buffer");
        let (trigger, sz): (Trigger, usize) = <Trigger>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_threshold, sz): (Int64, usize) = <Int64>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Waitcondition {
                trigger: trigger,
                event_threshold: event_threshold,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.trigger.size() + self.event_threshold.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct InitializeRequest {
    pub req_type: u8,
    pub length: u16,
    pub desired_major_version: Card8,
    pub desired_minor_version: Card8,
}
impl InitializeRequest {}
impl AsByteSequence for InitializeRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.desired_major_version.as_bytes(&mut bytes[index..]);
        index += self.desired_minor_version.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing InitializeRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (desired_major_version, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (desired_minor_version, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            InitializeRequest {
                req_type: req_type,
                length: length,
                desired_major_version: desired_major_version,
                desired_minor_version: desired_minor_version,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.desired_major_version.size()
            + self.desired_minor_version.size()
    }
}
impl Request for InitializeRequest {
    const OPCODE: u8 = 0;
    const EXTENSION: Option<&'static str> = Some("SYNC");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = InitializeReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct InitializeReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: Card8,
    pub minor_version: Card8,
}
impl InitializeReply {}
impl AsByteSequence for InitializeReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.major_version.as_bytes(&mut bytes[index..]);
        index += self.minor_version.as_bytes(&mut bytes[index..]);
        index += 22;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing InitializeReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_version, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_version, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 22;
        Some((
            InitializeReply {
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
            + 22
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ListSystemCountersRequest {
    pub req_type: u8,
    pub length: u16,
}
impl ListSystemCountersRequest {}
impl AsByteSequence for ListSystemCountersRequest {
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
        log::trace!("Deserializing ListSystemCountersRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ListSystemCountersRequest {
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
impl Request for ListSystemCountersRequest {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = Some("SYNC");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ListSystemCountersReply<'static, 'static>;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ListSystemCountersReply<'c, 'b> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub counters: Cow<'c, [Systemcounter<'b>]>,
}
impl<'c, 'b> ListSystemCountersReply<'c, 'b> {}
impl<'c, 'b> AsByteSequence for ListSystemCountersReply<'c, 'b> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.counters.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.counters, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Systemcounter<'b>>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListSystemCountersReply from byte buffer");
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
        let (counters, block_len): (Cow<'_, [Systemcounter<'_>]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Systemcounter<'b>>());
        Some((
            ListSystemCountersReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                counters: counters,
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
                let block_len: usize = self.counters.iter().map(|i| i.size()).sum();
                let pad: usize =
                    buffer_pad(block_len, ::core::mem::align_of::<Systemcounter<'b>>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CreateCounterRequest {
    pub req_type: u8,
    pub length: u16,
    pub id: Counter,
    pub initial_value: Int64,
}
impl CreateCounterRequest {}
impl AsByteSequence for CreateCounterRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.id.as_bytes(&mut bytes[index..]);
        index += self.initial_value.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateCounterRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (id, sz): (Counter, usize) = <Counter>::from_bytes(&bytes[index..])?;
        index += sz;
        let (initial_value, sz): (Int64, usize) = <Int64>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CreateCounterRequest {
                req_type: req_type,
                length: length,
                id: id,
                initial_value: initial_value,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.id.size() + self.initial_value.size()
    }
}
impl Request for CreateCounterRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = Some("SYNC");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DestroyCounterRequest {
    pub req_type: u8,
    pub length: u16,
    pub counter: Counter,
}
impl DestroyCounterRequest {}
impl AsByteSequence for DestroyCounterRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.counter.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DestroyCounterRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (counter, sz): (Counter, usize) = <Counter>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DestroyCounterRequest {
                req_type: req_type,
                length: length,
                counter: counter,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.counter.size()
    }
}
impl Request for DestroyCounterRequest {
    const OPCODE: u8 = 6;
    const EXTENSION: Option<&'static str> = Some("SYNC");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryCounterRequest {
    pub req_type: u8,
    pub length: u16,
    pub counter: Counter,
}
impl QueryCounterRequest {}
impl AsByteSequence for QueryCounterRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.counter.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryCounterRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (counter, sz): (Counter, usize) = <Counter>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryCounterRequest {
                req_type: req_type,
                length: length,
                counter: counter,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.counter.size()
    }
}
impl Request for QueryCounterRequest {
    const OPCODE: u8 = 5;
    const EXTENSION: Option<&'static str> = Some("SYNC");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryCounterReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryCounterReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub counter_value: Int64,
}
impl QueryCounterReply {}
impl AsByteSequence for QueryCounterReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.counter_value.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryCounterReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (counter_value, sz): (Int64, usize) = <Int64>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryCounterReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                counter_value: counter_value,
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
            + self.counter_value.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct AwaitRequest<'d> {
    pub req_type: u8,
    pub length: u16,
    pub wait_list: Cow<'d, [Waitcondition]>,
}
impl<'d> AwaitRequest<'d> {}
impl<'d> AsByteSequence for AwaitRequest<'d> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.wait_list, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Waitcondition>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AwaitRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (wait_list, block_len): (Cow<'_, [Waitcondition]>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Waitcondition>());
        Some((
            AwaitRequest {
                req_type: req_type,
                length: length,
                wait_list: wait_list,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + {
            let block_len: usize = self.wait_list.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Waitcondition>());
            block_len + pad
        }
    }
}
impl<'d> Request for AwaitRequest<'d> {
    const OPCODE: u8 = 7;
    const EXTENSION: Option<&'static str> = Some("SYNC");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ChangeCounterRequest {
    pub req_type: u8,
    pub length: u16,
    pub counter: Counter,
    pub amount: Int64,
}
impl ChangeCounterRequest {}
impl AsByteSequence for ChangeCounterRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.counter.as_bytes(&mut bytes[index..]);
        index += self.amount.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangeCounterRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (counter, sz): (Counter, usize) = <Counter>::from_bytes(&bytes[index..])?;
        index += sz;
        let (amount, sz): (Int64, usize) = <Int64>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ChangeCounterRequest {
                req_type: req_type,
                length: length,
                counter: counter,
                amount: amount,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.counter.size() + self.amount.size()
    }
}
impl Request for ChangeCounterRequest {
    const OPCODE: u8 = 4;
    const EXTENSION: Option<&'static str> = Some("SYNC");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetCounterRequest {
    pub req_type: u8,
    pub length: u16,
    pub counter: Counter,
    pub value: Int64,
}
impl SetCounterRequest {}
impl AsByteSequence for SetCounterRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.counter.as_bytes(&mut bytes[index..]);
        index += self.value.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetCounterRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (counter, sz): (Counter, usize) = <Counter>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value, sz): (Int64, usize) = <Int64>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetCounterRequest {
                req_type: req_type,
                length: length,
                counter: counter,
                value: value,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.counter.size() + self.value.size()
    }
}
impl Request for SetCounterRequest {
    const OPCODE: u8 = 3;
    const EXTENSION: Option<&'static str> = Some("SYNC");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CreateAlarmRequest {
    pub req_type: u8,
    pub length: u16,
    pub id: Alarm,
    pub value_mask: Ca,
    pub counter: Counter,
    pub value_type: Valuetype,
    pub value: Int64,
    pub test_type: Testtype,
    pub delta: Int64,
    pub events: Card32,
}
impl CreateAlarmRequest {}
impl AsByteSequence for CreateAlarmRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.id.as_bytes(&mut bytes[index..]);
        index += self.value_mask.as_bytes(&mut bytes[index..]);
        let cond0 = (self.value_mask);
        if cond0.counter() {
            index += self.counter.as_bytes(&mut bytes[index..]);
        }
        if cond0.value_type() {
            index += self.value_type.as_bytes(&mut bytes[index..]);
        }
        if cond0.value() {
            index += self.value.as_bytes(&mut bytes[index..]);
        }
        if cond0.test_type() {
            index += self.test_type.as_bytes(&mut bytes[index..]);
        }
        if cond0.delta() {
            index += self.delta.as_bytes(&mut bytes[index..]);
        }
        if cond0.events() {
            index += self.events.as_bytes(&mut bytes[index..]);
        }
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateAlarmRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (id, sz): (Alarm, usize) = <Alarm>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value_mask, sz): (Ca, usize) = <Ca>::from_bytes(&bytes[index..])?;
        index += sz;
        let cond0 = (value_mask);
        let counter: Counter = if cond0.counter() {
            let (counter, sz): (Counter, usize) = <Counter>::from_bytes(&bytes[index..])?;
            index += sz;
            counter
        } else {
            Default::default()
        };
        let value_type: Valuetype = if cond0.value_type() {
            let (value_type, sz): (Valuetype, usize) = <Valuetype>::from_bytes(&bytes[index..])?;
            index += sz;
            value_type
        } else {
            Default::default()
        };
        let value: Int64 = if cond0.value() {
            let (value, sz): (Int64, usize) = <Int64>::from_bytes(&bytes[index..])?;
            index += sz;
            value
        } else {
            Default::default()
        };
        let test_type: Testtype = if cond0.test_type() {
            let (test_type, sz): (Testtype, usize) = <Testtype>::from_bytes(&bytes[index..])?;
            index += sz;
            test_type
        } else {
            Default::default()
        };
        let delta: Int64 = if cond0.delta() {
            let (delta, sz): (Int64, usize) = <Int64>::from_bytes(&bytes[index..])?;
            index += sz;
            delta
        } else {
            Default::default()
        };
        let events: Card32 = if cond0.events() {
            let (events, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            events
        } else {
            Default::default()
        };
        Some((
            CreateAlarmRequest {
                req_type: req_type,
                length: length,
                id: id,
                value_mask: value_mask,
                counter: counter,
                value_type: value_type,
                value: value,
                test_type: test_type,
                delta: delta,
                events: events,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.id.size()
            + self.value_mask.size()
            + self.counter.size()
            + self.value_type.size()
            + self.value.size()
            + self.test_type.size()
            + self.delta.size()
            + self.events.size()
    }
}
impl Request for CreateAlarmRequest {
    const OPCODE: u8 = 8;
    const EXTENSION: Option<&'static str> = Some("SYNC");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ca {
    pub inner: u32,
}
impl Ca {
    #[inline]
    pub fn counter(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_counter(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn value_type(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_value_type(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn value(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_value(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn test_type(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_test_type(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn delta(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_delta(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn events(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_events(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn new(
        counter: bool,
        value_type: bool,
        value: bool,
        test_type: bool,
        delta: bool,
        events: bool,
    ) -> Self {
        let mut inner: u32 = 0;
        if counter {
            inner |= 1 << 0;
        }
        if value_type {
            inner |= 1 << 1;
        }
        if value {
            inner |= 1 << 2;
        }
        if test_type {
            inner |= 1 << 3;
        }
        if delta {
            inner |= 1 << 4;
        }
        if events {
            inner |= 1 << 5;
        }
        Ca { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const COUNTER: Self = Self { inner: 1 };
    pub const VALUE_TYPE: Self = Self { inner: 2 };
    pub const VALUE: Self = Self { inner: 4 };
    pub const TEST_TYPE: Self = Self { inner: 8 };
    pub const DELTA: Self = Self { inner: 16 };
    pub const EVENTS: Self = Self { inner: 32 };
    pub const COMPLETE: Self = Self { inner: 63 };
}
impl AsByteSequence for Ca {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((Ca { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for Ca {
    type Output = Ca;
    #[inline]
    fn not(self) -> Ca {
        Ca { inner: !self.inner }
    }
}
impl core::ops::BitAnd for Ca {
    type Output = Ca;
    #[inline]
    fn bitand(self, rhs: Ca) -> Ca {
        Ca {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for Ca {
    type Output = Ca;
    #[inline]
    fn bitor(self, rhs: Ca) -> Ca {
        Ca {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for Ca {
    type Output = Ca;
    #[inline]
    fn bitxor(self, rhs: Ca) -> Ca {
        Ca {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ChangeAlarmRequest {
    pub req_type: u8,
    pub length: u16,
    pub id: Alarm,
    pub value_mask: Ca,
    pub counter: Counter,
    pub value_type: Valuetype,
    pub value: Int64,
    pub test_type: Testtype,
    pub delta: Int64,
    pub events: Card32,
}
impl ChangeAlarmRequest {}
impl AsByteSequence for ChangeAlarmRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.id.as_bytes(&mut bytes[index..]);
        index += self.value_mask.as_bytes(&mut bytes[index..]);
        let cond0 = (self.value_mask);
        if cond0.counter() {
            index += self.counter.as_bytes(&mut bytes[index..]);
        }
        if cond0.value_type() {
            index += self.value_type.as_bytes(&mut bytes[index..]);
        }
        if cond0.value() {
            index += self.value.as_bytes(&mut bytes[index..]);
        }
        if cond0.test_type() {
            index += self.test_type.as_bytes(&mut bytes[index..]);
        }
        if cond0.delta() {
            index += self.delta.as_bytes(&mut bytes[index..]);
        }
        if cond0.events() {
            index += self.events.as_bytes(&mut bytes[index..]);
        }
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangeAlarmRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (id, sz): (Alarm, usize) = <Alarm>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value_mask, sz): (Ca, usize) = <Ca>::from_bytes(&bytes[index..])?;
        index += sz;
        let cond0 = (value_mask);
        let counter: Counter = if cond0.counter() {
            let (counter, sz): (Counter, usize) = <Counter>::from_bytes(&bytes[index..])?;
            index += sz;
            counter
        } else {
            Default::default()
        };
        let value_type: Valuetype = if cond0.value_type() {
            let (value_type, sz): (Valuetype, usize) = <Valuetype>::from_bytes(&bytes[index..])?;
            index += sz;
            value_type
        } else {
            Default::default()
        };
        let value: Int64 = if cond0.value() {
            let (value, sz): (Int64, usize) = <Int64>::from_bytes(&bytes[index..])?;
            index += sz;
            value
        } else {
            Default::default()
        };
        let test_type: Testtype = if cond0.test_type() {
            let (test_type, sz): (Testtype, usize) = <Testtype>::from_bytes(&bytes[index..])?;
            index += sz;
            test_type
        } else {
            Default::default()
        };
        let delta: Int64 = if cond0.delta() {
            let (delta, sz): (Int64, usize) = <Int64>::from_bytes(&bytes[index..])?;
            index += sz;
            delta
        } else {
            Default::default()
        };
        let events: Card32 = if cond0.events() {
            let (events, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            events
        } else {
            Default::default()
        };
        Some((
            ChangeAlarmRequest {
                req_type: req_type,
                length: length,
                id: id,
                value_mask: value_mask,
                counter: counter,
                value_type: value_type,
                value: value,
                test_type: test_type,
                delta: delta,
                events: events,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.id.size()
            + self.value_mask.size()
            + self.counter.size()
            + self.value_type.size()
            + self.value.size()
            + self.test_type.size()
            + self.delta.size()
            + self.events.size()
    }
}
impl Request for ChangeAlarmRequest {
    const OPCODE: u8 = 9;
    const EXTENSION: Option<&'static str> = Some("SYNC");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DestroyAlarmRequest {
    pub req_type: u8,
    pub length: u16,
    pub alarm: Alarm,
}
impl DestroyAlarmRequest {}
impl AsByteSequence for DestroyAlarmRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.alarm.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DestroyAlarmRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (alarm, sz): (Alarm, usize) = <Alarm>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DestroyAlarmRequest {
                req_type: req_type,
                length: length,
                alarm: alarm,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.alarm.size()
    }
}
impl Request for DestroyAlarmRequest {
    const OPCODE: u8 = 11;
    const EXTENSION: Option<&'static str> = Some("SYNC");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryAlarmRequest {
    pub req_type: u8,
    pub length: u16,
    pub alarm: Alarm,
}
impl QueryAlarmRequest {}
impl AsByteSequence for QueryAlarmRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.alarm.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryAlarmRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (alarm, sz): (Alarm, usize) = <Alarm>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryAlarmRequest {
                req_type: req_type,
                length: length,
                alarm: alarm,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.alarm.size()
    }
}
impl Request for QueryAlarmRequest {
    const OPCODE: u8 = 10;
    const EXTENSION: Option<&'static str> = Some("SYNC");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryAlarmReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryAlarmReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub trigger: Trigger,
    pub delta: Int64,
    pub events: bool,
    pub state: Alarmstate,
}
impl QueryAlarmReply {}
impl AsByteSequence for QueryAlarmReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.trigger.as_bytes(&mut bytes[index..]);
        index += self.delta.as_bytes(&mut bytes[index..]);
        index += self.events.as_bytes(&mut bytes[index..]);
        index += self.state.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryAlarmReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (trigger, sz): (Trigger, usize) = <Trigger>::from_bytes(&bytes[index..])?;
        index += sz;
        let (delta, sz): (Int64, usize) = <Int64>::from_bytes(&bytes[index..])?;
        index += sz;
        let (events, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (state, sz): (Alarmstate, usize) = <Alarmstate>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            QueryAlarmReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                trigger: trigger,
                delta: delta,
                events: events,
                state: state,
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
            + self.trigger.size()
            + self.delta.size()
            + self.events.size()
            + self.state.size()
            + 2
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Alarmstate {
    Active = 0,
    Inactive = 1,
    Destroyed = 2,
}
impl AsByteSequence for Alarmstate {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Active, sz)),
            1 => Some((Self::Inactive, sz)),
            2 => Some((Self::Destroyed, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for Alarmstate {
    #[inline]
    fn default() -> Alarmstate {
        Alarmstate::Active
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetPriorityRequest {
    pub req_type: u8,
    pub length: u16,
    pub id: Card32,
    pub priority: Int32,
}
impl SetPriorityRequest {}
impl AsByteSequence for SetPriorityRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.id.as_bytes(&mut bytes[index..]);
        index += self.priority.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetPriorityRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (id, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (priority, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetPriorityRequest {
                req_type: req_type,
                length: length,
                id: id,
                priority: priority,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.id.size() + self.priority.size()
    }
}
impl Request for SetPriorityRequest {
    const OPCODE: u8 = 12;
    const EXTENSION: Option<&'static str> = Some("SYNC");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetPriorityRequest {
    pub req_type: u8,
    pub length: u16,
    pub id: Card32,
}
impl GetPriorityRequest {}
impl AsByteSequence for GetPriorityRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.id.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPriorityRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (id, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetPriorityRequest {
                req_type: req_type,
                length: length,
                id: id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.id.size()
    }
}
impl Request for GetPriorityRequest {
    const OPCODE: u8 = 13;
    const EXTENSION: Option<&'static str> = Some("SYNC");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetPriorityReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetPriorityReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub priority: Int32,
}
impl GetPriorityReply {}
impl AsByteSequence for GetPriorityReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.priority.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPriorityReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (priority, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetPriorityReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                priority: priority,
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
            + self.priority.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CreateFenceRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub fence: Fence,
    pub initially_triggered: bool,
}
impl CreateFenceRequest {}
impl AsByteSequence for CreateFenceRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.fence.as_bytes(&mut bytes[index..]);
        index += self.initially_triggered.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateFenceRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (fence, sz): (Fence, usize) = <Fence>::from_bytes(&bytes[index..])?;
        index += sz;
        let (initially_triggered, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CreateFenceRequest {
                req_type: req_type,
                length: length,
                drawable: drawable,
                fence: fence,
                initially_triggered: initially_triggered,
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
    }
}
impl Request for CreateFenceRequest {
    const OPCODE: u8 = 14;
    const EXTENSION: Option<&'static str> = Some("SYNC");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct TriggerFenceRequest {
    pub req_type: u8,
    pub length: u16,
    pub fence: Fence,
}
impl TriggerFenceRequest {}
impl AsByteSequence for TriggerFenceRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.fence.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing TriggerFenceRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (fence, sz): (Fence, usize) = <Fence>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            TriggerFenceRequest {
                req_type: req_type,
                length: length,
                fence: fence,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.fence.size()
    }
}
impl Request for TriggerFenceRequest {
    const OPCODE: u8 = 15;
    const EXTENSION: Option<&'static str> = Some("SYNC");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ResetFenceRequest {
    pub req_type: u8,
    pub length: u16,
    pub fence: Fence,
}
impl ResetFenceRequest {}
impl AsByteSequence for ResetFenceRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.fence.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ResetFenceRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (fence, sz): (Fence, usize) = <Fence>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ResetFenceRequest {
                req_type: req_type,
                length: length,
                fence: fence,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.fence.size()
    }
}
impl Request for ResetFenceRequest {
    const OPCODE: u8 = 16;
    const EXTENSION: Option<&'static str> = Some("SYNC");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DestroyFenceRequest {
    pub req_type: u8,
    pub length: u16,
    pub fence: Fence,
}
impl DestroyFenceRequest {}
impl AsByteSequence for DestroyFenceRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.fence.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DestroyFenceRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (fence, sz): (Fence, usize) = <Fence>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DestroyFenceRequest {
                req_type: req_type,
                length: length,
                fence: fence,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.fence.size()
    }
}
impl Request for DestroyFenceRequest {
    const OPCODE: u8 = 17;
    const EXTENSION: Option<&'static str> = Some("SYNC");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryFenceRequest {
    pub req_type: u8,
    pub length: u16,
    pub fence: Fence,
}
impl QueryFenceRequest {}
impl AsByteSequence for QueryFenceRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.fence.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryFenceRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (fence, sz): (Fence, usize) = <Fence>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryFenceRequest {
                req_type: req_type,
                length: length,
                fence: fence,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.fence.size()
    }
}
impl Request for QueryFenceRequest {
    const OPCODE: u8 = 18;
    const EXTENSION: Option<&'static str> = Some("SYNC");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryFenceReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryFenceReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub triggered: bool,
}
impl QueryFenceReply {}
impl AsByteSequence for QueryFenceReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.triggered.as_bytes(&mut bytes[index..]);
        index += 23;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryFenceReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (triggered, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 23;
        Some((
            QueryFenceReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                triggered: triggered,
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
            + self.triggered.size()
            + 23
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct AwaitFenceRequest<'e> {
    pub req_type: u8,
    pub length: u16,
    pub fence_list: Cow<'e, [Fence]>,
}
impl<'e> AwaitFenceRequest<'e> {}
impl<'e> AsByteSequence for AwaitFenceRequest<'e> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.fence_list, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fence>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AwaitFenceRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (fence_list, block_len): (Cow<'_, [Fence]>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fence>());
        Some((
            AwaitFenceRequest {
                req_type: req_type,
                length: length,
                fence_list: fence_list,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + {
            let block_len: usize = self.fence_list.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fence>());
            block_len + pad
        }
    }
}
impl<'e> Request for AwaitFenceRequest<'e> {
    const OPCODE: u8 = 19;
    const EXTENSION: Option<&'static str> = Some("SYNC");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct AlarmError {
    pub _error_type: u8,
    pub error_code: u8,
    pub major_code: u8,
    pub minor_code: u8,
    pub sequence: u16,
    pub bad_alarm: Card32,
    pub minor_opcode: Card16,
    pub major_opcode: Card8,
}
impl AlarmError {}
impl AsByteSequence for AlarmError {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self._error_type.as_bytes(&mut bytes[index..]);
        index += self.error_code.as_bytes(&mut bytes[index..]);
        index += self.major_code.as_bytes(&mut bytes[index..]);
        index += self.minor_code.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.bad_alarm.as_bytes(&mut bytes[index..]);
        index += self.minor_opcode.as_bytes(&mut bytes[index..]);
        index += self.major_opcode.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AlarmError from byte buffer");
        let (_error_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (error_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bad_alarm, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_opcode, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_opcode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            AlarmError {
                _error_type: _error_type,
                error_code: error_code,
                major_code: major_code,
                minor_code: minor_code,
                sequence: sequence,
                bad_alarm: bad_alarm,
                minor_opcode: minor_opcode,
                major_opcode: major_opcode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self._error_type.size()
            + self.error_code.size()
            + self.major_code.size()
            + self.minor_code.size()
            + self.sequence.size()
            + self.bad_alarm.size()
            + self.minor_opcode.size()
            + self.major_opcode.size()
    }
}
impl crate::auto::Error for AlarmError {
    const OPCODE: u8 = 1;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CounterError {
    pub _error_type: u8,
    pub error_code: u8,
    pub major_code: u8,
    pub minor_code: u8,
    pub sequence: u16,
    pub bad_counter: Card32,
    pub minor_opcode: Card16,
    pub major_opcode: Card8,
}
impl CounterError {}
impl AsByteSequence for CounterError {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self._error_type.as_bytes(&mut bytes[index..]);
        index += self.error_code.as_bytes(&mut bytes[index..]);
        index += self.major_code.as_bytes(&mut bytes[index..]);
        index += self.minor_code.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.bad_counter.as_bytes(&mut bytes[index..]);
        index += self.minor_opcode.as_bytes(&mut bytes[index..]);
        index += self.major_opcode.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CounterError from byte buffer");
        let (_error_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (error_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bad_counter, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_opcode, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_opcode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CounterError {
                _error_type: _error_type,
                error_code: error_code,
                major_code: major_code,
                minor_code: minor_code,
                sequence: sequence,
                bad_counter: bad_counter,
                minor_opcode: minor_opcode,
                major_opcode: major_opcode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self._error_type.size()
            + self.error_code.size()
            + self.major_code.size()
            + self.minor_code.size()
            + self.sequence.size()
            + self.bad_counter.size()
            + self.minor_opcode.size()
            + self.major_opcode.size()
    }
}
impl crate::auto::Error for CounterError {
    const OPCODE: u8 = 0;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct AlarmNotifyEvent {
    pub event_type: u8,
    pub kind: Card8,
    pub sequence: u16,
    pub alarm: Alarm,
    pub counter_value: Int64,
    pub alarm_value: Int64,
    pub timestamp: Timestamp,
    pub state: Alarmstate,
}
impl AlarmNotifyEvent {}
impl AsByteSequence for AlarmNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.kind.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.alarm.as_bytes(&mut bytes[index..]);
        index += self.counter_value.as_bytes(&mut bytes[index..]);
        index += self.alarm_value.as_bytes(&mut bytes[index..]);
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += self.state.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AlarmNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (kind, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (alarm, sz): (Alarm, usize) = <Alarm>::from_bytes(&bytes[index..])?;
        index += sz;
        let (counter_value, sz): (Int64, usize) = <Int64>::from_bytes(&bytes[index..])?;
        index += sz;
        let (alarm_value, sz): (Int64, usize) = <Int64>::from_bytes(&bytes[index..])?;
        index += sz;
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (state, sz): (Alarmstate, usize) = <Alarmstate>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            AlarmNotifyEvent {
                event_type: event_type,
                kind: kind,
                sequence: sequence,
                alarm: alarm,
                counter_value: counter_value,
                alarm_value: alarm_value,
                timestamp: timestamp,
                state: state,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.kind.size()
            + self.sequence.size()
            + self.alarm.size()
            + self.counter_value.size()
            + self.alarm_value.size()
            + self.timestamp.size()
            + self.state.size()
            + 3
    }
}
impl crate::auto::Event for AlarmNotifyEvent {
    const OPCODE: u8 = 1;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CounterNotifyEvent {
    pub event_type: u8,
    pub kind: Card8,
    pub sequence: u16,
    pub counter: Counter,
    pub wait_value: Int64,
    pub counter_value: Int64,
    pub timestamp: Timestamp,
    pub count: Card16,
    pub destroyed: bool,
}
impl CounterNotifyEvent {}
impl AsByteSequence for CounterNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.kind.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.counter.as_bytes(&mut bytes[index..]);
        index += self.wait_value.as_bytes(&mut bytes[index..]);
        index += self.counter_value.as_bytes(&mut bytes[index..]);
        index += self.timestamp.as_bytes(&mut bytes[index..]);
        index += self.count.as_bytes(&mut bytes[index..]);
        index += self.destroyed.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CounterNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (kind, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (counter, sz): (Counter, usize) = <Counter>::from_bytes(&bytes[index..])?;
        index += sz;
        let (wait_value, sz): (Int64, usize) = <Int64>::from_bytes(&bytes[index..])?;
        index += sz;
        let (counter_value, sz): (Int64, usize) = <Int64>::from_bytes(&bytes[index..])?;
        index += sz;
        let (timestamp, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (count, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (destroyed, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            CounterNotifyEvent {
                event_type: event_type,
                kind: kind,
                sequence: sequence,
                counter: counter,
                wait_value: wait_value,
                counter_value: counter_value,
                timestamp: timestamp,
                count: count,
                destroyed: destroyed,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.kind.size()
            + self.sequence.size()
            + self.counter.size()
            + self.wait_value.size()
            + self.counter_value.size()
            + self.timestamp.size()
            + self.count.size()
            + self.destroyed.size()
            + 1
    }
}
impl crate::auto::Event for CounterNotifyEvent {
    const OPCODE: u8 = 0;
}
