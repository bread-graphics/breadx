// MIT/Apache2 License

use super::{ResolveRequestFuture, SendRequestFuture};
use crate::{
    display::{AsyncDisplay, RequestCookie},
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
}

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
        loop {
            match mem::replace(&mut *self, Self::Complete) {
                Self::SendRequest(mut srf) => match srf.poll(cx) {
                    Poll::Pending => {
                        *self = Self::SendRequest(srf);
                        return Poll::Pending;
                    }
                    Poll::Ready(Err(e)) => {
                        return Poll::Ready(Err(e));
                    }
                    Poll::Ready(Ok(tok)) => {
                        let disp = srf.display();
                        *self = Self::ResolveRequest(ResolveRequestFuture::run(disp, tok));
                    }
                },
                Self::ResolveRequest(mut rrf) => match rrf.poll(cx) {
                    Poll::Pending => {
                        *self = Self::ResolveRequest(rrf);
                        return Poll::Pending;
                    }
                    Poll::Ready(res) => {
                        return Poll::Ready(res);
                    }
                },
                Self::Complete => panic!("Attempted to poll future after completion"),
            }
        }
    }
}
