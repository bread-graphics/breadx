// MIT/Apache2 License

#[cfg(all(feature = "std", unix))]
mod unix;

use crate::Fd;
use alloc::vec::Vec;

#[cfg(feature = "async")]
use async_io::block_on;
#[cfg(feature = "async")]
use async_net::TcpStream as AsyncTcpStream;
#[cfg(all(feature = "async", not(unix)))]
use futures_lite::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "std")]
use std::net::TcpStream;

#[cfg(all(feature = "async", unix))]
use async_net::unix::UnixStream as AsyncUnixStream;

#[cfg(all(feature = "std", not(unix)))]
use std::io::{Read, Write};

#[cfg(all(feature = "std", unix))]
use std::os::unix::net::UnixStream;

#[cfg(feature = "async")]
use alloc::boxed::Box;
#[cfg(feature = "async")]
use core::{future::Future, pin::Pin};

/// A boxed future, for returning from a trait.
#[cfg(feature = "async")]
pub type GenericFuture<'future> = Pin<Box<dyn Future<Output = crate::Result> + Send + 'future>>;

/// A trait that represents the ability to send and receive bytes across a connection. This is used as a two-way
/// stream to send and receive data from the X server.
pub trait Connection: Send + Sync {
    /// Send bytes in a packet across the connection in a blocking manner. In addition, it may send a set of
    /// file descriptors across the connection as well, if supported.
    fn send_packet(&mut self, bytes: &[u8], fds: &mut Vec<Fd>) -> crate::Result;
    /// Read a packet/request from the connection in a blocking manner. In addition, it can read in file
    /// descriptors, if supported.
    fn read_packet(&mut self, bytes: &mut [u8], fds: &mut Vec<Fd>) -> crate::Result;
    /// Send bytes in a packet across the connection in an async manner.
    #[cfg(feature = "async")]
    fn send_packet_async<'future, 'a, 'b, 'c>(
        &'a mut self,
        bytes: &'b [u8],
        fds: &'c mut Vec<Fd>,
    ) -> GenericFuture<'future>
    where
        'a: 'future,
        'b: 'future,
        'c: 'future;
    /// Read a packet/request from the connection in an async manner.
    #[cfg(feature = "async")]
    fn read_packet_async<'future, 'a, 'b, 'c>(
        &'a mut self,
        bytes: &'b mut [u8],
        fds: &'c mut Vec<Fd>,
    ) -> GenericFuture<'future>
    where
        'a: 'future,
        'b: 'future,
        'c: 'future;
    /// Is this object async capable?
    #[cfg(feature = "async")]
    fn is_async(&self) -> bool;
}

#[cfg(not(unix))]
#[inline]
fn standard_fd_warning(fds: &mut Vec<Fd>) {
    if !fds.is_empty() {
        log::error!("File descriptor transportation is not supported on non-Unix operating systems. This can");
        log::error!("cause unavoidable protocol errors.");
    }
}

#[cfg(feature = "std")]
impl Connection for TcpStream {
    #[inline]
    fn send_packet(&mut self, bytes: &[u8], fds: &mut Vec<Fd>) -> crate::Result {
        #[cfg(not(unix))]
        {
            standard_fd_warning(fds);
            self.write_all(bytes)?;
            Ok(())
        }
        #[cfg(unix)]
        {
            unix::send_packet_unix(self, bytes, fds)
        }
    }

    #[inline]
    fn read_packet(&mut self, bytes: &mut [u8], fds: &mut Vec<Fd>) -> crate::Result {
        #[cfg(not(unix))]
        {
            let _ = fds;
            self.read_exact(bytes)?;
            Ok(())
        }
        #[cfg(unix)]
        {
            unix::read_packet_unix(self, bytes, fds)
        }
    }

    #[cfg(feature = "async")]
    #[inline]
    fn send_packet_async<'future, 'a, 'b, 'c>(
        &'a mut self,
        _bytes: &'b [u8],
        _fds: &'c mut Vec<Fd>,
    ) -> GenericFuture<'future>
    where
        'a: 'future,
        'b: 'future,
        'c: 'future,
    {
        Box::pin(async { Err(crate::BreadError::WouldBlock) })
    }

    #[cfg(feature = "async")]
    #[inline]
    fn read_packet_async<'future, 'a, 'b, 'c>(
        &'a mut self,
        _bytes: &'b mut [u8],
        _fds: &'c mut Vec<Fd>,
    ) -> GenericFuture<'future>
    where
        'a: 'future,
        'b: 'future,
        'c: 'future,
    {
        Box::pin(async { Err(crate::BreadError::WouldBlock) })
    }

    #[cfg(feature = "async")]
    fn is_async(&self) -> bool {
        false
    }
}

#[cfg(feature = "async")]
impl Connection for AsyncTcpStream {
    #[inline]
    fn send_packet(&mut self, bytes: &[u8], fds: &mut Vec<Fd>) -> crate::Result {
        log::warn!("Called blocking send_packet for async TcpStream");
        block_on(self.send_packet_async(bytes, fds))?;
        Ok(())
    }

    #[inline]
    fn read_packet(&mut self, bytes: &mut [u8], fds: &mut Vec<Fd>) -> crate::Result {
        log::warn!("Called blocking read_packet for async TcpStream");
        block_on(self.read_packet_async(bytes, fds))?;
        Ok(())
    }

    #[inline]
    fn send_packet_async<'future, 'a, 'b, 'c>(
        &'a mut self,
        bytes: &'b [u8],
        fds: &'c mut Vec<Fd>,
    ) -> GenericFuture<'future>
    where
        'a: 'future,
        'b: 'future,
        'c: 'future,
    {
        #[inline]
        async fn inner(
            this: &mut AsyncTcpStream,
            bytes: &[u8],
            fds: &mut Vec<Fd>,
        ) -> crate::Result {
            #[cfg(not(unix))]
            {
                standard_fd_warning(fds);
                this.write_all(bytes).await?;
            }
            #[cfg(unix)]
            {
                unix::send_packet_unix_async(this.clone().into(), bytes, fds).await?;
            }
            Ok(())
        }

        Box::pin(inner(self, bytes, fds))
    }

    #[inline]
    fn read_packet_async<'future, 'a, 'b, 'c>(
        &'a mut self,
        bytes: &'b mut [u8],
        fds: &'c mut Vec<Fd>,
    ) -> GenericFuture<'future>
    where
        'a: 'future,
        'b: 'future,
        'c: 'future,
    {
        #[inline]
        async fn inner(
            this: &mut AsyncTcpStream,
            bytes: &mut [u8],
            fds: &mut Vec<Fd>,
        ) -> crate::Result {
            #[cfg(not(unix))]
            {
                let _ = fds;
                this.read_exact(bytes).await?;
            }
            #[cfg(unix)]
            {
                unix::read_packet_unix_async(this.clone().into(), bytes, fds).await?;
            }
            Ok(())
        }

        Box::pin(inner(self, bytes, fds))
    }

    #[inline]
    fn is_async(&self) -> bool {
        true
    }
}

#[cfg(all(feature = "std", unix))]
impl Connection for UnixStream {
    #[inline]
    fn send_packet(&mut self, bytes: &[u8], fds: &mut Vec<Fd>) -> crate::Result {
        #[cfg(not(unix))]
        {
            standard_fd_warning(fds);
            self.write_all(bytes)?;
            Ok(())
        }
        #[cfg(unix)]
        {
            unix::send_packet_unix(self, bytes, fds)
        }
    }

    #[inline]
    fn read_packet(&mut self, bytes: &mut [u8], fds: &mut Vec<Fd>) -> crate::Result {
        #[cfg(not(unix))]
        {
            let _ = fds;
            self.read_exact(bytes)?;
            Ok(())
        }
        #[cfg(unix)]
        {
            unix::read_packet_unix(self, bytes, fds)
        }
    }

    #[cfg(feature = "async")]
    #[inline]
    fn send_packet_async<'future, 'a, 'b, 'c>(
        &'a mut self,
        _bytes: &'b [u8],
        _fds: &'c mut Vec<Fd>,
    ) -> GenericFuture<'future>
    where
        'a: 'future,
        'b: 'future,
        'c: 'future,
    {
        Box::pin(async { Err(crate::BreadError::WouldBlock) })
    }

    #[cfg(feature = "async")]
    #[inline]
    fn read_packet_async<'future, 'a, 'b, 'c>(
        &'a mut self,
        _bytes: &'b mut [u8],
        _fds: &'c mut Vec<Fd>,
    ) -> GenericFuture<'future>
    where
        'a: 'future,
        'b: 'future,
        'c: 'future,
    {
        Box::pin(async { Err(crate::BreadError::WouldBlock) })
    }

    #[cfg(feature = "async")]
    #[inline]
    fn is_async(&self) -> bool {
        false
    }
}

#[cfg(all(feature = "async", unix))]
impl Connection for AsyncUnixStream {
    #[inline]
    fn send_packet(&mut self, bytes: &[u8], fds: &mut Vec<Fd>) -> crate::Result {
        log::warn!("Called blocking send_packet for async UnixStream");
        block_on(self.send_packet_async(bytes, fds))?;
        Ok(())
    }

    #[inline]
    fn read_packet(&mut self, bytes: &mut [u8], fds: &mut Vec<Fd>) -> crate::Result {
        log::warn!("Called blocking read_packet for async UnixStream");
        block_on(self.read_packet_async(bytes, fds))?;
        Ok(())
    }

    #[inline]
    fn send_packet_async<'future, 'a, 'b, 'c>(
        &'a mut self,
        bytes: &'b [u8],
        fds: &'c mut Vec<Fd>,
    ) -> GenericFuture<'future>
    where
        'a: 'future,
        'b: 'future,
        'c: 'future,
    {
        #[inline]
        async fn inner(
            this: &mut AsyncUnixStream,
            bytes: &[u8],
            fds: &mut Vec<Fd>,
        ) -> crate::Result {
            #[cfg(not(unix))]
            {
                standard_fd_warning(fds);
                this.write_all(bytes).await?;
            }
            #[cfg(unix)]
            {
                unix::send_packet_unix_async(this.clone().into(), bytes, fds).await?;
            }
            Ok(())
        }

        Box::pin(inner(self, bytes, fds))
    }

    #[inline]
    fn read_packet_async<'future, 'a, 'b, 'c>(
        &'a mut self,
        bytes: &'b mut [u8],
        fds: &'c mut Vec<Fd>,
    ) -> GenericFuture<'future>
    where
        'a: 'future,
        'b: 'future,
        'c: 'future,
    {
        #[inline]
        async fn inner(
            this: &mut AsyncUnixStream,
            bytes: &mut [u8],
            fds: &mut Vec<Fd>,
        ) -> crate::Result {
            #[cfg(not(unix))]
            {
                let _ = fds;
                this.read_exact(bytes).await?;
            }
            #[cfg(unix)]
            {
                unix::read_packet_unix_async(this.clone().into(), bytes, fds).await?;
            }
            Ok(())
        }

        Box::pin(inner(self, bytes, fds))
    }

    #[inline]
    fn is_async(&self) -> bool {
        true
    }
}
