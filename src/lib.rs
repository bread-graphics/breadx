// MIT/Apache2 License

#![forbid(unsafe_code)]

extern crate alloc;
extern crate core;

pub mod auth_info;
pub(crate) mod auto;
pub mod client_message_data;
pub mod display;
pub mod error;

pub use error::*;

/// An X11 ID.
pub type XID = u32;

/// A type that acts as a wrapper around an XID.
pub trait XidType {
    fn xid(&self) -> XID;
    fn from_xid(xid: XID) -> Self;
}

impl<T: XidType> auto::AsByteSequence for T {
    #[inline]
    fn size() -> usize {
        XID::size()
    }

    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.xid().as_bytes(bytes)
    }

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (xid, len) = XID::from_bytes(bytes)?;
        Some((Self::from_xid(xid), len))
    }
}

/// A request.
pub trait Request: auto::AsByteSequence {
    type Reply: auto::AsByteSequence;
    // Excerpt from the X Window System Protocol:
    //
    // Every request contains an 8-bit major opcode
    fn opcode(&self) -> u8;
}

pub use display::*;
