// MIT/Apache2 License

use core::iter;
use tinyvec::{Array, TinyVec};

#[inline]
pub(crate) fn cycled_zeroes<A: Array<Item = u8>>(len: usize) -> TinyVec<A> {
    iter::once(0).cycle().take(len).collect()
}
