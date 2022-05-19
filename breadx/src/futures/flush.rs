// MIT/Apache2 License

use super::TryWithDyn;
use crate::{
    display::{AsyncDisplay, AsyncDisplayExt, AsyncStatus},
    Result,
};
use alloc::boxed::Box;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use futures_util::FutureExt;

/// The future returned by the `flush()` function.
pub struct Flush<'this, Dpy: ?Sized> {
    innards: TryWithDyn<'this, (), Dpy>,
}

impl<'this, Dpy: AsyncDisplay + ?Sized> Flush<'this, Dpy> {
    #[allow(clippy::redundant_closure_for_method_calls)]
    pub(crate) fn polling(display: &'this mut Dpy) -> Self {
        let func: Box<
            dyn FnMut(&mut Dpy, &mut Context<'_>) -> Result<AsyncStatus<()>> + Send + 'static,
        > = Box::new(|display, ctx| display.try_flush(ctx));
        let try_with = display.try_with(func);

        Self { innards: try_with }
    }
}

impl<'this, Dpy: AsyncDisplay + ?Sized> Future for Flush<'this, Dpy> {
    type Output = Result<()>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<()>> {
        let this = self.get_mut();
        this.innards.poll_unpin(ctx)
    }
}
