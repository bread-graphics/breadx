// MIT/Apache2 License

use super::WaitFuture;
use crate::{display::AsyncDisplay, util::take_mut};
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use futures_lite::prelude::*;

/// A way to handle a wait loop.
#[doc(hidden)]
pub trait WaitLoopHandler {
    type Output;

    fn handle<D: AsyncDisplay + ?Sized>(&self, display: &mut &mut D) -> Option<Self::Output>;
}

/// A future where the end result is to loop until an object is present, by waiting.
#[derive(Debug)]
#[doc(hidden)]
#[must_use = "futures do nothing unless you poll or .await them"]
pub struct WaitLoopFuture<'a, D: ?Sized, Handler> {
    inner: Inner<'a, D>,
    handler: Handler,
}

#[derive(Debug)]
enum Inner<'a, D: ?Sized> {
    Waiter(WaitFuture<'a, D>),
    FirstTake(&'a mut D),
    Complete(&'a mut D),
    Hole,
}

impl<'a, D: ?Sized> Default for Inner<'a, D> {
    #[inline]
    fn default() -> Self {
        Self::Hole
    }
}
impl<'a, D: ?Sized> Unpin for Inner<'a, D> {}
impl<'a, D: ?Sized, Handler: Unpin> Unpin for WaitLoopFuture<'a, D, Handler> {}

impl<'a, D: ?Sized, Handler> WaitLoopFuture<'a, D, Handler> {
    #[inline]
    pub(crate) fn construct(display: &'a mut D, handler: Handler) -> Self {
        Self {
            inner: Inner::FirstTake(display),
            handler,
        }
    }

    #[inline]
    pub(crate) fn cannibalize(self) -> &'a mut D {
        match self.inner {
            Inner::Waiter(w) => w.cannibalize(),
            Inner::FirstTake(d) => d,
            Inner::Complete(d) => d,
            Inner::Hole => panic!("Attempted to cannibalize an empty hole"),
        }
    }
}

// every WaitLoopHandler we define is Clone and Unpin
impl<'a, D: AsyncDisplay + ?Sized, Handler: WaitLoopHandler + Clone + Unpin> Future
    for WaitLoopFuture<'a, D, Handler>
{
    type Output = crate::Result<Handler::Output>;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let handler = self.handler.clone();
        let mut result = None;
        loop {
            take_mut(&mut self.inner, |inner| {
                macro_rules! check_for_handler {
                    ($display: expr, $handler: expr, $result: ident) => {{
                        if let Some(output) = ($handler).handle(&mut $display) {
                            $result = Some(Poll::Ready(Ok(output)));
                            return Inner::Complete($display);
                        };
                    }};
                }

                // check for the condition or wait until we can
                let display = match inner {
                    Inner::FirstTake(mut display) => {
                        check_for_handler!(display, handler, result);
                        display
                    }
                    Inner::Waiter(mut wf) => match wf.poll(cx) {
                        Poll::Pending => {
                            result = Some(Poll::Pending);
                            return Inner::Waiter(wf);
                        }
                        Poll::Ready(Err(e)) => {
                            result = Some(Poll::Ready(Err(e)));
                            return Inner::Complete(wf.cannibalize());
                        }
                        Poll::Ready(Ok(())) => {
                            let mut display = wf.cannibalize();
                            check_for_handler!(display, handler, result);
                            display
                        }
                    },
                    Inner::Complete(..) => panic!("Attempted to poll future past completion"),
                    Inner::Hole => panic!("Cannot pole an empty hole"),
                };

                Inner::Waiter(WaitFuture::run(display))
            });

            if let Some(result) = result.take() {
                return result;
            }
        }
    }
}
