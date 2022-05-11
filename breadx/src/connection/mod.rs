// MIT/Apache2 License

use crate::{Error, Fd, InvalidState, Result};
use alloc::vec::Vec;

mod buffered;
pub use buffered::BufConnection;

cfg_std_unix! {
    mod sendmsg;
    pub use sendmsg::SendmsgConnection;
}

cfg_test! {
    mod test;
    pub(crate) use test::{TestConnection, with_test_connection};
}

cfg_std! {
    pub(crate) type IoSlice<'a> = std::io::IoSlice<'a>;
    pub(crate) type IoSliceMut<'a> = std::io::IoSliceMut<'a>;

    mod std_wrapper;
    pub use std_wrapper::StdConnection;
}

cfg_no_std! {
    pub(crate) type IoSlice<'a> = &'a [u8];
    pub(crate) type IoSliceMut<'a> = &'a mut [u8];
}

/// A "suitable byte stream" where communication with the X11 server can occur.
pub trait Connection {
    /// Write a series of I/O slices and a series of file descriptors to
    /// the X11 server.
    ///
    /// This calls the platform's writing utility to write the slices and
    /// file descriptors, and returns the number of bytes written. If
    /// the call succeeded, the `fds` array should be empty after operation.
    ///
    /// If the `fds` array is empty, this function call is allowed to
    /// degenerate to a standard vectored write.
    ///
    /// # Blocking
    ///
    /// This operation may block under normal circumstances. However, if this
    /// type implements `AsRawFd` or `AsRawSocket`, and if `set_nonblocking`
    /// or an equivalent method has been called on this object earlier, then
    /// this operation should not block, and return a `WouldBlock` error if
    /// it would.
    ///
    /// # Errors
    ///
    /// Some `Connection` implementations do not support FD passing. If an
    /// FD is passed into these implementations, an `unsupported()` [`Error`]
    /// will be raised.
    ///
    /// In addition, any platform I/O errors will be bubbled up to the user.
    fn send_slices_and_fds(&mut self, slices: &[IoSlice<'_>], fds: &mut Vec<Fd>) -> Result<usize>;

    /// Write a series of I/O slices to the X11 server.
    ///
    /// This calls the platform's writing utility to write the slices, and
    /// returns the number of bytes written. By default, this is implemented
    /// as a call to `send_slices_and_fds` without any file descriptors.
    /// Certain implementations can optimize away having to keep track
    /// of file descriptors.
    ///
    /// # Blocking
    ///
    /// Same as `send_slices_and_fds`.
    ///
    /// # Errors
    ///
    /// Same as `send_slices_and_fds`.
    fn send_slices(&mut self, slices: &[IoSlice<'_>]) -> Result<usize> {
        self.send_slices_and_fds(slices, &mut Vec::new())
    }

    /// Write a slice of data to the X11 server.
    ///
    /// This calls the platform's writing utility to write the slice, and
    /// returs the number of bytes written. By default, this is implemented
    /// as a call to `send_slices`.
    ///
    /// # Blocking
    ///
    /// Same as `send_slices_and_fds`.
    ///
    /// # Errors
    ///
    /// Same as `send_slices.
    fn send_slice(&mut self, slice: &[u8]) -> Result<usize> {
        self.send_slices(&[new_io_slice(slice)])
    }

    /// Read data to a series of I/O slices and a buffer for file
    /// descriptors.
    ///
    /// This calls the platform's reading utility to read into the buffers,
    /// and returns the total number of bytes read.
    ///
    /// # Blocking
    ///
    /// This operation may block under normal circumstances. However, if this
    /// type implements `AsRawFd` or `AsRawSocket`, and if `set_nonblocking`
    /// or an equivalent method has been called on this object earlier, then
    /// this operation should not block, and return a `WouldBlock` error if
    /// it would.
    ///
    /// # Errors
    ///
    /// Any platform I/O errors will be bubbled up to the user.
    fn recv_slices_and_fds(
        &mut self,
        slices: &mut [IoSliceMut<'_>],
        fds: &mut Vec<Fd>,
    ) -> Result<usize>;

    /// Read data to a single I/O slice and a buffer for file
    /// descriptors.
    ///
    /// This calls the platform's reading utility to read into the buffers,
    /// and returns the total number of bytes read. By default, this is
    /// implemented as a call to `recv_slices_and_fds` with a single
    /// slice.
    ///
    /// # Blocking
    ///
    /// Same as `recv_slices_and_fds`.
    ///
    /// # Errors
    ///
    /// Any platform I/O errors will be bubbled up to the user.
    fn recv_slice_and_fds(&mut self, slice: &mut [u8], fds: &mut Vec<Fd>) -> Result<usize> {
        self.recv_slices_and_fds(&mut [new_io_slice_mut(slice)], fds)
    }

    /// Read data for a single I/O slice.
    ///
    /// This calls the platform's reading utility to read into the buffer,
    /// and returns the total number of bytes read. By default, this is
    /// implemented as a call to `recv_slice_and_fds` with a single slice.
    ///
    /// # Blocking
    ///
    /// Same as `recv_slices_and_fds`.
    ///
    /// # Errors
    ///
    /// If `recv_slice_and_fds` returns any file descriptors, this
    /// function will return an `invalid_state()` error. It is encouraged
    /// for implementors to override this behavior so that this check is
    /// not necessary.
    ///
    /// In addition, any platform I/O errors will be bubbled up to the
    /// user.
    fn recv_slice(&mut self, slice: &mut [u8]) -> Result<usize> {
        let mut fds = Vec::new();
        let result = self.recv_slice_and_fds(slice, &mut fds);

        // check to ensure fds is empty
        if fds.is_empty() {
            result
        } else {
            Err(Error::make_invalid_state(InvalidState::UnexpectedFds))
        }
    }

    /// Flush all data in this connection's buffer.
    ///
    /// # Blocking
    ///
    /// Unless this connection has been set into non-blocking mode, this
    /// method is expected to block until all bytes in the buffer are
    /// written.
    ///
    /// # Errors
    ///
    /// Any platform I/O errors will be bubbled up to the user.
    fn flush(&mut self) -> Result<()>;

    /// Receive data from the X11 server into a set of I/O slices, in a
    /// non-blocking manner.
    ///
    /// # Blocking
    ///
    /// Even if the connection is in blocking mode, this function should
    /// never block.
    fn non_blocking_recv_slices_and_fds(
        &mut self,
        slices: &mut [IoSliceMut<'_>],
        fds: &mut Vec<Fd>,
    ) -> Result<usize>;

    /// Receive data from the X11 server into a single slice, in a
    /// non-blocking manner.
    fn non_blocking_recv_slice_and_fds(
        &mut self,
        slice: &mut [u8],
        fds: &mut Vec<Fd>,
    ) -> Result<usize> {
        self.non_blocking_recv_slices_and_fds(&mut [new_io_slice_mut(slice)], fds)
    }

    /// Shutdown this connection.
    ///
    /// This should have the same effect as dropping this object, but
    /// any OS errors should be able to be caught.
    fn shutdown(&self) -> Result<()>;
}

// implement Connection for all &mut impl Connection
impl<C: Connection + ?Sized> Connection for &mut C {
    fn send_slices_and_fds(&mut self, slices: &[IoSlice<'_>], fds: &mut Vec<Fd>) -> Result<usize> {
        (**self).send_slices_and_fds(slices, fds)
    }

    fn send_slices(&mut self, slices: &[IoSlice<'_>]) -> Result<usize> {
        (**self).send_slices(slices)
    }

    fn send_slice(&mut self, slice: &[u8]) -> Result<usize> {
        (**self).send_slice(slice)
    }

    fn recv_slices_and_fds(
        &mut self,
        slices: &mut [IoSliceMut<'_>],
        fds: &mut Vec<Fd>,
    ) -> Result<usize> {
        (**self).recv_slices_and_fds(slices, fds)
    }

    fn recv_slice_and_fds(&mut self, slice: &mut [u8], fds: &mut Vec<Fd>) -> Result<usize> {
        (**self).recv_slice_and_fds(slice, fds)
    }

    fn recv_slice(&mut self, slice: &mut [u8]) -> Result<usize> {
        (**self).recv_slice(slice)
    }

    fn flush(&mut self) -> Result<()> {
        (**self).flush()
    }

    fn non_blocking_recv_slices_and_fds(
        &mut self,
        slices: &mut [IoSliceMut<'_>],
        fds: &mut Vec<Fd>,
    ) -> Result<usize> {
        (**self).non_blocking_recv_slices_and_fds(slices, fds)
    }

    fn non_blocking_recv_slice_and_fds(
        &mut self,
        slice: &mut [u8],
        fds: &mut Vec<Fd>,
    ) -> Result<usize> {
        (**self).non_blocking_recv_slice_and_fds(slice, fds)
    }

    fn shutdown(&self) -> Result<()> {
        (**self).shutdown()
    }
}

// TODO: windows, wasi and any other platforms we want to support

cfg_std! {
    pub(crate) fn new_io_slice(sl: &[u8]) -> IoSlice<'_> {
        IoSlice::new(sl)
    }

    pub(crate) fn new_io_slice_mut(sl: &mut [u8]) -> IoSliceMut<'_> {
        IoSliceMut::new(sl)
    }
}

cfg_no_std! {
    pub(crate) fn new_io_slice(sl: &[u8]) -> IoSlice<'_> {
        sl
    }

    pub(crate) fn new_io_slice_mut(sl: &mut [u8]) -> IoSliceMut<'_> {
        sl
    }
}
