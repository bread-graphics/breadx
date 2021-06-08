// MIT/Apache2 License

use crate::display::AsyncDisplay;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// The future created by the `AsyncDisplayExt::wait_async` method; runs the `poll_wait` function until
/// it returns `Ready`.
#[derive(Debug)]
#[must_use = "futures do nothing unless you poll or .await them"]
pub struct WaitFuture<'a, D: ?Sized> {
    display: &'a mut D,
    finished: bool,
}

// both &mut _ and bool are Inpin
impl<'a, D: ?Sized> Unpin for WaitFuture<'a, D> {}

impl<'a, D: ?Sized> WaitFuture<'a, D> {
    #[inline]
    pub(crate) fn run(display: &'a mut D) -> Self {
        Self {
            display,
            finished: false,
        }
    }

    /// Consumes the future and returns the display we are currently waiting on.
    #[inline]
    pub(crate) fn cannibalize(self) -> &'a mut D {
        self.display
    }
}

impl<'a, D: AsyncDisplay + ?Sized> Future for WaitFuture<'a, D> {
    type Output = crate::Result;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<crate::Result> {
        if self.finished {
            panic!("Attempted to poll future more than once");
        }
        let res = self.display.poll_wait(cx);
        if res.is_ready() {
            self.finished = true;
        }
        res
    }
}
