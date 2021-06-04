// MIT/Apache2 License

use super::{ResolveRequestFuture, SendRequestFuture};
use crate::{
    auto::xproto::PutImageRequest,
    display::{AsyncDisplay, RequestToken},
};
use alloc::vec::{IntoIter as VecIter, Vec};
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use futures_lite::prelude::*;

/// The future returned by the `put_image_async` function.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled or .awaited"]
pub enum PutImageFuture<'a, D: ?Sized, I: IntoIterator> {
    /// We haven't been polled yet.
    #[doc(hidden)]
    AwaitingPoll { display: &'a mut D, requests: I },
    /// We are beginning to poll these requests.
    #[doc(hidden)]
    PollingRequests {
        remaining: I::IntoIter,
        inner: SendRequestFuture<'a, D, PutImageRequest>,
        tokens: Vec<RequestToken<PutImageRequest>>,
    },
    /// We have sent all the requests, now collect them.
    #[doc(hidden)]
    ResolvingRequests {
        remaining: VecIter<RequestToken<PutImageRequest>>,
        inner: ResolveRequestFuture<'a, D, PutImageRequest>,
    },
    /// Edge case: no requests, but we are complete... or we've error'd out.
    #[doc(hidden)]
    Complete,
}

impl<'a, D: ?Sized, I: IntoIterator> PutImageFuture<'a, D, I> {
    #[inline]
    pub(crate) fn run(display: &'a mut D, requests: I) -> Self {
        Self::AwaitingPoll { display, requests }
    }
}

impl<'a, D: AsyncDisplay + ?Sized, I: IntoIterator<Item = PutImageRequest> + Unpin> Future
    for PutImageFuture<'a, D, I>
{
    type Output = crate::Result;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<crate::Result> {
        match mem::replace(&mut *self, PutImageFuture::Complete) {
            PutImageFuture::AwaitingPoll { display, requests } => {
                let mut requests = requests.into_iter();
                let first_request = match requests.next() {
                    Some(first_request) => first_request,
                    None => return Poll::Ready(Ok(())),
                };

                let inner = SendRequestFuture::run(display, first_request);
                let remaining_len = requests.size_hint().0;
                *self = PutImageFuture::PollingRequests {
                    inner,
                    remaining: requests,
                    tokens: Vec::with_capacity(remaining_len + 1),
                };
            }
            PutImageFuture::PollingRequests {
                mut remaining,
                mut inner,
                mut tokens,
            } => {
                // see if we are finished
                match inner.poll(cx) {
                    Poll::Pending => {
                        *self = PutImageFuture::PollingRequests {
                            remaining,
                            inner,
                            tokens,
                        };
                        return Poll::Pending;
                    }
                    Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                    Poll::Ready(Ok(tok)) => {
                        tokens.push(tok);
                    }
                }

                // if we've finished with the current request, pull another request and poll it
                let display = inner.display();
                match remaining.next() {
                    Some(request) => {
                        *self = PutImageFuture::PollingRequests {
                            remaining,
                            inner: SendRequestFuture::run(display, request),
                            tokens,
                        };
                    }
                    None => {
                        let mut tok = tokens.pop();
                        *self = PutImageFuture::ResolvingRequests {
                            tokens: tokens.into_iter(),
                            inner: ResolveRequestFuture::run(display, tok),
                        };
                        // TODO: might be worth it to collaboratively give up a time slice here
                    }
                }
            }
            PutImageFuture::ResolvingRequests {
                mut tokens,
                mut inner,
            } => {
                match inner.poll(cx) {
                    Poll::Pending => {
                        *self = PutImageFuture::ResolvingRequests { tokens, inner };
                        return Poll::Pending;
                    }
                    Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                    Poll::Ready(Ok(())) => {}
                }

                let display = inner.display();
                match tokens.next() {
                    Some(tok) => {
                        *self = PutImageFuture::ResolvingRequests {
                            tokens,
                            inner: ResolveRequestFuture::run(display, tok),
                        };
                    }
                    None => return Poll::Ready(Ok(())),
                }
            }
            PutImageFuture::Complete => panic!("Attempted to poll future after completion"),
        }
    }
}
