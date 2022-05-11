// MIT/Apache2 License

cfg_async! {
    mod unblock;
    pub(crate) use unblock::Unblock;
}

cfg_test! {
    mod test;
    pub(crate) use test::setup_tracing;
}

use ahash::RandomState;
use hashbrown::{HashMap as HbHashMap, HashSet as HbHashSet};

/// A hash map that uses the AHash algorithm.
///
/// It is marginally more vulnerable to denial of service attacks than
/// default hashmaps, but it is also much faster.
pub(crate) type HashMap<K, V> = HbHashMap<K, V, RandomState>;

/// A hash set that uses the AHash algorithm.
pub(crate) type HashSet<V> = HbHashSet<V, RandomState>;

pub(crate) trait ResultExt<T>: Sized {
    fn trace(self, f: impl FnOnce(&T)) -> Self;
}

impl<T> ResultExt<T> for crate::Result<T> {
    fn trace(self, f: impl FnOnce(&T)) -> Self {
        if let Ok(ref t) = self {
            f(t);
        }

        self
    }
}
