// MIT/Apache2 License

#![cfg(test)]

use alloc::vec::Vec;

use super::{Connection, IoSlice};
use crate::Fd;

/// It is useful, for testing, to just have a `Connection` that reads and writes from
/// internal buffers.
pub(crate) struct TestConnection<'a> {
    /// Bytes that are passed to the reader.
    read_bytes: &'a [u8],
    /// File descriptors that are passed to the reader.
    read_fds: &'a mut Vec<Fd>,

    /// Bytes that are passed from the writer.
    written_bytes: &'a mut Vec<u8>,
    /// File descriptors that are passed from the writer.
    written_fds: &'a mut Vec<Fd>,
}

impl<'a> TestConnection<'a> {
    /// Create a new `TestConnection`.
    pub fn new(
        read_bytes: &'a [u8],
        read_fds: &'a mut Vec<Fd>,
        written_bytes: &'a mut Vec<u8>,
        written_fds: &'a mut Vec<Fd>,
    ) -> Self {
        Self {
            read_bytes,
            read_fds,
            written_bytes,
            written_fds,
        }
    }
}

impl<'a> Connection for TestConnection<'a> {
    fn write_slices_with_fds(
        &mut self,
        slices: &[IoSlice<'_>],
        fds: &mut Vec<Fd>,
    ) -> crate::Result<usize> {
        let mut len = 0;
        for slice in slices {
            len += slice.len();
            self.written_bytes.extend_from_slice(&*slice);
        }

        self.written_fds.append(fds);
        Ok(len)
    }

    fn read_to_buffer_with_fds(
        &mut self,
        buffer: &mut [u8],
        fds: &mut Vec<Fd>,
    ) -> crate::Result<usize> {
        let n = std::cmp::min(self.read_bytes.len(), buffer.len());
        buffer[..n].copy_from_slice(&self.read_bytes[..n]);
        self.read_bytes = &self.read_bytes[n..];

        self.read_fds.append(fds);

        Ok(n)
    }
}
