// MIT/Apache2 License

use crate::{display::AsyncConnection, Fd};
use alloc::vec::Vec;
use core::{
    future::Future,
    mem,
    pin::Pin,
    task::{Context, Poll},
};

/// The future returned by `AsyncConnectionExt::read_packet`.
#[derive(Debug)]
pub struct ReadPacketFuture<'a, 'b, 'c, Conn: ?Sized> {
    connection: &'a mut Conn,
    bytes: &'b mut [u8],
    fds: &'c mut Vec<Fd>,
}

impl<'a, 'b, 'c, Conn: ?Sized> ReadPacketFuture<'a, 'b, 'c, Conn> {
    #[inline]
    pub(crate) fn run(connection: &'a mut Conn, bytes: &'b mut [u8], fds: &'c mut Vec<Fd>) -> Self {
        Self {
            connection,
            bytes,
            fds,
        }
    }
}

impl<'a, 'b, 'c, Conn: AsyncConnection + Unpin + ?Sized> Future
    for ReadPacketFuture<'a, 'b, 'c, Conn>
{
    type Output = crate::Result;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<crate::Result> {
        let mut fds = mem::take(self.fds);
        let mut total_len = 0;
        let bytes = mem::replace(&mut self.bytes, &mut []);

        let res = self
            .connection
            .poll_read_packet(bytes, &mut fds, cx, &mut total_len);
        *self.fds = fds;
        self.bytes = &mut bytes[total_len..];
        res
    }
}
