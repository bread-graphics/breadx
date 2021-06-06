// MIT/Apache2 License

use super::{ResolveRequestFuture, SendRequestFuture};
use crate::{
    display::{AsyncDisplay, RequestCookie},
    Request,
};
use core::{
    future::Future,
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
}

impl<'a, D: AsyncDisplay + ?Sized, R: Request> ExchangeRequestFuture<'a, D, R> {
    #[inline]
    pub(crate) fn new(display: &mut D, request: R) -> Self {
        Self::SendRequest(SendRequestFuture::run(display, request))
    }
}

impl<'a, D: AsyncDisplay + ?Sized, R: Request> Future for ExchangeRequestFuture<'a, D, R>
where
    R::Reply: Default,
{
    type Output = crate::Result<R::Reply>;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match &mut *self {
                Self::SendRequest(srf) => match srf.poll(cx) {
                    Poll::Pending => return Poll::Pending,
                    Poll::Ready(Err(e)) => {
                        *self = Self::Complete;
                        return Poll::Ready(Err(e));
                    }
                    Poll::Ready(Ok(tok)) => {
                        let disp = srf.display();
                        *self = Self::ResolveRequest(ResolveRequestFuture::run(disp, tok));
                    }
                },
                Self::ResolveRequest(rrf) => match rrf.poll(cx) {
                    Poll::Pending => return Poll::Pending,
                    Poll::Ready(res) => {
                        *self = Self::Complete;
                        return Poll::Ready(res);
                    }
                },
                Self::Complete => panic!("Attempted to poll future after completion"),
            }
        }
    }
}
