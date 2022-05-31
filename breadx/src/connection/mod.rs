// MIT/Apache2 License

//! Implementation of the [`Connection`] type, which is used as a byte
//! transport for the actual X11 protocol.
//!
//! X11 communication can take place over "any reliable byte stream"
//! ([source]). Although this byte stream is most often a TCP connection
//! or a Unix domain socket, it can be anything else. The [`Connection`]
//! trait aims to define the interface for this byte stream.
//!
//! Note that in the overwhelming majority of cases, [`NameConnection`] will
//! fulfill all of your connection-related needs. All other connection types
//! exist to give the user as much freedom as possible in using the protocol.
//!
//! ## Details
//!
//! In X11, the requirements for a byte stream include:
//!
//! - Being able to write bytes.
//! - Being able to read bytes.
//! - Being able to read bytes without blocking.
//!
//! In addition, certain extensions requires the ability to pass file
//! descriptors between the client and the server. This is not a requirement.
//!
//! - [`StdConnection`] (enabled with the `std` feature) is a wrapper around
//!   any type that implements [`Read`] and [`Write`]. In addition, it also
//!   requires [`AsRawFd`] on Unix and [`AsRawSocket`] on Windows, in order to
//!   take advantage of system APIs for non-blocking I/O.
//! - [`SendmsgConnection`] (requires Unix) is a wrapper around a Unix domain
//!   socket that includes message passing functionality.
//! - [`BufConnection`] is a wrapper around anything that implements [`Connection`]
//!   that buffers all data written to and read from it.
//!
//! [`Connection`]: crate::connection::Connection
//! [source]: https://www.x.org/releases/X11R7.5/doc/x11proto/proto.pdf
//! [`NameConnection`]: crate::name::NameConnection
//! [`StdConnection`]: crate::connection::StdConnection
//! [`Read`]: std::io::Read
//! [`Write`]: std::io::Write
//! [`AsRawFd`]: std::os::unix::io::AsRawFd
//! [`AsRawSocket`]: std::os::windows::io::AsRawSocket
//! [`SendmsgConnection`]: crate::connection::SendmsgConnection
//! [`BufConnection`]: crate::connection::BufConnection

use crate::{Error, Fd, InvalidState, Result};
use alloc::{boxed::Box, vec::Vec, rc::Rc, sync::Arc};
use __private::Sealed;

mod buffered;
pub use buffered::BufConnection;

mod split;
pub use split::{ClonableConnection, SplitConnection};

cfg_std_unix! {
    mod sendmsg;
    pub use sendmsg::SendmsgConnection;
}

cfg_test! {
    mod test;
    #[cfg(all(feature = "std", unix))]
    pub(crate) use test::with_test_connection;
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

/// The "read half" of a [`Connection`].
/// 
/// This type contains the reading methods necessary for a [`Connection`]. When
/// used on its own, it is intended to be used in the case where a [`SplitConnection`]
/// is split into two halves.
/// 
/// [`Connection`]: crate::connection::Connection
/// [`SplitConnection`]: crate::connection::SplitConnection
pub trait ReadHalf {
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

    /// Receive data from the X11 server into a set of I/O slices, in a
    /// non-blocking manner.
    ///
    /// # Blocking
    ///
    /// Even if the connection is in blocking mode, this function should
    /// never block.
    ///
    /// # Errors
    ///
    /// This will return a `WouldBlock` I/O error if the function would
    /// block. Otherwise, it should bubble up any platform I/O errors.
    fn non_blocking_recv_slices_and_fds(
        &mut self,
        slices: &mut [IoSliceMut<'_>],
        fds: &mut Vec<Fd>,
    ) -> Result<usize>;

    /// Receive data from the X11 server into a single slice, in a
    /// non-blocking manner.
    ///
    /// # Errors
    ///
    /// Same as `non_blocking_recv_slices_and_fds`.
    fn non_blocking_recv_slice_and_fds(
        &mut self,
        slice: &mut [u8],
        fds: &mut Vec<Fd>,
    ) -> Result<usize> {
        self.non_blocking_recv_slices_and_fds(&mut [new_io_slice_mut(slice)], fds)
    }
}

/// The "write half" of a [`Connection`].
/// 
/// This type contains the writing methods necessary for a [`Connection`]. When
/// used on its own, it is intended to be used in the case where a [`SplitConnection`]
/// is split into two halves.
/// 
/// [`Connection`]: crate::connection::Connection
/// [`SplitConnection`]: crate::connection::SplitConnection
pub trait WriteHalf {
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
    /// Same as `send_slices`.
    fn send_slice(&mut self, slice: &[u8]) -> Result<usize> {
        self.send_slices(&[new_io_slice(slice)])
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
}

/// A "suitable byte stream" where communication with the X11 server can occur.
///
/// See the [module level documentation](index.html) for more details.
pub trait Connection: ReadHalf + WriteHalf + Sealed {}

impl<T: ReadHalf + WriteHalf + Sealed + ?Sized> Connection for T {}

// implement Connection for all &mut impl Connection
impl<C: ReadHalf + ?Sized> ReadHalf for &mut C {
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
}

impl<C: WriteHalf + ?Sized> WriteHalf for &mut C {
    fn send_slices_and_fds(&mut self, slices: &[IoSlice<'_>], fds: &mut Vec<Fd>) -> Result<usize> {
        (**self).send_slices_and_fds(slices, fds)
    }

    fn send_slices(&mut self, slices: &[IoSlice<'_>]) -> Result<usize> {
        (**self).send_slices(slices)
    }

    fn send_slice(&mut self, slice: &[u8]) -> Result<usize> {
        (**self).send_slice(slice)
    }

    fn flush(&mut self) -> Result<()> {
        (**self).flush()
    }
}

// implement for Box<impl Connection>

impl<C: ReadHalf + ?Sized> ReadHalf for Box<C> {
    fn non_blocking_recv_slice_and_fds(&mut self, slice: &mut [u8], fds: &mut Vec<Fd>) -> Result<usize> {
        (**self).non_blocking_recv_slice_and_fds(slice, fds)
    }

    fn non_blocking_recv_slices_and_fds(&mut self, slices: &mut [IoSliceMut<'_>], fds: &mut Vec<Fd>) -> Result<usize> {
        (**self).non_blocking_recv_slices_and_fds(slices, fds)
    }

    fn recv_slice(&mut self, slice: &mut [u8]) -> Result<usize> {
        (**self).recv_slice(slice)
    }

    fn recv_slice_and_fds(&mut self, slice: &mut [u8], fds: &mut Vec<Fd>) -> Result<usize> {
        (**self).recv_slice_and_fds(slice, fds)
    }

    fn recv_slices_and_fds(&mut self, slices: &mut [IoSliceMut<'_>], fds: &mut Vec<Fd>) -> Result<usize> {
        (**self).recv_slices_and_fds(slices, fds)
    }
}

impl<C: WriteHalf + ?Sized> WriteHalf for Box<C> {
    fn flush(&mut self) -> Result<()> {
        (**self).flush()
    }

    fn send_slice(&mut self, slice: &[u8]) -> Result<usize> {
        (**self).send_slice(slice)
    }

    fn send_slices(&mut self, slices: &[IoSlice<'_>]) -> Result<usize> {
        (**self).send_slices(slices)
    }

    fn send_slices_and_fds(&mut self, slices: &[IoSlice<'_>], fds: &mut Vec<Fd>) -> Result<usize> {
        (**self).send_slices_and_fds(slices, fds)
    }
}

// impl for Rc<impl &Connection>
impl<C: ?Sized> ReadHalf for Rc<C>
    where for<'a> &'a C: ReadHalf {
    fn non_blocking_recv_slice_and_fds(&mut self, slice: &mut [u8], fds: &mut Vec<Fd>) -> Result<usize> {
        (&**self).non_blocking_recv_slice_and_fds(slice, fds)
    }

    fn non_blocking_recv_slices_and_fds(&mut self, slices: &mut [IoSliceMut<'_>], fds: &mut Vec<Fd>) -> Result<usize> {
        (&**self).non_blocking_recv_slices_and_fds(slices, fds)
    }

    fn recv_slice(&mut self, slice: &mut [u8]) -> Result<usize> {
        (&**self).recv_slice(slice)
    }

    fn recv_slice_and_fds(&mut self, slice: &mut [u8], fds: &mut Vec<Fd>) -> Result<usize> {
        (&**self).recv_slice_and_fds(slice, fds)
    }

    fn recv_slices_and_fds(&mut self, slices: &mut [IoSliceMut<'_>], fds: &mut Vec<Fd>) -> Result<usize> {
        (&**self).recv_slices_and_fds(slices, fds)
    }
}

impl<C: ?Sized> WriteHalf for Rc<C>
    where for<'a> &'a C: WriteHalf {
    fn flush(&mut self) -> Result<()> {
        (&**self).flush()
    }

    fn send_slice(&mut self, slice: &[u8]) -> Result<usize> {
        (&**self).send_slice(slice)
    }

    fn send_slices(&mut self, slices: &[IoSlice<'_>]) -> Result<usize> {
        (&**self).send_slices(slices)
    }

    fn send_slices_and_fds(&mut self, slices: &[IoSlice<'_>], fds: &mut Vec<Fd>) -> Result<usize> {
        (&**self).send_slices_and_fds(slices, fds)
    }
}

// impl for Arc<impl &Connection>

impl<C: ?Sized> ReadHalf for Arc<C>
    where for<'a> &'a C: ReadHalf {
    fn non_blocking_recv_slice_and_fds(&mut self, slice: &mut [u8], fds: &mut Vec<Fd>) -> Result<usize> {
        (&**self).non_blocking_recv_slice_and_fds(slice, fds)
    }

    fn non_blocking_recv_slices_and_fds(&mut self, slices: &mut [IoSliceMut<'_>], fds: &mut Vec<Fd>) -> Result<usize> {
        (&**self).non_blocking_recv_slices_and_fds(slices, fds)
    }

    fn recv_slice(&mut self, slice: &mut [u8]) -> Result<usize> {
        (&**self).recv_slice(slice)
    }

    fn recv_slice_and_fds(&mut self, slice: &mut [u8], fds: &mut Vec<Fd>) -> Result<usize> {
        (&**self).recv_slice_and_fds(slice, fds)
    }

    fn recv_slices_and_fds(&mut self, slices: &mut [IoSliceMut<'_>], fds: &mut Vec<Fd>) -> Result<usize> {
        (&**self).recv_slices_and_fds(slices, fds)
    }
}

impl<C: ?Sized> WriteHalf for Arc<C> where for<'a> &'a C: WriteHalf {
    fn flush(&mut self) -> Result<()> {
        (&**self).flush()
    }

    fn send_slice(&mut self, slice: &[u8]) -> Result<usize> {
        (&**self).send_slice(slice)
    }

    fn send_slices(&mut self, slices: &[IoSlice<'_>]) -> Result<usize> {
        (&**self).send_slices(slices)
    }

    fn send_slices_and_fds(&mut self, slices: &[IoSlice<'_>], fds: &mut Vec<Fd>) -> Result<usize> {
        (&**self).send_slices_and_fds(slices, fds)
    }
}

cfg_std! {
    pub(crate) fn new_io_slice(sl: &[u8]) -> IoSlice<'_> {
        IoSlice::new(sl)
    }

    pub(crate) fn new_io_slice_mut(sl: &mut [u8]) -> IoSliceMut<'_> {
        IoSliceMut::new(sl)
    }

    pub(crate) fn advance_io(sl: &mut IoSlice<'_>, bytes: usize) {
        advance::advance(sl, bytes);
    }
}

cfg_no_std! {
    pub(crate) fn new_io_slice(sl: &[u8]) -> IoSlice<'_> {
        sl
    }

    pub(crate) fn new_io_slice_mut(sl: &mut [u8]) -> IoSliceMut<'_> {
        sl
    }

    pub(crate) fn advance_io(sl: &mut IoSlice<'_>, bytes: usize) {
        *sl = &sl[bytes..];
    }
}

mod __private {
    use super::{ReadHalf, WriteHalf};

    pub trait Sealed {
        fn __sealed_marker(&self) {}
    }

    impl<T: ReadHalf + WriteHalf + ?Sized> Sealed for T {}
}