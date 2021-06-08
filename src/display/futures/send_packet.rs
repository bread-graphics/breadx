// MIT/Apache2 License

use crate::{display::AsyncConnection, Fd};
use alloc::vec::Vec;
use core::{
    future::Future,
    mem,
    pin::Pin,
    task::{Context, Poll},
};

/// The future returned by `AsyncConnectionExt::send_packet`.
#[derive(Debug)]
pub struct SendPacketFuture<'a, 'b, 'c, Conn: ?Sized> {
    connection: &'a mut Conn,
    bytes: &'b [u8],
    fds: &'c mut Vec<Fd>,
}

impl<'a, 'b, 'c, Conn: ?Sized> SendPacketFuture<'a, 'b, 'c, Conn> {
    #[inline]
    pub(crate) fn run(connection: &'a mut Conn, bytes: &'b [u8], fds: &'c mut Vec<Fd>) -> Self {
        Self {
            connection,
            bytes,
            fds,
        }
    }
}

impl<'a, 'b, 'c, Conn: AsyncConnection + Unpin + ?Sized> Future
    for SendPacketFuture<'a, 'b, 'c, Conn>
{
    type Output = crate::Result;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<crate::Result> {
        let mut fds = mem::take(self.fds);
        let mut total_read = 0;
        let bytes = self.bytes;

        let res = self
            .connection
            .poll_send_packet(bytes, &mut fds, cx, &mut total_read);
        *self.fds = fds;
        self.bytes = &self.bytes[total_read..];
        res
    }
}
