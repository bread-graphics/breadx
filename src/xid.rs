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
    fn size(&self) -> usize {
        self.xid().size()
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
    pub inc: XID,
    pub base: XID,
    mask: XID,
}

impl XidGenerator {
    #[inline]
    pub const fn new(base: XID, mask: XID) -> Self {
        Self {
            last: 0,
            max: 0,
            base,
            inc: mask & mask.wrapping_neg(),
            mask,
        }
    }

    #[inline]
    pub const fn eval_in_place(&self) -> XID {
        self.last | self.base
    }

    #[inline]
    pub fn next(&mut self) -> Option<XID> {
        if self.last >= self.max.wrapping_sub(self.inc).wrapping_add(1) {
            assert_eq!(self.last, self.max);
            if self.last == 0 {
                self.max = self.mask;
                self.last = self.inc;
            } else {
                return None;
            }
        } else {
            self.last = self.last.wrapping_add(self.inc);
        }

        Some(self.eval_in_place())
    }
}
