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
    connection: &mut Conn,
    bytes: &'b [u8],
    fds: &'c mut Vec<Fd>,
}

impl<'a, 'b, 'c, Conn: ?Sized> SendPacketFuture<'a, 'b, 'c, Conn> {
    #[inline]
    pub(crate) fn run(connection: &mut Conn, bytes: &'b [u8], fds: &'c mut Vec<Fd>) -> Self {
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
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<crate::Result> {
        let mut fds = mem::take(&mut self.fds);
        let mut bytes = mem::replace(&mut self.bytes, &[]);
        let res = self.connection.poll_send_packet(&mut bytes, &mut fds, cx);
        self.fds = fds;
        self.bytes = bytes;
        res
    }
}