// MIT/Apache2 License

use super::async_establish_connection;
use crate::{
    auth_info::AuthInfo,
    auto::xproto::Setup,
    display::{ReadPacketFuture, SendPacketFuture},
    XidGenerator,
};
use core::task::{Context, Poll};

#[cfg(all(feature = "std", unix))]
use super::unix;
use crate::Fd;
use alloc::{boxed::Box, vec::Vec};
use core::{future::Future, pin::Pin};

#[cfg(all(feature = "std", unix))]
use async_net::unix::UnixStream;
#[cfg(feature = "std")]
use async_net::TcpStream;
#[cfg(feature = "std")]
use std::io;

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
    fn poll_send_packet(
        &mut self,
        bytes: &mut &[u8],
        fds: &mut Vec<Fd>,
        cx: &mut Context<'_>,
    ) -> Poll<crate::Result>;

    /// Read a packet from the connection in an async manner.
    fn poll_read_packet(
        &mut self,
        bytes: &mut &mut [u8],
        fds: &mut Vec<Fd>,
        cx: &mut Context<'_>,
    ) -> Poll<crate::Result>;

    /// Establish a connection to the server.
    #[inline]
    fn establish<'future>(
        &'future mut self,
        auth_info: Option<AuthInfo>,
    ) -> GenericConnFuture<'future, (Setup, XidGenerator)>
    where
        Self: Send,
    {
        Box::pin(async move { async_establish_connection(self, auth_info) })
    }
}

impl<C: AsyncConnection + ?Sized> AsyncConnection for &mut C {
    #[inline]
    fn poll_send_packet(
        &mut self,
        bytes: &mut &[u8],
        fds: &mut Vec<Fd>,
        cx: &mut Context<'_>,
    ) -> Poll<crate::Result> {
        (**self).poll_send_packet(bytes, fds, cx)
    }

    #[inline]
    fn poll_read_packet(
        &mut self,
        bytes: &mut &mut [u8],
        fds: &mut Vec<Fd>,
        cx: &mut Context<'_>,
    ) -> Poll<crate::Result> {
        (**self).poll_read_packet(bytes, fds, cx)
    }
}

/// Extension trait for `AsyncConnection` that provides futures.
pub trait AsyncConnectionExt {
    fn read_packet_async<'a, 'b, 'c>(
        &mut self,
        bytes: &'b [u8],
        fds: &'c mut Vec<Fd>,
    ) -> ReadPacketFuture<'a, 'b, 'c, Self>;
    fn send_packet_async<'a, 'b, 'c>(
        &mut self,
        bytes: &'b mut [u8],
        fds: &'c mut Vec<Fd>,
    ) -> SendPacketFuture<'a, 'b, 'c, Self>;
}

impl<C: AsyncConnection + ?Sized> AsyncConnectionExt for C {
    #[inline]
    fn read_packet_async<'a, 'b, 'c>(
        &mut self,
        bytes: &'b [u8],
        fds: &'c mut Vec<Fd>,
    ) -> ReadPacketFuture<'a, 'b, 'c, Self> {
        ReadPacketFuture::run(self, bytes, fds)
    }
    #[inline]
    fn send_packet_async<'a, 'b, 'c>(
        &mut self,
        bytes: &'b mut [u8],
        fds: &'c mut Vec<Fd>,
    ) -> SendPacketFuture<'a, 'b, 'c, Self> {
        SendPacketFuture::run(self, bytes, fds)
    }
}

macro_rules! unix_aware_async_connection_impl {
    ($name: ident) => {
        impl AsyncConnection for $name {
            #[inline]
            fn poll_send_packet(
                &mut self,
                bytes: &mut &[u8],
                fds: &mut Vec<Fd>,
                cx: &mut Context<'_>,
            ) -> Poll<crate::Result> {
                cfg_if::cfg_if! {
                    if #[cfg(unix)] {
                        unix::poll_send_packet_unix(self.clone().into(), bytes, fds)
                    } else {
                        standard_fd_warning(fds);
                        while !bytes.is_empty() {
                            match Pin::new(&mut **self).poll_write(cx, *bytes) {
                                Poll::Pending => return Poll::Pending,
                                Poll::Ready(Err(e)) => return Poll::Ready(Err(e.into())),
                                Poll::Ready(Ok(0)) => {
                                    return Poll::Ready(Err(io::ErrorKind::WriteZero.into().into()));
                                }
                                Poll::Ready(Ok(n)) => {
                                    let (_, rest) = mem::replace(bytes, &[]).split_at(n);
                                    *bytes = rest;
                                }
                            }
                        }

                        Poll::Ready(Ok(()))
                    }
                }
            }

            #[inline]
            fn poll_read_packet(
                &mut self,
                bytes: &mut &mut [u8],
                fds: &mut Vec<Fd>,
                cx: &mut Context<'_>,
            ) -> Poll<crate::Result> {
                cfg_if::cfg_if! {
                    if #[cfg(unix)] {
                        unix::poll_read_packet_unix(self.clone().into(), bytes, fds)
                    } else {
                        let _ = fds;
                        while !bytes.is_empty() {
                            let taken_bytes = mem::replace(bytes, &mut []);
                            match Pin::new(&mut **self).poll_read(cx, taken_bytes) {
                                Poll::Pending => return Poll::Pending,
                                Poll::Ready(Err(e)) => return Poll::Ready(Err(e.into())),
                                Poll::Ready(Ok(0)) => {
                                    return Poll::Ready(Err(io::ErrorKind::UnexpectedEof.into().into()));
                                }
                                Poll::Ready(Ok(n)) => {
                                    *bytes = &mut taken_bytes[n..];
                                }
                            }
                        }

                        Poll::Ready(Ok(()))
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
