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

/// A trait that represents the ability to send and receive bytes across a connection.
#[cfg_attr(feature = "async", async_trait::async_trait)]
pub trait Connection {
    /// Send bytes in a packet across the connection in a blocking manner.
    fn send_packet(&mut self, bytes: &[u8]) -> crate::Result;
    /// Read a packet/request from the connection in a blocking manner.
    fn read_packet(&mut self, bytes: &mut [u8]) -> crate::Result;
    /// Send bytes in a packet across the connection in an async manner.
    #[cfg(feature = "async")]
    async fn send_packet_async(&mut self, bytes: &[u8]) -> crate::Result;
    /// Read a packet/request from the connection in an async manner.
    #[cfg(feature = "async")]
    async fn read_packet_async(&mut self, bytes: &mut [u8]) -> crate::Result;
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "async", async_trait::async_trait)]
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
    async fn send_packet_async(&mut self, bytes: &[u8]) -> crate::Result {
        log::error!("Called send_packet_async for non-async TcpStream");
        self.write_all(bytes)?;
        Ok(())
    }

    #[cfg(feature = "async")]
    #[inline]
    async fn read_packet_async(&mut self, bytes: &mut [u8]) -> crate::Result {
        log::error!("Called read_packet_async for non-async TcpStream");
        self.read_exact(bytes)?;
        Ok(())
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
impl Connection for AsyncTcpStream {
    #[inline]
    fn send_packet(&mut self, bytes: &[u8]) -> crate::Result {
        log::warn!("Called send_packet for async TcpStream");
        block_on(self.write(bytes))?;
        Ok(())
    }

    #[inline]
    fn read_packet(&mut self, bytes: &mut [u8]) -> crate::Result {
        log::warn!("Called read_packet for async TcpStream");
        block_on(self.read(bytes))?;
        Ok(())
    }

    #[inline]
    async fn send_packet_async(&mut self, bytes: &[u8]) -> crate::Result {
        self.write(bytes).await?;
        Ok(())
    }

    #[inline]
    async fn read_packet_async(&mut self, bytes: &mut [u8]) -> crate::Result {
        self.read(bytes).await?;
        Ok(())
    }
}

#[cfg(all(feature = "std", unix))]
#[cfg_attr(feature = "async", async_trait::async_trait)]
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
    async fn send_packet_async(&mut self, bytes: &[u8]) -> crate::Result {
        log::warn!("Called send_packet_async for non-async UnixStream");
        self.write_all(bytes)?;
        Ok(())
    }

    #[cfg(feature = "async")]
    #[inline]
    async fn read_packet_async(&mut self, bytes: &mut [u8]) -> crate::Result {
        log::warn!("Called read_packet_async for non-async UnixStrean");
        self.read_exact(bytes)?;
        Ok(())
    }
}

#[cfg(all(feature = "async", unix))]
#[async_trait::async_trait]
impl Connection for AsyncUnixStream {
    #[inline]
    fn send_packet(&mut self, bytes: &[u8]) -> crate::Result {
        log::warn!("Called send_packet for async UnixStream");
        block_on(self.write(bytes))?;
        Ok(())
    }

    #[inline]
    fn read_packet(&mut self, bytes: &mut [u8]) -> crate::Result {
        log::warn!("Called read_packet for async UnixStream");
        block_on(self.read(bytes))?;
        Ok(())
    }

    #[inline]
    async fn send_packet_async(&mut self, bytes: &[u8]) -> crate::Result {
        self.write(bytes).await?;
        Ok(())
    }

    #[inline]
    async fn read_packet_async(&mut self, bytes: &mut [u8]) -> crate::Result {
        self.read(bytes).await?;
        Ok(())
    }
}
