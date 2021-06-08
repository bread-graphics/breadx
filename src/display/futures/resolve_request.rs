// MIT/Apache2 License

use super::{ResolveRequestRawFuture, SynchronizeFuture};
use crate::{
    display::{decode_reply, AsyncDisplay, PendingReply, RequestCookie},
    util::take_mut,
    Request,
};
use core::{
    future::Future,
    marker::PhantomData,
    mem,
    pin::Pin,
    task::{Context, Poll},
};
use futures_lite::prelude::*;

/// The future returned by `AsyncDisplayExt::resolve_request_async`.
#[derive(Debug)]
#[must_use = "futures do nothing unless you poll or .await them"]
pub enum ResolveRequestFuture<'a, D: ?Sized, R: Request> {
    /// We can fast-path to the answer, since we aren't checked.
    #[doc(hidden)]
    FastPath { display: &'a mut D },
    /// The reply type is zero sized, and the display is currently synchronizing.
    #[doc(hidden)]
    Synchronizing {
        sf: SynchronizeFuture<'a, D>,
        tok: RequestCookie<R>,
    },
    /// The reply type is not zero typed, so we are calling the raw function.
    #[doc(hidden)]
    Resolving {
        rrrf: ResolveRequestRawFuture<'a, D>,
    },
    /// We've completed.
    #[doc(hidden)]
    Complete { display: &'a mut D },
    #[doc(hidden)]
    Hole,
}

impl<'a, D: ?Sized, R: Request> Default for ResolveRequestFuture<'a, D, R> {
    #[inline]
    fn default() -> Self {
        Self::Hole
    }
}
impl<'a, D: ?Sized, R: Request> Unpin for ResolveRequestFuture<'a, D, R> {}

impl<'a, D: AsyncDisplay + ?Sized, R: Request> ResolveRequestFuture<'a, D, R> {
    #[inline]
    pub(crate) fn run(display: &'a mut D, tok: RequestCookie<R>) -> Self {
        log::info!(
            "Resolving for a {} from the server",
            core::any::type_name::<R>()
        );

        match (mem::size_of::<R::Reply>(), display.checked()) {
            (0, false) => ResolveRequestFuture::FastPath { display },
            (0, true) => ResolveRequestFuture::Synchronizing {
                sf: SynchronizeFuture::run(display),
                tok,
            },
            _ => ResolveRequestFuture::Resolving {
                rrrf: ResolveRequestRawFuture::run(display, tok.sequence()),
            },
        }
    }

    #[inline]
    pub(crate) fn cannibalize(self) -> &'a mut D {
        match self {
            ResolveRequestFuture::FastPath { display } => display,
            ResolveRequestFuture::Complete { display } => display,
            ResolveRequestFuture::Synchronizing { sf, .. } => sf.cannibalize(),
            ResolveRequestFuture::Resolving { rrrf } => rrrf.cannibalize(),
            ResolveRequestFuture::Hole => panic!("Cannot cannibalize an empty hole"),
        }
    }
}

impl<'a, D: AsyncDisplay + ?Sized, R: Request> Future for ResolveRequestFuture<'a, D, R>
where
    R::Reply: Default,
{
    type Output = crate::Result<R::Reply>;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<crate::Result<R::Reply>> {
        let mut result = Poll::Pending;
        take_mut(&mut *self, |this| match this {
            ResolveRequestFuture::FastPath { display } => {
                result = Poll::Ready(Ok(R::Reply::default()));
                ResolveRequestFuture::Complete { display }
            }
            ResolveRequestFuture::Synchronizing { mut sf, tok } => match sf.poll(cx) {
                Poll::Pending => ResolveRequestFuture::Synchronizing { sf, tok },
                Poll::Ready(Err(e)) => {
                    result = Poll::Ready(Err(e));
                    ResolveRequestFuture::Complete {
                        display: sf.cannibalize(),
                    }
                }
                Poll::Ready(Ok(())) => {
                    let seq = tok.sequence();
                    let display = sf.cannibalize();
                    display.take_pending_request(seq);
                    result = match display.check_for_pending_error(seq) {
                        Ok(()) => Poll::Ready(Ok(R::Reply::default())),
                        Err(e) => Poll::Ready(Err(e)),
                    };

                    ResolveRequestFuture::Complete { display }
                }
            },
            ResolveRequestFuture::Resolving { mut rrrf } => match rrrf.poll(cx) {
                Poll::Pending => ResolveRequestFuture::Resolving { rrrf },
                Poll::Ready(Err(e)) => {
                    result = Poll::Ready(Err(e));
                    ResolveRequestFuture::Complete {
                        display: rrrf.cannibalize(),
                    }
                }
                Poll::Ready(Ok(PendingReply { data, fds })) => {
                    result = Poll::Ready(decode_reply::<R>(&data, fds));
                    ResolveRequestFuture::Complete {
                        display: rrrf.cannibalize(),
                    }
                }
            },
            ResolveRequestFuture::Complete { .. } => {
                panic!("Attempted to poll future past completion")
            }
            ResolveRequestFuture::Hole => panic!("Cannot pole an empty hole"),
        });
        result
    }
}
