// MIT/Apache2 License

use core::{mem, task::Poll};
use futures_util::{Future, FutureExt};
use tracing::Span;
use x11rb_protocol::x11_utils::TryParseFd;

use crate::{
    display::{AsyncDisplay, AsyncDisplayExt},
    Result,
};
use super::{SendRequest, WaitForReply};

/// A wrapper around [`SendRequest`] that immediately resolves the cookie.
pub struct CheckedSendRequest<'this, Dpy: ?Sized, Reply> {
    inner: Innards<'this, Dpy, Reply>,
    span: Option<Span>,
}

enum Innards<'this, Dpy: ?Sized, Reply> {
    SendRequest(SendRequest<'this, Dpy, Reply>),
    WaitForReply(WaitForReply<'this, Dpy, Reply>),
    Hole,
}

impl<'this, Dpy: ?Sized, Reply> From<SendRequest<'this, Dpy, Reply>>
    for CheckedSendRequest<'this, Dpy, Reply>
{
    fn from(mut inner: SendRequest<'this, Dpy, Reply>) -> Self {
        Self {
            span: inner.take_span(),
            inner: Innards::SendRequest(inner),
        }
    }
}

impl<'this, Dpy: AsyncDisplay + ?Sized, Reply: Unpin + TryParseFd> Future
    for CheckedSendRequest<'this, Dpy, Reply>
{
    type Output = Result<Reply>;

    fn poll(
        self: core::pin::Pin<&mut Self>,
        ctx: &mut core::task::Context<'_>,
    ) -> Poll<Self::Output> {
        let this = self.get_mut();

        // take the span
        let _enter = this.span.as_ref().map(Span::enter);

        loop {
            match this.inner {
                Innards::SendRequest(ref mut inner) => {
                    match inner.poll_unpin(ctx) {
                        Poll::Ready(Ok(cookie)) => {
                            // get the display references
                            let display_ref = match mem::replace(&mut this.inner, Innards::Hole) {
                                Innards::SendRequest(inner) => inner.cannibalize(),
                                _ => unreachable!(),
                            };

                            this.inner = Innards::WaitForReply(display_ref.wait_for_reply(cookie));
                        }
                        res => return res.map_ok(|_| unreachable!()),
                    }
                }
                Innards::WaitForReply(ref mut wfr) => return wfr.poll_unpin(ctx),
                Innards::Hole => unreachable!("cannot poll an empty hole"),
            }
        }
    }
}
