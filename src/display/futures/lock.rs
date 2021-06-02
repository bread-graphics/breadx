// MIT/Apache2 License

use crate::display::AsyncDisplay;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// The future returned by `AsyncDisplayExt::Lock`.
#[derive(Debug)]
pub struct LockFuture<'a, D: ?Sized> {
    display: &'a mut D,
    is_finished: bool,
}

impl<'a, D: ?Sized> LockFuture<'a, D> {
    #[inline]
    pub(crate) fn run(display: &'a mut D) -> Self {
        Self {
            display,
            is_finished: false,
        }
    }
}

impl<'a, D: AsyncDisplay + ?Sized> Future for LockFuture<'a, D> {
    type Output = ();

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        self.display.poll_wait(cx)
    }
}
