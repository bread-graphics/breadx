// MIT/Apache2 License

use super::{CheckForError, WaitForReplyRaw};
use crate::{
    display::{AsyncDisplay, AsyncDisplayExt, Cookie},
    Result,
};
use alloc::vec::Vec;
use core::{
    future::Future,
    marker::PhantomData,
    mem,
    pin::Pin,
    task::{Context, Poll},
};
use futures_util::FutureExt;
use x11rb_protocol::x11_utils::TryParseFd;

/// The future returned by the `wait_for_reply` function.
pub struct WaitForReply<'this, Dpy: ?Sized, Reply> {
    innards: Innards<'this, Dpy>,
    _marker: PhantomData<Reply>,
}

enum Innards<'this, Dpy: ?Sized> {
    Waiting(WaitForReplyRaw<'this, Dpy>),
    Checking(CheckForError<'this, Dpy>),
}

impl<'this, Dpy: AsyncDisplay + ?Sized, Reply> WaitForReply<'this, Dpy, Reply> {
    pub(crate) fn new(dpy: &'this mut Dpy, cookie: Cookie<Reply>) -> Self {
        Self {
            innards: if mem::size_of::<Reply>() == 0 {
                Innards::Checking(dpy.check_for_error(cookie.sequence()))
            } else {
                Innards::Waiting(dpy.wait_for_reply_raw(cookie.sequence()))
            },
            _marker: PhantomData,
        }
    }

    pub(crate) fn cannibalize(self) -> &'this mut Dpy {
        match self.innards {
            Innards::Waiting(innards) => innards.cannibalize(),
            Innards::Checking(innards) => innards.cannibalize(),
        }
    }
}

impl<'this, Dpy: AsyncDisplay + ?Sized, Reply: TryParseFd + Unpin> Future
    for WaitForReply<'this, Dpy, Reply>
{
    type Output = Result<Reply>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<Reply>> {
        let this = self.get_mut();

        // determine how it goes down
        match this.innards {
            Innards::Waiting(ref mut wait_for_reply_raw) => {
                match wait_for_reply_raw.poll_unpin(ctx) {
                    Poll::Ready(Ok(reply)) => Poll::Ready({
                        // try to resolve the reply
                        reply.into_reply()
                    }),
                    Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
                    Poll::Pending => Poll::Pending,
                }
            }
            Innards::Checking(ref mut check) => {
                check.poll_unpin(ctx).map_ok(|()| {
                    Reply::try_parse_fd(&[], &mut Vec::new())
                        .unwrap_or_else(|_| unreachable!())
                        .0
                })
            }
        }
    }
}
