// MIT/Apache2 License

use core::iter;
use tinyvec::{Array, TinyVec};

// equality isn't supported in where clauses yet
pub(crate) trait IsAU8: Default + Clone {
    #[inline]
    fn zero() -> Self;
}

impl IsAU8 for u8 {
    #[inline]
    fn zero() -> Self {
        0
    }
}

#[inline]
pub(crate) fn cycled_zeroes<U: IsAU8, A: Array<Item = U>>(len: usize) -> TinyVec<A> {
    iter::once(U::zero()).cycle().take(len).collect()
}
