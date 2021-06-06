// MIT/Apache2 License

use super::WaitFuture;
use crate::display::{AsyncDisplay, PendingReply};
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// A way to handle a wait loop.
#[doc(hidden)]
pub trait WaitLoopHandler {
    type Output;

    fn handle<D: AsyncDisplay + ?Sized>(display: &mut &mut D) -> Option<Self::Output>;
}

/// A future where the end result is to loop until an object is present, by waiting.
#[derive(Debug)]
#[doc(hidden)]
#[must_use = "futures do nothing unless you poll or .await them"]
pub struct WaitLoopFuture<'a, D: ?Sized, Handler> {
    inner: Inner<'a, D>,
    handler: Handler,
    completed: bool,
}

enum Inner<'a, D: ?Sized> {
    Waiter(WaitFuture<'a, D>),
    FirstTake(Option<&mut D>),
}

impl<'a, D: ?Sized, Handler> WaitLoopFuture<'a, D> {
    #[inline]
    pub(crate) fn construct(display: &mut D, handler: Handler) -> Self {
        Self {
            inner: Inner::FirstTake(display),
            handler,
            completed: bool,
        }
    }

    #[inline]
    pub(crate) fn display(&mut self) -> &mut D {
        match this.inner {
            Inner::Waiter(w) => w.display(),
            Inner::FirstTake(d) => d.take().expect("Display was already taken"),
        }
    }
}

impl<'a, D: AsyncDisplay + ?Sized, Handler: WaitLoopHandler + Unpin> Future
    for WaitLoopFuture<'a, D>
{
    type Output = crate::Result<Handler::Output>;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.completed {
            panic!("Attempted to poll future after completion");
        }

        loop {
            let display = match &mut self.inner {
                Inner::FirstTake(display) => {
                    let mut display = display.take().expect("Display was taken");

                    if let Some(output) = self.handler.handle(&mut display) {
                        self.completed = true;
                        break Poll::Ready(Ok(output));
                    }

                    display
                }
                Inner::Waiter(wf) => match wf.poll(cx) {
                    Poll::Pending => break Poll::Pending,
                    Poll::Ready(Err(e)) => {
                        self.completed = true;
                        break Poll::Ready(Err(e));
                    }
                    Poll::Ready(Ok(())) => {
                        let mut display = wf.display();
                        if let Some(output) = self.handler.handle(&mut display) {
                            self.completed = true;
                            break Poll::Ready(Ok(output));
                        }
                        display
                    }
                },
            };

            // begin waiting if we haven't already
            self.inner = Inner::Waiter(WaitFuture::run(display));
        }
    }
}
