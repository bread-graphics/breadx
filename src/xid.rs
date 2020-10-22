// MIT/Apache2 License

use super::auto;

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

/// XID Generator
#[derive(Debug, Default)]
pub(crate) struct XidGenerator {
    pub last: XID,
    pub max: XID,
    pub base: XID,
    pub inc: XID,
}

impl XidGenerator {
    #[inline]
    pub const fn new(base: XID, mask: XID) -> Self {
        let inc = mask & (-(mask as i32)) as XID;

        Self {
            last: 0,
            max: 0,
            base,
            inc: mask & mask.wrapping_neg(),
        }
    }
}
