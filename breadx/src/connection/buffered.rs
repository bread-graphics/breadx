// MIT/Apache2 License

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use core::cmp;
use core::ops::Range;

use super::Connection;
use super::{new_io_slice, new_io_slice_mut};
use crate::connection::{IoSlice, IoSliceMut};
use crate::{Fd, Result};

cfg_std_unix! {
    use std::os::unix::io::{AsRawFd, RawFd};
}

cfg_std_windows! {
    use std::os::windows::io::{AsRawSocket, RawSocket};
}

// libxcb uses these values by default
const DEFAULT_READ_CAPACITY: usize = 4096;
const DEFAULT_WRITE_CAPACITY: usize = 16384;

/// A wrapper around a [`Connection`] that buffers all of the I/O.
///
/// System calls for I/O are expensive, while reading to and writing from
/// memory is not. This type aims to provide a simple wrapper around a
/// [`Connection`] that buffers all of the I/O. It serves a similar purpose
/// to [`BufReader`] and [`BufWriter`], but for [`Connection`]s.
///
/// If the [`Connection`] is already in memory, then `BufReader` serves no
/// purpose. In addition, programs that make larger reads/writes tend to lose the
/// advantage of buffering. However, smaller, more frequent reads are
/// common in X11, so this type is useful in general.
///
/// ## Example
///
/// ```rust,no_run
/// use breadx::connection::{BufConnection, Connection, StdConnection};
/// use std::net::TcpStream;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // create a connection that isn't buffered
/// let socket = TcpStream::connect("localhost:6000")?;
/// let connection = StdConnection::new(socket);
///
/// // create a connection that is buffered
/// let mut connection = BufConnection::new(connection);
///
/// let (mut buf1, mut buf2) = ([0u8; 16], [0u8; 16]);
///
/// // these two reads would normally result in two syscalls
/// // however, with buffering, only one syscall occurs
/// connection.recv_slice(&mut buf1)?;
/// connection.recv_slice(&mut buf2)?;
/// # Ok(()) }
/// ```
///
/// [`Connection`]: crate::connection::Connection
/// [`BufReader`]: std::io::BufReader
/// [`BufWriter`]: std::io::BufWriter
pub struct BufConnection<C> {
    /// The connection we're wrapping around.
    conn: C,
    /// The read buffer, containing data that has been read from the
    /// connection but not yet returned to the caller.
    read_buf: ReadBuffer,
    /// The write buffer, containing data that has been written to the
    /// connection but not yet flushed.
    write_buf: WriteBuffer,
}

/// The buffer used to store reads.
struct ReadBuffer {
    /// The buffer of data that we've read from the connection.
    buf: Box<[u8]>,
    /// The position into the buffer that contains data to return
    /// to the user.
    ///
    /// The lower bound is the position where the data that the user has
    /// already read stops. The upper bound is where the data that we
    /// haven't read to yet begins.
    valid_range: Range<usize>,
    /// The file descriptors that we have collected, but haven't yet
    /// returned to the user.
    fds: Vec<Fd>,
}

/// The buffer used to store writes.
struct WriteBuffer {
    /// The buffer of data that we haven't wrote to the connection.
    buf: Box<[u8]>,
    /// The position into the buffer that contains actual writable
    /// data.
    ///
    /// Every other byte should be considered uninitialized.
    writable: usize,
    /// The file descriptors that we have collected, but haven't yet
    /// written to the connection.
    fds: Vec<Fd>,
}

impl<C: Connection> From<C> for BufConnection<C> {
    fn from(conn: C) -> Self {
        Self::new(conn)
    }
}

impl<C: Connection> AsRef<C> for BufConnection<C> {
    fn as_ref(&self) -> &C {
        &self.conn
    }
}

impl<C: Connection> AsMut<C> for BufConnection<C> {
    fn as_mut(&mut self) -> &mut C {
        &mut self.conn
    }
}

cfg_std_unix! {
    impl<C: AsRawFd> AsRawFd for BufConnection<C> {
        fn as_raw_fd(&self) -> RawFd {
            self.conn.as_raw_fd()
        }
    }
}

cfg_std_windows! {
    impl<C: AsRawSocket> AsRawSocket for BufConnection<C> {
        fn as_raw_socket(&self) -> RawSocket {
            self.conn.as_raw_socket()
        }
    }
}

impl<C: Connection> BufConnection<C> {
    /// Create a new `BufConnection` from a given connection.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use breadx::connection::{BufConnection, Connection, StdConnection};
    /// use std::net::TcpStream;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let socket = TcpStream::connect("localhost:6000")?;
    /// let connection = StdConnection::new(socket);
    /// let connection = BufConnection::new(connection);
    /// # let _ = connection;
    /// # Ok(()) }
    /// ```
    pub fn new(conn: C) -> Self {
        Self::with_capacity(DEFAULT_READ_CAPACITY, DEFAULT_WRITE_CAPACITY, conn)
    }

    /// Create a new `BufConnection` from a given connection, with
    /// the given read and write capacities.
    ///
    /// This can be useful if you expect your program to use different
    /// amounts of read and write data than normal ones do.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use breadx::connection::{BufConnection, Connection, StdConnection};
    /// use std::net::TcpStream;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let socket = TcpStream::connect("localhost:6000")?;
    /// let connection = StdConnection::new(socket);
    /// let connection = BufConnection::with_capacity(1024, 2048, connection);
    /// # let _ = connection;
    /// # Ok(()) }
    /// ```
    pub fn with_capacity(read_capacity: usize, write_capacity: usize, conn: C) -> Self {
        let read_buf = ReadBuffer {
            buf: vec![0; read_capacity].into_boxed_slice(),
            valid_range: 0..0,
            fds: Vec::new(),
        };

        let write_buf = WriteBuffer {
            buf: vec![0; write_capacity].into_boxed_slice(),
            writable: 0,
            fds: Vec::new(),
        };

        Self {
            conn,
            read_buf,
            write_buf,
        }
    }

    /// Flush the `WriteBuffer` to the underlying connection.
    ///
    /// This will flush the `WriteBuffer` to the underlying connection,
    /// and then clear the `WriteBuffer`.
    ///
    /// If the `WriteBuffer` is empty, this is a no-op.
    ///
    /// Returns the number of bytes written.
    fn flush_write_buffer(&mut self) -> Result<usize> {
        let mut nwritten = 0;
        while nwritten < self.write_buf.writable {
            // if we have file descriptors, make sure that we write
            // those
            let buffer = &self.write_buf.buf[nwritten..self.write_buf.writable];
            if self.write_buf.fds.is_empty() {
                nwritten += self.conn.send_slice(buffer)?;
            } else {
                nwritten += self
                    .conn
                    .send_slices_and_fds(&[new_io_slice(buffer)], &mut self.write_buf.fds)?;
            }
        }

        self.write_buf.flush();

        tracing::trace!("Flushed {} bytes to underlying connection", nwritten);

        Ok(nwritten)
    }

    /// Copy the slice of data into our write buffer.
    fn copy_slice_to_buffer(&mut self, slice: &[u8]) -> usize {
        let amt = cmp::min(self.write_buf.spare_capacity(), slice.len());

        // use copy_from_slice to make the copy
        let out = self.write_buf.empty_slice();
        out[..amt].copy_from_slice(&slice[..amt]);

        // update the write buffer
        self.write_buf.advance(amt);

        amt
    }

    /// Implementation for `send_slices_and_fds` and `send_slices`.
    ///
    /// fds is a `&mut Vec<Fd>`, which means we can't split it up among
    /// two functions. Therefore, `write_handler` handles both cases:
    ///
    /// - if we need to forward the slices, it will call with true
    /// - if we just need to push fds, it will call with false
    fn send_slices_impl(
        &mut self,
        slices: &[IoSlice<'_>],
        write_handler: impl FnOnce(&mut Self, &[IoSlice<'_>], bool) -> Result<usize>,
    ) -> Result<usize> {
        // get the total length of the data we're going to write
        let total_len = slices
            .iter()
            .map(|s| s.len())
            .fold(0usize, usize::saturating_add);

        let span = tracing::debug_span!(
            "BufConnection::send_slices_impl",
            num_slices = slices.len(),
            total_len = total_len
        );
        let _enter = span.enter();

        // if we can't fit it into the current buffer, flush it
        if self.write_buf.spare_capacity() <= total_len {
            tracing::trace!("flushing write buffer");
            self.flush_write_buffer()?;
        }

        // if the total length is larger than the buffer is, forward the
        // impl to the underlying connection
        if total_len > self.write_buf.capacity() {
            // calling write_handler with "true" indicates we're doing
            // a true write
            tracing::debug!(
                "write is too large for buffer, \
            forwarding to inner impl"
            );
            return write_handler(self, slices, true);
        }

        // write into our buffer and return
        let mut nwritten = 0;
        for slice in slices {
            nwritten += self.copy_slice_to_buffer(slice);
        }

        tracing::trace!("wrote {} bytes to buffer", nwritten);

        // if we have fds, copy them to the buffer
        // calling write_handler with "false" indicates we're just
        // pushing file descriptors into a buffer
        write_handler(self, &[], false)?;

        Ok(total_len)
    }

    /// Copy from the read buffer into the given slice.
    fn copy_into_slice(&mut self, slice: &mut [u8]) -> usize {
        let amt = cmp::min(slice.len(), self.read_buf.readable_slice().len());
        let buf = self.read_buf.readable_slice();

        slice[..amt].copy_from_slice(&buf[..amt]);
        self.read_buf.advance_read(amt);
        amt
    }

    /// Copy from the read buffer into the given I/O slices.
    ///
    /// Returns the number of bytes copied.
    fn copy_into_slices(&mut self, slices: &mut [IoSliceMut<'_>]) -> usize {
        let mut amt_copied = 0;

        for slice in slices {
            amt_copied += self.copy_into_slice(&mut *slice);
        }

        amt_copied
    }

    fn recv_slices_impl(
        &mut self,
        slices: &mut [IoSliceMut<'_>],
        fds: &mut Vec<Fd>,
        mut read_handler: impl FnMut(&mut C, &mut [u8], &mut Vec<Fd>) -> Result<usize>,
    ) -> Result<usize> {
        // get the total length of the data we're going to read
        let total_len = slices
            .iter()
            .map(|s| s.len())
            .fold(0usize, usize::saturating_add);

        let span = tracing::debug_span!(
            "BufConnection::recv_slices_impl",
            num_slices = slices.len(),
            total_len = total_len,
        );
        let _enter = span.enter();

        // if the amount that we need is not in the buffer, try to preform
        // a read
        if total_len > self.read_buf.readable() {
            tracing::debug!(
                "total length {} does not fit in buffer of size {}, \
                forwarding to read_handler",
                total_len,
                self.read_buf.readable()
            );

            let amt = read_handler(
                &mut self.conn,
                &mut self.read_buf.buf[self.read_buf.valid_range.end..],
                &mut self.read_buf.fds,
            )?;
            self.read_buf.advance_write(amt);
        }

        // copy the data into the slices
        let amt_copied = self.copy_into_slices(slices);
        fds.append(&mut self.read_buf.fds);

        tracing::trace!(
            "copied amt {} of {} bytes into buffer",
            amt_copied,
            total_len
        );

        Ok(amt_copied)
    }

    fn recv_slice_impl(
        &mut self,
        slice: &mut [u8],
        fds: Option<&mut Vec<Fd>>,
        mut read_handler: impl FnMut(&mut C, &mut [IoSliceMut<'_>], &mut Vec<Fd>) -> Result<usize>,
    ) -> Result<usize> {
        let span = tracing::debug_span!("BufConnection::recv_slice_impl", len = slice.len(),);
        let _enter = span.enter();

        // if the amount that we need is not in the buffer, try to preform
        // a read
        if slice.len() > self.read_buf.readable() {
            /*tracing::debug!(
                "total length {} does not fit in buffer of size {}, \
                forwarding to read_handler",
                slice.len(),
                self.read_buf.readable()
            );*/

            // only logically possible if the buffer is empty
            /*if self.read_buf.is_empty() {
                tracing::trace!("attempting vectored read to fill both buffers at once");

                // life hack: try reading into the user's slice and the buffer
                // at the same time using a vectored read
                let mut iov = [
                    new_io_slice_mut(slice),
                    new_io_slice_mut(&mut self.read_buf.buf[self.read_buf.valid_range.end..]),
                ];
                let amt = read_handler(&mut self.conn, &mut iov, &mut self.read_buf.fds)?;

                // determine how many of each got written where
                let buffer_bytes = amt.saturating_sub(slice.len());
                let slice_bytes = amt - buffer_bytes;

                tracing::trace!(buffer_bytes, slice_bytes,);

                self.read_buf.advance_write(buffer_bytes);
                return Ok(slice_bytes);
            }*/

            // just do a normal buffer read
            let mut iov = [new_io_slice_mut(
                &mut self.read_buf.buf[self.read_buf.valid_range.end..],
            )];

            let amt = read_handler(&mut self.conn, &mut iov, &mut self.read_buf.fds)?;
            self.read_buf.advance_write(amt);
        }

        let amt = self.copy_into_slice(slice);
        if let Some(fds) = fds {
            fds.append(&mut self.read_buf.fds);
        }

        tracing::trace!("copied amt {} of {} bytes into buffer", amt, slice.len());

        Ok(amt)
    }
}

impl<Conn: Connection> Connection for BufConnection<Conn> {
    fn send_slices_and_fds(&mut self, slices: &[IoSlice<'_>], fds: &mut Vec<Fd>) -> Result<usize> {
        self.send_slices_impl(slices, move |this, slices, true_write| {
            if true_write {
                this.conn.send_slices_and_fds(slices, fds)
            } else {
                this.write_buf.fds.append(fds);
                Ok(0)
            }
        })
    }

    fn send_slices(&mut self, slices: &[IoSlice<'_>]) -> Result<usize> {
        self.send_slices_impl(slices, |this, slice, true_write| {
            if true_write {
                this.conn.send_slices(slice)
            } else {
                Ok(0)
            }
        })
    }

    fn send_slice(&mut self, slice: &[u8]) -> Result<usize> {
        // if we can't fit the slice in the current write buffer,
        // flush the buffer
        if slice.len() >= self.write_buf.spare_capacity() {
            self.flush_write_buffer()?;
        }

        // if the slice will never fit in the current write buffer,
        // forward the call to the underlying connection
        if slice.len() > self.write_buf.capacity() {
            return self.conn.send_slice(slice);
        }

        // copy the slice into the write buffer
        self.copy_slice_to_buffer(slice);

        Ok(slice.len())
    }

    fn recv_slices_and_fds(
        &mut self,
        slices: &mut [IoSliceMut<'_>],
        fds: &mut Vec<Fd>,
    ) -> Result<usize> {
        self.recv_slices_impl(slices, fds, |conn, slice, fds| {
            conn.recv_slice_and_fds(slice, fds)
        })
    }

    fn recv_slice_and_fds(&mut self, slice: &mut [u8], fds: &mut Vec<Fd>) -> Result<usize> {
        self.recv_slice_impl(slice, Some(fds), |conn, slices, fds| {
            conn.recv_slices_and_fds(slices, fds)
        })
    }

    fn recv_slice(&mut self, slice: &mut [u8]) -> Result<usize> {
        self.recv_slice_impl(slice, None, |conn, slices, fds| {
            conn.recv_slices_and_fds(slices, fds)
        })
    }

    fn non_blocking_recv_slices_and_fds(
        &mut self,
        slices: &mut [IoSliceMut<'_>],
        fds: &mut Vec<Fd>,
    ) -> Result<usize> {
        self.recv_slices_impl(slices, fds, |conn, slice, fds| {
            conn.non_blocking_recv_slice_and_fds(slice, fds)
        })
    }

    fn non_blocking_recv_slice_and_fds(
        &mut self,
        slice: &mut [u8],
        fds: &mut Vec<Fd>,
    ) -> Result<usize> {
        self.recv_slice_impl(slice, Some(fds), |conn, slices, fds| {
            conn.non_blocking_recv_slices_and_fds(slices, fds)
        })
    }

    fn flush(&mut self) -> Result<()> {
        self.flush_write_buffer()?;
        self.conn.flush()
    }

    fn shutdown(&self) -> Result<()> {
        self.conn.shutdown()
    }
}

impl ReadBuffer {
    /// Get the slice of the `ReadBuffer` that has already been read
    /// and contains valid information
    fn readable_slice(&self) -> &[u8] {
        &self.buf[self.valid_range.clone()]
    }

    fn readable(&self) -> usize {
        self.readable_slice().len()
    }

    /// Indicate that we've read `n` bytes from the connection into
    /// the buffer.
    fn advance_write(&mut self, n: usize) {
        self.valid_range.end += n;
        debug_assert!(self.valid_range.end <= self.buf.len());
    }

    /// Indicate that we've given `n` bytes to the user.
    fn advance_read(&mut self, n: usize) {
        self.valid_range.start += n;
        debug_assert!(self.valid_range.start <= self.valid_range.end);

        // reset ringbuffer back to start if we're empty now
        if self.valid_range.is_empty() {
            self.valid_range = 0..0;
        }
    }
}

impl WriteBuffer {
    /// Get the slice of the `WriteBuffer` that hasn't been written to yet.
    fn empty_slice(&mut self) -> &mut [u8] {
        &mut self.buf[self.writable..]
    }

    /// Indicate that we've written `n` bytes to the connection.
    fn advance(&mut self, n: usize) {
        self.writable += n;
        debug_assert!(self.writable <= self.buf.len());
    }

    /// Indicate that we've flushed the buffer to the connection.
    fn flush(&mut self) {
        self.writable = 0;
    }

    /// Get the spare capacity of the buffer.
    fn spare_capacity(&self) -> usize {
        self.buf.len() - self.writable
    }

    /// Get the total capacity of the buffer.
    fn capacity(&self) -> usize {
        self.buf.len()
    }
}

#[cfg(all(feature = "std", test))]
mod tests {
    use super::*;
    use crate::{connection::with_test_connection, utils::setup_tracing};
    use core::mem::ManuallyDrop;

    #[test]
    fn test_write_vectored() {
        setup_tracing();

        for buf_size in [16384, 5] {
            with_test_connection(
                &[],
                vec![],
                |conn| {
                    let mut bc = BufConnection::with_capacity(buf_size, buf_size, conn);

                    let iov = [IoSlice::new(b"Hello,"), IoSlice::new(b" world!")];
                    let mut fds = (15..20).map(Fd::from).collect::<Vec<_>>();

                    let amt = bc.send_slices_and_fds(&iov, &mut fds).unwrap();

                    assert_eq!(amt, 13);

                    // flush it
                    bc.flush().unwrap();
                },
                |write_bytes, write_fds| {
                    assert_eq!(&write_bytes, b"Hello, world!".as_ref());
                    assert_eq!(write_fds, vec![15, 16, 17, 18, 19]);
                },
            );
        }
    }

    #[test]
    fn test_read_vectored() {
        setup_tracing();

        for buf_size in [16384] {
            with_test_connection(
                b"Hello, world!",
                vec![15, 16, 17, 18, 19],
                |conn| {
                    let mut bc = BufConnection::with_capacity(buf_size, buf_size, conn);

                    // read using vectored I/O
                    let mut buffer = [0; 13];
                    let (buf1, buf2) = buffer.split_at_mut(3);
                    let (buf2, buf3) = buf2.split_at_mut(3);
                    let buf3 = &mut buf3[..3];

                    let mut iov = [
                        IoSliceMut::new(buf1),
                        IoSliceMut::new(buf2),
                        IoSliceMut::new(buf3),
                    ];

                    let mut fds = ManuallyDrop::new(vec![]);

                    let amt = bc.recv_slices_and_fds(&mut iov, &mut fds).unwrap();
                    let fds = ManuallyDrop::into_inner(fds)
                        .into_iter()
                        .map(|f| f.as_raw_fd())
                        .collect::<Vec<_>>();
                    assert_eq!(amt, 9);

                    assert_eq!(&buffer[..9], b"Hello, wo".as_ref());
                    assert_eq!(fds, vec![15, 16, 17, 18, 19]);

                    // try to follow up with another read
                    let (buf1, buf2) = buffer.split_at_mut(2);
                    let buf2 = &mut buf2[..2];
                    let mut iov = [IoSliceMut::new(buf1), IoSliceMut::new(buf2)];
                    let mut fds = vec![];

                    assert_eq!(bc.recv_slices_and_fds(&mut iov, &mut fds).unwrap(), 4);
                    assert_eq!(&buffer[..4], b"rld!".as_ref());
                },
                |_, _| {},
            );
        }
    }

    #[test]
    fn test_write_vectored_without_fds() {
        setup_tracing();

        for buf_size in [16384, 5] {
            with_test_connection(
                &[],
                vec![],
                |conn| {
                    let mut bc = BufConnection::with_capacity(buf_size, buf_size, conn);

                    let iov = [IoSlice::new(b"Hello,"), IoSlice::new(b" world!")];

                    let amt = bc.send_slices(&iov).unwrap();
                    assert_eq!(amt, 13);

                    bc.flush().unwrap();
                },
                |write_bytes, write_fds| {
                    assert_eq!(&write_bytes, b"Hello, world!".as_ref());
                    assert_eq!(write_fds, vec![]);
                },
            );
        }
    }

    #[test]
    fn test_write_buffer() {
        setup_tracing();

        for buf_size in [16384, 5] {
            with_test_connection(
                &[],
                vec![],
                |conn| {
                    let mut bc = BufConnection::with_capacity(buf_size, buf_size, conn);

                    assert_eq!(bc.send_slice(b"Hello, world!").unwrap(), 13);

                    bc.flush().unwrap();
                },
                |write_bytes, write_fds| {
                    assert_eq!(&write_bytes, b"Hello, world!".as_ref());
                    assert_eq!(write_fds, vec![]);
                },
            );
        }
    }

    #[test]
    fn test_read_buffer() {
        setup_tracing();

        for buf_size in [16834, 6] {
            with_test_connection(
                b"Hello, world!",
                vec![],
                |conn| {
                    let mut bc = BufConnection::with_capacity(buf_size, buf_size, conn);

                    // read the first five bytes
                    let mut buf = [0; 5];
                    assert_eq!(bc.recv_slice(&mut buf).unwrap(), 5);
                    assert_eq!(buf, b"Hello".as_ref());

                    // read the rest of the bytes
                    let mut buf = [0; 8];
                    let mut nread = 0;
                    while nread < 8 {
                        nread += bc.recv_slice(&mut buf[nread..]).unwrap();
                    }
                    assert_eq!(buf, b", world!".as_ref());

                    // furhter reads should return nothing
                    assert_eq!(bc.recv_slice(&mut buf).unwrap(), 0);
                },
                |_, _| {},
            );
        }
    }
}
