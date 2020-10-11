// MIT/Apache2 License

use crate::auto::AsByteSequence;
use std::{
    mem,
    os::raw::{c_char, c_long, c_short},
};

/// The data returned from a client message.
#[derive(Default, Clone)]
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
        let mut res = [0; BYTE_LEN];
        // hopefully, this gets optimized out
        for (i, d) in self.data.iter().enumerate() {
            let d = {
                let d = d.to_ne_bytes();
                let mut res: [c_char; BYTE_LEN] = [0; BYTE_LEN];
                for (index, i) in d.iter().enumerate() {
                    res[index] = *i as _;
                }
                res
            };

            let start = i * BYTE_INTERVAL;
            (&mut res[start..start + BYTE_INTERVAL]).copy_from_slice(&d);
        }
        res
    }

    /// Get the shorts assocated with this sequence.
    #[inline]
    pub fn shorts(&self) -> [c_short; SHORT_LEN] {
        let mut res = [0; SHORT_LEN];
        // hopefully, this gets optimized out
        for (i, d) in self.data.iter().enumerate() {
            let d = d.to_ne_bytes();
            let start = i * SHORT_INTERVAL;
            res[start] = c_short::from_ne_bytes([d[0], d[1]]);
            res[start + 1] = c_short::from_ne_bytes([d[2], d[3]]);
        }
        res
    }

    /// Get the longs associated with this sequence.
    #[inline]
    pub fn longs(&self) -> [c_long; LONG_LEN] {
        self.data.clone()
    }
}

impl AsByteSequence for ClientMessageData {
    #[inline]
    fn size() -> usize {
        mem::size_of::<c_long>() * LONG_LEN
    }

    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) {
        let b = self.bytes();
        let mut res: [u8; BYTE_LEN] = [0; BYTE_LEN];
        for (index, i) in b.iter().enumerate() {
            res[index] = *i as _;
        }
        bytes.copy_from_slice(&res)
    }

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() <= BYTE_LEN {
            None
        } else {
            let mut b = [0; BYTE_LEN];
            b.copy_from_slice(bytes);
            let mut res = [0; LONG_LEN];
            for i in 0..LONG_LEN {
                let start = i * mem::size_of::<c_long>();
                let mut indiv_number = [0; mem::size_of::<c_long>()];
                for j in 0..mem::size_of::<c_long>() {
                    indiv_number[j] = b[start + j];
                }
                res[i] = c_long::from_ne_bytes(indiv_number);
            }
            Some(res)
        }
    }
}
