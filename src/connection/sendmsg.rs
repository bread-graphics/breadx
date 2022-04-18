// MIT/Apache2 License

#![cfg(unix)]

use super::Connection;

use core::{fmt, mem};

use nix::errno::Errno;
use nix::sys::socket::{recvmsg, sendmsg, ControlMessage, ControlMessageOwned, MsgFlags};

use std::net::UnixStream;
use std::os::unix::io::{AsRawFd, RawFd};

/// A variant of the `UnixStream` connection that uses `sendmsg` to send data
/// and `recvmsg` to receive it.
pub struct SendmsgConnection {
    stream: UnixStream,
}

impl fmt::Debug for SendmsgConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
    pub fn recvmsg(&self, buffer: &mut [u8], fds: &mut Vec<Fd>, flags: MsgFlags) -> Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }

        let conn = self.stream.as_raw_fd();
        let mut iov = [IoSliceMut::new(buffer)];

        // TODO: nix 0.24 will use io slices from std
        let iov = unsafe { std::mem::transmute(&mut iov) };

        let mut cmsg_space = nix::cmsg_space!([Fd; 32]);

        // run recvmsg
        let msg = loop {
            match recvmsg(conn, iov, Some(&mut cmsg_space), flags) {
                Ok(n) => break n,
                Err(Errno::EINTR) => continue,
                Err(e) => return Error::nix(e),
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
                .flatten(),
        );

        Ok(bytes_read)
    }
}

impl Connection for SendmsgConnection {
    fn write_slices_with_fds(&mut self, iov: &[IoSlice<'_>], fds: &mut Vec<Fd>) -> Result<usize> {
        // TODO: nix 0.24 will use io slices from std
        let iov = unsafe { std::mem::transmute(iov) };

        let fds = mem::take(fds);
        let control_msg = [ControlMessage::ScmRights(&fds)];
        let conn = self.stream.as_raw_fd();

        // send the message
        loop {
            match sendmsg(conn, iov, &control_msg, MsgFlags::empty()) {
                Ok(n) => return Ok(n),
                Err(Errno::EINTR) => continue,
                Err(e) => return Error::nix(e),
            }
        }

        // fds is dropped at the end here
    }

    fn write_slices(&mut self, iov: &[IoSlice<'_>]) -> Result<usize> {
        // since we don't have file descriptors, we can just skip the
        // special stuff and use write_vectored
        self.stream.write_vectored(iov)
    }

    fn write_slice(&mut self, buffer: &[u8]) -> Result<usize> {
        // same as above
        self.stream.write(buffer)
    }

    fn is_write_vectored(&self) -> bool {
        true
    }

    fn read_to_buffer_with_fds(&mut self, buffer: &mut [u8], fds: &mut Vec<Fd>) -> Result<usize> {
        self.recvmsg(buffer, fds, MsgFlags::empty())
    }

    fn flush(&mut self) -> Result<()> {
        self.stream.flush()
    }
}

impl NbConnection for SendmsgConnection {
    fn read_to_buffer_non_blocking(
        &mut self,
        buffer: &mut [u8],
        fds: &mut Vec<Fd>,
    ) -> Result<usize> {
        self.recvmsg(buffer, fds, MsgFlags::MSG_DONTWAIT)
    }
}
