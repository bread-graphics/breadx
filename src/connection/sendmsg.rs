// MIT/Apache2 License

#![cfg(unix)]

use super::Connection;
use crate::{Error, Fd, Result};

use alloc::vec::Vec;

use core::borrow::{Borrow, BorrowMut};
use core::{fmt, mem};
use std::io::{IoSlice, IoSliceMut, Read, Write};

use nix::errno::Errno;
use nix::sys::socket::{recvmsg, sendmsg, ControlMessage, ControlMessageOwned, MsgFlags};

use std::os::unix::io::{AsRawFd, RawFd};
use std::os::unix::net::UnixStream;

/// A variant of the `UnixStream` connection that uses `sendmsg` to send data
/// and `recvmsg` to receive it.
///
/// This connection supports file descriptor passing.
pub struct SendmsgConnection {
    stream: UnixStream,
}

impl fmt::Debug for SendmsgConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.stream, f)
    }
}

impl AsRawFd for SendmsgConnection {
    fn as_raw_fd(&self) -> RawFd {
        self.stream.as_raw_fd()
    }
}

impl From<UnixStream> for SendmsgConnection {
    fn from(stream: UnixStream) -> Self {
        SendmsgConnection { stream }
    }
}

impl From<SendmsgConnection> for UnixStream {
    fn from(conn: SendmsgConnection) -> Self {
        conn.stream
    }
}

impl AsRef<UnixStream> for SendmsgConnection {
    fn as_ref(&self) -> &UnixStream {
        &self.stream
    }
}

impl AsMut<UnixStream> for SendmsgConnection {
    fn as_mut(&mut self) -> &mut UnixStream {
        &mut self.stream
    }
}

impl Borrow<UnixStream> for SendmsgConnection {
    fn borrow(&self) -> &UnixStream {
        &self.stream
    }
}

impl BorrowMut<UnixStream> for SendmsgConnection {
    fn borrow_mut(&mut self) -> &mut UnixStream {
        &mut self.stream
    }
}

impl SendmsgConnection {
    /// Create a new connection from a `UnixStream`.
    pub fn new(stream: UnixStream) -> Self {
        SendmsgConnection { stream }
    }

    /// Convert this object back into a `UnixStream`.
    pub fn into_stream(self) -> UnixStream {
        self.stream
    }

    /// Call `recvmsg` with the given `MsgFlags`.
    fn recvmsg(
        &self,
        iov: &mut [IoSliceMut<'_>],
        fds: &mut Vec<Fd>,
        flags: MsgFlags,
    ) -> Result<usize> {
        let span = tracing::trace_span!("recvmsg");
        let _enter = span.enter();

        if iov.is_empty() {
            return Ok(0);
        }

        let conn = self.stream.as_raw_fd();

        let mut cmsg_space = nix::cmsg_space!([Fd; 32]);

        // run recvmsg
        let msg = loop {
            match recvmsg::<()>(conn, iov, Some(&mut cmsg_space), flags) {
                Ok(n) => break n,
                Err(Errno::EINTR) => continue,
                Err(e) => return Err(Error::nix(e)),
            }
        };

        // process the infomration
        let bytes_read = msg.bytes;
        fds.extend(
            msg.cmsgs()
                .filter_map(|cmsg| match cmsg {
                    ControlMessageOwned::ScmRights(rights) => Some(rights),
                    _ => None,
                })
                .flatten()
                .map(|fd| Fd::new(fd)),
        );

        tracing::trace!("read {} bytes and {} fds", bytes_read, fds.len());

        Ok(bytes_read)
    }

    /// Call the sendmsg() function for the socket.
    fn sendmsg(&self, iov: &[IoSlice<'_>], fds: &mut Vec<Fd>) -> Result<usize> {
        let span = tracing::trace_span!("sendmsg");
        let _enter = span.enter();

        if iov.is_empty() {
            return Ok(0);
        }

        let our_fds = mem::take(fds);
        let raw_fds = our_fds
            .iter()
            .map(|fd| fd.as_raw_fd())
            .collect::<Vec<i32>>();
        let control_msg = [ControlMessage::ScmRights(&raw_fds)];
        let conn = self.stream.as_raw_fd();

        // send the message
        loop {
            match sendmsg::<()>(conn, iov, &control_msg, MsgFlags::empty(), None) {
                Ok(n) => {
                    tracing::trace!("sent {} bytes and {} fds", n, our_fds.len());
                    return Ok(n);
                }
                Err(Errno::EINTR) => continue,
                Err(e) => {
                    // return fds to prevent drop
                    *fds = our_fds;
                    return Err(Error::nix(e));
                }
            }
        }

        // fds is dropped at the end here
    }
}

// so we can duplicate the impl for owned and &
macro_rules! impl_sendmsg_conn {
    ($($inner: tt)*) => {
        fn send_slices_and_fds(&mut self, iov: &[IoSlice<'_>], fds: &mut Vec<Fd>) -> Result<usize> {
            self.sendmsg(iov, fds)
        }

        fn send_slices(&mut self, iov: &[IoSlice<'_>]) -> Result<usize> {
            // since we don't have file descriptors, we can just skip the
            // special stuff and use write_vectored
            ($($inner)* self.stream).write_vectored(iov).map_err(Error::io)
        }

        fn send_slice(&mut self, buffer: &[u8]) -> Result<usize> {
            // same as above
            ($($inner)* self.stream).write(buffer).map_err(Error::io)
        }

        fn recv_slices_and_fds(
            &mut self,
            slices: &mut [IoSliceMut<'_>],
            fds: &mut Vec<Fd>,
        ) -> Result<usize> {
            // use our recvmsg helper function
            self.recvmsg(slices, fds, MsgFlags::empty())
        }

        fn recv_slice(&mut self, slice: &mut [u8]) -> Result<usize> {
            let span = tracing::trace_span!("recv_slice");
            let _enter = span.enter();

            // just use the read() function
            ($($inner)* self.stream).read(slice).map_err(Error::io)
        }

        fn flush(&mut self) -> Result<()> {
            ($($inner)* self.stream).flush().map_err(Error::io)
        }

        fn non_blocking_recv_slices_and_fds(
            &mut self,
            slices: &mut [IoSliceMut<'_>],
            fds: &mut Vec<Fd>,
        ) -> Result<usize> {
            // use the recvmsg function with MSG_DONTWAIT
            self.recvmsg(slices, fds, MsgFlags::MSG_DONTWAIT)
        }

        fn shutdown(&self) -> Result<()> {
            let span = tracing::trace_span!("shutdown");
            let _guard = span.enter();

            self.stream
                .shutdown(std::net::Shutdown::Both)
                .map_err(Error::io)
        }
    }
}

impl Connection for SendmsgConnection {
    impl_sendmsg_conn! { &mut }
}

impl Connection for &SendmsgConnection {
    impl_sendmsg_conn! { & }
}
