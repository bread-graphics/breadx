// MIT/Apache2 License

use super::unix;
use crate::Fd;
use alloc::{boxed::Box, vec::Vec};
use core::{future::Future, pin::Pin};

#[cfg(all(feature = "std", unix))]
use async_net::unix::UnixStream;
#[cfg(feature = "std")]
use async_net::TcpStream;

#[cfg(not(unix))]
use super::standard_fd_warning;

/// Generic future for connections;
pub type GenericConnFuture<'future> = Pin<Box<dyn Future<Output = crate::Result> + Send + 'future>>;

/// Asynchronous breadx connection.
pub trait AsyncConnection {
    /// Send a packet across the connection in an async manner.
    fn send_packet<'future, 'a, 'b, 'c>(
        &'a mut self,
        bytes: &'b [u8],
        fds: &'c mut Vec<Fd>,
    ) -> GenericConnFuture<'future>
    where
        'a: 'future,
        'b: 'future,
        'c: 'future;

    /// Read a packet from the connection in an async manner.
    fn read_packet<'future, 'a, 'b, 'c>(
        &'a mut self,
        bytes: &'b mut [u8],
        fds: &'c mut Vec<Fd>,
    ) -> GenericConnFuture<'future>
    where
        'a: 'future,
        'b: 'future,
        'c: 'future;
}

macro_rules! unix_aware_async_connection_impl {
    ($name: ident) => {
        impl AsyncConnection for $name {
            #[inline]
            fn send_packet<'future, 'a, 'b, 'c>(
                &'a mut self,
                bytes: &'b [u8],
                fds: &'c mut Vec<Fd>,
            ) -> GenericConnFuture<'future>
            where
                'a: 'future,
                'b: 'future,
                'c: 'future,
            {
                cfg_if::cfg_if! {
                    if #[cfg(unix)] {
                        Box::pin(
                            unix::send_packet_unix_async(self.clone().into(), bytes, fds)
                        )
                    } else {
                        standard_fd_warning(fds);
                        Box::pin(async move {
                            self.write_all(bytes)?;
                            Ok(())
                        })
                    }
                }
            }

            #[inline]
            fn read_packet<'future, 'a, 'b, 'c>(
                &'a mut self,
                bytes: &'b mut [u8],
                fds: &'c mut Vec<Fd>,
            ) -> GenericConnFuture<'future>
            where
                'a: 'future,
                'b: 'future,
                'c: 'future,
            {
                cfg_if::cfg_if! {
                    if #[cfg(unix)] {
                        Box::pin(
                           unix::read_packet_unix_async(self.clone().into(), bytes, fds)
                        )
                    } else {
                        let _ = fds;
                        Box::pin(async move {
                            self.read_exact(bytes)?;
                            Ok(())
                        })
                    }
                }
            }
        }
    };
}

#[cfg(feature = "std")]
unix_aware_async_connection_impl! { TcpStream }

#[cfg(all(feature = "std", unix))]
unix_aware_async_connection_impl! { UnixStream }
