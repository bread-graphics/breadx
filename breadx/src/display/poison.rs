// MIT/Apache2 License

use crate::{Error, Result};

/// A wrapper around a type that may be able to be poisoned if it runs
/// an operation that may put it into an invalid state.
pub(crate) struct Poisonable<T> {
    inner: Option<T>,
}

impl<T> Poisonable<T> {
    /// Run a poison-aware operation on the inner value.
    pub(crate) fn with<R>(&mut self, f: impl FnOnce(&mut T) -> Result<R>) -> Result<R> {
        let inner = match self.inner {
            Some(ref mut inner) => inner,
            None => return Err(Error::make_poisoned::<T>()),
        };

        match f(inner) {
            Err(err) if err.invalid_state() => {
                // we've been put into an invalid state, so poison ourselves
                self.inner = None;
                Err(err)
            }
            other_result => other_result,
        }
    }

    /// Sometimes we just want a ref.
    pub(crate) fn with_ref<R>(&self, f: impl FnOnce(&T) -> Result<R>) -> Result<R> {
        let inner = match self.inner {
            Some(ref inner) => inner,
            None => return Err(Error::make_poisoned::<T>()),
        };

        f(inner)
    }
}

impl<T> From<T> for Poisonable<T> {
    fn from(inner: T) -> Self {
        Self { inner: Some(inner) }
    }
}
