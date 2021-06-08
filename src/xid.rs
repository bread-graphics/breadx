// MIT/Apache2 License

use crate::auto;
use core::cell::Cell;

/// An X11 ID.
#[allow(clippy::upper_case_acronyms)]
pub type XID = u32;

/// A type that acts as a wrapper around an XID.
pub trait XidType {
    fn xid(&self) -> XID;
    fn from_xid(xid: XID) -> Self;

    #[inline]
    fn count_ones(&self) -> usize {
        self.xid().count_ones() as usize
    }
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
#[derive(Debug, Default, Clone, Copy)]
pub struct XidGenerator {
    pub last: XID,
    pub max: XID,
    pub inc: XID,
    pub base: XID,
    mask: XID,
}

impl XidGenerator {
    #[must_use]
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

    #[must_use]
    #[inline]
    pub const fn eval_in_place(&self) -> XID {
        self.last | self.base
    }

    #[inline]
    pub fn next_xid(&mut self) -> Option<XID> {
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

/// XID Generator, but implemented using `Cell`
#[derive(Debug, Default)]
pub struct CellXidGenerator {
    pub last: Cell<XID>,
    pub max: Cell<XID>,
    pub inc: XID,
    pub base: XID,
    mask: XID,
}

impl From<XidGenerator> for CellXidGenerator {
    #[inline]
    fn from(x: XidGenerator) -> Self {
        let XidGenerator {
            last,
            max,
            inc,
            base,
            mask,
        } = x;
        Self {
            last: Cell::new(last),
            max: Cell::new(max),
            inc,
            base,
            mask,
        }
    }
}

impl CellXidGenerator {
    #[must_use]
    #[inline]
    pub const fn new(base: XID, mask: XID) -> Self {
        Self {
            last: Cell::new(0),
            max: Cell::new(0),
            base,
            inc: mask & mask.wrapping_neg(),
            mask,
        }
    }

    #[inline]
    pub fn eval_in_place(&self) -> XID {
        self.last.get() | self.base
    }

    #[inline]
    pub fn next_xid(&self) -> Option<XID> {
        if self.last.get() >= self.max.get().wrapping_sub(self.inc).wrapping_add(1) {
            assert_eq!(self.last.get(), self.max.get());
            if self.last.get() == 0 {
                self.max.set(self.mask);
                self.last.set(self.inc);
            } else {
                return None;
            }
        } else {
            self.last.set(self.last.get().wrapping_add(self.inc));
        }

        Some(self.eval_in_place())
    }
}
