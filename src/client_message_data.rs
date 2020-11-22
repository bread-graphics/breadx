// MIT/Apache2 License

use crate::auto::AsByteSequence;
use core::mem;

/// The data returned from a client message.
#[derive(Default, Debug, Clone)]
#[repr(transparent)]
pub struct ClientMessageData {
    // convention states that it should be at least 5 u32's
    // wide
    data: [u32; LONG_LEN],
}

// constants that make things easier
pub const LONG_LEN: usize = 5;
pub const SHORT_LEN: usize = LONG_LEN * SHORT_INTERVAL;
pub const SHORT_INTERVAL: usize = mem::size_of::<u32>() / mem::size_of::<u16>();
pub const BYTE_LEN: usize = LONG_LEN * BYTE_INTERVAL;
pub const BYTE_INTERVAL: usize = mem::size_of::<u32>() / mem::size_of::<u8>();

impl ClientMessageData {
    /// Get the bytes assocated with this sequence.
    #[inline]
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        bytemuck::cast_slice::<u32, u8>(&self.data)
    }

    /// Get the short integers assocated with this sequence.
    #[inline]
    #[must_use]
    pub fn shorts(&self) -> &[u16] {
        bytemuck::cast_slice::<u32, u16>(&self.data)
    }

    /// Get the long integers associated with this sequence.
    #[inline]
    #[must_use]
    pub fn longs(&self) -> &[u32] {
        &self.data
    }

    /// Get a mutable reference to the bytes associated with this sequence.
    #[inline]
    #[must_use]
    pub fn bytes_mut(&mut self) -> &mut [u8] {
        bytemuck::cast_slice_mut::<u32, u8>(&mut self.data)
    }

    /// Get a mutable reference to the short integers associated with this sequence.
    #[inline]
    #[must_use]
    pub fn shorts_mut(&mut self) -> &mut [u16] {
        bytemuck::cast_slice_mut::<u32, u16>(&mut self.data)
    }

    /// Get a mutable reference to the long integers associated with this sequence.
    #[inline]
    #[must_use]
    pub fn longs_mut(&mut self) -> &mut [u32] {
        &mut self.data
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
        if bytes.len() < BYTE_LEN {
            None
        } else {
            let mut res: [u32; LONG_LEN] = [0; LONG_LEN];
            res.copy_from_slice(bytemuck::cast_slice(bytes));
            Some((ClientMessageData { data: res }, BYTE_LEN))
        }
    }
}
