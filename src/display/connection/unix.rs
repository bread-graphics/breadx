// MIT/Apache2 License
// This file is largely inspired by x11rb

#![cfg(all(feature = "std", unix))]

use crate::{util::convert_nix_error, Fd};
use alloc::{vec, vec::Vec};
use nix::sys::{
    socket::{recvmsg, sendmsg, ControlMessage, ControlMessageOwned, MsgFlags},
    uio::IoVec,
};
use std::{
    io::{self, Read, Write},
    os::unix::io::{AsRawFd, RawFd},
};

#[cfg(feature = "async")]
use alloc::sync::Arc;
#[cfg(feature = "async")]
use async_io::Async;

#[inline]
fn send_msg_packet(conn: RawFd, data: &[u8], fds: &mut Vec<Fd>) -> (usize, io::Result<()>) {
    #[inline]
    fn sendmsg_loop(
        conn: RawFd,
        mut data: &[u8],
        mut cmsgs: &[ControlMessage<'_>],
    ) -> (usize, io::Result<()>) {
        let mut datavec = [IoVec::from_slice(data)];
        let mut offset = 0;
        loop {
            match sendmsg(conn, &datavec, cmsgs, MsgFlags::empty(), None) {
                Ok(0) => return (offset, Ok(())),
                Ok(m) => {
                    log::debug!("sendmsg: yet to send {} bytes", data.len() - m);
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
pub fn send_packet_unix<Conn: AsRawFd + Write>(
    conn: &mut Conn,
    data: &[u8],
    fds: &mut Vec<Fd>,
) -> crate::Result {
    let connfd = conn.as_raw_fd();

    send_msg_packet(connfd, data, fds).1?;
    Ok(())
}

/// The same as the above function, but async.
#[cfg(feature = "async")]
#[inline]
pub async fn send_packet_unix_async<Conn: AsRawFd + Write + Unpin>(
    conn: Arc<Async<Conn>>,
    mut data: &[u8],
    fds: &mut Vec<Fd>,
) -> crate::Result {
    // TODO: make sure this isn't unsound. the way we use it, it shouldn't be
    conn.write_with(|conn| {
        let connfd = conn.as_raw_fd();
        let (offset, res) = send_msg_packet(connfd, data, fds);
        // the data stream might be interrupted; since this is called in a loop,
        // we need to keep track of what we've written
        data = &data[offset..];
        res
    })
    .await?;

    #[cfg(debug_assertions)]
    log::trace!("Done with write_with()");

    Ok(())
}

/// Read a packet, unix style. Includes fds.
#[allow(clippy::similar_names)]
#[inline]
fn read_msg_packet(conn: RawFd, mut data: &mut [u8], fds: &mut Vec<Fd>) -> io::Result<()> {
    const MAX_FDS: usize = 16;
    let mut cmsg = nix::cmsg_space!([Fd; MAX_FDS]);
    let mut datalen = data.len();
    let mut datavec = [IoVec::from_mut_slice(data)];

    let msg = loop {
        match recvmsg(conn, &datavec, Some(&mut cmsg), MsgFlags::empty()) {
            Ok(m) if m.bytes == 0 => break m,
            Ok(m) if m.bytes == datalen => break m,
            Ok(m) => {
                log::debug!("recvmsg: yet to receive {} bytes", data.len() - m.bytes);
                data = &mut data[m.bytes..];
                datalen = data.len();
                datavec = [IoVec::from_mut_slice(data)];
            }
            Err(nix::Error::Sys(nix::errno::Errno::EINTR)) => {
                log::debug!("Interrupt occurred during read ");
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
pub fn read_packet_unix<Conn: AsRawFd + Read>(
    conn: &mut Conn,
    data: &mut [u8],
    fds: &mut Vec<Fd>,
) -> crate::Result {
    let connfd = conn.as_raw_fd();
    read_msg_packet(connfd, data, fds)?;
    Ok(())
}

/// Read a packet, async redox.
#[cfg(feature = "async")]
#[inline]
pub async fn read_packet_unix_async<Conn: AsRawFd + Read + Unpin>(
    conn: Arc<Async<Conn>>,
    data: &mut [u8],
    fds: &mut Vec<Fd>,
) -> crate::Result {
    // TODO: same as above
    conn.read_with(|conn| {
        let connfd = conn.as_raw_fd();
        read_msg_packet(connfd, data, fds)
    })
    .await?;

    Ok(())
}
