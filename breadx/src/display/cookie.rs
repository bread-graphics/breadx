// MIT/Apache2 License

//! Annotated request cookie types.

use core::marker::PhantomData;
use core::{fmt, hash::Hash};

/// A sequence number indicating the state of a request.
pub struct Cookie<T> {
    sequence: u64,
    marker: PhantomData<T>,
}

impl<T> Clone for Cookie<T> {
    fn clone(&self) -> Self {
        Self {
            sequence: self.sequence,
            marker: PhantomData,
        }
    }
}

impl<T> Copy for Cookie<T> {}

impl<T> PartialEq for Cookie<T> {
    fn eq(&self, other: &Self) -> bool {
        self.sequence == other.sequence
    }
}

impl<T> Eq for Cookie<T> {}

impl<T> PartialOrd for Cookie<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.sequence.partial_cmp(&other.sequence)
    }
}

impl<T> Ord for Cookie<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.sequence.cmp(&other.sequence)
    }
}

impl<T> Hash for Cookie<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.sequence.hash(state);
    }
}

impl<T> fmt::Debug for Cookie<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.sequence, f)
    }
}

impl<T> fmt::Display for Cookie<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.sequence, f)
    }
}

impl<T> Cookie<T> {
    pub fn from_sequence(sequence: u64) -> Self {
        Self {
            sequence,
            marker: PhantomData,
        }
    }

    pub fn sequence(self) -> u64 {
        self.sequence
    }
}

impl<T> From<u64> for Cookie<T> {
    fn from(sequence: u64) -> Self {
        Self::from_sequence(sequence)
    }
}

impl<T> From<Cookie<T>> for u64 {
    fn from(cookie: Cookie<T>) -> Self {
        cookie.sequence
    }
}
