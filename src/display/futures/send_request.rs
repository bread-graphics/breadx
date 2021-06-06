// MIT/Apache2 License

use super::SendRequestRawFuture;
use crate::{
    display::{decode_reply, AsyncDisplay, RequestCookie, RequestInfo},
    Request,
};
use core::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use futures_lite::prelude::*;

/// The future returned by the `AsyncDisplayExt::send_request_async` method. It is a basic wrapper around
/// sending the raw request.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled or .awaited"]
pub struct SendRequestFuture<'a, D: ?Sized, R> {
    inner: SendRequestRawFuture<'a, D>,
    _phantom: PhantomData<Option<R>>,
}

impl<'a, D: AsyncDisplay + ?Sized, R: Request> SendRequestFuture<'a, D, R> {
    #[inline]
    pub(crate) fn run(display: &mut D, request: R) -> Self {
        Self {
            inner: SendRequestRawFuture::run(display, RequestInfo::from_request(request), true),
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub(crate) fn display(&mut self) -> &mut D {
        self.inner.display()
    }
}

impl<'a, D: AsyncDisplay + ?Sized, R: Request> Future for SendRequestFuture<'a, D, R> {
    type Output = crate::Result<RequestCookie<R>>;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<crate::Result<RequestCookie<R>>> {
        self.inner.poll(cx).map_ok(RequestCookie::from_sequence)
    }
}
