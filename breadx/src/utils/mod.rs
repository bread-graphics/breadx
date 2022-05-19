// MIT/Apache2 License

cfg_async! {
    mod unblock;
    pub(crate) use unblock::Unblock;
}

cfg_test! {
    mod test;
    pub(crate) use test::setup_tracing;
}

use crate::{display::AsyncStatus, Result};
use ahash::RandomState;
use hashbrown::HashMap as HbHashMap;

/// A hash map that uses the `ahash` algorithm.
///
/// It is marginally more vulnerable to denial of service attacks than
/// default hashmaps, but it is also much faster.
pub(crate) type HashMap<K, V> = HbHashMap<K, V, RandomState>;

pub(crate) trait ResultExt<T>: Sized {
    fn trace(self, f: impl FnOnce(&T)) -> Self;

    fn acopied<'a, R: 'a + Copy>(self) -> Result<AsyncStatus<R>>
    where
        T: Into<AsyncStatus<&'a R>>;
}

impl<T> ResultExt<T> for Result<T> {
    fn trace(self, f: impl FnOnce(&T)) -> Self {
        if let Ok(ref t) = self {
            f(t);
        }

        self
    }

    fn acopied<'a, R: 'a + Copy>(self) -> Result<AsyncStatus<R>>
    where
        T: Into<AsyncStatus<&'a R>>,
    {
        match self {
            Ok(t) => Ok(t.into().copied()),
            Err(e) => Err(e),
        }
    }
}
