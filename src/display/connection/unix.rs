// MIT/Apache2 License
// This file is largely inspired by x11rb

#![cfg(all(feature = "std", unix))]

use crate::{log_debug, log_trace, util::convert_nix_error, Fd};
use alloc::{vec, vec::Vec};
use nix::sys::{
    socket::{recvmsg, sendmsg, ControlMessage, ControlMessageOwned, MsgFlags},
    uio::IoVec,
};
use std::{io, os::unix::io::RawFd};

#[cfg(feature = "async")]
use async_io::Async;
#[cfg(feature = "async")]
use core::task::{Context, Poll};
#[cfg(feature = "async")]
use std::{
    os::unix::io::AsRawFd,
};

#[inline]
fn send_msg_packet(conn: RawFd, data: &[u8], fds: &mut Vec<Fd>) -> (usize, io::Result<()>) {
    #[inline]
    fn sendmsg_loop(
        conn: RawFd,
        mut data: &[u8],
        mut cmsgs: &[ControlMessage<'_>],
    ) -> (usize, io::Result<()>) {
        let datalen = data.len();
        let mut datavec = [IoVec::from_slice(data)];
        let mut offset = 0;
        loop {
            match sendmsg(conn, &datavec, cmsgs, MsgFlags::empty(), None) {
                Ok(0) => {
                    log_debug!("sendmsg sent 0 bytes; request is likely finished");
                    return (offset, Ok(()));
                }
                Ok(m) if m == datalen => {
                    log_debug!(
                        "sendmsg sent {} bytes; this encompasses the entire buffer",
                        m
                    );
                    offset += m;
                    return (offset, Ok(()));
                }
                Ok(m) => {
                    log_debug!(
                        "sendmsg: sent {} bytes, yet to send {} bytes",
                        m,
                        data.len() - m
                    );
                    offset += m;
                    data = &data[m..];
                    datavec = [IoVec::from_slice(data)];
                    // ensure we never send the file descriptors more than once
                    cmsgs = &[];
                }
                Err(nix::Error::Sys(nix::errno::Errno::EINTR)) => (),
                Err(e) => return (offset, Err(convert_nix_error(e))),
            }
        }
    }

    log_trace!("Beginning send_msg_packet (*nix implementation of send_packet())");

    let res = if fds.is_empty() {
        sendmsg_loop(conn, data, &[])
    } else {
        let cmsgs = [ControlMessage::ScmRights(&fds)];
        sendmsg_loop(conn, data, &cmsgs)
    };

    if res.0 > 0 {
        fds.clear();
    }

    res
}

/// For Unix stream types, we can use this function to send FDs.
#[inline]
pub(crate) fn send_packet_unix(conn: RawFd, data: &[u8], fds: &mut Vec<Fd>) -> crate::Result {
    send_msg_packet(conn, data, fds).1?;
    Ok(())
}

/// The same as the above function, but in polling form.
#[cfg(feature = "async")]
#[inline]
pub(crate) fn poll_send_packet_unix<Conn: ConnSource>(
    conn: &mut Conn,
    mut data: &[u8],
    fds: &mut Vec<Fd>,
    cx: &mut Context<'_>,
    bytes_read: &mut usize,
) -> Poll<crate::Result> {
    let connfd = conn.raw_fd();
    loop {
        // try to run until we encounter unwritability
        let (offset, res) = send_msg_packet(connfd, data, fds);
        data = &data[offset..];
        *bytes_read += offset;

        match res {
            Ok(()) => break Poll::Ready(Ok(())),
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => break Poll::Ready(Err(e.into())),
        }

        // poll for writability
        match conn.poll_writable(cx) {
            Poll::Pending => break Poll::Pending,
            Poll::Ready(Ok(())) => { /* continue loop */ }
            Poll::Ready(Err(e)) => break Poll::Ready(Err(e.into())),
        }
    }
}

/// Read a packet, unix style. Includes fds.
#[allow(clippy::similar_names)]
#[inline]
fn read_msg_packet(
    conn: RawFd,
    mut data: &mut [u8],
    fds: &mut Vec<Fd>,
    total_read: &mut usize,
) -> io::Result<()> {
    const MAX_FDS: usize = 16;

    log_trace!("Beginning read_msg_packet (*nix implementation of read_packet())");

    if data.is_empty() {
        return Ok(());
    }

    let mut cmsg = nix::cmsg_space!([Fd; MAX_FDS]);
    let mut datalen = data.len();
    let mut datavec = [IoVec::from_mut_slice(data)];

    let msg = loop {
        log::debug!("Calling recvmsg with a data buffer of length {}", datalen);
        match recvmsg(conn, &datavec, Some(&mut cmsg), MsgFlags::empty()) {
            Ok(m) if m.bytes == 0 => {
                log_debug!("recvmsg read 0 bytes; we've likely read everything");
                break m;
            }
            Ok(m) if m.bytes == datalen => {
                log_debug!(
                    "recvmsg read {} bytes; this fills our entire buffer",
                    m.bytes
                );
                *total_read += m.bytes;
                break m;
            }
            Ok(m) => {
                log_debug!(
                    "recvmsg read {} bytes, yet to receive {} bytes",
                    m.bytes,
                    data.len() - m.bytes
                );
                let bytes = m.bytes;
                data = &mut data[bytes..];
                *total_read += bytes;

                datalen = data.len();
                datavec = [IoVec::from_mut_slice(data)];
            }
            Err(nix::Error::Sys(nix::errno::Errno::EINTR)) => {
                log::warn!("Interrupt occurred during read");
            }
            Err(e) => return Err(convert_nix_error(e)),
        }
    };

    fds.extend(msg.cmsgs().flat_map(|cmsg| match cmsg {
        ControlMessageOwned::ScmRights(r) => r,
        _ => vec![],
    }));

    Ok(())
}

/// Read a packet, unix style.
#[inline]
pub(crate) fn read_packet_unix(conn: RawFd, data: &mut [u8], fds: &mut Vec<Fd>) -> crate::Result {
    let mut _total_read = 0;
    read_msg_packet(conn, data, fds, &mut _total_read)?;
    Ok(())
}

/// Read a packet, async redox.
#[cfg(feature = "async")]
#[inline]
pub(crate) fn poll_read_packet_unix<Conn: ConnSource>(
    conn: &mut Conn,
    data: &mut [u8],
    fds: &mut Vec<Fd>,
    cx: &mut Context<'_>,
    bytes_read: &mut usize,
) -> Poll<crate::Result> {
    let connfd = conn.raw_fd();
    loop {
        // try to read until we can't anymore
        match read_msg_packet(connfd, data, fds, bytes_read) {
            Ok(()) => break Poll::Ready(Ok(())),
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => break Poll::Ready(Err(e.into())),
        }

        match conn.poll_readable(cx) {
            Poll::Pending => break Poll::Pending,
            Poll::Ready(Ok(())) => { /* continue loop */ }
            Poll::Ready(Err(e)) => break Poll::Ready(Err(e.into())),
        }
    }
}

#[cfg(feature = "async")]
pub(crate) trait ConnSource {
    fn poll_readable(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>>;
    fn poll_writable(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>>;
    fn raw_fd(&mut self) -> RawFd;
}

#[cfg(feature = "async")]
impl<Conn: AsRawFd + Unpin> ConnSource for Async<Conn> {
    #[inline]
    fn poll_readable(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Async::<Conn>::poll_readable(self, cx)
    }
    #[inline]
    fn poll_writable(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Async::<Conn>::poll_writable(self, cx)
    }
    #[inline]
    fn raw_fd(&mut self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(feature = "async")]
impl<Conn: AsRawFd + Unpin> ConnSource for &Async<Conn> {
    #[inline]
    fn poll_readable(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Async::<Conn>::poll_readable(self, cx)
    }
    #[inline]
    fn poll_writable(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Async::<Conn>::poll_writable(self, cx)
    }
    #[inline]
    fn raw_fd(&mut self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(feature = "tokio-support")]
impl ConnSource for tokio::net::TcpStream {
    #[inline]
    fn poll_readable(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.poll_read_ready(cx)
    }
    #[inline]
    fn poll_writable(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.poll_write_ready(cx)
    }
    #[inline]
    fn raw_fd(&mut self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(feature = "tokio-support")]
impl ConnSource for tokio::net::UnixStream {
    #[inline]
    fn poll_readable(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.poll_read_ready(cx)
    }
    #[inline]
    fn poll_writable(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.poll_write_ready(cx)
    }
    #[inline]
    fn raw_fd(&mut self) -> RawFd {
        self.as_raw_fd()
    }
}
