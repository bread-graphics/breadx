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
use x11rb_protocol::protocol::Event;

/// The future returned by the `generate_xid` function.
pub struct GenerateXid<'this, Dpy: ?Sized> {
    innards: TryWithDyn<'this, u32, Dpy>,
}

impl<'this, Dpy: AsyncDisplay + ?Sized> GenerateXid<'this, Dpy> {
    pub(crate) fn polling(display: &'this mut Dpy) -> Self {
        let func: Box<
            dyn FnMut(&mut Dpy, &mut Context<'_>) -> Result<AsyncStatus<u32>> + Send + 'static,
        > = Box::new(|display, ctx| display.try_generate_xid(ctx));
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
