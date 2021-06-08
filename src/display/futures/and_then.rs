// MIT/Apache2 License

use super::ExchangeRequestFuture;
use crate::{
    display::{generate_xid, AsyncDisplay},
    util::take_mut,
    xid::XidType,
    Request,
};
use core::{
    future::Future,
    mem,
    pin::Pin,
    task::{Context, Poll},
};
use futures_lite::prelude::*;

/// An `ExchangeRequestFuture`, but it potentially returns an XID.
#[derive(Debug)]
#[must_use = "futures do nothing unless you poll or .await them"]
pub enum ExchangeXidFuture<'a, D: ?Sized, R: Request, U, F> {
    /// We need to generate an XID.
    #[doc(hidden)]
    GeneratingXid { display: &'a mut D, to_request: F },
    /// We're using that XID to run an `ExchangeRequestFuture`.
    #[doc(hidden)]
    Exchanging {
        exchange: ExchangeRequestFuture<'a, D, R>,
        xid: U,
    },
    /// The future has been completed.
    #[doc(hidden)]
    Complete,
    #[doc(hidden)]
    Hole,
}

impl<'a, D: ?Sized, R: Request, U, F> Default for ExchangeXidFuture<'a, D, R, U, F> {
    #[inline]
    fn default() -> Self {
        Self::Hole
    }
}

impl<'a, D: ?Sized, R: Request, U, F> ExchangeXidFuture<'a, D, R, U, F> {
    #[inline]
    pub(crate) fn run(display: &'a mut D, to_request: F) -> Self {
        ExchangeXidFuture::GeneratingXid {
            display,
            to_request,
        }
    }
}

impl<
        'a,
        D: AsyncDisplay + ?Sized,
        R: Request + Unpin + 'a,
        U: XidType + Unpin,
        F: FnOnce(U) -> R + Unpin,
    > Future for ExchangeXidFuture<'a, D, R, U, F>
where
    R::Reply: Default + Unpin,
{
    type Output = crate::Result<U>;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<crate::Result<U>> {
        let mut result = None;
        loop {
            take_mut(&mut *self, |this| match this {
                ExchangeXidFuture::Complete => panic!("Attempted to poll future past completion"),
                ExchangeXidFuture::Hole => panic!("Cannot pole an empty hole"),
                ExchangeXidFuture::GeneratingXid {
                    display,
                    to_request,
                } => {
                    let xid = match generate_xid(display) {
                        Ok(xid) => xid,
                        Err(e) => {
                            result = Some(Poll::Ready(Err(e)));
                            return ExchangeXidFuture::Complete;
                        }
                    };
                    let request: R = to_request(U::from_xid(xid));
                    let exchange = ExchangeRequestFuture::run(display, request);
                    ExchangeXidFuture::Exchanging {
                        exchange,
                        xid: U::from_xid(xid),
                    }
                }
                ExchangeXidFuture::Exchanging { mut exchange, xid } => match exchange.poll(cx) {
                    Poll::Pending => {
                        result = Some(Poll::Pending);
                        ExchangeXidFuture::Exchanging { exchange, xid }
                    }
                    Poll::Ready(Err(e)) => {
                        result = Some(Poll::Ready(Err(e)));
                        ExchangeXidFuture::Complete
                    }
                    Poll::Ready(Ok(..)) => {
                        result = Some(Poll::Ready(Ok(xid)));
                        ExchangeXidFuture::Complete
                    }
                },
            });

            if let Some(result) = result.take() {
                return result;
            }
        }
    }
}
