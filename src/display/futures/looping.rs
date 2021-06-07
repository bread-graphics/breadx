// MIT/Apache2 License

use super::WaitFuture;
use crate::display::{AsyncDisplay, PendingReply};
use core::{
    future::Future,
    mem,
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
    FirstTake(Option<&'a mut D>),
    Complete(Option<&'a mut D>),
}

impl<'a, D: ?Sized, Handler> WaitLoopFuture<'a, D, Handler> {
    #[inline]
    pub(crate) fn construct(display: &'a mut D, handler: Handler) -> Self {
        Self {
            inner: Inner::FirstTake(Some(display)),
            handler,
        }
    }

    #[inline]
    pub(crate) fn display(&mut self) -> &'a mut D {
        match &mut self.inner {
            Inner::Waiter(w) => w.display(),
            Inner::FirstTake(d) => d.take().expect("Display was already taken"),
            Inner::Complete(d) => d.take().expect("Display was already taken"),
        }
    }
}

impl<'a, D: AsyncDisplay + ?Sized, Handler: WaitLoopHandler + Unpin> Future
    for WaitLoopFuture<'a, D, Handler>
{
    type Output = crate::Result<Handler::Output>;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            let display = match mem::replace(&mut self.inner, Inner::Complete(None)) {
                Inner::FirstTake(display) => {
                    let mut display = display.expect("Display was taken");

                    if let Some(output) = self.handler.handle(&mut display) {
                        self.inner = Inner::Complete(Some(display));
                        break Poll::Ready(Ok(output));
                    }

                    display
                }
                Inner::Waiter(mut wf) => match wf.poll(cx) {
                    Poll::Pending => break Poll::Pending,
                    Poll::Ready(Err(e)) => {
                        break Poll::Ready(Err(e));
                    }
                    Poll::Ready(Ok(())) => {
                        let mut display = wf.display();
                        if let Some(output) = self.handler.handle(&mut display) {
                            self.inner = Inner::Complete(Some(display));
                            break Poll::Ready(Ok(output));
                        }
                        display
                    }
                },
                Inner::Complete(..) => panic!("Attempted to poll future past completion"),
            };

            // begin waiting if we haven't already
            self.inner = Inner::Waiter(WaitFuture::run(display));
        }
    }
}
