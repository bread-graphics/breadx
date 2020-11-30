// MIT/Apache2 License

#[cfg(feature = "async")]
use async_io::block_on;
#[cfg(feature = "async")]
use async_net::TcpStream as AsyncTcpStream;
#[cfg(feature = "async")]
use futures_lite::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "std")]
use std::{io::prelude::*, net::TcpStream};

#[cfg(all(feature = "async", unix))]
use async_net::unix::UnixStream as AsyncUnixStream;
#[cfg(all(feature = "std", unix))]
use std::os::unix::net::UnixStream;

#[cfg(feature = "async")]
use alloc::boxed::Box;
#[cfg(feature = "async")]
use core::{future::Future, pin::Pin};
#[cfg(feature = "async")]
use futures_lite::future;

/// A boxed future, for returning from a trait.
#[cfg(feature = "async")]
pub type GenericFuture<'future> = Pin<Box<dyn Future<Output = crate::Result> + Send + 'future>>;

/// A trait that represents the ability to send and receive bytes across a connection. This is used as a two-way
/// stream to send and receive data from the X server.
pub trait Connection : Sync {
    /// Send bytes in a packet across the connection in a blocking manner.
    fn send_packet(&mut self, bytes: &[u8]) -> crate::Result;
    /// Read a packet/request from the connection in a blocking manner.
    fn read_packet(&mut self, bytes: &mut [u8]) -> crate::Result;
    /// Send bytes in a packet across the connection in an async manner.
    #[cfg(feature = "async")]
    fn send_packet_async<'future, 'a, 'b>(&'a mut self, bytes: &'b [u8]) -> GenericFuture<'future>
    where
        'a: 'future,
        'b: 'future;
    /// Read a packet/request from the connection in an async manner.
    #[cfg(feature = "async")]
    fn read_packet_async<'future, 'a, 'b>(
        &'a mut self,
        bytes: &'b mut [u8],
    ) -> GenericFuture<'future>
    where
        'a: 'future,
        'b: 'future;
}

#[cfg(feature = "std")]
impl Connection for TcpStream {
    #[inline]
    fn send_packet(&mut self, bytes: &[u8]) -> crate::Result {
        self.write_all(bytes)?;
        Ok(())
    }

    #[inline]
    fn read_packet(&mut self, bytes: &mut [u8]) -> crate::Result {
        self.read_exact(bytes)?;
        Ok(())
    }

    #[cfg(feature = "async")]
    #[inline]
    fn send_packet_async<'future, 'a, 'b>(&'a mut self, _bytes: &'b [u8]) -> GenericFuture<'future>
    where
        'a: 'future,
        'b: 'future,
    {
        Box::pin(future::ready(Err(crate::BreadError::WouldBlock)))
    }

    #[cfg(feature = "async")]
    #[inline]
    fn read_packet_async<'future, 'a, 'b>(
        &'a mut self,
        _bytes: &'b mut [u8],
    ) -> GenericFuture<'future>
    where
        'a: 'future,
        'b: 'future,
    {
        Box::pin(future::ready(Err(crate::BreadError::WouldBlock)))
    }
}

#[cfg(feature = "async")]
impl Connection for AsyncTcpStream {
    #[inline]
    fn send_packet(&mut self, bytes: &[u8]) -> crate::Result {
        log::warn!("Called blocking send_packet for async TcpStream");
        block_on(self.write_all(bytes))?;
        Ok(())
    }

    #[inline]
    fn read_packet(&mut self, bytes: &mut [u8]) -> crate::Result {
        log::warn!("Called blocking read_packet for async TcpStream");
        block_on(self.read_exact(bytes))?;
        Ok(())
    }

    #[inline]
    fn send_packet_async<'future, 'a, 'b>(&'a mut self, bytes: &'b [u8]) -> GenericFuture<'future>
    where
        'a: 'future,
        'b: 'future,
    {
        #[inline]
        async fn inner(this: &mut AsyncTcpStream, bytes: &[u8]) -> crate::Result {
            this.write_all(bytes).await?;
            Ok(())
        }

        Box::pin(inner(self, bytes))
    }

    #[inline]
    fn read_packet_async<'future, 'a, 'b>(&'a mut self, bytes: &'b mut [u8]) -> GenericFuture<'future>
    where
        'a: 'future,
        'b: 'future,
    {
        #[inline]
        async fn inner(this: &mut AsyncTcpStream, bytes: &mut [u8]) -> crate::Result {
            this.read_exact(bytes).await?;
            Ok(())
        }

        Box::pin(inner(self, bytes))
    }
}

#[cfg(all(feature = "std", unix))]
impl Connection for UnixStream {
    #[inline]
    fn send_packet(&mut self, bytes: &[u8]) -> crate::Result {
        self.write_all(bytes)?;
        Ok(())
    }

    #[inline]
    fn read_packet(&mut self, bytes: &mut [u8]) -> crate::Result {
        self.read_exact(bytes)?;
        Ok(())
    }

    #[cfg(feature = "async")]
    #[inline]
    fn send_packet_async<'future, 'a, 'b>(&'a mut self, _bytes: &'b [u8]) -> GenericFuture<'future>
    where
        'a: 'future,
        'b: 'future,
    {
        Box::pin(future::ready(Err(crate::BreadError::WouldBlock)))
    }

    #[cfg(feature = "async")]
    #[inline]
    fn read_packet_async<'future, 'a, 'b>(
        &'a mut self,
        _bytes: &'b mut [u8],
    ) -> GenericFuture<'future>
    where
        'a: 'future,
        'b: 'future,
    {
        Box::pin(future::ready(Err(crate::BreadError::WouldBlock)))
    }
}

#[cfg(all(feature = "async", unix))]
impl Connection for AsyncUnixStream {
    #[inline]
    fn send_packet(&mut self, bytes: &[u8]) -> crate::Result {
        log::warn!("Called blocking send_packet for async UnixStream");
        block_on(self.write(bytes))?;
        Ok(())
    }

    #[inline]
    fn read_packet(&mut self, bytes: &mut [u8]) -> crate::Result {
        log::warn!("Called blocking read_packet for async UnixStream");
        block_on(self.read(bytes))?;
        Ok(())
    }

    #[inline]
    fn send_packet_async<'future, 'a, 'b>(&'a mut self, bytes: &'b [u8]) -> GenericFuture<'future>
    where
        'a: 'future,
        'b: 'future,
    {
        #[inline]
        async fn inner(this: &mut AsyncUnixStream, bytes: &[u8]) -> crate::Result {
            this.write(bytes).await?;
            Ok(())
        }

        Box::pin(inner(self, bytes))
    }

    #[inline]
    fn read_packet_async<'future, 'a, 'b>(&'a mut self, bytes: &'b mut [u8]) -> GenericFuture<'future>
    where
        'a: 'future,
        'b: 'future,
    {
        #[inline]
        async fn inner(this: &mut AsyncUnixStream, bytes: &mut [u8]) -> crate::Result {
            this.read(bytes).await?;
            Ok(())
        }

        Box::pin(inner(self, bytes))
    }
}
