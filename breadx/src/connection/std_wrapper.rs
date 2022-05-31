// MIT/Apache2 License

#![cfg(feature = "std")]

use super::{ReadHalf, WriteHalf, ClonableConnection};
use crate::{Error, Fd, Result, ResultExt, Unsupported};

use alloc::vec::Vec;

use core::any::type_name;
use core::borrow::{Borrow, BorrowMut};
use core::fmt;
use core::ops::{Deref, DerefMut};

use std::io::{IoSlice, IoSliceMut, Read, Write};

cfg_std_unix! {
    use nix::sys::socket;
    use std::os::unix::io::{AsRawFd, RawFd};
}

cfg_std_windows! {
    use std::{io, net, os::windows::io::{AsRawSocket, RawSocket}};
}

/// A newtype wrapper around a type that implements [`Connection`] for
/// certain types in the standard library.
///
/// Types within the Rust ecosystem can function as a reliable byte steam.
/// This newtype allows these tpyes to be used in places that expect a
/// [`Connection`]. [`Connection`] is implemented for types that implement
/// the following traits:
///
/// - [`Read`]
/// - [`Write`]
/// - [`AsRawFd`] for Unix platforms
/// - [`AsRawSocket`] for Windows platforms
///
/// In addition, if [`Read`] and [`Write`] are implemented for `&T`, then
/// [`Connection`] is implemented for `&StdConnection<T>`, allowing it to
/// be used in shared contexts.
///
/// This type does not preform FD passing. If you need to pass
/// file descriptors, either use [`NameConnection`], [`SendmsgConnection`],
/// or build your own type.
///
/// ## Example
///
/// ```rust,no_run
/// use breadx::connection::{Connection, StdConnection};
/// use std::net::TcpStream;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let socket = TcpStream::connect("localhost:6000")?;
/// let mut connection = StdConnection::new(socket);
/// let mut buf = [0; 1024];
/// connection.recv_slice(&mut buf)?;
/// # Ok(()) }
/// ```
///
/// [`Connection`]: crate::connection::Connection
/// [`Read`]: std::io::Read
/// [`Write`]: std::io::Write
/// [`AsRawFd`]: std::os::unix::io::AsRawFd
/// [`AsRawSocket`]: std::os::windows::io::AsRawSocket
/// [`NameConnection`]: crate::name::NameConnection
/// [`SendmsgConnection`]: crate::connection::SendmsgConnection
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct StdConnection<C: ?Sized> {
    inner: C,
}

impl<C: fmt::Debug> fmt::Debug for StdConnection<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, f)
    }
}

impl<C> StdConnection<C> {
    /// Create a new `StdConnection` wrapping around an existing
    /// connection.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use breadx::connection::{Connection, StdConnection};
    /// use std::net::TcpStream;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let socket = TcpStream::connect("localhost:6000")?;
    /// let connection = StdConnection::new(socket);
    /// # let _ = connection;
    /// # Ok(()) }
    /// ```
    pub fn new(inner: C) -> Self {
        Self { inner }
    }

    /// Unwrap this newtype to get the underlying connection.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use breadx::connection::{Connection, StdConnection};
    /// use std::net::TcpStream;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let socket = TcpStream::connect("localhost:6000")?;
    /// let connection = StdConnection::new(socket);
    ///
    /// // we need the connection back
    /// let socket = connection.into_inner();
    /// # let _ = socket;
    /// # Ok(()) }
    /// ```
    pub fn into_inner(self) -> C {
        self.inner
    }
}

impl<C: ?Sized> StdConnection<C> {
    /// Get a reference to the underlying connection.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use breadx::connection::{Connection, StdConnection};
    /// use std::net::TcpStream;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let socket = TcpStream::connect("localhost:6000")?;
    /// let connection = StdConnection::new(socket);
    ///
    /// let peer_addr = connection.get_ref().peer_addr()?;
    /// println!("peer address: {}", peer_addr);
    /// # Ok(()) }
    /// ```
    pub fn get_ref(&self) -> &C {
        &self.inner
    }

    /// Get a mutable reference to the underlying connection.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use breadx::connection::{Connection, StdConnection};
    /// use std::net::TcpStream;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let socket = TcpStream::connect("localhost:6000")?;
    /// let mut connection = StdConnection::new(socket);
    ///
    /// let peer_addr = connection.get_mut().peer_addr()?;
    /// println!("peer address: {}", peer_addr);
    /// # Ok(()) }
    /// ```
    pub fn get_mut(&mut self) -> &mut C {
        &mut self.inner
    }
}

// implement traits to ensure we are a wrapper around a connection
impl<C> From<C> for StdConnection<C> {
    fn from(inner: C) -> Self {
        Self { inner }
    }
}

impl<C: ?Sized> AsRef<C> for StdConnection<C> {
    fn as_ref(&self) -> &C {
        &self.inner
    }
}

impl<C: ?Sized> AsMut<C> for StdConnection<C> {
    fn as_mut(&mut self) -> &mut C {
        &mut self.inner
    }
}

impl<C> Borrow<C> for StdConnection<C> {
    fn borrow(&self) -> &C {
        &self.inner
    }
}

impl<C> BorrowMut<C> for StdConnection<C> {
    fn borrow_mut(&mut self) -> &mut C {
        &mut self.inner
    }
}

impl<C> Deref for StdConnection<C> {
    type Target = C;

    fn deref(&self) -> &C {
        &self.inner
    }
}

impl<C> DerefMut for StdConnection<C> {
    fn deref_mut(&mut self) -> &mut C {
        &mut self.inner
    }
}

cfg_std_unix! {
    impl<C: AsRawFd + ?Sized> AsRawFd for StdConnection<C> {
        fn as_raw_fd(&self) -> RawFd {
            self.inner.as_raw_fd()
        }
    }
}

cfg_std_windows! {
    impl<C: AsRawSocket + ?Sized> AsRawSocket for StdConnection<C> {
        fn as_raw_socket(&self) -> RawSocket {
            self.inner.as_raw_socket()
        }
    }
}

// macro to implement items that aren't tied to OS functionality
// the inner is either & or &mut, so we can implement this for
// either StdConnection or &StdConnection
macro_rules! impl_read_half {
    ($($inner: tt)*) => {
        fn recv_slices_and_fds(
            &mut self,
            slices: &mut [IoSliceMut<'_>],
            _fds: &mut Vec<Fd>,
        ) -> Result<usize> {
            let span = tracing::trace_span!(
                "{tyname}::recv_slices_and_fds",
                tyname = type_name::<Self>()
            );
            let _enter = span.enter();

            // forward to read_vectored()
            ($($inner)* self.inner)
                .read_vectored(slices)
                .map_err(Error::io)
                .trace(|amt| {
                    tracing::trace!("Received {} bytes", amt);
                })
        }

        fn recv_slice_and_fds(&mut self, slice: &mut [u8], _fds: &mut Vec<Fd>) -> Result<usize> {
            let span = tracing::trace_span!(
                "{tyname}::recv_slice_and_fds",
                tyname = type_name::<Self>()
            );
            let _enter = span.enter();

            // forward to read()
            ($($inner)* self.inner)
                .read(slice)
                .map_err(Error::io)
                .trace(|amt| {
                    tracing::trace!("Received {} bytes", amt);
                })
        }

        fn recv_slice(&mut self, slice: &mut [u8]) -> Result<usize> {
            let span = tracing::trace_span!(
                "{tyname}::recv_slice",
                tyname = type_name::<Self>()
            );
            let _enter = span.enter();

            // forward to read()
            ($($inner)* self.inner)
                .read(slice)
                .map_err(Error::io)
                .trace(|amt| {
                    tracing::trace!("Received {} bytes", amt);
                })
        }
    }
}

macro_rules! impl_write_half {
    ($($inner: tt)*) => {
        fn send_slices_and_fds(
            &mut self,
            slices: &[IoSlice<'_>],
            fds: &mut Vec<Fd>,
        ) -> Result<usize> {
            let span = tracing::trace_span!(
                "{tyname}::send_slices_and_fds",
                tyname = type_name::<Self>()
            );
            let _enter = span.enter();

            // error out if we have fds, and forward to
            // write_vectored()
            if !fds.is_empty() {
                tracing::error!("Attempted to send fds with non-unix connection");
                return Err(Error::make_unsupported(Unsupported::Fds));
            }

            ($($inner)* self.inner)
                .write_vectored(slices)
                .map_err(Error::io)
                .trace(|amt| {
                    tracing::trace!("Sent {} bytes", amt);
                })
        }

        fn send_slices(&mut self, slices: &[IoSlice<'_>]) -> Result<usize> {
            let span = tracing::trace_span!(
                "{tyname}::send_slices",
                tyname = type_name::<Self>()
            );
            let _enter = span.enter();

            // forward to write_vectored()
            ($($inner)* self.inner)
                .write_vectored(slices)
                .map_err(Error::io)
                .trace(|amt| {
                    tracing::trace!("Sent {} bytes", amt);
                })
        }

        fn send_slice(&mut self, slice: &[u8]) -> Result<usize> {
            let span = tracing::trace_span!(
                "{tyname}::send_slice",
                tyname = type_name::<Self>()
            );
            let _enter = span.enter();

            // forward to write()
            ($($inner)* self.inner)
                .write(slice)
                .map_err(Error::io)
                .trace(|amt| {
                    tracing::trace!("Sent {} bytes", amt);
                })
        }

        fn flush(&mut self) -> Result<()> {
            let span = tracing::trace_span!(
                "{tyname}::flush",
                tyname = type_name::<Self>()
            );
            let _enter = span.enter();

            ($($inner)* self.inner).flush().map_err(Error::io)
        }
    };
}

cfg_std_unix! {
    // avoid duplication in implementation
    macro_rules! impl_items_unix {
        ($($inner: tt)*) => {
            fn non_blocking_recv_slices_and_fds(
                &mut self,
                slices: &mut [IoSliceMut<'_>],
                _fds: &mut Vec<Fd>,
            ) -> Result<usize> {
                let span = tracing::trace_span!(
                    "{tyname}::non_blocking_recv_slices_and_fds",
                    tyname = type_name::<Self>()
                );
                let _enter = span.enter();

                // use recvmsg() with the MSG_DONTWAIT flag
                let raw_fd = self.inner.as_raw_fd();
                let msg = socket::recvmsg::<()>(
                    raw_fd,
                    slices,
                    None,
                    socket::MsgFlags::MSG_DONTWAIT,
                ).map_err(Error::nix)?;

                tracing::trace!("Received {} bytes", msg.bytes);

                Ok(msg.bytes)
            }

            fn non_blocking_recv_slice_and_fds(
                &mut self,
                slice: &mut [u8],
                _fds: &mut Vec<Fd>,
            ) -> Result<usize> {
                let span = tracing::trace_span!(
                    "{tyname}::non_blocking_recv_slice_and_fds",
                    tyname = type_name::<Self>()
                );
                let _enter = span.enter();

                // use recv() with MSG_DONTWAIT
                let raw_fd = self.inner.as_raw_fd();
                socket::recv(
                    raw_fd,
                    slice,
                    socket::MsgFlags::MSG_DONTWAIT
                ).map_err(Error::nix)
                 .trace(|amt| {
                    tracing::trace!("Received {} bytes", amt);
                })
            }
        }
    }

    impl<C: Read + AsRawFd + ?Sized> ReadHalf for StdConnection<C> {
        impl_read_half! { &mut }
        impl_items_unix! { &mut }
    }

    impl<C: Write + AsRawFd + ?Sized> WriteHalf for StdConnection<C> {
        impl_write_half! { &mut }
    }

    impl<'a, C: AsRawFd + ?Sized> ReadHalf for &'a StdConnection<C>
        where &'a C: Read {
        impl_read_half! { & }
        impl_items_unix! { & }
    }

    impl<'a, C: AsRawFd + ?Sized> WriteHalf for &'a StdConnection<C>
        where &'a C: Write {
        impl_write_half! { & }
    }
}

cfg_std_windows! {
    impl<C: Write + AsRawSocket> WriteHalf for StdConnection<C> {
        impl_write_half! { &mut }
    }

    impl<C: Read + AsRawSocket> ReadHalf for StdConnection<C> {
        impl_read_half! { &mut }

        fn non_blocking_recv_slices_and_fds(
            &mut self,
            slices: &mut [IoSliceMut<'_>],
            fds: &mut Vec<Fd>,
        ) -> Result<usize> {
            // if the read queue is empty, a read call will block
            if fionread::fionread(&*self).map_err(Error::io)? == 0 {
                Err(Error::io(io::ErrorKind::WouldBlock.into()))
            } else {
                self.recv_slices_and_fds(slices, fds)
            }
        }
    }

    impl<'a, C: AsRawSocket> ReadHalf for &'a StdConnection<C>
        where &'a C: Read
    {
        impl_read_half! { & }

        fn non_blocking_recv_slices_and_fds(
            &mut self,
            slices: &mut [IoSliceMut<'_>],
            fds: &mut Vec<Fd>,
        ) -> Result<usize> {
            // if the read queue is empty, a read call will block
            if fionread::fionread(*self).map_err(Error::io)? == 0 {
                Err(Error::io(io::ErrorKind::WouldBlock.into()))
            } else {
                self.recv_slices_and_fds(slices, fds)
            }
        }
    }

    impl<'a, C: AsRawSocket> WriteHalf for &'a StdConnection<C>
        where &'a C: Write {
        impl_write_half! { &}
    }
}

// TODO: this system could cover more cases than just the ones we have
// could implement SplitConnection for all Conn: AsRawFd that split it
// off into an Fd-based reader or writer
// but that's complicated and potentially unsound, this is safer but
// less flexible
macro_rules! impl_clonable_conn {
    ($($(#[$meta: meta])? $tys: ty),*) => {
        $(
            $(#[$meta])?
            impl ClonableConnection for StdConnection<$tys> {
                fn try_clone(&self) -> Result<Self> {
                    Ok(StdConnection {
                        inner: self.inner.try_clone().map_err(Error::io)?
                    })
                }
            }
        )*
    }
}

impl_clonable_conn! {
    std::net::TcpStream,
    #[cfg(unix)]
    std::os::unix::net::UnixStream
}

// TODO: implement Connection for WASI