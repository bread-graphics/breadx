// MIT/Apache2 License

use super::{ResolveRequestRawFuture, SynchronizeFuture};
use crate::{
    display::{decode_reply, AsyncDisplay, RequestCookie},
    Request,
};
use core::{
    future::Future,
    marker::PhantomData,
    mem,
    pin::Pin,
    task::{Context, Poll},
};

/// The future returned by `AsyncDisplayExt::resolve_request_async`.
#[derive(Debug)]
#[must_use = "futures do nothing unless you poll or .await them"]
pub enum ResolveRequestFuture<'a, D: ?Sized, R: Request> {
    /// We can fast-path to the answer, since we aren't checked.
    #[doc(hidden)]
    FastPath,
    /// The reply type is zero sized, and the display is currently synchronizing.
    #[doc(hidden)]
    Synchronizing {
        sf: SynchronizeFuture<'a, D>,
        tok: RequestCookie<R>,
    },
    /// The reply type is not zero typed, so we are calling the raw function.
    #[doc(hidden)]
    Resolving(ResolveRequestRawFuture<'a, D>),
}

impl<'a, D: AsyncDisplay + ?Sized, R: Request> ResolveRequestFuture<'a, D, R> {
    #[inline]
    pub(crate) fn run(display: &'a mut D, tok: RequestCookie<R>) -> Self {
        match (mem::size_of::<R::Reply>(), display.checked()) {
            (0, false) => ResolveRequestFuture::FastPath,
            (0, true) => ResolveRequestFuture::Synchronizing {
                sf: SynchronizeFuture::run(display),
                tok,
            },
            _ => ResolveRequestFuture::Resolving(ResolveRequestRawFuture::run(
                display,
                tok.sequence(),
            )),
        }
    }
}

impl<'a, D: AsyncDisplay + ?Sized, R: Request> Future for ResolveRequestFuture<'a, D, R>
where
    R::Reply: Default,
{
    type Output = crate::Result<R::Reply>;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<crate::Result<R::Reply>> {
        macro_rules! std_try {
            ($e: expr) => {{
                match ($e) {
                    Ok(e) => e,
                    Err(e) => return Poll::Ready(Err(e)),
                }
            }};
        }

        match self {
            ResolveRequestFuture::FastPath => Poll::Ready(Ok(R::Reply::default())),
            ResolveRequestFuture::Synchronizing { sf, tok } => match sf.poll(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
                Poll::Ready(Ok(())) => {
                    // check for errors and remove the pending request
                    let seq = tok.sequence();
                    let display = sf.display();
                    display.take_pending_request(seq);
                    std_try!(display.check_for_pending_error());

                    Poll::Ready(Ok(R::Reply::default()));
                }
            },
            ResolveRequestFuture::Resolving(rrrf) => match rrrf.poll(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
                Poll::Ready(Ok(PendingReply { data, fds })) => {
                    Poll::Ready(decode_reply::<R>(&data, fds))
                }
            },
        }
    }
}
