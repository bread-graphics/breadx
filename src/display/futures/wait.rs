// MIT/Apache2 License

use crate::display::AsyncDisplay;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// The future created by the `AsyncDisplayExt::wait_async` method; polls the `poll_wait` function until
/// it returns `Ready`.
#[derive(Debug)]
#[must_use = "futures do nothing unless you poll or .await them"]
pub struct WaitFuture<'a, D: ?Sized> {
    display: Option<&'a mut D>,
    finished: bool,
}

impl<'a, D: ?Sized> WaitFuture<'a, D> {
    #[inline]
    pub(crate) fn run(display: &'a mut D) -> Self {
        Self {
            display: Some(display),
            finished: false,
        }
    }

    /// Returns the display we are currently waiting on, but disables the future.
    #[inline]
    pub(crate) fn display(&mut self) -> &'a mut D {
        self.display.take().expect("Display was already taken")
    }
}

impl<'a, D: AsyncDisplay + ?Sized> Future for WaitFuture<'a, D> {
    type Output = crate::Result;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<crate::Result> {
        if self.finished {
            panic!("Attempted to poll future more than once");
        }
        let res = self
            .display
            .as_mut()
            .expect("Display was taken")
            .poll_wait(cx);
        if res.is_ready() {
            self.finished = true;
        }
        res
    }
}
