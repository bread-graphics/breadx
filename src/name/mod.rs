// MIT/Apache2 License

#![cfg(feature = "std")]

//! Defines the `NameConnection` type.

use crate::connection::{Connection, IoSlice};
use crate::{Error, Fd, Result, Unsupported};

use core::fmt;

use std::net::TcpStream;

use alloc::vec::Vec;
use x11rb_protocol::parse_display::{parse_display, ConnectAddress, ParsedDisplay};

cfg_std_unix! {
    use crate::connection::SendmsgConnection;
}

cfg_async! {
    mod nb_connect;
    use nb_connect::nb_connect;
}

/// A connection that can be derived from a parsed display name.
pub struct NameConnection {
    inner: Inner,
}

#[derive(Debug)]
enum Inner {
    Tcp(TcpStream),
    #[cfg(unix)]
    Unix(SendmsgConnection),
}

impl fmt::Debug for NameConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
        let parsed_display = parse_display(name).ok_or_else(|| Error::couldnt_parse_display())?;

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
        Err(error.unwrap_or_else(|| Error::couldnt_parse_display()))
    }

    pub(crate) fn from_tcp_stream(stream: TcpStream) -> Self {
        NameConnection {
            inner: Inner::Tcp(stream),
        }
    }
}

cfg_std_unix! {
    impl NameConnection {
        pub(crate) fn from_unix_stream(stream: UnixStream) -> Self {
            NameConnection {
                inner: Inner::Unix(SendmsgConnection::from(stream)),
            }
        }
    }
}

cfg_async! {
    impl NameConnection {
        /// Create a `NameConnection`, established in a non-blocking manner.
        pub async fn new_async(name: Option<&str>) -> Result<Self> {
            let parsed_display = parse_display(name).ok_or_else(|| Error::couldnt_parse_display())?;

            // iterate over the potential connection types
            let mut error: Option<Error> = None;

            for connect_address in parsed_display.connect_instruction() {
                match nb_connect(connect_address) {
                    Ok(s) => return Ok(s),
                    Err(e) => { error = Some(e); }
                }
            }

            // failed to connect over all addresses; return the last error
            Err(error.unwrap_or_else(|| Error::couldnt_parse_display()))
        }
    }
}

impl Connection for NameConnection {
    fn write_slices_with_fds(&mut self, iov: &[IoSlice<'_>], fds: &mut Vec<Fd>) -> Result<usize> {
        match &mut self.inner {
            Inner::Tcp(tcp) => tcp.write_slices_with_fds(iov, fds),
            #[cfg(unix)]
            Inner::Unix(unix) => unix.write_slices_with_fds(iov, fds),
        }
    }

    fn write_slices(&mut self, iov: &[IoSlice<'_>]) -> Result<usize> {
        match &mut self.inner {
            Inner::Tcp(tcp) => tcp.write_slices(iov),
            #[cfg(unix)]
            Inner::Unix(unix) => unix.write_slices(iov),
        }
    }

    fn write_slice(&mut self, buf: &[u8]) -> Result<usize> {
        match &mut self.inner {
            Inner::Tcp(tcp) => tcp.write_slice(buf),
            #[cfg(unix)]
            Inner::Unix(unix) => unix.write_slice(buf),
        }
    }

    fn is_write_vectored(&self) -> bool {
        match &self.inner {
            Inner::Tcp(tcp) => tcp.is_write_vectored(),
            #[cfg(unix)]
            Inner::Unix(unix) => unix.is_write_vectored(),
        }
    }

    fn read_to_buffer_with_fds(&mut self, buf: &mut [u8], fds: &mut Vec<Fd>) -> Result<usize> {
        match &mut self.inner {
            Inner::Tcp(tcp) => tcp.read_to_buffer_with_fds(buf, fds),
            #[cfg(unix)]
            Inner::Unix(unix) => unix.read_to_buffer_with_fds(buf, fds),
        }
    }

    fn flush(&mut self) -> Result<()> {
        match &mut self.inner {
            Inner::Tcp(tcp) => tcp.flush(),
            #[cfg(unix)]
            Inner::Unix(unix) => unix.flush(),
        }
    }
}
