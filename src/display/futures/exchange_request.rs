// MIT/Apache2 License

use super::{ResolveRequestFuture, SendRequestFuture};
use crate::{
    display::{AsyncDisplay, RequestCookie},
    util::take_mut,
    Request,
};
use core::{
    future::Future,
    mem,
    pin::Pin,
    task::{Context, Poll},
};
use futures_lite::prelude::*;

/// The future returned by the `AsyncDisplayExt::request_exchange_async` function.
#[derive(Debug)]
#[must_use = "futures do nothing unless you poll or .await them"]
pub enum ExchangeRequestFuture<'a, D: ?Sized, R: Request> {
    /// Sending request...
    #[doc(hidden)]
    SendRequest(SendRequestFuture<'a, D, R>),
    /// Resolving request...
    #[doc(hidden)]
    ResolveRequest(ResolveRequestFuture<'a, D, R>),
    /// Request is complete.
    #[doc(hidden)]
    Complete,
    /// Request has been dug out and replaced with a hole.
    #[doc(hidden)]
    Hole,
}

impl<'a, D: ?Sized, R: Request> Default for ExchangeRequestFuture<'a, D, R> {
    #[inline]
    fn default() -> Self {
        Self::Hole
    }
}
impl<'a, D: ?Sized, R: Request> Unpin for ExchangeRequestFuture<'a, D, R> {}

impl<'a, D: AsyncDisplay + ?Sized, R: Request> ExchangeRequestFuture<'a, D, R> {
    #[inline]
    pub(crate) fn run(display: &'a mut D, request: R) -> Self {
        Self::SendRequest(SendRequestFuture::run(display, request))
    }
}

impl<'a, D: AsyncDisplay + ?Sized, R: Request + Unpin + 'a> Future
    for ExchangeRequestFuture<'a, D, R>
where
    R::Reply: Default + Unpin,
{
    type Output = crate::Result<R::Reply>;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut result = None;
        loop {
            take_mut(&mut *self, |this| match this {
                ExchangeRequestFuture::SendRequest(mut srf) => match srf.poll(cx) {
                    Poll::Pending => {
                        result = Some(Poll::Pending);
                        ExchangeRequestFuture::SendRequest(srf)
                    }
                    Poll::Ready(Err(e)) => {
                        result = Some(Poll::Ready(Err(e)));
                        ExchangeRequestFuture::Complete
                    }
                    Poll::Ready(Ok(tok)) => ExchangeRequestFuture::ResolveRequest(
                        ResolveRequestFuture::run(srf.cannibalize(), tok),
                    ),
                },
                ExchangeRequestFuture::ResolveRequest(mut rrf) => match rrf.poll(cx) {
                    Poll::Pending => {
                        result = Some(Poll::Pending);
                        ExchangeRequestFuture::ResolveRequest(rrf)
                    }
                    Poll::Ready(res) => {
                        result = Some(Poll::Ready(res));
                        ExchangeRequestFuture::Complete
                    }
                },
                ExchangeRequestFuture::Complete => {
                    panic!("Attempted to poll future after completion")
                }
                ExchangeRequestFuture::Hole => panic!("Cannot pole an empty hole"),
            });

            if let Some(result) = result.take() {
                return result;
            }
        }
    }
}
