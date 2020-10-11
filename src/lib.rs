// MIT/Apache2 License

extern crate alloc;
extern crate core;

pub(crate) mod auto;
pub mod client_message_data;
pub(crate) mod connection;
pub mod error;

pub use error::*;

/// An X11 ID.
pub type XID = std::os::raw::c_ulong;

/// A type that acts as a wrapper around an XID.
pub trait XidType {
    fn xid(&self) -> XID;
    fn from_xid(xid: XID) -> Self;
}

impl<T: XidType> auto::AsByteSequence for T {
    #[inline]
    fn size() -> usize {
        use auto::AsByteSequence;
        XID::size()
    }

    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) {
        use auto::AsByteSequence;
        self.xid().as_bytes(bytes)
    }

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        use auto::AsByteSequence;
        Some(Self::from_xid(XID::from_bytes(bytes)?))
    }
}

/// A request.
pub trait Request: auto::AsByteSequence {
    type Reply: auto::AsByteSequence;
    fn opcode(&self) -> usize;
}
