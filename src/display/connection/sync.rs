// MIT/Apache2 License

use super::establish_connection;
use crate::{auth_info::AuthInfo, display::StaticSetup, XidGenerator};

#[cfg(all(feature = "std", unix))]
use super::unix;
use crate::Fd;
use alloc::vec::Vec;

#[cfg(not(unix))]
use super::standard_fd_warning;
#[cfg(all(not(unix), feature = "std"))]
use std::io::{Read, Write};

#[cfg(feature = "std")]
use std::net::TcpStream;
#[cfg(all(feature = "std", unix))]
use std::os::unix::{io::AsRawFd, net::UnixStream};

/// Synchronous breadx connection.
pub trait Connection {
    /// Send a packet across the connection in a blocking manner.
    fn send_packet(&mut self, bytes: &[u8], fds: &mut Vec<Fd>) -> crate::Result;
    /// Read a packet from the connection in a blocking manner.
    fn read_packet(&mut self, bytes: &mut [u8], fds: &mut Vec<Fd>) -> crate::Result;
    /// Establish a setup using this connection.
    #[inline]
    fn establish(
        &mut self,
        auth_info: Option<AuthInfo>,
    ) -> crate::Result<(StaticSetup, XidGenerator)> {
        establish_connection(self, auth_info)
    }
}

impl<C: Connection + ?Sized> Connection for &mut C {
    #[inline]
    fn send_packet(&mut self, bytes: &[u8], fds: &mut Vec<Fd>) -> crate::Result {
        (**self).send_packet(bytes, fds)
    }

    #[inline]
    fn read_packet(&mut self, bytes: &mut [u8], fds: &mut Vec<Fd>) -> crate::Result {
        (**self).read_packet(bytes, fds)
    }
}

// Implement Connection on TcpStream and UnixStream

macro_rules! unix_aware_connection_impl {
    (#[$attr: meta] $name: ty) => {
        #[$attr]
        impl Connection for $name {
            #[inline]
            fn send_packet(&mut self, bytes: &[u8], fds: &mut Vec<Fd>) -> crate::Result {
                cfg_if::cfg_if! {
                    if #[cfg(unix)] {
                        // take the unix sendmsg way that lets us send file descriptors
                        unix::send_packet_unix(self.as_raw_fd(), bytes, fds)
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
                cfg_if::cfg_if! {
                    if #[cfg(unix)] {
                        unix::read_packet_unix(self.as_raw_fd(), bytes, fds)
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

unix_aware_connection_impl! { #[cfg(feature = "std")] TcpStream }
unix_aware_connection_impl! { #[cfg(all(feature = "std", unix))] UnixStream }
unix_aware_connection_impl! { #[cfg(feature = "std")] &TcpStream }
unix_aware_connection_impl! { #[cfg(all(feature = "std", unix))] &UnixStream }
