// MIT/Apache2 License

use crate::auto::AsByteSequence;
use core::mem;
use cty::{c_char, c_long, c_short};
use tinyvec::ArrayVec;

/// The data returned from a client message.
#[derive(Default, Debug, Clone)]
#[repr(transparent)]
pub struct ClientMessageData {
    // convention states that it should be at least 5 c longs
    // wide
    data: [c_long; LONG_LEN],
}

// constants that make things easier
pub const LONG_LEN: usize = 5;
pub const SHORT_LEN: usize = LONG_LEN * SHORT_INTERVAL;
pub const SHORT_INTERVAL: usize = mem::size_of::<c_long>() / mem::size_of::<c_short>();
pub const BYTE_LEN: usize = LONG_LEN * BYTE_INTERVAL;
pub const BYTE_INTERVAL: usize = mem::size_of::<c_long>() / mem::size_of::<c_char>();

impl ClientMessageData {
    /// Get the bytes assocated with this sequence.
    #[inline]
    pub fn bytes(&self) -> [c_char; BYTE_LEN] {
        let s = bytemuck::cast_slice::<c_long, c_char>(&self.data);
        let mut res = [0; BYTE_LEN];
        res.copy_from_slice(s);
        res
    }

    /// Get the shorts assocated with this sequence.
    #[inline]
    pub fn shorts(&self) -> [c_short; SHORT_LEN] {
        bytemuck::cast(self.data.clone())
    }

    /// Get the longs associated with this sequence.
    #[inline]
    pub fn longs(&self) -> [c_long; LONG_LEN] {
        self.data.clone()
    }
}

impl AsByteSequence for ClientMessageData {
    #[inline]
    fn size(&self) -> usize {
        BYTE_LEN
    }

    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        bytes.copy_from_slice(bytemuck::cast_slice(&self.data));
        BYTE_LEN
    }

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        if bytes.len() <= BYTE_LEN {
            None
        } else {
            let mut res: [c_long; LONG_LEN] = [0; LONG_LEN];
            res.copy_from_slice(bytemuck::cast_slice(bytes));
            Some((ClientMessageData { data: res }, BYTE_LEN))
        }
    }
}
