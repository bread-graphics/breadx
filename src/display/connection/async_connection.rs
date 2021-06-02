// MIT/Apache2 License

use super::async_establish_connection;
use crate::{auth_info::AuthInfo, auto::xproto::Setup, XidGenerator};

#[cfg(all(feature = "std", unix))]
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

#[cfg(all(not(unix), feature = "std"))]
use futures_lite::io::{AsyncReadExt, AsyncWriteExt};

/// Generic future for connections;
pub type GenericConnFuture<'future, T = ()> =
    Pin<Box<dyn Future<Output = crate::Result<T>> + Send + 'future>>;

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

    /// Establish a connection to the server.
    #[inline]
    fn establish<'future>(
        &'future mut self,
        auth_info: Option<AuthInfo>,
    ) -> GenericConnFuture<'future, (Setup, XidGenerator)> {
        Box::pin(async move { async_establish_connection(self, auth_info) })
    }
}

impl<C: AsyncConnection + ?Sized> AsyncConnection for &mut C {
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
        (**self).send_packet(bytes, fds)
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
        (**self).read_packet(bytes, fds)
    }
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
                            self.write_all(bytes).await?;
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
                            self.read_exact(bytes).await?;
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
#[cfg(feature = "std")]
unix_aware_async_connection_impl! { &TcpStream }
#[cfg(all(feature = "std", unix))]
unix_aware_async_connection_impl! { &UnixStream }
