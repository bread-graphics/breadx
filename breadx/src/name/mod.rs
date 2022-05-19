// MIT/Apache2 License

#![cfg(feature = "std")]

//! Defines the p`NameConnection`] type.
//!
//! Consult the documentation for [`NameConnection`] for more information.
//!
//! [`NameConnection`]: crate::name::NameConnection

use crate::connection::{Connection, StdConnection};
use crate::{Error, Fd, Result};

use core::fmt;

use std::io::{IoSlice, IoSliceMut};
use std::net::{Ipv4Addr, SocketAddr, TcpStream};

use alloc::{string::String, vec::Vec};
use x11rb_protocol::parse_display::{parse_display, ConnectAddress, ParsedDisplay};
use x11rb_protocol::xauth::Family;

cfg_std_unix! {
    use crate::connection::SendmsgConnection;
    use std::os::unix::{net::UnixStream, io::{AsRawFd, RawFd}};
}

cfg_std_windows! {
    use std::os::windows::io::{AsRawSocket, RawSocket};
}

cfg_async! {
    mod nb_connect;

    use core::future::Future;
}

/// A connection that can be derived from a parsed display name.
///
/// This is the primary [`Connection`] type that is used in `breadx`, as it
/// represents the most common byte streams that are used to connect to the
/// X11 server on modern OS. The display name, usually stored in the `DISPLAY`
/// environment variable, is parsed and the connection is created from the parsed
/// information.
///
/// This is a wrapper around either [`StdConnection<TcpStream>`] or, if on Unix,
/// [`SendmsgConnection`].
///
/// [`Connection`]: crate::connection::Connection
/// [`StdConnection<TcpStream>`]: crate::connection::StdConnection
/// [`SendmsgConnection`]: crate::connection::SendmsgConnection
pub struct NameConnection {
    inner: Inner,
}

#[derive(Debug)]
enum Inner {
    Tcp(StdConnection<TcpStream>),
    #[cfg(unix)]
    Unix(SendmsgConnection),
}

impl fmt::Debug for NameConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, f)
    }
}

impl NameConnection {
    /// Creates a new connection from a display name.
    ///
    /// # Blocking
    ///
    /// This function blocks while it tries to connect to the server. Use the
    /// [`new_async`] function if you would like a non-blocking variant.
    pub fn new(name: Option<&str>) -> Result<Self> {
        let parsed_display =
            parse_display(name).ok_or_else(|| Error::couldnt_parse_display(name.is_none()))?;

        Self::from_parsed_display(&parsed_display, name.is_none())
    }

    /// Creates a new connection from an already parsed display name.
    ///
    /// # Blocking
    ///
    /// This function blocks while it tries to connect to the server. Use the
    /// [`from_parsed_display_async`] function if you would like a non-blocking variant.
    pub fn from_parsed_display(parsed_display: &ParsedDisplay, is_env: bool) -> Result<Self> {
        // iterate over the potential connection types
        let mut error: Option<Error> = None;

        for connect_address in parsed_display.connect_instruction() {
            match connect_address {
                ConnectAddress::Hostname(hostname, port) => {
                    // begin a TCP connection to the hostname and port
                    match TcpStream::connect((hostname, port)) {
                        Ok(stream) => return Ok(Self::from_tcp_stream(stream)),
                        Err(err) => error = Some(Error::io(err)),
                    }
                }
                ConnectAddress::Socket(path) => {
                    cfg_if::cfg_if! {
                        if #[cfg(unix)] {
                            // begin a Unix domain socket connection to the path
                            match UnixStream::connect(path) {
                                Ok(stream) => return Ok(Self::from_unix_stream(stream)),
                                Err(err) => { error = Some(Error::io(err)) },
                            }
                        } else {
                            // we don't support unix domain sockets
                            let _ = path;
                            error = Some(Error::unsupported(Unsupported::Socket));
                        }
                    }
                }
            }
        }

        // failed to connect over all addresses; return the last error
        Err(error.unwrap_or_else(|| Error::couldnt_parse_display(is_env)))
    }

    pub(crate) fn from_tcp_stream(stream: TcpStream) -> Self {
        NameConnection {
            inner: Inner::Tcp(stream.into()),
        }
    }

    /// Get the peer address and family used in this connection.
    ///
    /// This is useful, since the `xauth` interface requires the family and
    /// address to be known.
    pub fn get_address(&self) -> Result<(Family, Vec<u8>)> {
        if let Inner::Tcp(ref connection) = self.inner {
            let ip = match connection.peer_addr().map_err(Error::io)? {
                SocketAddr::V4(ip) => *ip.ip(),
                SocketAddr::V6(ip) => {
                    // try to translate this to an IPv4 address
                    let ip = *ip.ip();
                    if ip.is_loopback() {
                        Ipv4Addr::LOCALHOST
                    } else if let Some(ip) = ip.to_ipv4() {
                        ip
                    } else {
                        // just use the IPv6 address
                        return Ok((Family::INTERNET6, ip.octets().to_vec()));
                    }
                }
            };

            // loopback case is handeled below
            if !ip.is_loopback() {
                return Ok((Family::INTERNET, ip.octets().to_vec()));
            }
        }

        // connection is connected to our own computer,
        // use the local hostname as the address
        let hostname = gethostname::gethostname()
            .into_string()
            .map_or_else(|_| Vec::new(), String::into_bytes);
        Ok((Family::LOCAL, hostname))
    }

    /// If there is an error associated with this socket, take it.
    #[must_use]
    pub fn take_error(&self) -> Option<Error> {
        let taken_error = match self.inner {
            Inner::Tcp(ref t) => t.take_error(),
            #[cfg(unix)]
            Inner::Unix(ref u) => u.as_ref().take_error(),
        };

        match taken_error {
            Err(err) | Ok(Some(err)) => Some(Error::io(err)),
            _ => None,
        }
    }
}

cfg_async! {
    impl NameConnection {
        /// Create a `NameConnection` in a non-blocking manner.
        ///
        /// This function does not block, and will select the first connection
        /// that is available. The `resolv` parameter should poll the connection
        /// until it is writable using the currently availably runtime.
        pub fn from_parsed_display_async<'a, Fut, R: 'a>(
            parsed_display: &'a ParsedDisplay,
            is_env: bool,
            resolv: impl FnMut(NameConnection) -> Fut + 'a,
        ) -> impl Future<Output = Result<R>> + 'a where Fut: Future<Output = Result<R>> + 'a {
            nb_connect::nb_connect(parsed_display, is_env, resolv)
        }
    }
}

cfg_std_unix! {
    impl NameConnection {
        pub(crate) fn from_unix_stream(stream: UnixStream) -> Self {
            NameConnection {
                inner: Inner::Unix(stream.into()),
            }
        }
    }

    impl AsRawFd for NameConnection {
        fn as_raw_fd(&self) -> RawFd {
            match self.inner {
                Inner::Tcp(ref connection) => connection.as_raw_fd(),
                Inner::Unix(ref connection) => connection.as_raw_fd(),
            }
        }
    }
}

cfg_std_windows! {
    impl AsRawSocket for NameConnection {
        fn as_raw_socket(&self) -> RawSocket {
            match self.inner {
                Inner::Tcp(ref connection) => connection.as_raw_socket(),
            }
        }
    }
}

macro_rules! forward_impl {
    (&$self: expr, $fn_name: ident, $($val: expr),*) => {
        match ($self).inner {
            Inner::Tcp(ref c) => {
                #[allow(unused_mut)]
                let mut c = c;
                c.$fn_name($($val),*)
            }
            #[cfg(unix)]
            Inner::Unix(ref c) => {
                #[allow(unused_mut)]
                let mut c = c;
                c.$fn_name($($val),*)
            }
        }
    };
    (&mut $self: expr, $fn_name: ident, $($val: expr),*) => {
        match ($self).inner {
            Inner::Tcp(ref mut c) => c.$fn_name($($val),*),
            #[cfg(unix)]
            Inner::Unix(ref mut c) => c.$fn_name($($val),*),
        }
    }
}

impl Connection for NameConnection {
    fn send_slices_and_fds(&mut self, slices: &[IoSlice<'_>], fds: &mut Vec<Fd>) -> Result<usize> {
        forward_impl!(&mut self, send_slices_and_fds, slices, fds)
    }

    fn recv_slices_and_fds(
        &mut self,
        slices: &mut [IoSliceMut<'_>],
        fds: &mut Vec<Fd>,
    ) -> Result<usize> {
        forward_impl!(&mut self, recv_slices_and_fds, slices, fds)
    }

    fn send_slices(&mut self, slices: &[crate::connection::IoSlice<'_>]) -> Result<usize> {
        forward_impl!(&mut self, send_slices, slices)
    }

    fn send_slice(&mut self, slice: &[u8]) -> Result<usize> {
        forward_impl!(&mut self, send_slice, slice)
    }

    fn recv_slice_and_fds(&mut self, slice: &mut [u8], fds: &mut Vec<Fd>) -> Result<usize> {
        forward_impl!(&mut self, recv_slice_and_fds, slice, fds)
    }

    fn recv_slice(&mut self, slice: &mut [u8]) -> Result<usize> {
        forward_impl!(&mut self, recv_slice, slice)
    }

    fn flush(&mut self) -> Result<()> {
        forward_impl!(&mut self, flush,)
    }

    fn shutdown(&self) -> Result<()> {
        forward_impl!(&self, shutdown,)
    }

    fn non_blocking_recv_slice_and_fds(
        &mut self,
        slice: &mut [u8],
        fds: &mut Vec<Fd>,
    ) -> Result<usize> {
        forward_impl!(&mut self, non_blocking_recv_slice_and_fds, slice, fds)
    }

    fn non_blocking_recv_slices_and_fds(
        &mut self,
        slices: &mut [IoSliceMut<'_>],
        fds: &mut Vec<Fd>,
    ) -> Result<usize> {
        forward_impl!(&mut self, non_blocking_recv_slices_and_fds, slices, fds)
    }
}

impl Connection for &NameConnection {
    fn send_slices_and_fds(&mut self, slices: &[IoSlice<'_>], fds: &mut Vec<Fd>) -> Result<usize> {
        forward_impl!(&self, send_slices_and_fds, slices, fds)
    }

    fn recv_slices_and_fds(
        &mut self,
        slices: &mut [IoSliceMut<'_>],
        fds: &mut Vec<Fd>,
    ) -> Result<usize> {
        forward_impl!(&self, recv_slices_and_fds, slices, fds)
    }

    fn send_slices(&mut self, slices: &[crate::connection::IoSlice<'_>]) -> Result<usize> {
        forward_impl!(&self, send_slices, slices)
    }

    fn send_slice(&mut self, slice: &[u8]) -> Result<usize> {
        forward_impl!(&self, send_slice, slice)
    }

    fn recv_slice_and_fds(&mut self, slice: &mut [u8], fds: &mut Vec<Fd>) -> Result<usize> {
        forward_impl!(&self, recv_slice_and_fds, slice, fds)
    }

    fn recv_slice(&mut self, slice: &mut [u8]) -> Result<usize> {
        forward_impl!(&self, recv_slice, slice)
    }

    fn flush(&mut self) -> Result<()> {
        forward_impl!(&self, flush,)
    }

    fn shutdown(&self) -> Result<()> {
        forward_impl!(&self, shutdown,)
    }

    fn non_blocking_recv_slice_and_fds(
        &mut self,
        slice: &mut [u8],
        fds: &mut Vec<Fd>,
    ) -> Result<usize> {
        forward_impl!(&self, non_blocking_recv_slice_and_fds, slice, fds)
    }

    fn non_blocking_recv_slices_and_fds(
        &mut self,
        slices: &mut [IoSliceMut<'_>],
        fds: &mut Vec<Fd>,
    ) -> Result<usize> {
        forward_impl!(&self, non_blocking_recv_slices_and_fds, slices, fds)
    }
}
