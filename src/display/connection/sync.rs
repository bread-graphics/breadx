// MIT/Apache2 License

use super::{standard_fd_warning, unix};
use crate::Fd;
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::net::TcpStream;
#[cfg(not(feature = "std"))]
use std::os::unix::net::UnixStream;

/// Synchronous breadx connection.
pub trait Connection {
    /// Send a packet across the connection in a blocking manner.
    fn send_packet(&mut self, bytes: &[u8], fds: &mut Vec<Fd>) -> crate::Result;
    /// Read a packet from the connection in a blocking manner.
    fn read_packet(&mut self, bytes: &mut [u8], fds: &mut Vec<Fd>) -> crate::Result;
}

// Implement Connection on TcpStream and UnixStream

macro_rules! unix_aware_connection_impl {
    ($name: ident) => {
        impl Connection for $name {
            #[inline]
            fn send_packet(&mut self, bytes: &[u8], fds: &mut Vec<Fd>) -> crate::Result {
                cfg_if! {
                    if #[cfg(unix)] {
                        // take the unix sendmsg way that lets us send file descriptors
                        unix::send_packet_unix(self, bytes, fds)
                    } else {
                        // use write_all as a generic way of sending bytes across the stream
                        standard_fd_warning(fds);
                        self.write_all(bytes)?;
                        Ok(())
                    }
                }
            }

            #[inline]
            fn read_packet(&mut self, bytes: &mut [u8], fds: &mut Vec<Fd>) -> crate::Result {
                cfg_if! {
                    if #[cfg(unix)] {
                        unix::read_packet_unix(self, bytes, fds)
                    } else {
                        // just ignore the file descriptors
                        let _ = fds;
                        self.read_exact(bytes)?;
                        Ok(())
                    }
                }
            }
        }
    };
}

#[cfg(feature = "std")]
unix_aware_connection_impl! { TcpStream }

#[cfg(all(feature = "std", unix))]
unix_aware_connection_impl! { UnixStream }
