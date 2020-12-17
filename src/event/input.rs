// MIT/Apache2 License

#![cfg(feature = "input")]

use crate::auto::AsByteSequence;
use alloc::boxed::Box;

/// TODO: placeholder for input events
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum InputEvent {
    NoneOfTheAbove(Box<[u8]>),
}

impl Default for InputEvent {
    #[inline]
    fn default() -> InputEvent {
        Self::NoneOfTheAbove(Default::default())
    }
}

impl AsByteSequence for InputEvent {
    #[inline]
    fn size(&self) -> usize {
        match self {
            Self::NoneOfTheAbove(data) => data.len(),
        }
    }

    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        match self {
            Self::NoneOfTheAbove(data) => {
                (&mut bytes[..data.len()]).copy_from_slice(data);
                data.len()
            }
        }
    }

    #[inline]
    fn from_bytes(_bytes: &[u8]) -> Option<(Self, usize)> {
        panic!("Unable to deserialize InputEvent from bytes")
    }
}
