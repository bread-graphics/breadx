// MIT/Apache2 License

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use core::cmp;
use core::fmt;

use super::Connection;
use crate::connection::IoSlice;
use crate::{Error, Fd, InvalidState, Result};

// libxcb uses these values by default
const DEFAULT_READ_CAPACITY: usize = 4096;
const DEFAULT_WRITE_CAPACITY: usize = 16384;

/// A wrapper around a [`Connection`] that buffers reads and writes.
#[derive(Debug)]
pub struct BufConnection<C> {
    // the object we are wrapping
    conn: C,

    // the buffer for reads
    read_buf: Buffer,

    // the cursor into the read buffer that we've read to
    read_cursor: usize,

    // the buffer for writes
    write_buf: Buffer,
}

struct Buffer {
    /// The bytes that we haven't written yet.
    bytes: Box<[u8]>,
    /// The number of bytes in `bytes` that are valid.
    len: usize,
    /// A list of file descriptors that we haven't sent yet.
    fds: Vec<Fd>,
}

impl<C: Connection> BufConnection<C> {
    /// Create a new `BufConnection`.
    pub fn new(conn: C) -> Self {
        Self::with_capacity(DEFAULT_READ_CAPACITY, DEFAULT_WRITE_CAPACITY, conn)
    }

    /// Create a new `BufConnection` with a specified capacity.
    pub fn with_capacity(read_capacity: usize, write_capacity: usize, conn: C) -> Self {
        Self {
            conn,
            read_buf: Buffer::with_capacity(read_capacity),
            read_cursor: 0,
            write_buf: Buffer::with_capacity(write_capacity),
        }
    }

    /// Get the inner connection.
    pub fn get_ref(&self) -> &C {
        &self.conn
    }

    /// Get the inner connection.
    pub fn get_mut(&mut self) -> &mut C {
        &mut self.conn
    }

    /// Get the amount of space we have left in our write buffer.
    fn spare_write_capacity(&self) -> usize {
        self.write_buf.spare_capacity()
    }

    /// Get the amount of space we have left in our read buffer.
    fn spare_read_capacity(&self) -> usize {
        self.read_buf.spare_capacity()
    }

    /// Flush the write buffer to the output.
    fn flush_write_buffer(&mut self) -> Result<()> {
        /// Ensures that the buffer is updated in one fell swoop.
        struct Guard<'a> {
            buffer: &'a mut Box<[u8]>,
            len: &'a mut usize,
            written: usize,
        }

        impl<'a> Guard<'a> {
            fn remaining(&self) -> &[u8] {
                &self.buffer[self.written..*self.len]
            }

            fn advance(&mut self, n: usize) {
                self.written += n;
            }

            fn done(&self) -> bool {
                self.written >= *self.len
            }
        }

        impl<'a> Drop for Guard<'a> {
            fn drop(&mut self) {
                // update the length
                *self.len = (*self.len).saturating_sub(self.written);
            }
        }

        let mut guard = Guard {
            buffer: &mut self.read_buf.bytes,
            len: &mut self.read_buf.len,
            written: 0,
        };

        // write to the output
        while !guard.done() {
            #[cfg(not(feature = "std"))]
            let iov = [guard.remaining()];
            #[cfg(feature = "std")]
            let iov = [IoSlice::new(guard.remaining())];

            // write bytes to the output
            let n = self
                .conn
                .write_slices_with_fds(&iov, &mut self.read_buf.fds)?;
            guard.advance(n);
        }

        // return a normal error if file descriptors aren't written
        if self.write_buf.fds.len() != 0 {
            Err(Error::invalid_state(InvalidState::FdsNotWritten))
        } else {
            Ok(())
        }
    }

    /// Write as much of the buffer as we can to the write buffer.
    fn write_to_buf(&mut self, data: &[u8]) -> usize {
        let available = self.spare_write_capacity();
        let buffered = cmp::min(available, data.len());

        // copy as much as we can into the buffer
        let cur_len = self.write_buf.len;
        self.write_buf.bytes[cur_len..cur_len + buffered].copy_from_slice(&data[..buffered]);

        // update the length
        self.write_buf.len += buffered;

        buffered
    }

    fn write_slice_impl(&mut self, slice: &[u8], fds: &mut Vec<Fd>) -> Result<usize> {
        // flush the buffer if we need to
        if slice.len() > self.spare_write_capacity() {
            self.flush_write_buffer()?;
        }

        // forward to the write_slice impl if the slice doesn't fit
        if slice.len() >= self.write_buf.capacity() {
            if !fds.is_empty() {
                // sigh, we have to vectorize it anyways
                #[cfg(not(feature = "std"))]
                let iov = [slice];
                #[cfg(feature = "std")]
                let iov = [IoSlice::new(slice)];

                return self.conn.write_slices_with_fds(&iov, fds);
            }

            return self.conn.write_slice(slice);
        }

        // write to buffer
        self.write_buf.fds.append(fds);
        Ok(self.write_to_buf(slice))
    }

    fn write_slices_impl(&mut self, iov: &[IoSlice<'_>], fds: &mut Vec<Fd>) -> Result<usize> {
        if self.conn.is_write_vectored() {
            // take advantage of vectored writes
            let total_len = iov
                .iter()
                .fold(0usize, |acc, slice| acc.saturating_add(slice.len()));

            // flush the buffer if the write is too large for it
            if total_len > self.spare_write_capacity() {
                self.flush_write_buffer()?;
            }

            // if the total length is too large for even our empty buffer to hold, just
            // forward to the connection's write impl
            if total_len >= self.write_buf.capacity() {
                return self.conn.write_slices_with_fds(iov, fds);
            }

            // write the remaining slices to the buffer
            self.write_buf.fds.append(fds);
            iov.iter().for_each(|slice| {
                self.write_to_buf(&*slice);
            });

            Ok(total_len)
        } else {
            // we only write one slice at a time
            self.write_slice_impl(&*iov[0], fds)
        }
    }

    /// Make sure the read cursor is in a sane place.
    fn update_read_cursor(&mut self) {
        // if the total window is empty, reset it to zero
        if self.read_buffer_empty() {
            self.read_cursor = 0;
            self.read_buf.len = 0;
        }
    }

    /// Is our read buffer empty?
    fn read_buffer_empty(&self) -> bool {
        self.read_window_size() == 0
    }

    /// The total amount of unread data in the buffer.
    fn read_window_size(&self) -> usize {
        self.read_buf.len - self.read_cursor
    }
}

impl<C: Connection> Connection for BufConnection<C> {
    fn write_slices_with_fds(&mut self, iov: &[IoSlice<'_>], fds: &mut Vec<Fd>) -> Result<usize> {
        self.write_slices_impl(iov, fds)
    }

    fn write_slices(&mut self, iov: &[IoSlice<'_>]) -> Result<usize> {
        self.write_slices_impl(iov, &mut Vec::new())
    }

    fn write_slice(&mut self, slice: &[u8]) -> Result<usize> {
        self.write_slice_impl(slice, &mut Vec::new())
    }

    fn is_write_vectored(&self) -> bool {
        self.conn.is_write_vectored()
    }

    fn read_to_buffer_with_fds(&mut self, buf: &mut [u8], fds: &mut Vec<Fd>) -> Result<usize> {
        // if we have no data in our buffer, and the read is larger than the buffer,
        // forward to the udnerlying read
        if self.read_buffer_empty() && buf.len() >= self.read_buf.capacity() {
            return self.conn.read_to_buffer_with_fds(buf, fds);
        }

        // if the internal buffer is not yet full, read to it
        if self.spare_read_capacity() > 0 {
            let write_window = &mut self.read_buf.bytes[self.read_buf.len..];
            let n = self.conn.read_to_buffer_with_fds(write_window, fds)?;
            self.read_buf.len += n;
        }

        // copy as much as we can from the buffer to the user's buffer
        let copy_amount = cmp::min(buf.len(), self.read_window_size());
        self.read_cursor += copy_amount;
        let read_window = &self.read_buf.bytes[self.read_cursor..self.read_cursor + copy_amount];
        buf[..copy_amount].copy_from_slice(read_window);

        self.update_read_cursor();

        Ok(copy_amount)
    }

    fn flush(&mut self) -> Result<()> {
        self.flush_write_buffer()?;
        self.conn.flush()
    }
}

impl fmt::Debug for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use alloc::format;

        let description = if self.fds.is_empty() {
            format!(
                "{}/{} with {} file descriptors",
                self.len,
                self.bytes.len(),
                self.fds.len()
            )
        } else {
            format!("{}/{}", self.len, self.bytes.len())
        };

        f.debug_tuple("Buffer").field(&description).finish()
    }
}

impl Buffer {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            bytes: vec![0; capacity].into_boxed_slice(),
            len: 0,
            fds: Vec::new(),
        }
    }

    fn spare_capacity(&self) -> usize {
        self.bytes.len() - self.len
    }

    fn capacity(&self) -> usize {
        self.bytes.len()
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::BufConnection;
    use crate::connection::{Connection, TestConnection};

    #[test]
    fn buffered_reads() {
        const DATA: &[u8] = b"This is a long enough sentence to require buffering.";

        let mut _rfds = vec![];
        let mut _wfds = vec![];
        let mut _wbytes = vec![];

        let conn = TestConnection::new(DATA, &mut _rfds, &mut _wfds, &mut _wbytes);

        let mut bufconn = BufConnection::with_capacity(10, 10, conn);

        // several small writes
        let mut buf = [0u8; 3];
        let mut fds = vec![];

        bufconn.read_to_buffer_with_fds(&mut buf, &mut fds).unwrap();
        assert_eq!(buf, [b'T', b'h', b'i']);

        bufconn.read_to_buffer_with_fds(&mut buf, &mut fds).unwrap();
        assert_eq!(buf, [b'i', b' ', b'a']);

        bufconn.read_to_buffer_with_fds(&mut buf, &mut fds).unwrap();
        assert_eq!(buf, [b' ', b'l', b'o']);

        bufconn.read_to_buffer_with_fds(&mut buf, &mut fds).unwrap();
        assert_eq!(buf, [b'n', b'g', b' ']);

        // a larger buffer
        let mut buf = [0u8; 20];
        bufconn.read_to_buffer_with_fds(&mut buf, &mut fds).unwrap();
        assert_eq!(buf.as_ref(), b"enough sentence to req");
    }
}
