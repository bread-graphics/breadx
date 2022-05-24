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

/// The future returned by the `generate_xid` function.
pub struct GenerateXid<'this, Dpy: ?Sized> {
    innards: TryWithDyn<'this, u32, Dpy>,
}

type FnTy = Box<
            dyn FnMut(&mut dyn AsyncDisplay, &mut Context<'_>) -> Result<AsyncStatus<u32>> + Send + 'static,
        >;

impl<'this, Dpy: AsyncDisplay + ?Sized> GenerateXid<'this, Dpy> {
    #[allow(clippy::redundant_closure_for_method_calls)]
    pub(crate) fn polling(display: &'this mut Dpy) -> Self {
        let func: FnTy = Box::new(|display, ctx| display.try_generate_xid(ctx));
        let try_with = display.try_with(func);

        Self { innards: try_with }
    }
}

impl<'this, Dpy: AsyncDisplay + ?Sized> Future for GenerateXid<'this, Dpy> {
    type Output = Result<u32>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<u32>> {
        let this = self.get_mut();
        this.innards.poll_unpin(ctx)
    }
}
